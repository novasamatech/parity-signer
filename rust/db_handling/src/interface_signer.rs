use bip39::{Language, Mnemonic};
use blake2_rfc::blake2b::blake2b;
use hex;
use parity_scale_codec::Encode;
use plot_icon::EMPTY_PNG;
use sp_core::{sr25519, Pair};
use sp_runtime::MultiSigner;
use std::collections::HashMap;
use zeroize::{Zeroize, ZeroizeOnDrop};

use constants::{HISTORY, MAX_WORDS_DISPLAY, TRANSACTION};
use definitions::{
    error::{AddressGenerationCommon, ErrorSource},
    error_signer::{DatabaseSigner, ErrorSigner, InterfaceSigner, NotFoundSigner, Signer},
    helpers::{
        make_identicon_from_multisigner, multisigner_to_encryption, multisigner_to_public,
        pic_meta, print_multisigner_as_base58,
    },
    keyring::{AddressKey, NetworkSpecsKey, VerifierKey},
    navigation::{
        Address, DerivationCheck as NavDerivationCheck, DerivationDestination, DerivationEntry,
        DerivationPack, MBackup, MDeriveKey, MKeyDetails, MKeysCard, MMMNetwork, MMManageNetworks,
        MMNetwork, MMetadataRecord, MNetworkDetails, MNetworkMenu, MNewSeedBackup, MRawKey,
        MSeedKeyCard, MTypesInfo, MVerifier, Network, SeedNameCard,
    },
    network_specs::{NetworkSpecs, ValidCurrentVerifier},
    print::export_plain_vector,
    qr_transfers::ContentLoadTypes,
    users::AddressDetails,
};
use qrcode_static::png_qr_from_string;

use crate::helpers::{
    get_address_details, get_general_verifier, get_meta_values_by_name,
    get_meta_values_by_name_version, get_network_specs, make_batch_clear_tree, open_db, open_tree,
    try_get_types,
};
use crate::identities::{
    derivation_check, generate_random_phrase, get_addresses_by_seed_name, get_all_addresses,
    DerivationCheck,
};
use crate::network_details::get_all_networks;
use crate::{db_transactions::TrDbCold, helpers::get_valid_current_verifier};

/// Function to print all seed names with identicons
/// Gets used only on the Signer side, interacts with the user interface.
pub fn get_all_seed_names_with_identicons(
    database_name: &str,
    names_phone_knows: &[String],
) -> Result<Vec<SeedNameCard>, ErrorSigner> {
    let mut data_set: HashMap<String, Vec<MultiSigner>> = HashMap::new();
    for (multisigner, address_details) in get_all_addresses(database_name)?.into_iter() {
        if address_details.is_root() {
            // found a root; could be any of the supported encryptions;
            match data_set.get(&address_details.seed_name) {
                Some(root_set) => {
                    for id in root_set.iter() {
                        if multisigner_to_encryption(id) == address_details.encryption {
                            return Err(ErrorSigner::Database(DatabaseSigner::TwoRootKeys {
                                seed_name: address_details.seed_name.to_string(),
                                encryption: address_details.encryption.to_owned(),
                            }));
                        }
                    }
                    let mut new_root_set = root_set.to_vec();
                    new_root_set.push(multisigner);
                    data_set.insert(address_details.seed_name.to_string(), new_root_set);
                }
                None => {
                    data_set.insert(address_details.seed_name.to_string(), vec![multisigner]);
                }
            }
        } else if data_set.get(&address_details.seed_name).is_none() {
            data_set.insert(address_details.seed_name.to_string(), Vec::new());
        }
    }
    for x in names_phone_knows.iter() {
        if data_set.get(x).is_none() {
            data_set.insert(x.to_string(), Vec::new());
        }
    }
    let mut res: Vec<_> = data_set
        .into_iter()
        .map(|(seed_name, multisigner_set)| SeedNameCard {
            seed_name,
            identicon: preferred_multisigner_identicon(&multisigner_set),
        })
        .collect();
    res.sort_by(|a, b| a.seed_name.cmp(&b.seed_name));
    Ok(res)
}

