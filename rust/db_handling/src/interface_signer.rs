//! Utils to communicate with the Signer frontend
use bip39::{Language, Mnemonic};
use hex;
use parity_scale_codec::Encode;
use plot_icon::EMPTY_PNG;
use sp_core::{blake2_256, sr25519, Pair};
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
        DerivationPack, MBackup, MDeriveKey, MKeyDetails, MKeysCard, MMMNetwork, MMNetwork,
        MManageMetadata, MMetadataRecord, MNetworkDetails, MNetworkMenu, MNewSeedBackup, MRawKey,
        MSCNetworkInfo, MSeedKeyCard, MTypesInfo, MVerifier, Network, SeedNameCard,
    },
    network_specs::{NetworkSpecs, ValidCurrentVerifier},
    qr_transfers::ContentLoadTypes,
    users::AddressDetails,
};
use qrcode_static::png_qr_from_string;

use crate::helpers::{
    get_address_details, get_all_networks, get_general_verifier, get_meta_values_by_name,
    get_meta_values_by_name_version, get_network_specs, make_batch_clear_tree, open_db, open_tree,
    try_get_types,
};
use crate::identities::{
    derivation_check, generate_random_phrase, get_addresses_by_seed_name, get_all_addresses,
    DerivationCheck,
};
use crate::{db_transactions::TrDbCold, helpers::get_valid_current_verifier};

