//! All db handling related to seeds and addresses
//! seed phrases should be stored in hw encrypted by
//! best available tool and here they are only processed in plaintext.
//! Zeroization is mostly delegated to os

use bip39::{Language, Mnemonic, MnemonicType};
use lazy_static::lazy_static;
use parity_scale_codec::Encode;
use regex::Regex;
use sled::Batch;
use sp_core::{Pair, ed25519, sr25519, ecdsa};
use sp_runtime::MultiSigner;
use zeroize::Zeroize;

use constants::{ADDRTREE, MAX_WORDS_DISPLAY, TRANSACTION};
use defaults::get_default_chainspecs;
use definitions::{crypto::Encryption, error::{Active, AddressGeneration, AddressGenerationCommon, ErrorActive, ErrorSigner, ErrorSource, ExtraAddressGenerationSigner, InputActive, InputSigner, InterfaceSigner, Signer, SpecsKeySource}, helpers::multisigner_to_public, history::{Event, IdentityHistory}, keyring::{NetworkSpecsKey, AddressKey}, network_specs::NetworkSpecs, print::export_plain_vector, qr_transfers::ContentDerivations, users::AddressDetails};

use crate::db_transactions::{TrDbCold, TrDbColdDerivations};
use crate::helpers::{open_db, open_tree, make_batch_clear_tree, upd_id_batch, get_network_specs, get_address_details};
use crate::interface_signer::addresses_set_seed_name_network;
use crate::manage_history::{events_to_batch};
use crate::network_details::get_all_networks;

const ALICE_SEED_PHRASE: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";

lazy_static! {
// stolen from sp_core
// removed seed phrase part
// last '+' used to be '*', but empty password is an error
    static ref REG_PATH: Regex = Regex::new(r"^(?P<path>(//?[^/]+)*)(///(?P<password>.+))?$").expect("known value");
}

/// Get all identities from database.
/// Function gets used only on the Signer side.
pub fn get_all_addresses (database_name: &str) -> Result<Vec<(MultiSigner, AddressDetails)>, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let identities = open_tree::<Signer>(&database, ADDRTREE)?;
    let mut out: Vec<(MultiSigner, AddressDetails)> = Vec::new();
    for x in identities.iter() {
        if let Ok((address_key_vec, address_entry)) = x {
            let address_key = AddressKey::from_ivec(&address_key_vec);
            let (multisigner, address_details) = AddressDetails::process_entry_with_key_checked::<Signer>(&address_key, address_entry)?;
            out.push((multisigner, address_details));
        }
    }
    Ok(out)
}

/// Filter identities by given seed_name.
/// Function gets used only on the Signer side.
pub fn get_addresses_by_seed_name (database_name: &str, seed_name: &str) -> Result<Vec<(MultiSigner, AddressDetails)>, ErrorSigner> {
    Ok(get_all_addresses(database_name)?.into_iter().filter(|(_, address_details)| address_details.seed_name == seed_name).collect())
}

/// Generate random phrase with given number of words.
/// Function gets used only on the Signer side.
/// Open to user interface.
pub fn generate_random_phrase (words_number: u32) -> Result<String, ErrorSigner> {
    let mnemonic_type = match MnemonicType::for_word_count(words_number as usize) {
        Ok(a) => a,
        Err(e) => return Err(ErrorSigner::AddressGeneration(AddressGeneration::Extra(ExtraAddressGenerationSigner::RandomPhraseGeneration(e)))),
    };
    let mnemonic = Mnemonic::new(mnemonic_type, Language::English);
    Ok(mnemonic.into_phrase())
}

/// Validate user-proposed random phrase.
/// Function gets used only on the Signer side.
/// Open to user interface.
pub fn validate_phrase (seed_phrase_proposal: &str) -> Result<(), ErrorSigner> {
    match Mnemonic::validate(seed_phrase_proposal, Language::English) {
        Ok(_) => Ok(()),
        Err(e) => return Err(ErrorSigner::AddressGeneration(AddressGeneration::Extra(ExtraAddressGenerationSigner::RandomPhraseValidation(e)))),
    }
}