fn preferred_multisigner_identicon(multisigner_set: &[MultiSigner]) -> String {
    if multisigner_set.is_empty() {
        hex::encode(EMPTY_PNG)
    } else {
        let mut got_sr25519 = None;
        let mut got_ed25519 = None;
        let mut got_ecdsa = None;
        for x in multisigner_set.iter() {
            match x {
                MultiSigner::Ed25519(_) => got_ed25519 = Some(x.to_owned()),
                MultiSigner::Sr25519(_) => got_sr25519 = Some(x.to_owned()),
                MultiSigner::Ecdsa(_) => got_ecdsa = Some(x.to_owned()),
            }
        }
        if let Some(a) = got_sr25519 {
            hex::encode(make_identicon_from_multisigner(&a))
        } else if let Some(a) = got_ed25519 {
            hex::encode(make_identicon_from_multisigner(&a))
        } else if let Some(a) = got_ecdsa {
            hex::encode(make_identicon_from_multisigner(&a))
        } else {
            hex::encode(EMPTY_PNG)
        }
    }
}

/// Function to print all identities (seed names AND derication paths) with identicons
/// Gets used only on the Signer side, interacts with the user interface.
pub fn print_all_identities(database_name: &str) -> Result<Vec<MRawKey>, ErrorSigner> {
    Ok(get_all_addresses(database_name)?
        .into_iter()
        .map(|(multisigner, address_details)| {
            let address_key = AddressKey::from_multisigner(&multisigner); // to click
            let public_key = multisigner_to_public(&multisigner); // to display
            let identicon = hex::encode(make_identicon_from_multisigner(&multisigner));
            MRawKey {
                seed_name: address_details.seed_name,
                address_key: hex::encode(address_key.key()),
                public_key: hex::encode(public_key),
                identicon,
                has_pwd: address_details.has_pwd,
                path: address_details.path,
            }
        })
        .collect())
}

/// Function to print separately root identity and derived identities for given seed name and network specs key.
/// Is used only on the Signer side, interacts only with navigation.
pub fn print_identities_for_seed_name_and_network(
    database_name: &str,
    seed_name: &str,
    network_specs_key: &NetworkSpecsKey,
    swiped_key: Option<MultiSigner>,
    multiselect: Vec<MultiSigner>,
) -> Result<(MSeedKeyCard, Vec<MKeysCard>, String, String), ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let identities = addresses_set_seed_name_network(database_name, seed_name, network_specs_key)?;
    let mut root_id = None;
    let mut other_id: Vec<(MultiSigner, AddressDetails, Vec<u8>, bool, bool)> = Vec::new();
    for (multisigner, address_details) in identities.into_iter() {
        let identicon = make_identicon_from_multisigner(&multisigner);
        let base58 = print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix));
        let address_key = AddressKey::from_multisigner(&multisigner);
        let swiped = {
            if let Some(ref swiped_multisigner) = swiped_key {
                swiped_multisigner == &multisigner
            } else {
                false
            }
        };
        let multiselect = multiselect.contains(&multisigner);
        if address_details.is_root() {
            println!("ISROOT {}", seed_name);
            if root_id.is_some() {
                return Err(ErrorSigner::Database(DatabaseSigner::TwoRootKeys {
                    seed_name: seed_name.to_string(),
                    encryption: network_specs.encryption,
                }));
            }
            root_id = Some(MSeedKeyCard {
                seed_name: seed_name.to_string(),
                identicon: hex::encode(identicon),
                address_key: hex::encode(address_key.key()),
                base58,
                swiped,
                multiselect,
            });
        } else {
            other_id.push((multisigner, address_details, identicon, swiped, multiselect))
        }
    }
    let root = root_id.unwrap_or_default();
    let set: Vec<_> = other_id
        .into_iter()
        .map(
            |(multisigner, address_details, identicon, swiped, multiselect)| MKeysCard {
                address_key: hex::encode(AddressKey::from_multisigner(&multisigner).key()),
                base58: print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix)),
                identicon: hex::encode(identicon),
                has_pwd: address_details.has_pwd,
                path: address_details.path,
                swiped,
                multiselect,
            },
        )
        .collect();

    Ok((root, set, network_specs.title, network_specs.logo))
}

