use sled::{Db, Tree, open};
use sp_core::{Pair, ed25519, sr25519, ecdsa, crypto::Ss58Codec};
use parity_scale_codec::{Decode, Encode};
use parity_scale_codec_derive;
use ethsign::{Protected, keyfile::Crypto};
use super::chainspecs::ChainSpecs;
use super::constants::{ADDRTREE, IDTREE, SPECSTREE};
use super::db_utils::{generate_seed_key, generate_address_key, generate_network_key, SeedKey, AddressKey, NetworkKey};
use serde_json;
use serde;
use bip39::{Language, Mnemonic, MnemonicType};
use blake2_rfc::blake2b::blake2b;
use base58::ToBase58;
use zeroize::Zeroize;

#[cfg(test)]
use std::fs;

#[cfg(test)]
use super::chainspecs::load_chainspecs;

///Struct associated with public address that has secret key available
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode, Debug)]
pub struct AddressDetails {
    pub name_for_seed: SeedKey,
    pub path: String,
    pub has_pwd: bool,
    pub name: String,
    pub network_id: Vec<NetworkKey>,
}

///Type of encryption; only allow supported types here - compile-time check for that is happening
///here.
//TODO: check if it is redundant
//Could not be replaced by sp_core::...::CRYPTO_ID as that doesn't do anything at compile time
#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Encryption {
    Sr25519,
    Ed25519,
    Ecdsa,
}

///Struct to store seeds
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct StoredSeed {
    pub crypto: Crypto,
    pub encryption: Encryption,
}

///get all seed names to display selector

pub fn get_seed_names_list (database_name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let database: Db = open(database_name)?;
    let seeds: Tree = database.open_tree(IDTREE)?;
    match seeds
        .iter()
        .collect::<Result<Vec<_>,_>>()?
        .into_iter()
        .map(|(key, _)| <String>::decode(&mut &key[..]))
        .collect::<Result<Vec<_>,_>>()
        {
            Ok(a) => Ok(a),
            Err(e) => return Err(Box::from(e)),
        }
}

///get all identities within given seed and network
pub fn get_relevant_identities (seed_name: &str, network_id_string: &str, database_name: &str) -> Result<Vec<AddressDetails>, Box<dyn std::error::Error>> {
    let network_id = generate_network_key(hex::decode(network_id_string)?); //TODO: add whatever is needed for parachains?
    let database: Db = open(database_name)?;
    let identities: Tree = database.open_tree(ADDRTREE)?;
    let name_for_seed = generate_seed_key(seed_name);
    Ok(identities
        .iter()
        .collect::<Result<Vec<_>,_>>()?
        .into_iter()
        .map(|(_, value)| <AddressDetails>::decode(&mut &value[..]))
        .collect::<Result<Vec<_>,_>>()?
        .into_iter()
        .filter(|identity| (identity.network_id.contains(&network_id)) && (identity.name_for_seed == name_for_seed))
        .collect())
}

fn generate_random_phrase (words_number: u32) -> Result<String, Box<dyn std::error::Error>> {
    let mnemonic_type = MnemonicType::for_word_count(words_number as usize)?;
	let mnemonic = Mnemonic::new(mnemonic_type, Language::English);
	Ok(mnemonic.into_phrase())
}

///Create encrypted seed in database
fn create_seed (database: &Db, seed_phrase: &str, password: Protected, seed_name: &str, encryption: Encryption) -> Result<(), Box<dyn std::error::Error>> {
    //TODO: zeroize
    let seeds: Tree = database.open_tree(IDTREE)?;
    let data = seed_phrase.as_bytes();
    let stored_seed = StoredSeed {
        crypto: Crypto::encrypt(data, &password, 10240)?,
        encryption: encryption,
    };
    if seeds.contains_key(generate_seed_key(seed_name))? { return Err(Box::from("This name already exists")); }
    seeds.insert(generate_seed_key(seed_name), serde_json::to_string(&stored_seed)?.as_bytes())?;
    Ok(())
}

/*
fn get_seed (database: &Db, seed_name: &str, password: Protected) -> Result<String, Box<dyn std::error::Error>> {
    let seeds: Tree = database.open_tree(IDTREE)?;
}*/

///Create address from seed and path
fn create_address (
    database: &Db, 
    path: &str, 
    network: &ChainSpecs,
    network_id: NetworkKey,
    name: &str, 
    seed: &str, 
    seed_name: &str, 
    encryption: Encryption, 
    path_password: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    //TODO: password?
    let mut full_address = seed.to_owned() + path;
    let address_key = match encryption {
        Encryption::Sr25519 => {
            match sr25519::Pair::from_string(&full_address, path_password) {
                Ok(a) => generate_address_key(a.public().to_vec()),
                Err(_) => return Err(Box::from("Error generating sr25519 address")),
            }
        },
        Encryption::Ed25519 => {
            match ed25519::Pair::from_string(&full_address, path_password) {
                Ok(a) => generate_address_key(a.public().to_vec()),
                Err(_) => return Err(Box::from("Error generating ed25519 address")),
            }
        },
        Encryption::Ecdsa => {
            match ecdsa::Pair::from_string(&full_address, path_password) {
                Ok(a) => generate_address_key(a.public().0.to_vec()),
                Err(_) => return Err(Box::from("Error generating ecdsa address")),
            }
        },
    };
    full_address.zeroize();
    println!("{:?}", address_key);
    let identities: Tree = database.open_tree(ADDRTREE)?;
    match identities.get(&address_key)? {
        Some(address_record) => {
            let mut address = <AddressDetails>::decode(&mut &address_record[..])?;
            if !address.network_id.contains(&network_id) {
                address.network_id.push(network_id);
                identities.insert(address_key, address.encode())?;
            };
        }
        None => {
            let address = AddressDetails {
                name_for_seed: generate_seed_key(seed_name),
                path: path.to_string(),
                has_pwd: path_password != None,
                name: name.to_string(),
                network_id: vec!(network_id),
            };
            identities.insert(address_key, address.encode())?;
        }
    }


    Ok(())
}


