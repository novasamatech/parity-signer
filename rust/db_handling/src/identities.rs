//! All db handling related to seeds and addresses
//! seed phrases should be stored in hw encrypted by
//! best available tool and here they are only processed in plaintext.
//! Zeroization is mostly delegated to os

use sled::{Db, Tree};
use sp_core::{Pair, ed25519, sr25519, ecdsa};
use parity_scale_codec::Encode;
use regex::Regex;
use constants::{ADDRTREE, HISTORY, SPECSTREE};
use definitions::{crypto::Encryption, history::Event, network_specs::{NetworkKey, generate_network_key}, users::{AddressDetails, SeedObject, AddressKey, print_as_base58, IdentityHistory}};
use bip39::{Language, Mnemonic, MnemonicType};
use zeroize::Zeroize;
use lazy_static::lazy_static;
use anyhow;
use qrcode_static::png_qr_from_string;

use crate::error::{Error, NotFound, NotHex, CreateAddress};
use crate::chainspecs::get_network;
use crate::helpers::{open_db, open_tree, drop_tree, flush_db, insert_into_tree, remove_from_tree, unhex, get_and_decode_chain_specs, decode_chain_specs, decode_address_details, get_network_encryption, generate_address_key, reverse_address_key, reverse_network_key};
use crate::manage_history::{enter_events_into_tree};


lazy_static! {
// stolen from sp_core
// removed seed phrase part
// last '+' used to be '*', but empty password is an error
    static ref REG_PATH: Regex = Regex::new(r"^(?P<path>(//?[^/]+)*)(///(?P<password>.+))?$").expect("known value");
}

/// get all identities from database for given seed_name (internal use only!)
fn get_seed_identities (database: &Db, seed_name: &str) -> anyhow::Result<Vec<(AddressKey, AddressDetails)>> {
    let identities = open_tree(&database, ADDRTREE)?;
    filter_addresses_by_seed_name (&identities, seed_name)
}

/// get all identities from database
fn get_all_identities (database: &Db) -> anyhow::Result<Vec<(AddressKey, AddressDetails)>> {
    let identities = open_tree(&database, ADDRTREE)?;
    let mut out: Vec<(AddressKey, AddressDetails)> = Vec::new();
    for x in identities.iter() {
        if let Ok((key, value)) = x {
            let address_key = key.to_vec();
            let address_details = decode_address_details(value)?;
            out.push((address_key, address_details));
        }
    }
    Ok(out)
}

/// filter identities by given seed_name
fn filter_addresses_by_seed_name (identities: &Tree, seed_name: &str) -> anyhow::Result<Vec<(AddressKey, AddressDetails)>> {
    let mut out: Vec<(AddressKey, AddressDetails)> = Vec::new();
    for x in identities.iter() {
        if let Ok((key, value)) = x {
            let address_key = key.to_vec();
            let address_details = decode_address_details(value)?;
            if address_details.seed_name == seed_name {
                out.push((address_key, address_details));
            }
        }
    }
    Ok(out)
}

/// filter identities by given seed_name and name
fn filter_addresses_by_seed_name_and_name (identities: &Tree, seed_name: &str, name: &str) -> anyhow::Result<Vec<(AddressKey, AddressDetails)>> {
    let mut out: Vec<(AddressKey, AddressDetails)> = Vec::new();
    for x in identities.iter() {
        if let Ok((key, value)) = x {
            let address_key = key.to_vec();
            let address_details = decode_address_details(value)?;
            if (address_details.seed_name == seed_name)&&(address_details.name == name) {
                out.push((address_key, address_details));
            }
        }
    }
    Ok(out)
}

/// get all identities for given seed_name and network_key as hex string
pub fn get_relevant_identities (seed_name: &str, network_key_string: &str, database_name: &str) -> anyhow::Result<Vec<(AddressKey, AddressDetails)>> {
    
    let network_key = unhex(network_key_string, NotHex::NetworkKey)?; //TODO: add whatever is needed for parachains?
    let database = open_db(database_name)?;
    let identities_out = {
        if seed_name == "" {get_all_identities(&database)?}
        else {get_seed_identities(&database, seed_name)?}
    };
    Ok(identities_out.into_iter().filter(|(_, details)| details.network_id.contains(&network_key)).collect())
}