/// Function to get addresses for given seed name and network specs key
pub fn addresses_set_seed_name_network(
    database_name: &str,
    seed_name: &str,
    network_specs_key: &NetworkSpecsKey,
) -> Result<Vec<(MultiSigner, AddressDetails)>, ErrorSigner> {
    Ok(get_addresses_by_seed_name(database_name, seed_name)?
        .into_iter()
        .filter(|(_, address_details)| address_details.network_id.contains(network_specs_key))
        .collect())
}

/// Function to print all networks, with bool indicator which one is currently selected
pub fn show_all_networks_with_flag(
    database_name: &str,
    network_specs_key: &NetworkSpecsKey,
) -> Result<MNetworkMenu, ErrorSigner> {
    let mut networks: Vec<_> = get_all_networks::<Signer>(database_name)?
        .into_iter()
        .map(|network| {
            let network_specs_key_current =
                NetworkSpecsKey::from_parts(network.genesis_hash.as_bytes(), &network.encryption);
            let mut n: Network = network.into();
            n.selected = network_specs_key == &network_specs_key_current;
            n
        })
        .collect();
    networks.sort_by(|a, b| a.order.cmp(&b.order));

    Ok(MNetworkMenu { networks })
}

/// Function to print all networks without any selection
pub fn show_all_networks(database_name: &str) -> Result<Vec<MMNetwork>, ErrorSigner> {
    let networks = get_all_networks::<Signer>(database_name)?;
    let mut networks = networks
        .into_iter()
        .map(|n| MMNetwork {
            key: hex::encode(
                NetworkSpecsKey::from_parts(&n.genesis_hash.as_bytes(), &n.encryption).key(),
            ),
            title: n.title,
            logo: n.logo,
            order: n.order,
        })
        .collect::<Vec<_>>();
    networks.sort_by(|a, b| a.order.cmp(&b.order));

    Ok(networks)
}

/// Function to sort networks by the order and get the network specs for the first network on the list.
/// If no networks in the system, throws error
pub fn first_network(database_name: &str) -> Result<NetworkSpecs, ErrorSigner> {
    let mut networks = get_all_networks::<Signer>(database_name)?;
    if networks.is_empty() {
        return Err(ErrorSigner::NoNetworksAvailable);
    }
    networks.sort_by(|a, b| a.order.cmp(&b.order));
    Ok(networks.remove(0))
}

/// Function to prepare the export key screen.
/// Contains among else the QR code with identity information, in format
/// `substrate:{public_key as as_base58}:0x{network_key}`,
/// this string is transformed into bytes, then into png qr code, then qr code
/// content is hexed so that it could be transferred into app.
pub fn export_key(
    database_name: &str,
    multisigner: &MultiSigner,
    expected_seed_name: &str,
    network_specs_key: &NetworkSpecsKey,
) -> Result<MKeyDetails, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let address_key = AddressKey::from_multisigner(multisigner);
    let address_details = get_address_details(database_name, &address_key)?;
    if address_details.seed_name != expected_seed_name {
        return Err(ErrorSigner::Interface(
            InterfaceSigner::SeedNameNotMatching {
                address_key,
                expected_seed_name: expected_seed_name.to_string(),
                real_seed_name: address_details.seed_name,
            },
        ));
    }
    let base58 = print_multisigner_as_base58(multisigner, Some(network_specs.base58prefix));
    let public_key = multisigner_to_public(multisigner);
    let identicon = make_identicon_from_multisigner(multisigner);
    let qr_prep = {
        if address_details.network_id.contains(network_specs_key) {
            match png_qr_from_string(&format!(
                "substrate:{}:0x{}",
                base58,
                hex::encode(&network_specs.genesis_hash)
            )) {
                Ok(a) => a,
                Err(e) => return Err(ErrorSigner::Qr(e.to_string())),
            }
        } else {
            return Err(ErrorSigner::NotFound(
                NotFoundSigner::NetworkSpecsKeyForAddress {
                    network_specs_key: network_specs_key.to_owned(),
                    address_key,
                },
            ));
        }
    };
    Ok(MKeyDetails {
        qr: hex::encode(qr_prep),
        pubkey: hex::encode(public_key),
        base58,
        identicon: hex::encode(identicon),
        seed_name: address_details.seed_name,
        path: address_details.path,
        network_title: network_specs.title,
        network_logo: network_specs.logo,
    })
}