/// Return a `Vec` with all seed names with seed key identicons if seed key is
/// available.
///
/// Function processes all seeds known to Signer KMS (which are input as
/// `&[String]`), including seeds without any corresponding addresses currently
/// known to Signer (orphans).
///
/// If the same seed has more than one seed key in the database, i.e. it has
/// been used to create seed keys with more than one
/// [`Encryption`](definitions::crypto::Encryption) algorithm, only one
/// identicon is selected, in order of preference: `Sr25519`, `Ed25519`,
/// `Ecdsa`.
pub fn get_all_seed_names_with_identicons(
    database_name: &str,
    names_phone_knows: &[String],
) -> Result<Vec<SeedNameCard>, ErrorSigner> {
    let mut data_set: HashMap<String, Vec<MultiSigner>> = HashMap::new();
    for (multisigner, address_details) in get_all_addresses(database_name)?.into_iter() {
        if address_details.is_root() {
            // found a seed key; could be any of the supported encryptions;
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

/// Craete a `png` identicon data, for preferred encryption if
/// multiple encryption algorithms are supported.
///
/// Output is:
///
/// - empty `png` if no seed key is available
/// - the available seed key if there is only one
/// - preferred seed key, if there are more than one; order of preference:
/// `Sr25519`, `Ed25519`, `Ecdsa`
fn preferred_multisigner_identicon(multisigner_set: &[MultiSigner]) -> Vec<u8> {
    if multisigner_set.is_empty() {
        EMPTY_PNG.to_vec()
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
            make_identicon_from_multisigner(&a)
        } else if let Some(a) = got_ed25519 {
            make_identicon_from_multisigner(&a)
        } else if let Some(a) = got_ecdsa {
            make_identicon_from_multisigner(&a)
        } else {
            EMPTY_PNG.to_vec()
        }
    }
}

/// Return a `Vec` with address-associated public data for all addresses from the
/// Signer database.
///
/// Function is used to show users all possible addresses, when selecting the
/// address to generate
/// [`SufficientCrypto`](definitions::crypto::SufficientCrypto) for signing
/// updates with the Signer.
pub fn print_all_identities(database_name: &str) -> Result<Vec<MRawKey>, ErrorSigner> {
    Ok(get_all_addresses(database_name)?
        .into_iter()
        .map(|(multisigner, address_details)| {
            let address_key = AddressKey::from_multisigner(&multisigner); // to click
            let public_key = multisigner_to_public(&multisigner); // to display
            let identicon = make_identicon_from_multisigner(&multisigner);
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

/// Return `Vec` with address-associated public data for all addresses from the
/// Signer database with given seed name and network [`NetworkSpecsKey`].
///
/// In addition, marks with flags swiped key or group of keys in multiselect
/// selection.
///
/// Separately processes the seed key. If there is no seed key, empty export for
/// seed key is still generated.
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
            if root_id.is_some() {
                return Err(ErrorSigner::Database(DatabaseSigner::TwoRootKeys {
                    seed_name: seed_name.to_string(),
                    encryption: network_specs.encryption,
                }));
            }
            root_id = Some(MSeedKeyCard {
                seed_name: seed_name.to_string(),
                identicon,
                address_key: hex::encode(address_key.key()),
                base58,
                swiped,
                multiselect,
            });
        } else {
            other_id.push((multisigner, address_details, identicon, swiped, multiselect))
        }
    }
    let root = root_id.unwrap_or(MSeedKeyCard {
        seed_name: seed_name.to_string(),
        identicon: EMPTY_PNG.to_vec(),
        ..Default::default()
    });
    let set: Vec<_> = other_id
        .into_iter()
        .map(
            |(multisigner, address_details, identicon, swiped, multiselect)| MKeysCard {
                address_key: hex::encode(AddressKey::from_multisigner(&multisigner).key()),
                base58: print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix)),
                identicon,
                has_pwd: address_details.has_pwd,
                path: address_details.path,
                swiped,
                multiselect,
            },
        )
        .collect();

    Ok((root, set, network_specs.title, network_specs.logo))
}

/// Get address-associated public data for all addresses from the Signer
/// database with given seed name and network [`NetworkSpecsKey`].
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

/// Return `Vec` with network information for all networks in the Signer database,
/// with bool indicator which one is currently selected.
pub fn show_all_networks_with_flag(
    database_name: &str,
    network_specs_key: &NetworkSpecsKey,
) -> Result<MNetworkMenu, ErrorSigner> {
    let mut networks: Vec<_> = get_all_networks::<Signer>(database_name)?
        .into_iter()
        .map(|network| {
            let network_specs_key_current =
                NetworkSpecsKey::from_parts(&network.genesis_hash, &network.encryption);
            let mut n: Network = network.into();
            n.selected = network_specs_key == &network_specs_key_current;
            n
        })
        .collect();
    networks.sort_by(|a, b| a.order.cmp(&b.order));

    Ok(MNetworkMenu { networks })
}

/// Make `Vec` with network information for all networks in the Signer database,
/// without any selection.
pub fn show_all_networks(database_name: &str) -> Result<Vec<MMNetwork>, ErrorSigner> {
    let networks = get_all_networks::<Signer>(database_name)?;
    let mut networks = networks
        .into_iter()
        .map(|n| MMNetwork {
            key: hex::encode(NetworkSpecsKey::from_parts(&n.genesis_hash, &n.encryption).key()),
            title: n.title,
            logo: n.logo,
            order: n.order,
        })
        .collect::<Vec<_>>();
    networks.sort_by(|a, b| a.order.cmp(&b.order));

    Ok(networks)
}

/// Sort database networks by the order and get the network specs for the first
/// network on the list.
///
/// If there are no networks in the system, throws error.
// TODO: should be an option, not an error. Forbid getting to this point from ui
// for the seed making process, allow backups.
pub fn first_network(database_name: &str) -> Result<NetworkSpecs, ErrorSigner> {
    let mut networks = get_all_networks::<Signer>(database_name)?;
    if networks.is_empty() {
        return Err(ErrorSigner::NoNetworksAvailable);
    }
    networks.sort_by(|a, b| a.order.cmp(&b.order));
    Ok(networks.remove(0))
}

/// Prepare export key screen struct [`MKeyDetails`].
///
/// For QR code the address information is put in format
/// `substrate:{public key as base58}:0x{network genesis hash}`
/// transformed into bytes, to be compatible with `polkadot-js` interface.
///
/// Note that no [`Encryption`](definitions::crypto::Encryption) algorithm
/// information is contained in the QR code. If there are multiple `Encryption`
/// algorithms supported by the network, the only visible difference in exports
/// would be the identicon.
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
    let qr = {
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
    let address = Address {
        base58,
        path: address_details.path,
        has_pwd: address_details.has_pwd,
        identicon,
        seed_name: address_details.seed_name,
        multiselect: None,
    };

    let network_info = MSCNetworkInfo {
        network_title: network_specs.title,
        network_logo: network_specs.logo,
    };

    Ok(MKeyDetails {
        qr,
        pubkey: hex::encode(public_key),
        network_info,
        address,
    })
}

/// Prepare seed backup screen struct [`MBackup`] for given seed name.
///
/// Function inputs seed name, outputs vec with all known derivations in all
/// networks.
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
            &NetworkSpecsKey::from_parts(&network.genesis_hash, &network.encryption),
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

/// Prepare key derivation screen struct [`MDeriveKey`].
///
/// Function inputs seed name, network [`NetworkSpecsKey`] and user-suggested
/// derivation, outputs struct with derived address data and, if the derived
/// address already exists in the database, shows the its data.
///
// TODO: the `collision` part is actually a mislabel, it is really
// `derivation_exists`, and is referring to the derivation with same
// [`AddressKey`] in same network (that would be cause by same seed used, same
// derivation path and same password if any password exists) - this mislabel
// should be corrected, after json fix; `seed_name` in existing derivation
// display also seems to be excessive
pub fn derive_prep(
    database_name: &str,
    seed_name: &str,
    network_specs_key: &NetworkSpecsKey,
    collision: Option<(MultiSigner, AddressDetails)>,
    suggest: &str,
    keyboard: bool,
) -> Result<MDeriveKey, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;

    let derivation_check = match collision {
        Some((multisigner, address_details)) => {
            let base58 =
                print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix));
            let path = address_details.path;
            let has_pwd = address_details.has_pwd;
            let identicon = make_identicon_from_multisigner(&multisigner);
            let seed_name = seed_name.to_string();
            let collision = Address {
                base58,
                path,
                has_pwd,
                identicon,
                seed_name,
                multiselect: None,
            };

            NavDerivationCheck {
                collision: Some(collision),
                ..Default::default()
            }
        }
        None => dynamic_path_check_unhexed(database_name, seed_name, suggest, network_specs_key),
    };

    Ok(MDeriveKey {
        seed_name: seed_name.to_string(),
        network_title: network_specs.title,
        network_logo: network_specs.logo,
        network_specs_key: hex::encode(network_specs_key.key()),
        suggested_derivation: suggest.to_string(),
        keyboard,
        derivation_check,
    })
}

