//! Utils to communicate with the Signer frontend
//!
//! This will be obsolete or majorly reworked at least when jsons "manual"
//! export is gone.
//!
//! Currently Signer receives from the frontend String inputs with
//! **individual** object identifiers (e.g. single hexadecimal
//! [`NetworkSpecsKey`] or single hexadecimal [`AddressKey`]). Signer sends
//! json-like Strings with multiple fields, to be processed in the frontend to
//! generate screens.
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
    network_specs::NetworkSpecs,
    print::{export_complex_vector, export_plain_vector},
    qr_transfers::ContentLoadTypes,
    users::AddressDetails,
};
use qrcode_static::png_qr_from_string;

use crate::db_transactions::TrDbCold;
use crate::helpers::{
    get_address_details, get_all_networks, get_general_verifier, get_meta_values_by_name,
    get_meta_values_by_name_version, get_network_specs, get_valid_current_verifier,
    make_batch_clear_tree, open_db, open_tree, try_get_types,
};
use crate::identities::{
    derivation_check, generate_random_phrase, get_addresses_by_seed_name, get_all_addresses,
    DerivationCheck,
};

/// Make json with all seed names with seed key identicons if seed key is
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
pub fn print_all_seed_names_with_identicons(
    database_name: &str,
    names_phone_knows: &[String],
) -> Result<String, ErrorSigner> {
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
    let mut print_set: Vec<(String, String)> = Vec::new();
    for (seed_name, multisigner_set) in data_set.into_iter() {
        let identicon_string = preferred_multisigner_identicon(&multisigner_set);
        print_set.push((identicon_string, seed_name))
    }
    print_set.sort_by(|a, b| a.1.cmp(&b.1));
    Ok(export_complex_vector(
        &print_set,
        |(identicon_string, seed_name)| {
            format!(
                "\"identicon\":\"{}\",\"seed_name\":\"{}\"",
                identicon_string, seed_name
            )
        },
    ))
}

/// Print hex encoded `png` identicon data, for preferred encryption if
/// multiple encryption algorithms are supported.
///
/// Output is:
///
/// - empty `png` if no seed key is available
/// - the available seed key if there is only one
/// - preferred seed key, if there are more than one; order of preference:
/// `Sr25519`, `Ed25519`, `Ecdsa`
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

/// Make json with address-associated public data for all addresses from the
/// Signer database.
///
/// Function is used to show users all possible addresses, when selecting the
/// address to generate
/// [`SufficientCrypto`](definitions::crypto::SufficientCrypto) for signing
/// updates with the Signer.
pub fn print_all_identities(database_name: &str) -> Result<String, ErrorSigner> {
    Ok(export_complex_vector(
        &get_all_addresses(database_name)?,
        |(multisigner, address_details)| {
            let address_key = AddressKey::from_multisigner(multisigner); // to click
            let public_key = multisigner_to_public(multisigner); // to display
            let hex_identicon = hex::encode(make_identicon_from_multisigner(multisigner));
            format!("\"seed_name\":\"{}\",\"address_key\":\"{}\",\"public_key\":\"{}\",\"identicon\":\"{}\",\"has_pwd\":{},\"path\":\"{}\"", address_details.seed_name, hex::encode(address_key.key()), hex::encode(public_key), hex_identicon, address_details.has_pwd, address_details.path)
        },
    ))
}

/// Make json with address-associated public data for all addresses from the
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
) -> Result<String, ErrorSigner> {
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
        let is_multiselect = multiselect.contains(&multisigner);
        if address_details.is_root() {
            if root_id.is_some() {
                return Err(ErrorSigner::Database(DatabaseSigner::TwoRootKeys {
                    seed_name: seed_name.to_string(),
                    encryption: network_specs.encryption,
                }));
            }
            root_id = Some(format!("\"seed_name\":\"{}\",\"identicon\":\"{}\",\"address_key\":\"{}\",\"base58\":\"{}\",\"swiped\":{},\"multiselect\":{}", seed_name, hex::encode(identicon), hex::encode(address_key.key()), base58, swiped, is_multiselect));
        } else {
            other_id.push((
                multisigner,
                address_details,
                identicon,
                swiped,
                is_multiselect,
            ))
        }
    }
    let root_print = match root_id {
        Some(a) => a,
        None => format!("\"seed_name\":\"{}\",\"identicon\":\"{}\",\"address_key\":\"\",\"base58\":\"\",\"swiped\":false,\"multiselect\":false", seed_name, hex::encode(EMPTY_PNG)),
    };
    let other_print = export_complex_vector(
        &other_id,
        |(multisigner, address_details, identicon, swiped, is_multiselect)| {
            format!("\"address_key\":\"{}\",\"base58\":\"{}\",\"identicon\":\"{}\",\"has_pwd\":{},\"path\":\"{}\",\"swiped\":{},\"multiselect\":{}", hex::encode(AddressKey::from_multisigner(multisigner).key()), print_multisigner_as_base58(multisigner, Some(network_specs.base58prefix)), hex::encode(identicon), address_details.has_pwd, address_details.path, swiped, is_multiselect)
        },
    );

    Ok(format!(
        "\"root\":{{{}}},\"set\":{},\"network\":{{\"title\":\"{}\",\"logo\":\"{}\"}}",
        root_print, other_print, network_specs.title, network_specs.logo
    ))
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