/// Function to prepare seed backup screen.
/// Gets seed name, outputs all known derivations in all networks.
pub fn backup_prep(database_name: &str, seed_name: &str) -> Result<MBackup, ErrorSigner> {
    let networks = get_all_networks::<Signer>(database_name)?;
    if networks.is_empty() {
        return Err(ErrorSigner::NoNetworksAvailable);
    }
    let mut derivations = Vec::new();
    for network in networks.into_iter() {
        let id_set: Vec<_> = addresses_set_seed_name_network(
            database_name,
            seed_name,
            &NetworkSpecsKey::from_parts(network.genesis_hash.as_bytes(), &network.encryption),
        )?
        .into_iter()
        .map(|a| DerivationEntry {
            path: a.1.path,
            has_pwd: a.1.has_pwd,
        })
        .collect();
        if !id_set.is_empty() {
            derivations.push(DerivationPack {
                network_title: network.title,
                network_logo: network.logo,
                network_order: network.order.to_string(),
                id_set,
            });
        }
    }

    derivations.sort_by(|a, b| a.network_order.cmp(&b.network_order));

    Ok(MBackup {
        seed_name: seed_name.to_string(),
        derivations,
    })
}

/// Function to prepare key derivation screen.
/// Gets seed name, network specs key and suggested derivation
pub fn derive_prep(
    database_name: &str,
    seed_name: &str,
    network_specs_key: &NetworkSpecsKey,
    collision: Option<(MultiSigner, AddressDetails)>,
    suggest: &str,
    keyboard: bool,
) -> Result<MDeriveKey, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let _collision = collision.map(|(multisigner, address_details)| {
        let base58 = print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix));
        let path = address_details.path;
        let has_pwd = address_details.has_pwd;
        let identicon = hex::encode(make_identicon_from_multisigner(&multisigner));
        let seed_name = seed_name.to_string();
        Address {
            base58,
            path,
            has_pwd,
            identicon,
            seed_name,
            multiselect: None,
        }
    });

    Ok(MDeriveKey {
        seed_name: seed_name.to_string(),
        network_title: network_specs.title,
        network_logo: network_specs.logo,
        network_specs_key: hex::encode(network_specs_key.key()),
        suggested_derivation: suggest.to_string(),
        keyboard,
        derivation_check: None,
    })
}