/// Create address from seed and path and insert it into the database.
/// Helper gets used both on Active side (when generating test cold database with well-known addresses)
/// and on Signer side (when real addresses are actually created by the user).
fn create_address<T: ErrorSource> (database_name: &str, input_batch_prep: &Vec<(AddressKey, AddressDetails)>, path: &str, network_specs: &NetworkSpecs, seed_name: &str, seed_phrase: &str) -> Result<(Vec<(AddressKey, AddressDetails)>, Vec<Event>), T::Error> {
    
    let mut output_batch_prep = input_batch_prep.to_vec();
    let mut output_events: Vec<Event> = Vec::new();
    let network_specs_key = NetworkSpecsKey::from_parts(&network_specs.genesis_hash.to_vec(), &network_specs.encryption);
    
    let mut full_address = seed_phrase.to_owned() + path;
    let multisigner = match network_specs.encryption {
        Encryption::Ed25519 => {
            match ed25519::Pair::from_string(&full_address, None) {
                Ok(a) => {
                    full_address.zeroize();
                    MultiSigner::Ed25519(a.public())
                },
                Err(e) => {
                    full_address.zeroize();
                    return Err(<T>::address_generation_common(AddressGenerationCommon::SecretString(e)))
                },
            }
        },
        Encryption::Sr25519 => {
            match sr25519::Pair::from_string(&full_address, None) {
                Ok(a) => {
                    full_address.zeroize();
                    MultiSigner::Sr25519(a.public())
                },
                Err(e) => {
                    full_address.zeroize();
                    return Err(<T>::address_generation_common(AddressGenerationCommon::SecretString(e)))
                },
            }
        },
        Encryption::Ecdsa => {
            match ecdsa::Pair::from_string(&full_address, None) {
                Ok(a) => {
                    full_address.zeroize();
                    MultiSigner::Ecdsa(a.public())
                },
                Err(e) => {
                    full_address.zeroize();
                    return Err(<T>::address_generation_common(AddressGenerationCommon::SecretString(e)))
                },
            }
        },
    };
    
    let public_key = multisigner_to_public(&multisigner);
    let address_key = AddressKey::from_multisigner(&multisigner);
    
    let (cropped_path, has_pwd) = match REG_PATH.captures(path) {
        Some(caps) => match caps.name("path") {
            Some(a) => (a.as_str(), caps.name("password").is_some()),
            None => ("", caps.name("password").is_some()),
        },
        None => ("", false),
    };
    
    let identity_history = IdentityHistory::get(seed_name, &network_specs.encryption, &public_key, &cropped_path, &network_specs.genesis_hash.to_vec());
    output_events.push(Event::IdentityAdded(identity_history));
    
    let mut number_in_current = None;
    
// check if the same address key already participates in current database transaction
    for (i, (x_address_key, x_address_details)) in output_batch_prep.iter().enumerate() {
        if x_address_key == &address_key {
            if !x_address_details.network_id.contains(&network_specs_key) {
                number_in_current = Some(i);
                break;
            }
        }
    }
    match number_in_current {
        Some(i) => {
            let mut mod_entry = output_batch_prep.remove(i);
            mod_entry.1.network_id.push(network_specs_key);
            output_batch_prep.push(mod_entry);
            Ok((output_batch_prep, output_events))
        },
        None => {
            let database = open_db::<T>(database_name)?;
            let identities = open_tree::<T>(&database, ADDRTREE)?;
        // This address might be already created; maybe we just need to allow its use in another network?
            match identities.get(address_key.key()) {
                Ok(Some(address_entry)) => {
                    let mut address_details = AddressDetails::from_entry_with_key_checked::<T>(&address_key, address_entry)?;
                    if address_details.path != cropped_path {return Err(<T>::address_generation_common(AddressGenerationCommon::KeyCollision{seed_name: address_details.seed_name}))}
                    if !address_details.network_id.contains(&network_specs_key) {
                        address_details.network_id.push(network_specs_key);
                        output_batch_prep.push((address_key, address_details));
                        Ok((output_batch_prep, output_events))
                    }
                    else {
                        return Err(<T>::address_generation_common(AddressGenerationCommon::DerivationExists(multisigner, address_details, network_specs_key)))
                    }
                },
                Ok(None) => {
                    let address_details = AddressDetails {
                        seed_name: seed_name.to_string(),
                        path: cropped_path.to_string(),
                        has_pwd,
                        network_id: vec![network_specs_key],
                        encryption: network_specs.encryption.to_owned(),
                    };
                    output_batch_prep.push((address_key, address_details));
                    Ok((output_batch_prep, output_events))
                },
                Err(e) => return Err(<T>::db_internal(e)),
            }
        },
    }
}

/// Create addresses for all default paths in all default networks, and insert them in the database
fn populate_addresses<T: ErrorSource> (database_name: &str, entry_batch: Batch, seed_name: &str, seed_phrase: &str, roots: bool) -> Result<(Batch, Vec<Event>), T::Error> {
// TODO: check zeroize
    let mut identity_adds: Vec<(AddressKey, AddressDetails)> = Vec::new();
    let mut current_events: Vec<Event> = Vec::new();
    let specs_set = get_all_networks::<T>(database_name)?;
    for network_specs in specs_set.iter() {
        if roots {
            let (adds, events) = create_address::<T> (database_name, &identity_adds, "", &network_specs, seed_name, seed_phrase)?;
            identity_adds = adds;
            current_events.extend_from_slice(&events);
        }
        if let Ok((adds, events)) = create_address::<T> (database_name, &identity_adds, &network_specs.path_id, &network_specs, seed_name, seed_phrase) {
            identity_adds = adds;
            current_events.extend_from_slice(&events);
        }
    }
    Ok((upd_id_batch(entry_batch, identity_adds), current_events))
}

/// Generate new seed and populate all known networks with default accounts
pub fn try_create_seed (seed_name: &str, seed_phrase: &str, roots: bool, database_name: &str) -> Result<(), ErrorSigner> {
    let mut events: Vec<Event> = vec![Event::SeedCreated(seed_name.to_string())];
    let (id_batch, add_events) = populate_addresses::<Signer>(database_name, Batch::default(), seed_name, seed_phrase, roots)?;
    events.extend_from_slice(&add_events);
    TrDbCold::new()
        .set_addresses(id_batch) // add addresses just made in populate_addresses
        .set_history(events_to_batch::<Signer>(&database_name, events)?) // add corresponding history
        .apply::<Signer>(&database_name)
}

/// Function removes address by multisigner identifier and and network id
/// Function removes network_key from network_id vector for database record with address_key corresponding to given public key
pub fn remove_key(database_name: &str, multisigner: &MultiSigner, network_specs_key: &NetworkSpecsKey) -> Result<(), ErrorSigner> {
    remove_keys_set(database_name, &vec![multisigner.to_owned()], network_specs_key)
}

/// Function removes a set of addresses within one network by set of multisigner identifier and and network id
/// Function removes network_key from network_id vector for a defined set of database records
pub fn remove_keys_set(database_name: &str, multiselect: &Vec<MultiSigner>, network_specs_key: &NetworkSpecsKey) -> Result<(), ErrorSigner> {
    let mut id_batch = Batch::default();
    let mut events: Vec<Event> = Vec::new();
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    for multisigner in multiselect.iter() {
        let public_key = multisigner_to_public(multisigner);
        let address_key = AddressKey::from_multisigner(multisigner);
        let mut address_details = get_address_details(database_name, &address_key)?;
        let identity_history = IdentityHistory::get(&address_details.seed_name, &network_specs.encryption, &public_key, &address_details.path, &network_specs.genesis_hash.to_vec());
        events.push(Event::IdentityRemoved(identity_history));
        address_details.network_id = address_details.network_id.into_iter().filter(|id| id != network_specs_key).collect();
        if address_details.network_id.is_empty() {id_batch.remove(address_key.key())}
        else {id_batch.insert(address_key.key(), address_details.encode())}
    }
    TrDbCold::new()
        .set_addresses(id_batch) // modify existing address entries
        .set_history(events_to_batch::<Signer>(&database_name, events)?) // add corresponding history
        .apply::<Signer>(&database_name)
}