/// Make json with network information for all networks in the Signer database,
/// with bool indicator which one is currently selected.
pub fn show_all_networks_with_flag(
    database_name: &str,
    network_specs_key: &NetworkSpecsKey,
) -> Result<String, ErrorSigner> {
    let mut networks = get_all_networks::<Signer>(database_name)?;
    networks.sort_by(|a, b| a.order.cmp(&b.order));
    Ok(format!(
        "\"networks\":{}",
        export_complex_vector(&networks, |a| {
            let network_specs_key_current =
                NetworkSpecsKey::from_parts(&a.genesis_hash, &a.encryption);
            format!(
                "\"key\":\"{}\",\"title\":\"{}\",\"logo\":\"{}\",\"order\":{},\"selected\":{}",
                hex::encode(network_specs_key_current.key()),
                a.title,
                a.logo,
                a.order,
                &network_specs_key_current == network_specs_key
            )
        })
    ))
}

/// Make json with network information for all networks in the Signer database,
/// without any selection.
pub fn show_all_networks(database_name: &str) -> Result<String, ErrorSigner> {
    let mut networks = get_all_networks::<Signer>(database_name)?;
    networks.sort_by(|a, b| a.order.cmp(&b.order));
    Ok(format!(
        "\"networks\":{}",
        export_complex_vector(&networks, |a| {
            let network_specs_key_current =
                NetworkSpecsKey::from_parts(&a.genesis_hash, &a.encryption);
            format!(
                "\"key\":\"{}\",\"title\":\"{}\",\"logo\":\"{}\",\"order\":{}",
                hex::encode(network_specs_key_current.key()),
                a.title,
                a.logo,
                a.order
            )
        })
    ))
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

/// Prepare export key screen json.
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
) -> Result<String, ErrorSigner> {
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
    let address_base58 = print_multisigner_as_base58(multisigner, Some(network_specs.base58prefix));
    let public_key = multisigner_to_public(multisigner);
    let identicon = make_identicon_from_multisigner(multisigner);
    let qr_prep = {
        if address_details.network_id.contains(network_specs_key) {
            match png_qr_from_string(&format!(
                "substrate:{}:0x{}",
                address_base58,
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
    Ok(format!("\"qr\":\"{}\",\"pubkey\":\"{}\",\"base58\":\"{}\",\"identicon\":\"{}\",\"seed_name\":\"{}\",\"path\":\"{}\",\"network_title\":\"{}\",\"network_logo\":\"{}\"", hex::encode(qr_prep), hex::encode(public_key), address_base58, hex::encode(identicon), address_details.seed_name, address_details.path, network_specs.title, network_specs.logo))
}

/// Prepare seed backup screen json for given seed name.
///
/// Function inputs seed name, outputs json with all known derivations in all
/// networks.
pub fn backup_prep(database_name: &str, seed_name: &str) -> Result<String, ErrorSigner> {
    let networks = get_all_networks::<Signer>(database_name)?;
    if networks.is_empty() {
        return Err(ErrorSigner::NoNetworksAvailable);
    }
    let mut export: Vec<(NetworkSpecs, Vec<AddressDetails>)> = Vec::new();
    for x in networks.into_iter() {
        let id_set = addresses_set_seed_name_network(
            database_name,
            seed_name,
            &NetworkSpecsKey::from_parts(&x.genesis_hash, &x.encryption),
        )?;
        if !id_set.is_empty() {
            export.push((x, id_set.into_iter().map(|(_, a)| a).collect()))
        }
    }
    export.sort_by(|(a, _), (b, _)| a.order.cmp(&b.order));
    Ok(format!(
        "\"seed_name\":\"{}\",\"derivations\":{}",
        seed_name,
        export_complex_vector(&export, |(specs, id_set)| format!(
            "\"network_title\":\"{}\",\"network_logo\":\"{}\",\"network_order\":{},\"id_set\":{}",
            specs.title,
            specs.logo,
            specs.order,
            export_complex_vector(id_set, |a| format!(
                "\"path\":\"{}\",\"has_pwd\":{}",
                a.path, a.has_pwd
            ))
        ))
    ))
}

/// Prepare key derivation screen json.
///
/// Function inputs seed name, network [`NetworkSpecsKey`] and user-suggested
/// derivation, outputs json with derived address data and, if the derived
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
) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    match collision {
        Some((multisigner, address_details)) => {
            let address_base58 = print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix));
            let hex_identicon = hex::encode(make_identicon_from_multisigner(&multisigner));
            let collision_display = format!("\"base58\":\"{}\",\"path\":\"{}\",\"has_pwd\":{},\"identicon\":\"{}\",\"seed_name\":\"{}\"", address_base58, address_details.path, address_details.has_pwd, hex_identicon, seed_name);
            Ok(format!("\"seed_name\":\"{}\",\"network_title\":\"{}\",\"network_logo\":\"{}\",\"network_specs_key\":\"{}\",\"suggested_derivation\":\"{}\",\"collision\":{{{}}}", seed_name, network_specs.title, network_specs.logo, hex::encode(network_specs_key.key()), suggest, collision_display))
        },
        None => Ok(format!("\"seed_name\":\"{}\",\"network_title\":\"{}\",\"network_logo\":\"{}\",\"network_specs_key\":\"{}\",\"suggested_derivation\":\"{}\"", seed_name, network_specs.title, network_specs.logo, hex::encode(network_specs_key.key()), suggest)),
    }
}