///Create addresses for all default paths in all default networks
fn populate_addresses (database: &Db, seed_name: &str, seed: &str, encryption: Encryption) -> Result<(), Box<dyn std::error::Error>> {
    let networks: Tree = database.open_tree(SPECSTREE)?;
    for result in networks.iter() {
        match result {
            Ok ((key, value)) => {
                let network = <ChainSpecs>::decode(&mut &value[..])?;
                create_address (
                    database, 
                    "", 
                    &network,
                    key.to_vec(),
                    "root address", 
                    seed, 
                    seed_name, 
                    encryption, 
                    None)?;
                create_address (
                    database, 
                    &network.path_id, 
                    &network,
                    key.to_vec(),
                    &format!("{} root address", network.name),
                    seed,
                    seed_name,
                    encryption,
                    None)?;
            }
            Err (e) => return Err(Box::from(e)),
        }
    }
    Ok(())
}

///Generate new seed and populate all known networks with default accounts
pub fn try_create_seed (seed_name: &str, encryption_name: &str, password: &str, database_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let database: Db = open(database_name)?;
    let mut seed = generate_random_phrase(24)?; //TODO: should we let user choose?
    let pw = Protected::new(password.as_bytes());
    //pray that OS clears plaintext password
    //TODO: zeroize password and seed

    let encryption = match encryption_name {
        "sr25519" => Encryption::Sr25519,
        "ed25519" => Encryption::Ed25519,
        "ecdsa" => Encryption::Ecdsa,
        _ => return Err(Box::from("System error: unknown encryption algorithm")),
    };

    create_seed(&database, &seed, pw, seed_name, encryption)?;
    populate_addresses(&database, seed_name, &seed, encryption)?;
    seed.zeroize();
    database.flush()?;
    Ok(())
}

///Recover seed and pupulate all known networks with default accounts
pub fn try_recover_seed (seed_name: &str, encryption_name: &str, seed_phrase: &str, password: &str, database_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if Mnemonic::validate(seed_phrase, Language::English).is_err() { return Err(Box::from("Invalid seed phrase")); }
    let database: Db = open(database_name)?;
    let pw = Protected::new(password.as_bytes());
    //pray that OS clears plaintext password
    //TODO: zeroize password and seed

    let encryption = match encryption_name {
        "sr25519" => Encryption::Sr25519,
        "ed25519" => Encryption::Ed25519,
        "ecdsa" => Encryption::Ecdsa,
        _ => return Err(Box::from("System error: unknown encryption algorithm")),
    };

    create_seed(&database, seed_phrase, pw, seed_name, encryption)?;
    populate_addresses(&database, seed_name, seed_phrase, encryption)?;
    database.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::constants::get_default_chainspecs;
    static PASSWORD: &str = "very long and unguessable phrase";
    static SEED: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    static ENCRYPTION_NAME: &str = "sr25519";

    #[test]
    fn test_create_seed() {
        let seed_name = "test seed name";
        let dbname = "tests/test_create_seed";
        fs::remove_dir_all(&dbname).unwrap();
        let password: Protected = Protected::new(PASSWORD.as_bytes());
        let database: Db = open(dbname).unwrap();
        create_seed(&database, SEED, password, "Alice", Encryption::Sr25519);
    }

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
        fs::remove_dir_all(&dbname).unwrap();
        load_chainspecs(dbname);
        try_create_seed("Randy", ENCRYPTION_NAME, PASSWORD, dbname).unwrap();
        let chainspecs = get_default_chainspecs();
        let random_addresses = get_relevant_identities("Randy", &hex::encode(chainspecs[0].genesis_hash), dbname).unwrap();
        assert!(random_addresses.len()>0);
    }

    #[test]
    fn test_generate_default_addresses_for_alice() {
        let dbname = "tests/test_generate_default_addresses_for_Alice";
        fs::remove_dir_all(&dbname).unwrap();
        load_chainspecs(dbname);
        try_recover_seed("Alice", ENCRYPTION_NAME, SEED, PASSWORD, dbname).unwrap();
        let chainspecs = get_default_chainspecs();
        let default_addresses = get_relevant_identities("Alice", &hex::encode(chainspecs[0].genesis_hash), dbname).unwrap();
        assert!(default_addresses.len()>0);
        let database: Db = open(dbname).unwrap();
        let identities: Tree = database.open_tree(ADDRTREE).unwrap();
        let test_key = generate_address_key(hex::decode("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a").unwrap());
        println!("{:?}", test_key);
        assert!(identities.contains_key(test_key).unwrap());
    }
}