/// Add a bunch of new derived addresses, N+1, N+2, etc.
pub fn create_increment_set(increment: u32, multisigner: &MultiSigner, network_specs_key: &NetworkSpecsKey, seed_phrase: &str, database_name: &str) -> Result<(), ErrorSigner> {
    let address_details = get_address_details(database_name, &AddressKey::from_multisigner(multisigner))?;
    let existing_identities = addresses_set_seed_name_network(database_name, &address_details.seed_name, network_specs_key)?;
    let mut last_index = 0;
    for (_, details) in existing_identities.iter() {
        if let Some(("", suffix)) = details.path.split_once(&address_details.path) {
            if let Some(could_be_number) = suffix.get(2..) {
                if let Ok(index) = could_be_number.parse::<u32>() {
                    last_index = std::cmp::max(index+1, last_index);
                }
            }
        }
    }
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let mut identity_adds: Vec<(AddressKey, AddressDetails)> = Vec::new();
    let mut current_events: Vec<Event> = Vec::new();
    for i in 0..increment {
        let path = address_details.path.to_string() + "//" + &(last_index+i).to_string();
        let (adds, events) = create_address::<Signer>(database_name, &identity_adds, &path, &network_specs, &address_details.seed_name, seed_phrase)?;
        identity_adds = adds;
        current_events.extend_from_slice(&events);
    }
    let id_batch = upd_id_batch(Batch::default(), identity_adds);
    TrDbCold::new()
        .set_addresses(id_batch) // add created address
        .set_history(events_to_batch::<Signer>(&database_name, current_events)?) // add corresponding history
        .apply::<Signer>(&database_name)
}

/// Check derivation format and determine whether there is a password
pub fn check_derivation_format(path: &str) -> Result<bool, ErrorSigner> {
    match REG_PATH.captures(path) {
        Some(caps) => Ok(caps.name("password").is_some()),
        None => return Err(ErrorSigner::AddressGeneration(AddressGeneration::Extra(ExtraAddressGenerationSigner::InvalidDerivation))),
    }
}

/// Function to cut the password to be verified later in UI.
/// Expects password, if sees no password, returns error.
pub fn cut_path(path: &str) -> Result<(String, String), ErrorSigner> {
    match REG_PATH.captures(path) {
        Some(caps) => {
            let cropped_path = match caps.name("path") {
                Some(a) => a.as_str().to_string(),
                None => "".to_string(),
            };
            match caps.name("password") {
                Some(pwd) => Ok((cropped_path, pwd.as_str().to_string())),
                None => return Err(ErrorSigner::Interface(InterfaceSigner::LostPwd)),
            }
        },
        None => return Err(ErrorSigner::AddressGeneration(AddressGeneration::Extra(ExtraAddressGenerationSigner::InvalidDerivation))),
    }
}

/// Generate new identity (api for create_address())
/// Function is open to user interface
pub fn try_create_address (seed_name: &str, seed_phrase: &str, path: &str, network_specs_key: &NetworkSpecsKey, database_name: &str) -> Result<(), ErrorSigner> {
    match derivation_no_pwd_exists(seed_name, path, network_specs_key, database_name)? {
        Some((multisigner, address_details)) => return Err(<Signer>::address_generation_common(AddressGenerationCommon::DerivationExists(multisigner, address_details, network_specs_key.to_owned()))),
        None => {
            let network_specs = get_network_specs(database_name, network_specs_key)?;
            let (adds, events) = create_address::<Signer>(database_name, &Vec::new(), path, &network_specs, seed_name, seed_phrase)?;
            let id_batch = upd_id_batch(Batch::default(), adds);
            TrDbCold::new()
                .set_addresses(id_batch) // add created address
                .set_history(events_to_batch::<Signer>(&database_name, events)?) // add corresponding history
                .apply::<Signer>(&database_name)
        },
    }
}

/// Function to check if proposed address collides,
/// i.e. if the proposed path for given seed name and network specs key already exists
pub fn derivation_no_pwd_exists (seed_name: &str, path: &str, network_specs_key: &NetworkSpecsKey, database_name: &str) -> Result<Option<(MultiSigner, AddressDetails)>, ErrorSigner> {
    if check_derivation_format(path)? {Ok(None)}
    else {
        let mut found_collision = None;
        for (multisigner, address_details) in get_all_addresses (database_name)?.into_iter() {
            if (address_details.seed_name == seed_name)&&(address_details.path == path)&&(address_details.network_id.contains(network_specs_key))&&(!address_details.has_pwd) {
                found_collision = Some((multisigner, address_details));
                break;
            }
        }
        Ok(found_collision)
    }
}

/// Function to generate identities batch with Alice information
pub fn generate_test_identities (database_name: &str) -> Result<(), ErrorActive> {
    let (id_batch, events) = {
        let entry_batch = make_batch_clear_tree::<Active>(&database_name, ADDRTREE)?;
        let mut events = vec![Event::IdentitiesWiped];
        let (mut id_batch, new_events) = populate_addresses::<Active>(database_name, entry_batch, "Alice", ALICE_SEED_PHRASE, true)?;
        events.extend_from_slice(&new_events);
        for network_specs in get_default_chainspecs().iter() {
            if (network_specs.name == "westend")&&(network_specs.encryption == Encryption::Sr25519) {
                let (adds, updated_events) = create_address::<Active>(database_name, &Vec::new(), "//Alice", network_specs, "Alice", ALICE_SEED_PHRASE)?;
                id_batch = upd_id_batch(id_batch, adds);
                events.extend_from_slice(&updated_events);
            }
        }
        (id_batch, events)
    };
    TrDbCold::new()
        .set_addresses(id_batch) // add created address
        .set_history(events_to_batch::<Active>(&database_name, events)?) // add corresponding history
        .apply::<Active>(&database_name)
}