/// Make json with allowed action details for new key derivation.
///
/// Function is used to dynamically check from the frontend if user is allowed
/// to proceed with the proposed derived key generation.
///
/// Note that the function is unfallible and always produces a json string.
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
) -> String {
    let content = match NetworkSpecsKey::from_hex(network_specs_key_hex) {
        Ok(network_specs_key) => match get_network_specs(database_name, &network_specs_key) {
            Ok(network_specs) => {
                match derivation_check(seed_name, path, &network_specs_key, database_name) {
                    Ok(DerivationCheck::BadFormat) => String::from("\"button_good\":false"),
                    Ok(DerivationCheck::Password) => {
                        String::from("\"button_good\":true,\"where_to\":\"pwd\"")
                    }
                    Ok(DerivationCheck::NoPassword(None)) => {
                        String::from("\"button_good\":true,\"where_to\":\"pin\"")
                    }
                    Ok(DerivationCheck::NoPassword(Some((multisigner, address_details)))) => {
                        let address_base58 = print_multisigner_as_base58(
                            &multisigner,
                            Some(network_specs.base58prefix),
                        );
                        let hex_identicon =
                            hex::encode(make_identicon_from_multisigner(&multisigner));
                        let collision_display = format!("\"base58\":\"{}\",\"path\":\"{}\",\"has_pwd\":{},\"identicon\":\"{}\",\"seed_name\":\"{}\"", address_base58, address_details.path, address_details.has_pwd, hex_identicon, seed_name);
                        format!(
                            "\"button_good\":false,\"collision\":{{{}}}",
                            collision_display
                        )
                    }
                    Err(e) => format!("\"error\":\"{}\"", <Signer>::show(&e)),
                }
            }
            Err(e) => format!("\"error\":\"{}\"", <Signer>::show(&e)),
        },
        Err(e) => format!("\"error\":\"{}\"", <Signer>::show(&e)),
    };
    format!("{{\"derivation_check\":{{{}}}}}", content)
}

/// Make json with network specs and metadata set information for network with
/// given [`NetworkSpecsKey`].
pub fn network_details_by_key(
    database_name: &str,
    network_specs_key: &NetworkSpecsKey,
) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let verifier_key = VerifierKey::from_parts(&network_specs.genesis_hash);
    let general_verifier = get_general_verifier(database_name)?;
    let valid_current_verifier = get_valid_current_verifier(&verifier_key, database_name)?;
    let relevant_meta = get_meta_values_by_name(database_name, &network_specs.name)?;
    let metadata_print = export_complex_vector(&relevant_meta, |a| {
        let meta_hash = blake2b(32, &[], &a.meta).as_bytes().to_vec();
        let hex_id_pic = hex::encode(pic_meta(&meta_hash));
        format!(
            "\"spec_version\":\"{}\",\"meta_hash\":\"{}\",\"meta_id_pic\":\"{}\"",
            a.version,
            hex::encode(meta_hash),
            hex_id_pic
        )
    });
    Ok(format!(
        "{},\"meta\":{}",
        network_specs.show(&valid_current_verifier, &general_verifier),
        metadata_print
    ))
}