/// Return [`NavDerivationCheck`] with allowed action details for new key derivation.
///
/// Function is used to dynamically check from the frontend if user is allowed
/// to proceed with the proposed derived key generation.
///
/// User is allowed to try to proceed only if the derivation is valid and, in
/// case of derivations without password, if the derivation does not already
/// exist in the database. Passworded valid derivations are allowed to proceed,
/// but result in an error later on, if the derivation exists.
///
/// Function makes only preliminary check on password-free derivations, it
/// **does not** use seed phrase and does not calculate the [`AddressKey`], i.e.
/// it can't check passworded derivations, and allows them to proceed anyway.
pub fn dynamic_path_check(
    database_name: &str,
    seed_name: &str,
    path: &str,
    network_specs_key_hex: &str,
) -> NavDerivationCheck {
    match NetworkSpecsKey::from_hex(network_specs_key_hex) {
        Ok(key) => dynamic_path_check_unhexed(database_name, seed_name, path, &key),
        Err(e) => NavDerivationCheck {
            error: Some(<Signer>::show(&e)),
            ..Default::default()
        },
    }
}

fn dynamic_path_check_unhexed(
    database_name: &str,
    seed_name: &str,
    path: &str,
    network_specs_key: &NetworkSpecsKey,
) -> NavDerivationCheck {
    match get_network_specs(database_name, network_specs_key) {
        Ok(network_specs) => {
            match derivation_check(seed_name, path, network_specs_key, database_name) {
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
                    let address_base58 =
                        print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix));
                    let identicon = make_identicon_from_multisigner(&multisigner);
                    let collision_display = Address {
                        base58: address_base58,
                        path: address_details.path,
                        has_pwd: address_details.has_pwd,
                        identicon,
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
    }
}

/// Return [`MNetworkDetails`] with network specs and metadata set information
/// for network with given [`NetworkSpecsKey`].
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
    let verifier_key = VerifierKey::from_parts(genesis_hash);
    let general_verifier = get_general_verifier(database_name)?;
    let current_verifier = get_valid_current_verifier(&verifier_key, database_name)?;
    let meta: Vec<_> = get_meta_values_by_name(database_name, &name)?
        .into_iter()
        .map(|m| {
            let meta_hash = blake2_256(&m.meta);
            let meta_id_pic = pic_meta(&meta_hash);

            MMetadataRecord {
                specname: m.name,
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
        genesis_hash,
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

/// Return [`MManageMetadata`] with metadata details for network with given
/// [`NetworkSpecsKey`] and given version.
pub fn metadata_details(
    database_name: &str,
    network_specs_key: &NetworkSpecsKey,
    network_version: u32,
) -> Result<MManageMetadata, ErrorSigner> {
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
                &network.genesis_hash,
                &network.encryption,
            ) == network_specs_key,
        })
        .collect();

    let meta_hash = blake2_256(&meta_values.meta);
    let meta_id_pic = pic_meta(&meta_hash);
    Ok(MManageMetadata {
        name: network_specs.name,
        version: network_version.to_string(),
        meta_hash: hex::encode(meta_hash),
        meta_id_pic,
        networks,
    })
}

/// Make types status display.
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

/// Generate new random seed phrase, make identicon for sr25519 public key,
/// and send to Signer screen.
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
    let identicon = make_identicon_from_multisigner(&MultiSigner::Sr25519(sr25519_pair.public()));
    Ok(MNewSeedBackup {
        seed: seed_name.to_string(),
        seed_phrase,
        identicon,
    })
}