/// Function to remove all identities associated with given seen_name
/// Function is open to the user interface
pub fn remove_seed (database_name: &str, seed_name: &str) -> Result<(), ErrorSigner> {
    let mut identity_batch = Batch::default();
    let mut events: Vec<Event> = Vec::new();
    let id_set = get_addresses_by_seed_name(database_name, seed_name)?;
    for (multisigner, address_details) in id_set.iter() {
        let address_key = AddressKey::from_multisigner(multisigner);
        identity_batch.remove(address_key.key());
        let public_key = multisigner_to_public(&multisigner);
        for network_specs_key in address_details.network_id.iter() {
            let (genesis_hash_vec, _) = network_specs_key.genesis_hash_encryption::<Signer>(SpecsKeySource::AddrTree(address_key.to_owned()))?;
            let identity_history = IdentityHistory::get(&seed_name, &address_details.encryption, &public_key, &address_details.path, &genesis_hash_vec);
            events.push(Event::IdentityRemoved(identity_history));
        }
    }
    TrDbCold::new()
        .set_addresses(identity_batch) // modify addresses
        .set_history(events_to_batch::<Signer>(&database_name, events)?) // add corresponding history
        .apply::<Signer>(&database_name)
}

/// Function to import derivations without password into Signer from qr code
/// i.e. create new (address key + address details) entry,
/// or add network specs key to the existing one
pub fn import_derivations (checksum: u32, seed_name: &str, seed_phrase: &str, database_name: &str) -> Result<(), ErrorSigner> {
    let content_derivations = TrDbColdDerivations::from_storage(database_name, checksum)?;
    let network_specs = content_derivations.network_specs();
    let mut adds: Vec<(AddressKey, AddressDetails)> = Vec::new();
    let mut events: Vec<Event> = Vec::new();
    for path in content_derivations.checked_derivations().iter() {
        match create_address::<Signer>(database_name, &adds, path, &network_specs, seed_name, seed_phrase) {
            Ok((mod_adds, mod_events)) => {
                adds = mod_adds;
                events.extend_from_slice(&mod_events);
            },
            Err(ErrorSigner::AddressGeneration(AddressGeneration::Common(AddressGenerationCommon::DerivationExists(_, _, _)))) => (),
            Err(e) => return Err(e),
        }
    }
    let identity_batch = upd_id_batch(Batch::default(), adds);
    let transaction_batch = make_batch_clear_tree::<Signer>(database_name, TRANSACTION)?;
    TrDbCold::new()
        .set_addresses(identity_batch) // modify addresses
        .set_history(events_to_batch::<Signer>(&database_name, events)?) // add corresponding history
        .set_transaction(transaction_batch) // clear transaction tree
        .apply::<Signer>(&database_name)
}

/// Function to check derivations before offering user to import them
pub fn check_derivation_set(derivations: &Vec<String>) -> Result<(), ErrorSigner> {
    for path in derivations.iter() {
        match REG_PATH.captures(path) {
            Some(caps) => if caps.name("password").is_some() {return Err(ErrorSigner::Input(InputSigner::OnlyNoPwdDerivations))},
            None => return Err(ErrorSigner::Input(InputSigner::InvalidDerivation(path.to_string()))),
        }
    }
    Ok(())
}

/// Function to prepare derivations export using regex and a text provided by the user
pub fn prepare_derivations_export (encryption: &Encryption, genesis_hash: &[u8;32], content: &str) -> Result<ContentDerivations, ErrorActive> {
    let mut derivations: Vec<String> = Vec::new();
    let content_set: Vec<&str> = content.trim().split('\n').collect();
    for path in content_set.iter() {
        if let Some(caps) = REG_PATH.captures(path) {
            if let Some(p) = caps.name("path") {
                if caps.name("password").is_none() {
                    derivations.push(p.as_str().to_string())
                }
            }
        }
    }
    if derivations.len() == 0 {return Err(ErrorActive::Input(InputActive::NoValidDerivationsToExport))}
    else {
        println!("Found and used {} valid password-free derivations:", derivations.len());
        for x in derivations.iter() {
            println!("\"{}\"", x);
        }
    }
    Ok(ContentDerivations::generate(encryption, genesis_hash, &derivations))
}

/// Function to display possible options of English code words from allowed words list
/// that start with already entered piece; for user requested easier seed recovery
/// Function interacts with user interface.
pub fn guess (word_part: &str) -> String {
    let dictionary = Language::English.wordlist();
    let words = dictionary.get_words_by_prefix(word_part);
    let words_set = {
        if words.len() > MAX_WORDS_DISPLAY {words[..MAX_WORDS_DISPLAY].to_vec()}
        else {words.to_vec()}
    };
    export_plain_vector(&words_set)
}

#[cfg(test)]
mod tests {
    use sled::{Db, Tree, open, Batch};
    use std::fs;
    
    use constants::ADDRTREE;
    use defaults::get_default_chainspecs;
    use definitions::{crypto::Encryption, keyring::{AddressKey, NetworkSpecsKey}, network_specs::Verifier};
    
    use crate::{cold_default::{populate_cold_no_metadata, signer_init_with_cert, populate_cold_release}, db_transactions::TrDbCold, helpers::{open_db, open_tree, upd_id_batch}, interface_signer::addresses_set_seed_name_network, manage_history::print_history};
    
    use super::*;

    #[test]
    fn test_generate_random_seed_phrase() {
        let random_phrase = generate_random_phrase(24).unwrap();
        assert!(Mnemonic::validate(&random_phrase, Language::English).is_ok());
        assert!(generate_random_phrase(1).is_err());
        let random_phrase2 = generate_random_phrase(24).unwrap();
        assert!(Mnemonic::validate(&random_phrase2, Language::English).is_ok());
        assert!(random_phrase2 != random_phrase);
    }