/// Function to print all relevant identities for given seed_name and network_key as hex string
pub fn print_relevant_identities (seed_name: &str, network_key_string: &str, database_name: &str) -> anyhow::Result<String> {
    let relevant_identities = get_relevant_identities (seed_name, network_key_string, database_name)?;
    let network_specs = get_network(database_name, network_key_string)?;
    let mut out = String::from("[");
    for (i, (address_key, address_details)) in relevant_identities.iter().enumerate() {
        if i>0 {out.push_str(",")}
        let base58print = match print_as_base58 (&address_key, address_details.encryption, Some(network_specs.base58prefix)) {
            Ok(a) => a,
            Err(e) => return Err(Error::Base58(e.to_string()).show()),
        };
        let public_key_helper = reverse_address_key(&address_key)?;
        let new = format!("{{\"public_key\":\"{}\",\"encryption\":\"{}\",\"ss58\":\"{}\",\"path\":\"{}\",\"has_password\":\"{}\",\"name\":\"{}\",\"seed_name\":\"{}\"}}", hex::encode(public_key_helper.public_key), public_key_helper.encryption.show(), base58print, address_details.path, address_details.has_pwd, address_details.name, address_details.seed_name);
        out.push_str(&new);
    }
    out.push_str("]");
    Ok(out)
}

/// Function to print all identities for all seed names;
/// ss58 line associated with each of public keys is printed with default base58prefix
pub fn print_all_identities (database_name: &str) -> anyhow::Result<String> {
    
    let database = open_db(database_name)?;
    let identities = open_tree(&database, ADDRTREE)?;
    let mut out = String::from("[");
    for (i, x) in identities.iter().enumerate() {
        if let Ok((address_key, address_details_encoded)) = x {
            if i>0 {out.push_str(",")}
            let address_details = decode_address_details(address_details_encoded)?;
            let base58print = match print_as_base58 (&address_key.to_vec(), address_details.encryption, None) {
                Ok(a) => a,
                Err(e) => return Err(Error::Base58(e.to_string()).show()),
            };
            let public_key_helper = reverse_address_key(&address_key.to_vec())?;
            let new = format!("{{\"public_key\":\"{}\",\"encryption\":\"{}\",\"ss58\":\"{}\",\"path\":\"{}\",\"has_password\":\"{}\",\"name\":\"{}\",\"seed_name\":\"{}\"}}", hex::encode(public_key_helper.public_key), public_key_helper.encryption.show(), base58print, address_details.path, address_details.has_pwd, address_details.name, address_details.seed_name);
            out.push_str(&new);
        }
    }
    out.push_str("]");
    Ok(out)
}

/// generate random phrase with given number of words
fn generate_random_phrase (words_number: u32) -> anyhow::Result<String> {
    let mnemonic_type = MnemonicType::for_word_count(words_number as usize)?;
    let mnemonic = Mnemonic::new(mnemonic_type, Language::English);
    Ok(mnemonic.into_phrase())
}