/// Function to show (dynamically) if the derivation with the provided path and no password already exists
/// and if it exists, prints its details
pub fn dynamic_path_check(
    database_name: &str,
    seed_name: &str,
    path: &str,
    network_specs_key_hex: &str,
) -> NavDerivationCheck {
    let content = match NetworkSpecsKey::from_hex(network_specs_key_hex) {
        Ok(network_specs_key) => match get_network_specs(database_name, &network_specs_key) {
            Ok(network_specs) => {
                match derivation_check(seed_name, path, &network_specs_key, database_name) {
                    Ok(DerivationCheck::BadFormat) => NavDerivationCheck {
                        button_good: false,
                        ..Default::default()
                    },
                    Ok(DerivationCheck::Password) => NavDerivationCheck {
                        button_good: true,
                        where_to: Some(DerivationDestination::Pwd),
                        ..Default::default()
                    },
                    Ok(DerivationCheck::NoPassword(None)) => NavDerivationCheck {
                        button_good: true,
                        where_to: Some(DerivationDestination::Pin),
                        ..Default::default()
                    },
                    Ok(DerivationCheck::NoPassword(Some((multisigner, address_details)))) => {
                        let address_base58 = print_multisigner_as_base58(
                            &multisigner,
                            Some(network_specs.base58prefix),
                        );
                        let hex_identicon =
                            hex::encode(make_identicon_from_multisigner(&multisigner));
                        let collision_display = Address {
                            base58: address_base58,
                            path: address_details.path,
                            has_pwd: address_details.has_pwd,
                            identicon: hex_identicon,
                            seed_name: seed_name.to_string(),
                            multiselect: None,
                        };
                        NavDerivationCheck {
                            button_good: false,
                            collision: Some(collision_display),
                            ..Default::default()
                        }
                    }
                    Err(e) => NavDerivationCheck {
                        error: Some(<Signer>::show(&e)),
                        ..Default::default()
                    },
                }
            }
            Err(e) => NavDerivationCheck {
                error: Some(<Signer>::show(&e)),
                ..Default::default()
            },
        },
        Err(e) => NavDerivationCheck {
            error: Some(<Signer>::show(&e)),
            ..Default::default()
        },
    };
    content
}

/// Print network specs and metadata set information for network with given network specs key.
pub fn network_details_by_key(
    database_name: &str,
    network_specs_key: &NetworkSpecsKey,
) -> Result<MNetworkDetails, ErrorSigner> {
    let NetworkSpecs {
        base58prefix,
        color,
        decimals,
        encryption,
        genesis_hash,
        logo,
        name,
        order,
        path_id,
        secondary_color,
        title,
        unit,
    } = get_network_specs(database_name, network_specs_key)?;
    let verifier_key = VerifierKey::from_parts(genesis_hash.as_bytes());
    let general_verifier = get_general_verifier(database_name)?;
    let current_verifier = get_valid_current_verifier(&verifier_key, database_name)?;
    let meta: Vec<_> = get_meta_values_by_name::<Signer>(database_name, &name)?
        .into_iter()
        .map(|m| {
            let meta_hash = blake2b(32, &[], &m.meta).as_bytes().to_vec();
            let meta_id_pic = hex::encode(pic_meta(&meta_hash));

            MMetadataRecord {
                specs_version: m.version.to_string(),
                meta_hash: hex::encode(meta_hash),
                meta_id_pic,
            }
        })
        .collect();

    let (ttype, details) = match current_verifier {
        ValidCurrentVerifier::General => ("general".to_string(), general_verifier.show_card()),
        ValidCurrentVerifier::Custom { v } => ("custom".to_string(), v.show_card()),
    };
    let current_verifier = MVerifier { ttype, details };

    Ok(MNetworkDetails {
        base58prefix,
        color,
        decimals,
        encryption,
        genesis_hash: format!("{:x}", genesis_hash),
        logo,
        name,
        order: order.to_string(),
        path_id,
        secondary_color,
        title,
        unit,
        current_verifier,
        meta,
    })
}

/// Print metadata details for given network specs key and version.
pub fn metadata_details(
    database_name: &str,
    network_specs_key: &NetworkSpecsKey,
    network_version: u32,
) -> Result<MMManageNetworks, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let meta_values = get_meta_values_by_name_version::<Signer>(
        database_name,
        &network_specs.name,
        network_version,
    )?;
    let networks: Vec<_> = get_all_networks::<Signer>(database_name)?
        .into_iter()
        .filter(|a| a.name == network_specs.name)
        .map(|network| MMMNetwork {
            title: network.title,
            logo: network.logo,
            order: network.order as u32,
            current_on_screen: &NetworkSpecsKey::from_parts(
                network.genesis_hash.as_bytes(),
                &network.encryption,
            ) == network_specs_key,
        })
        .collect();

    let meta_hash = blake2b(32, &[], &meta_values.meta).as_bytes().to_vec();
    let hex_id_pic = hex::encode(pic_meta(&meta_hash));
    Ok(MMManageNetworks {
        name: network_specs.name,
        version: network_version.to_string(),
        meta_hash: hex::encode(meta_hash),
        meta_id_pic: hex_id_pic,
        networks,
    })
}