    #[test]
    fn test_check_for_seed_validity() {
        assert!(Mnemonic::validate(ALICE_SEED_PHRASE, Language::English).is_ok());
        assert!(Mnemonic::validate("the fox is triangular", Language::English).is_err());
        assert!(Mnemonic::validate("", Language::English).is_err());
        assert!(Mnemonic::validate("низ ехать подчиняться озеро занавеска дым корзина держать гонка одинокий подходящий прогулка", Language::English).is_err());
    }

    #[test]
    fn test_generate_default_addresses_for_alice() {
        let dbname = "for_tests/test_generate_default_addresses_for_Alice";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        try_create_seed("Alice", ALICE_SEED_PHRASE, true, dbname).unwrap();
        {
            let database = open_db::<Signer>(dbname).unwrap();
            let addresses = open_tree::<Signer>(&database, ADDRTREE).unwrap();
            assert!(addresses.len() == 4, "real addresses length: {}", addresses.len());
        }
        let chainspecs = get_default_chainspecs();
        let default_addresses = addresses_set_seed_name_network (dbname, "Alice", &NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash.to_vec(), &Encryption::Sr25519)).unwrap();
        assert!(default_addresses.len()>0);
        assert!("[(MultiSigner::Sr25519(46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a (5DfhGyQd...)), AddressDetails { seed_name: \"Alice\", path: \"\", has_pwd: false, network_id: [NetworkSpecsKey([1, 128, 145, 177, 113, 187, 21, 142, 45, 56, 72, 250, 35, 169, 241, 194, 81, 130, 251, 142, 32, 49, 59, 44, 30, 180, 146, 25, 218, 122, 112, 206, 144, 195]), NetworkSpecsKey([1, 128, 176, 168, 212, 147, 40, 92, 45, 247, 50, 144, 223, 183, 230, 31, 135, 15, 23, 180, 24, 1, 25, 122, 20, 156, 169, 54, 84, 73, 158, 163, 218, 254]), NetworkSpecsKey([1, 128, 225, 67, 242, 56, 3, 172, 80, 232, 246, 248, 230, 38, 149, 209, 206, 158, 78, 29, 104, 170, 54, 193, 205, 44, 253, 21, 52, 2, 19, 243, 66, 62])], encryption: Sr25519 }), (MultiSigner::Sr25519(64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05 (5ELf63sL...)), AddressDetails { seed_name: \"Alice\", path: \"//kusama\", has_pwd: false, network_id: [NetworkSpecsKey([1, 128, 176, 168, 212, 147, 40, 92, 45, 247, 50, 144, 223, 183, 230, 31, 135, 15, 23, 180, 24, 1, 25, 122, 20, 156, 169, 54, 84, 73, 158, 163, 218, 254])], encryption: Sr25519 })]" == format!("{:?}", default_addresses), "Default addresses:\n{:?}", default_addresses);
        let database: Db = open(dbname).unwrap();
        let identities: Tree = database.open_tree(ADDRTREE).unwrap();
        let test_key = AddressKey::from_parts(&hex::decode("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a").unwrap(), &Encryption::Sr25519).unwrap();
        assert!(identities.contains_key(test_key.key()).unwrap());
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn must_check_for_valid_derivation_phrase() {
        assert!(!check_derivation_format("").expect("valid empty path"));
        assert!(check_derivation_format("//").is_err());
        assert!(!check_derivation_format("//path1").expect("valid path1"));
        assert!(!check_derivation_format("//path/path").expect("soft derivation"));
        assert!(!check_derivation_format("//path//path").expect("hard derivation"));
        assert!(check_derivation_format("//path///password").expect("path with password"));
        assert!(check_derivation_format("///").is_err());
        assert!(!check_derivation_format("//$~").expect("weird symbols"));
        assert!(check_derivation_format("abraca dabre").is_err());
        assert!(check_derivation_format("////").expect("//// - password is /"));
        assert!(check_derivation_format("//path///password///password").expect("password///password is a password"));
        assert!(!check_derivation_format("//путь").expect("valid utf8 abomination"));
    }

    #[test]
    fn test_derive() { 
        let dbname = "for_tests/test_derive";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let chainspecs = get_default_chainspecs();
        println!("[0]: {:?}, [1]: {:?}", chainspecs[0].name, chainspecs[1].name);
        let seed_name = "Alice";
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash.to_vec(), &Encryption::Sr25519);
        let network_id_1 = NetworkSpecsKey::from_parts(&chainspecs[1].genesis_hash.to_vec(), &Encryption::Sr25519);
        let both_networks = vec![network_id_0.to_owned(), network_id_1.to_owned()];
        let only_one_network = vec![network_id_0.to_owned()];