/// Create address from seed and path and insert it into the database
fn create_address (database: &Db, path: &str, network_key: NetworkKey, name: &str, seed_object: &SeedObject, has_pwd: bool) -> anyhow::Result<()> {

    // TODO: check zeroize

    let mut full_address = seed_object.seed_phrase.to_owned() + path;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    let history = open_tree(&database, HISTORY)?;
    
    if !chainspecs.contains_key(&network_key)? {return Err(Error::CreateAddress(CreateAddress::NetworkNotFound).show())}
    if get_network_encryption (&chainspecs, &network_key)? != seed_object.encryption {return Err(Error::CreateAddress(CreateAddress::EncryptionMismatch).show())}
    
    let public_key = match seed_object.encryption {
        Encryption::Ed25519 => {
            match ed25519::Pair::from_string(&full_address, None) {
                Ok(a) => a.public().to_vec(),
                Err(_) => return Err(Error::CreateAddress(CreateAddress::Ed25519).show()),
            }
        },
        Encryption::Sr25519 => {
            match sr25519::Pair::from_string(&full_address, None) {
                Ok(a) => a.public().to_vec(),
                Err(_) => return Err(Error::CreateAddress(CreateAddress::Sr25519).show()),
            }
        },
        Encryption::Ecdsa => {
            match ecdsa::Pair::from_string(&full_address, None) {
                Ok(a) => a.public().0.to_vec(),
                Err(_) => return Err(Error::CreateAddress(CreateAddress::Ecdsa).show()),
            }
        },
    };
    full_address.zeroize();
    
    let identity_history_print = IdentityHistory {
        seed_name: &seed_object.seed_name,
        encryption: seed_object.encryption,
        public_key: &hex::encode(&public_key),
        path: &path,
        network_genesis_hash: &hex::encode(&reverse_network_key(&network_key)?.genesis_hash),
    }.show();
    let events = vec![Event::IdentityAdded(identity_history_print)];
    
    let address_key = generate_address_key(&public_key, seed_object.encryption)?;
    
    let identities = open_tree(&database, ADDRTREE)?;
    let seed_name = seed_object.seed_name.to_string();
// This address might be already created; maybe we just need to allow its use in another network?
    match identities.get(&address_key) {
        Ok(Some(address_details_encoded)) => {
        // TODO: check that all collisions are handled
            let mut address_details = decode_address_details(address_details_encoded)?;
        // Check if something else resolved into this keypair
            if address_details.name != name || address_details.path != path {return Err(Error::AddressKeyCollision{name: address_details.name, seed_name: address_details.seed_name}.show())}
        // Append network to list of allowed networks
            if !address_details.network_id.contains(&network_key) {
                address_details.network_id.push(network_key);
                insert_into_tree(address_key, address_details.encode(), &identities)?;
                enter_events_into_tree(&history, events)?;
            }
        }
        Ok(None) => {
        // Check for collisions in name
            let collided = filter_addresses_by_seed_name_and_name(&identities, &seed_name, name)?;
            if collided.len() !=0 {return Err(Error::IdentityExists.show())}
            
            let cropped_path = match REG_PATH.captures(path) {
                Some(caps) => match caps.name("path") {
                    Some(a) => a.as_str(),
                    None => "",
                },
                None => "",
            };
            let address_details = AddressDetails {
                seed_name,
                path: cropped_path.to_string(),
                has_pwd,
                name: name.to_string(),
                network_id: vec![network_key],
                encryption: seed_object.encryption,
            };
            insert_into_tree(address_key, address_details.encode(), &identities)?;
            enter_events_into_tree(&history, events)?;
        },
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
    Ok(())
}


/// Create addresses for all default paths in all default networks, and insert them in the database
fn populate_addresses (database: &Db, seed_object: &SeedObject) -> anyhow::Result<()> {
// TODO: check zeroize
// TODO: compatibility with atomic operations
    let chainspecs = open_tree(&database, SPECSTREE)?;
    for x in chainspecs.iter() {
        if let Ok((network_key, network_specs_encoded)) = x {
            let network_specs = decode_chain_specs(network_specs_encoded, &network_key.to_vec())?;
            match create_address (database, "", network_key.to_vec(), "root address", seed_object, false) {
                Ok(()) => (),
                Err(e) => {
                    if e.to_string() == Error::CreateAddress(CreateAddress::EncryptionMismatch).show().to_string() {()}
                    else {return Err(e)}
                },
            }
            if let Err(_) = create_address (database, &network_specs.path_id, network_key.to_vec(), &format!("{} root address", network_specs.name), seed_object, false) {()}
        }
    }
    Ok(())
}

/// Generate new seed and populate all known networks with default accounts
pub fn try_create_seed (seed_name: &str, encryption_name: &str, seed_phrase_proposal: &str, seed_length: u32, database_name: &str) -> anyhow::Result<String> {
// TODO: atomize writes
    let database = open_db(database_name)?;
    let seed_phrase = match seed_phrase_proposal {
        "" => generate_random_phrase(seed_length)?,
        string => {
            Mnemonic::validate(string, Language::English)?;
            string.to_owned()
        }
    };

// TODO: zeroize seed

    let encryption = match encryption_name {
        "ed25519" => Encryption::Ed25519,
        "sr25519" => Encryption::Sr25519,
        "ecdsa" => Encryption::Ecdsa,
        _ => return Err(Error::UnknownEncryption.show()),
    };
    
    let seed_object = SeedObject {
        seed_name: seed_name.to_string(),
        seed_phrase: seed_phrase.to_string(),
        encryption: encryption,
    };

    populate_addresses(&database, &seed_object)?;
    flush_db(&database)?;
    Ok(seed_phrase)
}

/// Sanitize numbers in path (only for name suggestions!)
/// Removes zeroes
fn sanitize_number(could_be_number: &str) -> String {
    match could_be_number.parse::<u32>() {
        Ok(number) => number.to_string(),
        Err(_) => could_be_number.to_string(),
    }
}

/// Suggest name from path
// TODO: surprizingly - zeroize!
pub fn suggest_path_name(path_all: &str) -> String {
    let mut output = String::from("");
    if let Some(caps) = REG_PATH.captures(path_all) {
        if let Some(path) = caps.name("path") {
            if !path.as_str().is_empty() {
                for hard in path.as_str().split("//") {
                    let mut softened = hard.split("/");
                    if let Some(first) = softened.next() {
                        output.push_str(&sanitize_number(first));
                        let mut number_of_brackets = 0;
                        for soft in softened {
                            number_of_brackets+=1;
                            output.push_str(" (");
                            output.push_str(&sanitize_number(soft));
                        }
                        if number_of_brackets == 0 {
                            output.push_str(" ");
                        } else {
                            output.push_str(&") ".repeat(number_of_brackets));
                        }
                    }
                }
            }
        };
    }
    output = output.trim().to_string(); //is this good enough zeroization?
    output
}

/// Function removes identity as seen by user
/// Function removes network_key from network_id vector for database record with address_key corresponding to given public key
pub fn delete_address(pub_key: &str, network_key_string: &str, database_name: &str) -> anyhow::Result<()> {
    let database = open_db(database_name)?;
    let identities = open_tree(&database, ADDRTREE)?;
    let history = open_tree(&database, HISTORY)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    
    let network_key = unhex(network_key_string, NotHex::NetworkKey)?;
    let encryption = get_network_encryption (&chainspecs, &network_key)?;
    
    let address_key = generate_address_key(&unhex(pub_key, NotHex::PublicKey)?, encryption)?;

    match identities.get(&address_key) {
        Ok(Some(address_details_encoded)) => {
            let mut address_details = decode_address_details(address_details_encoded)?;
            let identity_history_print = IdentityHistory {
                seed_name: &address_details.seed_name,
                encryption,
                public_key: &pub_key,
                path: &address_details.path,
                network_genesis_hash: &hex::encode(&reverse_network_key(&network_key)?.genesis_hash),
            }.show();
            let events = vec![Event::IdentityRemoved(identity_history_print)];
            address_details.network_id = address_details.network_id.into_iter().filter(|id| *id != network_key).collect();
            if address_details.network_id.is_empty() {remove_from_tree(address_key, &identities)?}
            else {insert_into_tree(address_key, address_details.encode(), &identities)?}
            enter_events_into_tree(&history, events)?;
        },
        Ok(None) => return Err(Error::NotFound(NotFound::Address).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    Ok(())
}

/// Suggest address and name for weird N+1 feature request
pub fn suggest_n_plus_one(path: &str, seed_name: &str, network_key_string: &str, database_name: &str) -> anyhow::Result<String> {
    let identities = get_relevant_identities(seed_name, network_key_string, database_name)?;
    let mut last_index = 0;
    for (_, details) in identities {
        if let Some(("", suffix)) = details.path.split_once(path) {
            if let Some(could_be_number) = suffix.get(2..) {
                if let Ok(index) = could_be_number.parse::<u32>() {
                    last_index = std::cmp::max(index+1, last_index);
                }
            }
        }
    }
    Ok(path.to_string() + "//" + &last_index.to_string())
}

/// Check derivation format and determine whether there is a password
pub fn check_derivation_format(path: &str) -> anyhow::Result<bool> {
    match REG_PATH.captures(path) {
        Some(caps) => Ok(caps.name("password").is_some()),
        None => return Err(Error::InvalidDerivation.show()),
    }
}

/// Generate new identity (api for create_address())
pub fn try_create_address (id_name: &str, seed_name: &str, seed_phrase: &str, encryption_name: &str, path: &str, network_key_string: &str, has_pwd: bool, database_name: &str) -> anyhow::Result<()> {
    let database = open_db(database_name)?;

    let encryption = match encryption_name {
        "ed25519" => Encryption::Ed25519,
        "sr25519" => Encryption::Sr25519,
        "ecdsa" => Encryption::Ecdsa,
        _ => return Err(Error::UnknownEncryption.show()),
    };
    
    let seed_object = SeedObject {
        seed_name: seed_name.to_string(),
        seed_phrase: seed_phrase.to_string(),
        encryption,
    };

    let network_key = unhex(network_key_string, NotHex::NetworkKey)?;

    create_address(&database, path, network_key, id_name, &seed_object, has_pwd)
}

/// Function to populate test cold database with Alice information
pub fn load_test_identities (database_name: &str) -> anyhow::Result<()> {
    let database = open_db(database_name)?;
    drop_tree(&database, ADDRTREE)?;
    let history = open_tree(&database, HISTORY)?;
    enter_events_into_tree(&history, vec![Event::IdentitiesWiped])?;
    let alice_seed_object = SeedObject {
        seed_name: String::from("Alice"),
        seed_phrase: String::from("bottom drive obey lake curtain smoke basket hold race lonely fit walk"),
        encryption: Encryption::Sr25519,
    };
    populate_addresses (&database, &alice_seed_object)?;
    let westend_network_key = generate_network_key(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").expect("known value"), Encryption::Sr25519);
    create_address (&database, "//Alice", westend_network_key, "Alice_test_westend", &alice_seed_object, false)?;
    
    flush_db(&database)?;
    Ok(())
}


/// Function to remove all identities associated with given seen_name
pub fn remove_identities_for_seed (seed_name: &str, database_name: &str) -> anyhow::Result<()> {
    let database = open_db(database_name)?;
    let identities = open_tree(&database, ADDRTREE)?;
    let history = open_tree(&database, HISTORY)?;
    for x in identities.iter() {
        if let Ok((key, value)) = x {
            let address_details = decode_address_details(value)?;
            let public_key_helper = reverse_address_key(&key.to_vec())?;
            if address_details.seed_name == seed_name {
                remove_from_tree(key.to_vec(), &identities)?;
                let mut events: Vec<Event> = Vec::new();
                for y in address_details.network_id.iter() {
                    let identity_history_print = IdentityHistory {
                        seed_name: seed_name,
                        encryption: public_key_helper.encryption,
                        public_key: &hex::encode(&public_key_helper.public_key),
                        path: &address_details.path,
                        network_genesis_hash: &hex::encode(&reverse_network_key(&y)?.genesis_hash),
                    }.show();
                    events.push(Event::IdentityRemoved(identity_history_print));
                }
                enter_events_into_tree(&history, events)?;
            }
        }
    }
    flush_db(&database)?;
    Ok(())
}


/// Function to export identity as qr code readable by polkadot.js
/// Standard known format:
/// `substrate:{public_key as as_base58}:0x{network_key}:{seed_name}`
/// String is transformed into bytes, then into png qr code, then qr code
/// content is hexed so that it could be transferred into app
/// Note: if the resulting string is too long, seed_name is cut to length
pub fn export_identity (pub_key: &str, network_key_string: &str, database_name: &str) -> anyhow::Result<String> {
    
    let database = open_db(database_name)?;
    let identities = open_tree(&database, ADDRTREE)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    
    let network_key = unhex(network_key_string, NotHex::NetworkKey)?;
    let encryption = get_network_encryption (&chainspecs, &network_key)?;
    
    let address_key = generate_address_key(&unhex(pub_key, NotHex::PublicKey)?, encryption)?;

    let address_details = match identities.get(&address_key) {
        Ok(Some(address_details_encoded)) => {
            decode_address_details(address_details_encoded)?
        },
        Ok(None) => return Err(Error::NotFound(NotFound::Address).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    if address_details.network_id.contains(&network_key) {
        let network_specs = get_and_decode_chain_specs (&chainspecs, &network_key)?;
        let address_base58 = match print_as_base58 (&address_key, address_details.encryption, Some(network_specs.base58prefix)) {
            Ok(a) => a,
            Err(e) => return Err(Error::Base58(e.to_string()).show()),
        };
        let mut output = format!("substrate:{}:0x{}:{}", address_base58, hex::encode(&network_key), address_details.seed_name);
        if output.len() > 2953 {output = output[..2953].to_string();} // to fit into qr code, cut seed_name if needed
        Ok(hex::encode(png_qr_from_string(&output)?))
    }
    else {return Err(Error::NotFound(NotFound::NetworkKey).show())}
}


#[cfg(test)]
mod tests {
    use super::*;
    use definitions::{crypto::Encryption, defaults::get_default_chainspecs, network_specs::generate_network_key};
    use std::fs;
    use sled::{Db, Tree, open};
    use crate::{chainspecs::load_chainspecs, helpers::reverse_address_key};

    static SEED: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    static ENCRYPTION_NAME: &str = "sr25519";
    static FALSE_ENCRYPTION_NAME: &str = "ecdsa";

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
        assert!(Mnemonic::validate(SEED, Language::English).is_ok());
        assert!(Mnemonic::validate("the fox is triangular", Language::English).is_err());
        assert!(Mnemonic::validate("", Language::English).is_err());
        assert!(Mnemonic::validate("низ ехать подчиняться озеро занавеска дым корзина держать гонка одинокий подходящий прогулка", Language::English).is_err());
    }

    #[test]
    fn test_generate_random_account() {
        let dbname = "tests/test_generate_random_account";
        load_chainspecs(dbname).expect("create default database");
        try_create_seed("Randy", ENCRYPTION_NAME, "", 24, dbname).unwrap();
        let chainspecs = get_default_chainspecs();
        let random_addresses = get_relevant_identities("Randy", &hex::encode(generate_network_key(&chainspecs[0].genesis_hash.to_vec(), Encryption::Sr25519)), dbname).unwrap();
        assert!(random_addresses.len()>0);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    
    
    #[test]
    fn test_generate_random_account_bad_crypto() {
        let dbname = "tests/test_generate_random_account_bad_crypto";
        load_chainspecs(dbname).expect("create default database");
        try_create_seed("Kevin", FALSE_ENCRYPTION_NAME, "", 24, dbname).unwrap();
        let chainspecs = get_default_chainspecs();
        let random_addresses = get_relevant_identities("Kevin", &hex::encode(generate_network_key(&chainspecs[0].genesis_hash.to_vec(), Encryption::Sr25519)), dbname).unwrap();
        assert!(random_addresses.len()==0);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn test_generate_default_addresses_for_alice() {
        let dbname = "tests/test_generate_default_addresses_for_Alice";
        load_chainspecs(dbname).expect("create default database");
        try_create_seed("Alice", ENCRYPTION_NAME, SEED, 0, dbname).unwrap();
        let chainspecs = get_default_chainspecs();
        let default_addresses = get_relevant_identities("Alice", &hex::encode(generate_network_key(&chainspecs[0].genesis_hash.to_vec(), Encryption::Sr25519)), dbname).unwrap();
        assert!(default_addresses.len()>0);
        assert_eq!(r#"[([1, 70, 235, 221, 239, 140, 217, 187, 22, 125, 195, 8, 120, 215, 17, 59, 126, 22, 142, 111, 6, 70, 190, 255, 215, 125, 105, 211, 155, 173, 118, 180, 122], AddressDetails { seed_name: "Alice", path: "", has_pwd: false, name: "root address", network_id: [[1, 128, 133, 63, 175, 251, 252, 103, 19, 193, 248, 153, 191, 22, 84, 127, 207, 191, 115, 58, 232, 54, 27, 140, 160, 18, 150, 153, 208, 29, 79, 33, 129, 253], [1, 128, 145, 177, 113, 187, 21, 142, 45, 56, 72, 250, 35, 169, 241, 194, 81, 130, 251, 142, 32, 49, 59, 44, 30, 180, 146, 25, 218, 122, 112, 206, 144, 195], [1, 128, 176, 168, 212, 147, 40, 92, 45, 247, 50, 144, 223, 183, 230, 31, 135, 15, 23, 180, 24, 1, 25, 122, 20, 156, 169, 54, 84, 73, 158, 163, 218, 254], [1, 128, 225, 67, 242, 56, 3, 172, 80, 232, 246, 248, 230, 38, 149, 209, 206, 158, 78, 29, 104, 170, 54, 193, 205, 44, 253, 21, 52, 2, 19, 243, 66, 62]], encryption: Sr25519 }), ([1, 100, 163, 18, 53, 212, 191, 155, 55, 207, 237, 58, 250, 138, 166, 7, 84, 103, 95, 156, 73, 21, 67, 4, 84, 211, 101, 192, 81, 18, 120, 77, 5], AddressDetails { seed_name: "Alice", path: "//kusama", has_pwd: false, name: "kusama root address", network_id: [[1, 128, 176, 168, 212, 147, 40, 92, 45, 247, 50, 144, 223, 183, 230, 31, 135, 15, 23, 180, 24, 1, 25, 122, 20, 156, 169, 54, 84, 73, 158, 163, 218, 254]], encryption: Sr25519 })]"#, format!("{:?}", default_addresses)); //because JSON export is what we care about
        let database: Db = open(dbname).unwrap();
        let identities: Tree = database.open_tree(ADDRTREE).unwrap();
        let test_key = generate_address_key(&hex::decode("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a").unwrap(), Encryption::Sr25519).unwrap();
        println!("{:?}", test_key);
        assert!(identities.contains_key(test_key).unwrap());
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
    fn must_fail_on_duplicate_identity_name() { 
        let dbname = "tests/must_fail_on_duplicate_name";
        let path_should_fail_0 = "//path-should-fail-0";
        let path_should_succeed = "//path-should-succeed";
        let path_should_fail_1 = "//path-should-fail-1";
        let chainspecs = get_default_chainspecs();
        load_chainspecs(dbname).expect("create default database");
        try_create_seed("Alice", ENCRYPTION_NAME, SEED, 0, dbname).unwrap();
        assert!(try_create_address("root address", "Alice", SEED, ENCRYPTION_NAME, path_should_fail_0, &hex::encode(generate_network_key(&chainspecs[0].genesis_hash.to_vec(), Encryption::Sr25519)), false, dbname).is_err());
        try_create_address("clone", "Alice", SEED, ENCRYPTION_NAME, path_should_succeed, &hex::encode(generate_network_key(&chainspecs[0].genesis_hash.to_vec(), Encryption::Sr25519)), false, dbname).expect("creating unique address that should prohibit creation of similarly named adderss soon");
        assert!(try_create_address("clone", "Alice", SEED, ENCRYPTION_NAME, path_should_fail_1, &hex::encode(generate_network_key(&chainspecs[0].genesis_hash.to_vec(), Encryption::Sr25519)), false, dbname).is_err());
        let identities = get_relevant_identities("Alice", &hex::encode(generate_network_key(&chainspecs[0].genesis_hash.to_vec(), Encryption::Sr25519)), dbname).unwrap();
        let mut flag = false;
        for (_, address) in identities {
            if address.path == path_should_fail_0 || address.path == path_should_fail_1 { panic!("Wrong identity was created: {:?}", address);}
            flag = flag || address.path == path_should_succeed;
        }
        assert!(flag);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn test_derive() { 
        let dbname = "tests/test_derive";
        load_chainspecs(dbname).expect("create default database");
        let chainspecs = get_default_chainspecs();
        let seed_name = "Alice";
        let network_id_0 = &generate_network_key(&chainspecs[0].genesis_hash.to_vec(), Encryption::Sr25519);
        let network_id_1 = &generate_network_key(&chainspecs[1].genesis_hash.to_vec(), Encryption::Sr25519);
        let both_networks = vec![network_id_0.to_vec(), network_id_1.to_vec()];
        let only_one_network = vec![network_id_0.to_vec()];

        try_create_seed(seed_name, ENCRYPTION_NAME, SEED, 0, dbname).unwrap();
        let seed_object = SeedObject {
            seed_name: seed_name.to_string(),
            seed_phrase: SEED.to_string(),
            encryption: Encryption::Sr25519,
        };
        let database: Db = open(dbname).unwrap();
        create_address(&database, "//Alice", network_id_0.to_vec(), "Alice", &seed_object, false).expect("Create Alice in network 0");
        create_address(&database, "//Alice", network_id_1.to_vec(), "Alice", &seed_object, false).expect("Create Alice in network 1");
        create_address(&database, "//Alice/1", network_id_0.to_vec(), "Alice/1", &seed_object, false).expect("Create Alice/1 in network 0");
        let identities = get_seed_identities (&database, &seed_object.seed_name).unwrap();
        println!("{:?}", identities);
        let mut flag0 = false;
        let mut flag1 = false;
        for (_, details) in identities {
            flag0 = flag0 || (details.name == "Alice" && details.network_id == both_networks);
            flag1 = flag1 || (details.name == "Alice/1" && details.network_id == only_one_network);
        }
        assert!(flag0, "Something is wrong with //Alice");
        assert!(flag1, "Something is wrong with //Alice/1");
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn test_suggest_n_plus_one() { 
        let dbname = "tests/test_suggest_n_plus_one";
        load_chainspecs(dbname).expect("create default database");
        try_create_seed("Alice", ENCRYPTION_NAME, SEED, 0, dbname).unwrap();
        let chainspecs = get_default_chainspecs();
        let network_id_string_0 = hex::encode(generate_network_key(&chainspecs[0].genesis_hash.to_vec(), Encryption::Sr25519));
        try_create_address("clone", "Alice", SEED, ENCRYPTION_NAME, "//Alice//10", &network_id_string_0, false, dbname).expect("create a valid address //Alice//10");
        assert_eq!("//Alice//11", suggest_n_plus_one("//Alice", "Alice", &network_id_string_0, dbname).expect("at least some suggestion about new name should be produced unless db read resulted in a failure"));
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn test_sanitize_number() {
        assert_eq!("1", sanitize_number("1"));
        assert_eq!("1", sanitize_number("001"));
        assert_eq!("1f", sanitize_number("1f"));
        assert_eq!("a", sanitize_number("a"));
        assert_eq!("0a", sanitize_number("0a"));
        assert_eq!("0z", sanitize_number("0z"));
    }
    
    #[test]
    fn account_name_suggestions() {
        assert_eq!("Alice", suggest_path_name("//Alice"));
        assert_eq!("", suggest_path_name(""));
        assert_eq!("Alice verifier", suggest_path_name("//Alice//verifier"));
        assert_eq!("Alice", suggest_path_name("//Alice///password"));
        assert_eq!("Alice (alias)", suggest_path_name("//Alice/alias"));
        assert_eq!("Alice", suggest_path_name("//Alice///password///password"));
        assert_eq!("Лазарь Сигизмундович", suggest_path_name("//Лазарь//Сигизмундович"));
        assert_eq!("Вася (Пупкин)", suggest_path_name("//Вася/Пупкин"));
        assert_eq!("Антон", suggest_path_name("//Антон///секретный"));
        assert_eq!("Alice 1", suggest_path_name("//Alice//0001"));
        assert_eq!("Alice (brackets)", suggest_path_name("//Alice//(brackets)"));
        assert_eq!("Alice ((brackets))", suggest_path_name("//Alice/(brackets)"));
        assert_eq!("Alice", suggest_path_name("//Alice///(brackets)"));
        assert_eq!("(Alice)", suggest_path_name("/Alice"));
        assert_eq!("", suggest_path_name("///password"));
    }

    #[test]
    fn test_identity_deletion() {
        let dbname = "tests/test_identity_deletion";
        load_chainspecs(dbname).expect("create default database");
        try_create_seed("Alice", ENCRYPTION_NAME, SEED, 0, dbname).unwrap();
        let chainspecs = get_default_chainspecs();
        let network_id_string_0 = hex::encode(generate_network_key(&chainspecs[0].genesis_hash.to_vec(), Encryption::Sr25519));
        let network_id_string_1 = hex::encode(generate_network_key(&chainspecs[1].genesis_hash.to_vec(), Encryption::Sr25519));
        let mut identities = get_relevant_identities("Alice", &network_id_string_0, dbname).expect("Alice should have some addresses by default");
        println!("{:?}", identities);
        let (key0, _) = identities.remove(0); //TODO: this should be root key
        let public_key0 = reverse_address_key(&key0.to_vec()).unwrap().public_key;
        let (key1, _) = identities.remove(0); //TODO: this should be network-specific key
        let public_key1 = reverse_address_key(&key1.to_vec()).unwrap().public_key;
        delete_address(&hex::encode(&public_key0), &network_id_string_0, dbname).expect("delete an address");
        delete_address(&hex::encode(&public_key1), &network_id_string_0, dbname).expect("delete another address");
        let identities = get_relevant_identities("Alice", &network_id_string_0, dbname).expect("Alice still should have some addresses after deletion of two");
        for (address_key, _) in identities {
            assert_ne!(address_key, key0);
            assert_ne!(address_key, key1);
        }
        let identities = get_relevant_identities("Alice", &network_id_string_1, dbname).expect("Alice still should have some addresses after deletion of two");
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
}