/// Get database history tree checksum to be displayed in log screen.
pub fn history_hex_checksum(database_name: &str) -> Result<String, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let history = open_tree::<Signer>(&database, HISTORY)?;
    let checksum = history
        .checksum()
        .map_err(|e| ErrorSigner::Database(DatabaseSigner::Internal(e)))?;
    Ok(hex::encode(checksum.encode()).to_uppercase())
}

/// Clear transaction tree of the database.
///
/// Function is intended for cases when transaction is declined by the user
/// (e.g. user has scanned something, read it, clicked `back` or `decline`)
pub fn purge_transactions(database_name: &str) -> Result<(), ErrorSigner> {
    TrDbCold::new()
        .set_transaction(make_batch_clear_tree::<Signer>(database_name, TRANSACTION)?) // clear transaction
        .apply::<Signer>(database_name)
}

/// Get possible options of English bip39 words that start with user-entered
/// word part.
///
/// List lentgh limit is [`MAX_WORDS_DISPLAY`].
pub fn guess(word_part: &str) -> Vec<&'static str> {
    let dictionary = Language::English.wordlist();
    let words = dictionary.get_words_by_prefix(word_part);
    if words.len() > MAX_WORDS_DISPLAY {
        words[..MAX_WORDS_DISPLAY].to_vec()
    } else {
        words.to_vec()
    }
}

/// Maximum word count in bip39 standard.
///
/// See <https://docs.rs/tiny-bip39/0.8.2/src/bip39/mnemonic_type.rs.html#60>
pub const BIP_CAP: usize = 24;

/// Maximum word length in bip39 standard.
pub const WORD_LENGTH: usize = 8;

/// Zeroizeable seed phrase draft.
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct SeedDraft {
    /// User-entered word part.
    user_input: String,

    /// Already completed bip39 words.
    saved: Vec<SeedElement>,
}

/// Zeroizeable wrapper around complete bip39 word entered by user.
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
struct SeedElement(String);

impl SeedElement {
    /// Make `SeedElement` from checked bip39 word.
    fn from_checked_str(word: &str) -> Self {
        let mut new = String::with_capacity(WORD_LENGTH);
        new.push_str(word);
        SeedElement(new)
    }

    /// Get bip39 word from the `SeedElement`.
    fn word(&self) -> &str {
        &self.0
    }
}