/// Display types status
pub fn show_types_status(database_name: &str) -> Result<MTypesInfo, ErrorSigner> {
    match try_get_types::<Signer>(database_name)? {
        Some(a) => {
            let (types_hash, types_id_pic) = ContentLoadTypes::generate(&a).show();
            Ok(MTypesInfo {
                types_on_file: true,
                types_hash: Some(types_hash),
                types_id_pic: Some(types_id_pic),
            })
        }
        None => Ok(MTypesInfo {
            types_on_file: false,
            types_hash: None,
            types_id_pic: None,
        }),
    }
}

/// Function to generate new random seed phrase, make identicon for sr25519 public key,
/// and send to Signer screen
pub fn print_new_seed(seed_name: &str) -> Result<MNewSeedBackup, ErrorSigner> {
    let seed_phrase = generate_random_phrase(24)?;
    let sr25519_pair = match sr25519::Pair::from_string(&seed_phrase, None) {
        Ok(x) => x,
        Err(e) => {
            return Err(<Signer>::address_generation_common(
                AddressGenerationCommon::SecretString(e),
            ))
        }
    };
    let identicon = hex::encode(make_identicon_from_multisigner(&MultiSigner::Sr25519(
        sr25519_pair.public(),
    )));
    Ok(MNewSeedBackup {
        seed: seed_name.to_string(),
        seed_phrase,
        identicon,
    })
}

/// Function to get database history tree checksum to be displayed in log screen
pub fn history_hex_checksum(database_name: &str) -> Result<String, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let history = open_tree::<Signer>(&database, HISTORY)?;
    let checksum = history
        .checksum()
        .map_err(|e| ErrorSigner::Database(DatabaseSigner::Internal(e)))?;
    Ok(format!(
        "\"checksum\":\"{}\"",
        hex::encode(checksum.encode()).to_uppercase()
    ))
}

/// Function to clear transaction tree of the database, for cases when transaction is declined by the user
/// (e.g. user has scanned something, read it, clicked `back`)
pub fn purge_transactions(database_name: &str) -> Result<(), ErrorSigner> {
    TrDbCold::new()
        .set_transaction(make_batch_clear_tree::<Signer>(database_name, TRANSACTION)?) // clear transaction
        .apply::<Signer>(database_name)
}

/// Function to display possible options of English code words from allowed words list
/// that start with already entered piece
pub fn guess(word_part: &str) -> Vec<&'static str> {
    let dictionary = Language::English.wordlist();
    let words = dictionary.get_words_by_prefix(word_part);
    if words.len() > MAX_WORDS_DISPLAY {
        words[..MAX_WORDS_DISPLAY].to_vec()
    } else {
        words.to_vec()
    }
}

/// Function to dynamically update suggested seed phrase words,
/// based on user entered word part.
/// Produces json-like export with maximum 8 fitting words, sorted by alphabetical order.
pub fn print_guess(user_entry: &str) -> String {
    export_plain_vector(&guess(user_entry))
}

pub const BIP_CAP: usize = 24; // maximum word count in bip39 standard, see https://docs.rs/tiny-bip39/0.8.2/src/bip39/mnemonic_type.rs.html#60
pub const WORD_LENGTH: usize = 8; // maximum word length in bip39 standard
pub const SAFE_RESERVE: usize = 1000; // string length to reserve for json output of numbered draft; each element is {"order":**,"content":"********"}, at most 33+1(comma) symbols for each max BIP_CAP elements, two extras for [ and ], and some extra space just in case

/// Struct to store seed draft, as entered by user
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct SeedDraft {
    user_input: String,
    saved: Vec<SeedElement>,
}