/// Make json with metadata details for network with given [`NetworkSpecsKey`]
/// and given version.
pub fn metadata_details(
    database_name: &str,
    network_specs_key: &NetworkSpecsKey,
    network_version: u32,
) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let meta_values = get_meta_values_by_name_version::<Signer>(
        database_name,
        &network_specs.name,
        network_version,
    )?;
    let relevant_networks: Vec<NetworkSpecs> = get_all_networks::<Signer>(database_name)?
        .into_iter()
        .filter(|a| a.name == network_specs.name)
        .collect();
    let relevant_networks_print = export_complex_vector(&relevant_networks, |a| {
        format!(
            "\"title\":\"{}\",\"logo\":\"{}\",\"order\":{},\"current_on_screen\":{}",
            a.title,
            a.logo,
            a.order,
            &NetworkSpecsKey::from_parts(&a.genesis_hash, &a.encryption) == network_specs_key
        )
    });
    let meta_hash = blake2b(32, &[], &meta_values.meta).as_bytes().to_vec();
    let hex_id_pic = hex::encode(pic_meta(&meta_hash));
    Ok(format!("\"name\":\"{}\",\"version\":\"{}\",\"meta_hash\":\"{}\",\"meta_id_pic\":\"{}\",\"networks\":{}", network_specs.name, network_version, hex::encode(meta_hash), hex_id_pic, relevant_networks_print))
}

/// Make json with types status display.
pub fn show_types_status(database_name: &str) -> Result<String, ErrorSigner> {
    match try_get_types::<Signer>(database_name)? {
        Some(a) => Ok(format!(
            "\"types_on_file\":true,{}",
            ContentLoadTypes::generate(&a).show()
        )),
        None => Ok(String::from("\"types_on_file\":false")),
    }
}

/// Generate new random seed phrase, make identicon for sr25519 public key,
/// and send to Signer screen.
// TODO there should be zeroize and no format!(), but this is gone with json fix
// and so stays here for now
pub fn print_new_seed(seed_name: &str) -> Result<String, ErrorSigner> {
    let seed_phrase = generate_random_phrase(24)?;
    let sr25519_pair = match sr25519::Pair::from_string(&seed_phrase, None) {
        Ok(x) => x,
        Err(e) => {
            return Err(<Signer>::address_generation_common(
                AddressGenerationCommon::SecretString(e),
            ))
        }
    };
    let hex_identicon = hex::encode(make_identicon_from_multisigner(&MultiSigner::Sr25519(
        sr25519_pair.public(),
    )));
    Ok(format!(
        "\"seed\":\"{}\",\"seed_phrase\":\"{}\",\"identicon\":\"{}\"", // should have fixed that if that stays
        seed_name, seed_phrase, hex_identicon
    ))
}

/// Get database history tree checksum to be displayed in log screen.
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
pub(crate) fn guess(word_part: &str) -> Vec<&'static str> {
    let dictionary = Language::English.wordlist();
    let words = dictionary.get_words_by_prefix(word_part);
    if words.len() > MAX_WORDS_DISPLAY {
        words[..MAX_WORDS_DISPLAY].to_vec()
    } else {
        words.to_vec()
    }
}

/// Make json with possible options of English bip39 words that start with
/// user-entered word part.
///
//// List lentgh limit is [`MAX_WORDS_DISPLAY`], sorted alphabetically.
pub fn print_guess(user_entry: &str) -> String {
    export_plain_vector(&guess(user_entry))
}

/// Maximum word count in bip39 standard.
///
/// See <https://docs.rs/tiny-bip39/0.8.2/src/bip39/mnemonic_type.rs.html#60>
pub const BIP_CAP: usize = 24;

/// Maximum word length in bip39 standard.
pub const WORD_LENGTH: usize = 8;

/// String length to reserve for json output of numbered draft.
///
/// Each element is {"order":**,"content":"********"}, at most 33 + 1 (comma)
/// symbols for each max BIP_CAP elements, two extras for `[` and `]`, and some
/// extra space just in case.
pub const SAFE_RESERVE: usize = 1000;

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

    /// Make json with seed phrase draft.
    ///
    /// Resulting json contains a secret and should be handled as such.
    pub fn print(&self) -> String {
        let mut out = String::with_capacity(SAFE_RESERVE); // length set here to avoid reallocation
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

    /// Combines draft elements into seed phrase proposal, and checks its
    /// validity. If valid, output is `Some(secret seed phrase)`.
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