impl SeedDraft {
    /// Start new `SeedDraft`
    pub fn initiate() -> Self {
        Self {
            user_input: String::with_capacity(WORD_LENGTH), // capacity corresponds to maximum word length in bip39 standard;
            saved: Vec::with_capacity(BIP_CAP), // capacity corresponds to maximum word count in bip39 standard; set here to avoid reallocation;
        }
    }

    /// Modify `SeedDraft` with updated `user_text` from the frontend.
    ///
    /// Note that `user_text` input by default starts with ' ' (space). If user
    /// removes this space, it results in removing whole previous word.
    pub fn text_field_update(&mut self, user_text: &str) {
        if self.saved.len() < BIP_CAP {
            if user_text.is_empty() {
                // user has removed all text, including the first default symbol
                // if there are words in draft, remove the last one
                self.remove_last();
                // restore the user input to empty one
                self.user_input.clear();
            } else {
                let user_text = user_text.trim_start();

                // ' ' (space) in the end of the word indicates user attempt to
                // submit the word into seed phrase
                if user_text.ends_with(' ') {
                    let word = user_text.trim();
                    if self.added(word, None) {
                        self.user_input.clear() // added the word successfully, clear `user_input`
                    } else if !guess(word).is_empty() {
                        self.user_input = String::from(word) // did not add the word, there are still possible variants, keep trimmed `user_input`
                    }
                } else if !guess(user_text).is_empty() {
                    self.user_input = String::from(user_text)
                }
            }
        } else {
            self.user_input.clear()
        }
    }

    /// User tries to add the word to the `saved` field of the `SeedDraft`.
    /// Output is `true` if addition happens. `SeedDraft` gets modified in the
    /// process.
    ///
    /// Optional `position` input could be used to mark the position in seed
    /// phrase to add the word to.
    pub fn added(&mut self, word: &str, position: Option<u32>) -> bool {
        // maximum number of the words is not reached
        if self.saved.len() < BIP_CAP {
            let guesses = guess(word);
            let definitive_guess = {
                if guesses.len() == 1 {
                    Some(guesses[0]) // only one possible variant
                } else if guesses.contains(&word) {
                    Some(word) // exactly matching variant
                } else {
                    None // no definitive match, no addition
                }
            };
            if let Some(guess) = definitive_guess {
                let new = SeedElement::from_checked_str(guess);
                match position {
                    Some(p) => {
                        let p = p as usize;
                        if p <= self.saved.len() {
                            self.saved.insert(p, new) // position is reasonable, use it
                        } else {
                            self.saved.push(new) // position is **not** reasonable, add word at the end of the list
                        }
                    }
                    None => self.saved.push(new), // no position, add word at the end of the list
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

    /// Remove word at given position from the saved seed phrase draft.
    pub fn remove(&mut self, position: u32) {
        let position = position as usize;
        if position < self.saved.len() {
            self.saved.remove(position);
        }
    }

    /// Remove last word from the saved seed phrase draft.
    pub fn remove_last(&mut self) {
        if !self.saved.is_empty() {
            self.saved.remove(self.saved.len() - 1);
        }
    }

    /// Make a `String` with a seed phrase draft.
    ///
    /// Returned string contains a secret and should be handled as such.
    pub fn draft(&self) -> Vec<String> {
        self.saved.iter().map(|w| w.word().to_string()).collect()
    }

    /// Combines all draft elements into seed phrase proposal,
    /// and checks its validity.
    /// If valid, outputs secret seed phrase.
    pub fn try_finalize(&self) -> Option<String> {
        let mut seed_phrase_proposal = String::with_capacity((WORD_LENGTH + 1) * BIP_CAP);
        for (i, x) in self.saved.iter().enumerate() {
            if i > 0 {
                seed_phrase_proposal.push(' ');
            }
            seed_phrase_proposal.push_str(x.word());
        }
        if Mnemonic::validate(&seed_phrase_proposal, Language::English).is_ok() {
            Some(seed_phrase_proposal)
        } else {
            seed_phrase_proposal.zeroize();
            None
        }
    }

    /// Output the user input back into user interface.
    pub fn user_input(&self) -> &str {
        &self.user_input
    }
}