        try_create_seed(seed_name, ALICE_SEED_PHRASE, true, dbname).unwrap();
        let (adds1, events1) = {
            create_address::<Signer>(dbname, &Vec::new(), "//Alice", &chainspecs[0], seed_name, ALICE_SEED_PHRASE).unwrap()
        };
        TrDbCold::new()
            .set_addresses(upd_id_batch(Batch::default(), adds1)) // modify addresses
            .set_history(events_to_batch::<Signer>(&dbname, events1).unwrap()) // add corresponding history
            .apply::<Signer>(&dbname).unwrap();
        let (adds2, events2) = {
            create_address::<Signer>(dbname, &Vec::new(), "//Alice", &chainspecs[1], seed_name, ALICE_SEED_PHRASE).unwrap()
        };
        TrDbCold::new()
            .set_addresses(upd_id_batch(Batch::default(), adds2)) // modify addresses
            .set_history(events_to_batch::<Signer>(&dbname, events2).unwrap()) // add corresponding history
            .apply::<Signer>(&dbname).unwrap();
        let (adds3, events3) = {
            create_address::<Signer>(dbname, &Vec::new(), "//Alice/1", &chainspecs[0], seed_name, ALICE_SEED_PHRASE).unwrap()
        };
        TrDbCold::new()
            .set_addresses(upd_id_batch(Batch::default(), adds3)) // modify addresses
            .set_history(events_to_batch::<Signer>(&dbname, events3).unwrap()) // add corresponding history
            .apply::<Signer>(&dbname).unwrap();
        let identities = get_addresses_by_seed_name (&dbname, seed_name).unwrap();
        println!("{:?}", identities);
        let mut flag0 = false;
        let mut flag1 = false;
        for (_, details) in identities {
            flag0 = flag0 || details.network_id == both_networks;
            flag1 = flag1 || details.network_id == only_one_network;
        }
        assert!(flag0, "Something is wrong with //Alice");
        assert!(flag1, "Something is wrong with //Alice/1");
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn test_identity_deletion() {
        let dbname = "for_tests/test_identity_deletion";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        try_create_seed("Alice", ALICE_SEED_PHRASE, true, dbname).unwrap();
        let chainspecs = get_default_chainspecs();
        let network_specs_key_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash.to_vec(), &Encryption::Sr25519);
        let network_specs_key_1 = NetworkSpecsKey::from_parts(&chainspecs[1].genesis_hash.to_vec(), &Encryption::Sr25519);
        let mut identities = addresses_set_seed_name_network (dbname, "Alice", &network_specs_key_0).expect("Alice should have some addresses by default");
        println!("{:?}", identities);
        let (key0, _) = identities.remove(0); //TODO: this should be root key
        let (key1, _) = identities.remove(0); //TODO: this should be network-specific key
        remove_key(dbname, &key0, &network_specs_key_0).expect("delete an address");
        remove_key(dbname, &key1, &network_specs_key_0).expect("delete another address");
        let identities = addresses_set_seed_name_network (dbname, "Alice", &network_specs_key_0).expect("Alice still should have some addresses after deletion of two");
        for (address_key, _) in identities {
            assert_ne!(address_key, key0);
            assert_ne!(address_key, key1);
        }
        let identities = addresses_set_seed_name_network (dbname, "Alice", &network_specs_key_1).expect("Alice still should have some addresses after deletion of two");
        let mut flag_to_check_key0_remains = false;
        for (address_key, _) in identities {
            if address_key == key0 {
                flag_to_check_key0_remains = true;
            }
            assert_ne!(address_key, key1);
        }
        assert!(flag_to_check_key0_remains, "An address that should have only lost network was removed entirely");
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn history_with_identities() {
        let dbname = "for_tests/history_with_identities";
        populate_cold_release(dbname).unwrap();
        signer_init_with_cert(dbname).unwrap();
        let history_printed = print_history(dbname).unwrap();
        let element1 = r#"{"event":"database_initiated"}"#;
        let element2 = r#"{"event":"general_verifier_added","payload":{"hex":"c46a22b9da19540a77cbde23197e5fd90485c72b4ecf3c599ecca6998f39bd57","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035149444154789cedd8c16d14411085611cc2128d33e18ac8802311702403c4954c1c0d1bc2d24f5699ea764dbdaaeeead54acc7f581f76d4fde6932c199e6eb7db87ffbdbb205cafd7e94b2e97cb53fbb1b52d082b2fcdda81528ab0f3e5c72a314a10665efef9f3c7f6d9f7f2eb4ffbcc5581b18430f3f2c802906620d00ac634c20e00e9de105308b3006827029a814821acbcbcb41b41ca6084112a00d0bd1050142284900178f9fefe259fbffd7ba92c023b8f1581a008ab00921eee413000499fc762106508de60490fb720a200923ecf6b09210a802a47a3eaf33c8843840c00aa1e5d7d1e3a823011b200a87a74f57992051146a8fe1dfef9e3d23efbbe7cbdb6cfd7b2e7b17d5208210a20e98bbce17ab005204521f479d17dd2084111bc0b247d91355c0ff6002406a1cfcbee432ec20880662ef1ca22b066f7698813a1f5866001a0d94b8e7a14042410e508d64bea97b2be1f63cfebefb3fb746104e45da42fb0064b7a78f573d17d632904645da42ff0064b7ab8f53cfb7e4c3fcff65975080c20527634abfabca30071229c084904f6975b76f4cb6fe3bc4f0be7917d478511ac0b247d9137bc1b6c00485188eebce03eab10827781a42fb28677831d00894174e725f78d6d4160651158abfb4e84d689d0da82407f879308f4bce4beb11002f22ed2175883a56eb803c100a4eebce03eab3002b22ed2177883a56eb801110590baf3c8bea35208acec6856f579479d08ad13a1f586801804fbf77a76b4f53cfb7e4c3fcff65901a0fd78fdff04e421581748fa226fb81e5cfd5c74df5818c1bb40d21759c3f560ebfb31f6bcfe3ebb4fb70d8165bdd4987e49d6cabe7708c88258b9c4ea511004009d08ad0e018d10d94bd85f6e5904765e761fd200882220ef227d813558d2c33d080620e9f3a2fb248a80a210fa026fb0a4875b105100499fc7f64923000a23b0b2a359d5e74961049485a81e5d7d1eb200d02102ca40548fae3eef0800b908280a911dcd7e87b3e7797900a80c0179c3f5600b408a42e8f358cb086815420ff6002406a1cf6331001442401908af2cc24a11001446401510f7428802a01482b482b11b21f3f2d214029a85d889300380a611d00e887b03a025046906c3829801587979a904419ac198ade2e5a55204692746e5cb4b5b10c6565076bcf4d85d101ebdbfeadfbfac75cbd0ab0000000049454e44ae426082","encryption":"sr25519"}}"#;
        assert!(history_printed.contains(element1), "\nReal history check1:\n{}", history_printed);
        assert!(history_printed.contains(element2), "\nReal history check2:\n{}", history_printed);
        try_create_seed("Alice", ALICE_SEED_PHRASE, true, dbname).unwrap();
        let history_printed_after_create_seed = print_history(dbname).unwrap();
        let element3 = r#""events":[{"event":"seed_created","payload":"Alice"},{"event":"identity_added","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","path":"","network_genesis_hash":"91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"}},{"event":"identity_added","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","path":"//polkadot","network_genesis_hash":"91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3"}},{"event":"identity_added","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","path":"","network_genesis_hash":"b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"}},{"event":"identity_added","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","path":"//kusama","network_genesis_hash":"b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe"}},{"event":"identity_added","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","path":"","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}},{"event":"identity_added","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","path":"//westend","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}}]"#;
        assert!(history_printed_after_create_seed.contains(element1), "\nReal history check3:\n{}", history_printed_after_create_seed);
        assert!(history_printed_after_create_seed.contains(element2), "\nReal history check4:\n{}", history_printed_after_create_seed);
        assert!(history_printed_after_create_seed.contains(element3), "\nReal history check5:\n{}", history_printed_after_create_seed);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn word_search_1() {
        let word_part = "dri";
        let out = guess(word_part);
        let out_expected = r#"["drift","drill","drink","drip","drive"]"#;
        assert!(out == out_expected, "Found different word set:\n{:?}", out);
    }
    
    #[test]
    fn word_search_2() {
        let word_part = "umbra";
        let out = guess(word_part);
        let out_expected = r#"[]"#;
        assert!(out == out_expected, "Found different word set:\n{:?}", out);
    }
    
    #[test]
    fn word_search_3() {
        let word_part = "котик";
        let out = guess(word_part);
        let out_expected = r#"[]"#;
        assert!(out == out_expected, "Found different word set:\n{:?}", out);
    }
    
    #[test]
    fn word_search_4() {
        let word_part = "";
        let out = guess(word_part);
        let out_expected = r#"["abandon","ability","able","about","above","absent","absorb","abstract"]"#;
        assert!(out == out_expected, "Found different word set:\n{:?}", out);
    }
    
    #[test]
    fn word_search_5() {
        let word_part = " ";
        let out = guess(word_part);
        let out_expected = r#"[]"#;
        assert!(out == out_expected, "Found different word set:\n{:?}", out);
    }
    
    #[test]
    fn word_search_6() {
        let word_part = "s";
        let out = guess(word_part);
        let out_expected = r#"["sad","saddle","sadness","safe","sail","salad","salmon","salon"]"#;
        assert!(out == out_expected, "Found different word set:\n{:?}", out);
    }
    
    #[test]
    fn word_search_7() {
        let word_part = "se";
        let out = guess(word_part);
        let out_expected = r#"["sea","search","season","seat","second","secret","section","security"]"#;
        assert!(out == out_expected, "Found different word set:\n{:?}", out);
    }
    
    #[test]
    fn word_search_8() {
        let word_part = "sen";
        let out = guess(word_part);
        let out_expected = r#"["senior","sense","sentence"]"#;
        assert!(out == out_expected, "Found different word set:\n{:?}", out);
    }
    
    fn get_multisigner_path_set(dbname: &str) -> Vec<(MultiSigner, String)> {
        let db = open_db::<Signer>(dbname).unwrap();
        let identities = open_tree::<Signer>(&db, ADDRTREE).unwrap();
        let mut multisigner_path_set: Vec<(MultiSigner, String)> = Vec::new();
        for x in identities.iter() {
            if let Ok(a) = x {
                let (multisigner, address_details) = AddressDetails::process_entry_checked::<Signer>(a).unwrap();
                multisigner_path_set.push((multisigner, address_details.path.to_string()))
            }
        }
        multisigner_path_set
    }
    
    #[test]
    fn increment_identities_1() {
        let dbname = "for_tests/increment_identities_1";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        {
            let db = open_db::<Signer>(dbname).unwrap();
            let identities = open_tree::<Signer>(&db, ADDRTREE).unwrap();
            assert!(identities.len()==0);
        }
        let chainspecs = get_default_chainspecs();
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash.to_vec(), &Encryption::Sr25519);
        try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).unwrap();
        let multisigner_path_set = get_multisigner_path_set(dbname);
        assert!(multisigner_path_set.len() == 1, "Wrong number of identities: {:?}", multisigner_path_set);
        println!("{}", multisigner_path_set[0].1);
        create_increment_set(4, &multisigner_path_set[0].0, &network_id_0, ALICE_SEED_PHRASE, dbname).unwrap();
        let multisigner_path_set = get_multisigner_path_set(dbname);
        assert!(multisigner_path_set.len() == 5, "Wrong number of identities after increment: {:?}", multisigner_path_set);
        let path_set: Vec<String> = multisigner_path_set.iter().map(|(_, path)| path.to_string()).collect();
        assert!(path_set.contains(&String::from("//Alice//0")));
        assert!(path_set.contains(&String::from("//Alice//1")));
        assert!(path_set.contains(&String::from("//Alice//2")));
        assert!(path_set.contains(&String::from("//Alice//3")));
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn increment_identities_2() {
        let dbname = "for_tests/increment_identities_2";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        {
            let db = open_db::<Signer>(dbname).unwrap();
            let identities = open_tree::<Signer>(&db, ADDRTREE).unwrap();
            assert!(identities.len()==0);
        }
        let chainspecs = get_default_chainspecs();
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash.to_vec(), &Encryption::Sr25519);
        try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).unwrap();
        try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice//1", &network_id_0, dbname).unwrap();
        let multisigner_path_set = get_multisigner_path_set(dbname);
        let alice_multisigner_path = multisigner_path_set.iter().find(|(_, path)| path == "//Alice").unwrap();
        assert!(multisigner_path_set.len() == 2, "Wrong number of identities: {:?}", multisigner_path_set);
        create_increment_set(3, &alice_multisigner_path.0, &network_id_0, ALICE_SEED_PHRASE, dbname).unwrap();
        let multisigner_path_set = get_multisigner_path_set(dbname);
        assert!(multisigner_path_set.len() == 5, "Wrong number of identities after increment: {:?}", multisigner_path_set);
        let path_set: Vec<String> = multisigner_path_set.iter().map(|(_, path)| path.to_string()).collect();
        assert!(path_set.contains(&String::from("//Alice//2")));
        assert!(path_set.contains(&String::from("//Alice//3")));
        assert!(path_set.contains(&String::from("//Alice//4")));
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn increment_identities_3() {
        let dbname = "for_tests/increment_identities_3";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        {
            let db = open_db::<Signer>(dbname).unwrap();
            let identities = open_tree::<Signer>(&db, ADDRTREE).unwrap();
            assert!(identities.len()==0);
        }
        let chainspecs = get_default_chainspecs();
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash.to_vec(), &Encryption::Sr25519);
        try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).unwrap();
        try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice//1", &network_id_0, dbname).unwrap();
        let multisigner_path_set = get_multisigner_path_set(dbname);
        let alice_multisigner_path = multisigner_path_set.iter().find(|(_, path)| path == "//Alice//1").unwrap();
        assert!(multisigner_path_set.len() == 2, "Wrong number of identities: {:?}", multisigner_path_set);
        create_increment_set(3, &alice_multisigner_path.0, &network_id_0, ALICE_SEED_PHRASE, dbname).unwrap();
        let multisigner_path_set = get_multisigner_path_set(dbname);
        assert!(multisigner_path_set.len() == 5, "Wrong number of identities after increment: {:?}", multisigner_path_set);
        let path_set: Vec<String> = multisigner_path_set.iter().map(|(_, path)| path.to_string()).collect();
        assert!(path_set.contains(&String::from("//Alice//1//0")));
        assert!(path_set.contains(&String::from("//Alice//1//1")));
        assert!(path_set.contains(&String::from("//Alice//1//2")));
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn checking_derivation_set() {
        assert!(check_derivation_set(&vec!["/0".to_string(), "//Alice/westend".to_string(), "//secret//westend".to_string()]).is_ok());
        assert!(check_derivation_set(&vec!["/0".to_string(), "/0".to_string(), "//Alice/westend".to_string(), "//secret//westend".to_string()]).is_ok());
        assert!(check_derivation_set(&vec!["//remarkably///ugly".to_string()]).is_err());
        assert!(check_derivation_set(&vec!["no_path_at_all".to_string()]).is_err());
        assert!(check_derivation_set(&vec!["///".to_string()]).is_err());
    }
    
    #[test]
    fn creating_derivation_1() {
        let dbname = "for_tests/creating_derivation_1";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let chainspecs = get_default_chainspecs();
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash.to_vec(), &Encryption::Sr25519);
        assert!(try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).is_ok(), "Should be able to create //Alice derivation.");
        assert!(derivation_no_pwd_exists("Alice", "//Alice", &network_id_0, dbname).unwrap().is_some(), "Derivation should already exist.");
        match try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname) {
            Ok(()) => panic!("Should NOT be able to create //Alice derivation again."),
            Err(e) => assert!(<Signer>::show(&e) == "Error generating address. Seed Alice already has derivation //Alice for network specs key 0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe, public key d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d.", "Wrong error: {}", <Signer>::show(&e)),
        }
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn creating_derivation_2() {
        let dbname = "for_tests/creating_derivation_2";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let chainspecs = get_default_chainspecs();
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash.to_vec(), &Encryption::Sr25519);
        assert!(try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice///secret", &network_id_0, dbname).is_ok(), "Should be able to create //Alice/// secret derivation.");
        assert!(derivation_no_pwd_exists("Alice", "//Alice", &network_id_0, dbname).unwrap().is_none(), "Existing derivation has password and is diffenent.");
        assert!(try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).is_ok(), "Should be able to create //Alice derivation.");
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn creating_derivation_3() {
        let dbname = "for_tests/creating_derivation_3";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let chainspecs = get_default_chainspecs();
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash.to_vec(), &Encryption::Sr25519);
        assert!(try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice", &network_id_0, dbname).is_ok(), "Should be able to create //Alice derivation.");
        assert!(derivation_no_pwd_exists("Alice", "//Alice///secret", &network_id_0, dbname).unwrap().is_none(), "Existing derivation has no password and is diffenent.");
        assert!(try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice///secret", &network_id_0, dbname).is_ok(), "Should be able to create //Alice///secret derivation.");
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn creating_derivation_4() {
        let dbname = "for_tests/creating_derivation_4";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let chainspecs = get_default_chainspecs();
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash.to_vec(), &Encryption::Sr25519);
        assert!(try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice///secret1", &network_id_0, dbname).is_ok(), "Should be able to create //Alice///secret1 derivation.");
        assert!(derivation_no_pwd_exists("Alice", "//Alice///secret2", &network_id_0, dbname).unwrap().is_none(), "Existing derivation has different password.");
        assert!(try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice///secret2", &network_id_0, dbname).is_ok(), "Should be able to create //Alice///secret2 derivation.");
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn creating_derivation_5() {
        let dbname = "for_tests/creating_derivation_5";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let chainspecs = get_default_chainspecs();
        let network_id_0 = NetworkSpecsKey::from_parts(&chainspecs[0].genesis_hash.to_vec(), &Encryption::Sr25519);
        assert!(try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice///secret", &network_id_0, dbname).is_ok(), "Should be able to create //Alice derivation.");
        assert!(derivation_no_pwd_exists("Alice", "//Alice///secret", &network_id_0, dbname).unwrap().is_none(), "Derivation exists, but has password.");
        match try_create_address("Alice", ALICE_SEED_PHRASE, "//Alice///secret", &network_id_0, dbname) {
            Ok(()) => panic!("Should NOT be able to create //Alice///secret derivation again."),
            Err(e) => assert!(<Signer>::show(&e) == "Error generating address. Seed Alice already has derivation //Alice///<password> for network specs key 0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe, public key 08a5e583f74f54f3811cb5f7d74e686d473e3a466fd0e95738707a80c3183b15.", "Wrong error: {}", <Signer>::show(&e)),
        }
        fs::remove_dir_all(dbname).unwrap();
    }
}