/// Struct to store seed element, as entered by user
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
struct SeedElement(String);

impl SeedElement {
    fn from_checked_str(word: &str) -> Self {
        let mut new = String::with_capacity(WORD_LENGTH);
        new.push_str(word);
        SeedElement(new)
    }
    fn word(&self) -> &str {
        &self.0
    }
}

impl SeedDraft {
    pub fn initiate() -> Self {
        Self {
            user_input: String::with_capacity(WORD_LENGTH), // capacity corresponds to maximum word length in bip39 standard;
            saved: Vec::with_capacity(BIP_CAP), // capacity corresponds to maximum word count in bip39 standard; set here to avoid reallocation;
        }
    }
    pub fn text_field_update(&mut self, user_text: &str) {
        if self.saved.len() < BIP_CAP {
            if user_text.is_empty() {
                // user has removed all text, including the first default symbol
                // if there are words in draft, remove the last one
                self.remove_last();
                // restore the user input to emtpy one
                self.user_input.clear();
            } else {
                let user_text = user_text.trim_start();
                if user_text.ends_with(' ') {
                    let word = user_text.trim();
                    if self.added(word, None) {
                        self.user_input.clear()
                    } else if !guess(word).is_empty() {
                        self.user_input = String::from(word)
                    }
                } else if !guess(user_text).is_empty() {
                    self.user_input = String::from(user_text)
                }
            }
        } else {
            self.user_input.clear()
        }
    }

    pub fn added(&mut self, word: &str, position: Option<u32>) -> bool {
        if self.saved.len() < BIP_CAP {
            let guesses = guess(word);
            let definitive_guess = {
                if guesses.len() == 1 {
                    Some(guesses[0])
                } else if guesses.contains(&word) {
                    Some(word)
                } else {
                    None
                }
            };
            if let Some(guess) = definitive_guess {
                let new = SeedElement::from_checked_str(guess);
                match position {
                    Some(p) => {
                        let p = p as usize;
                        if p <= self.saved.len() {
                            self.saved.insert(p, new)
                        } else {
                            self.saved.push(new)
                        }
                    }
                    None => self.saved.push(new),
                }
                self.user_input.clear();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn remove(&mut self, position: u32) {
        let position = position as usize;
        if position < self.saved.len() {
            self.saved.remove(position);
        }
    }

    pub fn remove_last(&mut self) {
        if !self.saved.is_empty() {
            self.saved.remove(self.saved.len() - 1);
        }
    }

    pub fn print(&self) -> String {
        let mut out = String::with_capacity(SAFE_RESERVE);
        out.push('[');
        for (i, x) in self.saved.iter().enumerate() {
            if i > 0 {
                out.push(',')
            }
            out.push_str(&format!("{{\"order\":{},\"content\":\"", i));
            out.push_str(x.word());
            out.push_str("\"}");
        }
        out.push(']');
        out
    }

    pub fn draft(&self) -> Vec<String> {
        self.saved.iter().map(|w| w.word().to_string()).collect()
    }

    /// Combines all draft elements into seed phrase proposal,
    /// and checks its validity.
    /// If valid, outputs secret seed phrase.
    pub fn try_finalize(&self) -> Option<Vec<String>> {
        let mut seed_phrase_proposal = String::with_capacity((WORD_LENGTH + 1) * BIP_CAP);
        for (i, x) in self.saved.iter().enumerate() {
            if i > 0 {
                seed_phrase_proposal.push(' ');
            }
            seed_phrase_proposal.push_str(x.word());
        }
        if Mnemonic::validate(&seed_phrase_proposal, Language::English).is_ok() {
            Some(
                seed_phrase_proposal
                    .split_whitespace()
                    .map(|content| content.to_string())
                    .collect(),
            )
        } else {
            seed_phrase_proposal.zeroize();
            None
        }
    }

    /// Function to output the user input back into user interface
    pub fn user_input(&self) -> &str {
        &self.user_input
    }
}
