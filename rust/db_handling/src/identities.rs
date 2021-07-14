//! All db handling related to seeds and addresses
//! seed phrases should be stored in hw encrypted by
//! best available tool and here they are only processed in plaintext.
//! Zeroization is mostly delegated to os

use sled::{Db, Tree, open};
use sp_core::{Pair, ed25519, sr25519, ecdsa};
use parity_scale_codec::{Decode, Encode};
use regex::Regex;
use definitions::{constants::{ADDRTREE, SPECSTREE}, network_specs::{ChainSpecs, NetworkKey, generate_network_key}, users::{Encryption, AddressDetails, SeedObject, AddressKey, generate_address_key}};
use bip39::{Language, Mnemonic, MnemonicType};
use zeroize::Zeroize;
use lazy_static::lazy_static;

#[cfg(test)]
use std::fs;

#[cfg(test)]
use super::chainspecs::load_chainspecs;


lazy_static! {
// stolen from sp_core
// removed seed phrase part
// last '+' used to be '*', but empty password is an error
    static ref REG_PATH: Regex = Regex::new(r"^(?P<path>(//?[^/]+)*)(///(?P<password>.+))?$").expect("known value");
}

/// get all identities from database for given seed_name (internal use only!)
fn get_seed_identities (database: &Db, seed_name: &str) -> Result<Vec<(AddressKey, AddressDetails)>, Box<dyn std::error::Error>> {
    let identities: Tree = database.open_tree(ADDRTREE)?;
    filter_addresses_by_seed_name (&identities, seed_name)
}

/// filter identities by given seed_name
fn filter_addresses_by_seed_name (identities: &Tree, seed_name: &str) -> Result<Vec<(AddressKey, AddressDetails)>, Box<dyn std::error::Error>> {
    let mut out: Vec<(AddressKey, AddressDetails)> = Vec::new();
    for x in identities.iter() {
        if let Ok((key, value)) = x {
            let address_key = key.to_vec();
            let address_details = <AddressDetails>::decode(&mut &value[..])?;
            if address_details.seed_name == seed_name {
                out.push((address_key, address_details));
            }
        }
    }
    Ok(out)
}

/// filter identities by given seed_name and name
fn filter_addresses_by_seed_name_and_name (identities: &Tree, seed_name: &str, name: &str) -> Result<Vec<(AddressKey, AddressDetails)>, Box<dyn std::error::Error>> {
    let mut out: Vec<(AddressKey, AddressDetails)> = Vec::new();
    for x in identities.iter() {
        if let Ok((key, value)) = x {
            let address_key = key.to_vec();
            let address_details = <AddressDetails>::decode(&mut &value[..])?;
            if (address_details.seed_name == seed_name)&&(address_details.name == name) {
                out.push((address_key, address_details));
            }
        }
    }
    Ok(out)
}

/// get all identities for given seed_name and network_id as hex string
pub fn get_relevant_identities (seed_name: &str, network_id_string: &str, database_name: &str) -> Result<Vec<(AddressKey, AddressDetails)>, Box<dyn std::error::Error>> {
    let network_id = generate_network_key(&hex::decode(network_id_string)?); //TODO: add whatever is needed for parachains?
    let database: Db = open(database_name)?;
    let identities_out = get_seed_identities(&database, seed_name)?;
    Ok(identities_out.into_iter().filter(|(_, details)| details.network_id.contains(&network_id)).collect())
}

/// generate random phrase with given number of words
fn generate_random_phrase (words_number: u32) -> Result<String, Box<dyn std::error::Error>> {
    let mnemonic_type = MnemonicType::for_word_count(words_number as usize)?;
    let mnemonic = Mnemonic::new(mnemonic_type, Language::English);
    Ok(mnemonic.into_phrase())
}

/// Create address from seed and path and insert it into the database
fn create_address (database: &Db, path: &str, network_key: NetworkKey, name: &str, seed_object: &SeedObject, has_pwd: bool) -> Result<(), Box<dyn std::error::Error>> {

    // TODO: check zeroize

    let mut full_address = seed_object.seed_phrase.to_owned() + path;
    let chainspecs: Tree = database.open_tree(SPECSTREE)?;
    if !chainspecs.contains_key(&network_key)? { return Err(Box::from("Error: Create address: network not found")); }
    
    let address_key = match seed_object.encryption {
        Encryption::Ed25519 => {
            match ed25519::Pair::from_string(&full_address, None) {
                Ok(a) => generate_address_key(&a.public().to_vec()),
                Err(_) => return Err(Box::from("Error generating ed25519 address")),
            }
        },
        Encryption::Sr25519 => {
            match sr25519::Pair::from_string(&full_address, None) {
                Ok(a) => generate_address_key(&a.public().to_vec()),
                Err(_) => return Err(Box::from("Error generating sr25519 address")),
            }
        },
        Encryption::Ecdsa => {
            match ecdsa::Pair::from_string(&full_address, None) {
                Ok(a) => generate_address_key(&a.public().0.to_vec()),
                Err(_) => return Err(Box::from("Error generating ecdsa address")),
            }
        },
    };
    full_address.zeroize();
    
    let identities: Tree = database.open_tree(ADDRTREE)?;
    let seed_name = seed_object.seed_name.to_string();
// This address might be already created; maybe we just need to allow its use in another network?
    match identities.get(&address_key)? {
        Some(address_details_encoded) => {
        // TODO: check that all collisions are handled
            let mut address_details = <AddressDetails>::decode(&mut &address_details_encoded[..])?;
        // Check if something else resolved into this keypair
            if address_details.name != name || address_details.path != path { return Err(Box::from(format!("Address key collision with existing identity {} of seed {}", address_details.name, address_details.seed_name))) }
        // Append network to list of allowed networks
            if !address_details.network_id.contains(&network_key) {
                address_details.network_id.push(network_key);
                identities.insert(address_key, address_details.encode())?;
            }
        }
        None => {
        // Check for collisions in name
            let collided = filter_addresses_by_seed_name_and_name(&identities, &seed_name, name)?;
            if collided.len() !=0 {return Err(Box::from("Identity with this name already exists"))}
            
            let cropped_path = match REG_PATH.captures(path) {
                Some(caps) => match caps.name("path") {
                    Some(a) => a.as_str(),
                    None => "",
                },
                None => "",
            };
            let address = AddressDetails {
                seed_name,
                path: cropped_path.to_string(),
                has_pwd,
                name: name.to_string(),
                network_id: vec![network_key],
                encryption: seed_object.encryption,
            };
            identities.insert(address_key, address.encode())?;
        }
    }
    Ok(())
}


/// Create addresses for all default paths in all default networks, and insert them in the database
fn populate_addresses (database: &Db, seed_object: &SeedObject) -> Result<(), Box<dyn std::error::Error>> {
// TODO: check zeroize
// TODO: compatibility with atomic operations
    let chainspecs: Tree = database.open_tree(SPECSTREE)?;
    for x in chainspecs.iter() {
        if let Ok((network_key, network_specs_encoded)) = x {
            let network_specs = <ChainSpecs>::decode(&mut &network_specs_encoded[..])?;
            create_address (database, "", network_key.to_vec(), "root address", seed_object, false)?;
            match create_address (database, &network_specs.path_id, network_key.to_vec(), &format!("{} root address", network_specs.name), seed_object, false) {
                Ok(()) => (),
                Err(_) => (),
            }
        }
    }
    Ok(())
}

/// Generate new seed and populate all known networks with default accounts
pub fn try_create_seed (seed_name: &str, encryption_name: &str, seed_phrase_proposal: &str, seed_length: u32, database_name: &str) -> Result<String, Box<dyn std::error::Error>> {
// TODO: atomize writes
    let database: Db = open(database_name)?;
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
        _ => return Err(Box::from("System error: unknown encryption algorithm")),
    };
    
    let seed_object = SeedObject {
        seed_name: seed_name.to_string(),
        seed_phrase: seed_phrase.to_string(),
        encryption: encryption,
    };

    populate_addresses(&database, &seed_object)?;
    database.flush()?;
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

/// Delete identity
pub fn delete_address(pub_key: &str, network_id_string: &str, database_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let database: Db = open(database_name)?;
    let identities: Tree = database.open_tree(ADDRTREE)?;
    let address_key = generate_address_key(&hex::decode(pub_key)?);
    let network_key = generate_network_key(&hex::decode(network_id_string)?); //TODO: add whatever is needed for parachains?
    match identities.get(&address_key)? {
        Some(address_details_encoded) => {
            let mut address_details = <AddressDetails>::decode(&mut &address_details_encoded[..])?;
            address_details.network_id = address_details.network_id.into_iter().filter(|id| *id != network_key).collect();
            if address_details.network_id.is_empty() {identities.remove(&address_key)?} 
            else {identities.insert(address_key, address_details.encode())?}
        }
        None => return Err(Box::from("Error: this address does not exist in database")),
    };
    Ok(())
}

/// Suggest address and name for weird N+1 feature request
pub fn suggest_n_plus_one(path: &str, seed_name: &str, network_id_string: &str, database_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let identities = get_relevant_identities(seed_name, network_id_string, database_name)?;
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
pub fn check_derivation_format(path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    match REG_PATH.captures(path) {
        Some(caps) => Ok(caps.name("password").is_some()),
        None => return Err(Box::from("Invalid derivation format")),
    }
}

/// Generate new identity (api for create_address())
pub fn try_create_address (id_name: &str, seed_name: &str, seed_phrase: &str, encryption_name: &str, path: &str, network: &str, has_pwd: bool, database_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let database: Db = open(database_name)?;

    let encryption = match encryption_name {
        "ed25519" => Encryption::Ed25519,
        "sr25519" => Encryption::Sr25519,
        "ecdsa" => Encryption::Ecdsa,
        _ => return Err(Box::from("System error: unknown encryption algorithm")),
    };
    
    let seed_object = SeedObject {
        seed_name: seed_name.to_string(),
        seed_phrase: seed_phrase.to_string(),
        encryption,
    };

    let network_key = generate_network_key(&hex::decode(network)?);

    create_address(&database, path, network_key, id_name, &seed_object, has_pwd)
}

/// Function to populate test cold database with Alice information
pub fn load_test_identities (database_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let database: Db = open(database_name)?;
    let alice_seed_object = SeedObject {
        seed_name: String::from("Alice"),
        seed_phrase: String::from("bottom drive obey lake curtain smoke basket hold race lonely fit walk"),
        encryption: Encryption::Sr25519,
    };
    populate_addresses (&database, &alice_seed_object)?;
    let westend_network_key = generate_network_key(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").expect("known value"));
    create_address (&database, "//Alice", westend_network_key, "Alice_test_westend", &alice_seed_object, false)?;
    
    database.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use definitions::defaults::get_default_chainspecs;
    //static PASSWORD: &str = "very long and unguessable phrase";
    static SEED: &str = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    static ENCRYPTION_NAME: &str = "sr25519";

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
        let random_addresses = get_relevant_identities("Randy", &hex::encode(chainspecs[0].genesis_hash), dbname).unwrap();
        assert!(random_addresses.len()>0);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn test_generate_default_addresses_for_alice() {
        let dbname = "tests/test_generate_default_addresses_for_Alice";
        load_chainspecs(dbname).expect("create default database");
        try_create_seed("Alice", ENCRYPTION_NAME, SEED, 0, dbname).unwrap();
        let chainspecs = get_default_chainspecs();
        let default_addresses = get_relevant_identities("Alice", &hex::encode(chainspecs[0].genesis_hash), dbname).unwrap();
        assert!(default_addresses.len()>0);
        assert_eq!(r#"[([70, 235, 221, 239, 140, 217, 187, 22, 125, 195, 8, 120, 215, 17, 59, 126, 22, 142, 111, 6, 70, 190, 255, 215, 125, 105, 211, 155, 173, 118, 180, 122], AddressDetails { seed_name: "Alice", path: "", has_pwd: false, name: "root address", network_id: [[145, 177, 113, 187, 21, 142, 45, 56, 72, 250, 35, 169, 241, 194, 81, 130, 251, 142, 32, 49, 59, 44, 30, 180, 146, 25, 218, 122, 112, 206, 144, 195], [176, 168, 212, 147, 40, 92, 45, 247, 50, 144, 223, 183, 230, 31, 135, 15, 23, 180, 24, 1, 25, 122, 20, 156, 169, 54, 84, 73, 158, 163, 218, 254], [225, 67, 242, 56, 3, 172, 80, 232, 246, 248, 230, 38, 149, 209, 206, 158, 78, 29, 104, 170, 54, 193, 205, 44, 253, 21, 52, 2, 19, 243, 66, 62], [231, 195, 213, 237, 222, 125, 185, 100, 49, 124, 217, 181, 26, 58, 5, 157, 124, 217, 159, 129, 189, 188, 225, 73, 144, 4, 115, 84, 51, 76, 151, 121]], encryption: Sr25519 }), ([100, 163, 18, 53, 212, 191, 155, 55, 207, 237, 58, 250, 138, 166, 7, 84, 103, 95, 156, 73, 21, 67, 4, 84, 211, 101, 192, 81, 18, 120, 77, 5], AddressDetails { seed_name: "Alice", path: "//kusama", has_pwd: false, name: "kusama root address", network_id: [[176, 168, 212, 147, 40, 92, 45, 247, 50, 144, 223, 183, 230, 31, 135, 15, 23, 180, 24, 1, 25, 122, 20, 156, 169, 54, 84, 73, 158, 163, 218, 254]], encryption: Sr25519 })]"#, format!("{:?}", default_addresses)); //because JSON export is what we care about
        let database: Db = open(dbname).unwrap();
        let identities: Tree = database.open_tree(ADDRTREE).unwrap();
        let test_key = generate_address_key(&hex::decode("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a").unwrap());
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
        assert!(try_create_address("root address", "Alice", SEED, ENCRYPTION_NAME, path_should_fail_0, &hex::encode(chainspecs[0].genesis_hash), false, dbname).is_err());
        try_create_address("clone", "Alice", SEED, ENCRYPTION_NAME, path_should_succeed, &hex::encode(chainspecs[0].genesis_hash), false, dbname).expect("creating unique address that should prohibit creation of similarly named adderss soon");
        assert!(try_create_address("clone", "Alice", SEED, ENCRYPTION_NAME, path_should_fail_1, &hex::encode(chainspecs[0].genesis_hash), false, dbname).is_err());
        let identities = get_relevant_identities("Alice", &hex::encode(chainspecs[0].genesis_hash), dbname).unwrap();
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
        let network_id_0 = chainspecs[0].genesis_hash.to_vec();
        let network_id_1 = chainspecs[1].genesis_hash.to_vec();
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
        create_address(&database, "//Alice", network_id_1, "Alice", &seed_object, false).expect("Create Alice in network 1");
        create_address(&database, "//Alice/1", network_id_0, "Alice/1", &seed_object, false).expect("Create Alice/1 in network 0");
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
        let network_id_string_0 = &hex::encode(chainspecs[0].genesis_hash);
        try_create_address("clone", "Alice", SEED, ENCRYPTION_NAME, "//Alice//10", network_id_string_0, false, dbname).expect("create a valid address //Alice//10");
        assert_eq!("//Alice//11", suggest_n_plus_one("//Alice", "Alice", network_id_string_0, dbname).expect("at least some suggestion about new name should be produced unless db read resulted in a failure"));
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
        let network_id_string_0 = &hex::encode(chainspecs[0].genesis_hash);
        let network_id_string_1 = &hex::encode(chainspecs[1].genesis_hash);
        let mut identities = get_relevant_identities("Alice", network_id_string_0, dbname).expect("Alice should have some addresses by default");
        println!("{:?}", identities);
        let (key0, _) = identities.remove(0); //TODO: this should be root key
        let (key1, _) = identities.remove(0); //TODO: this should be network-specific key
        delete_address(&hex::encode(&key0), network_id_string_0, dbname).expect("delete and address");
        delete_address(&hex::encode(&key1), network_id_string_0, dbname).expect("delete another address");
        let identities = get_relevant_identities("Alice", network_id_string_0, dbname).expect("Alice still should have some addresses after deletion of two");
        for (pub_key, _) in identities {
            assert_ne!(pub_key, key0);
            assert_ne!(pub_key, key1);
        }
        let identities = get_relevant_identities("Alice", network_id_string_1, dbname).expect("Alice still should have some addresses after deletion of two");
        let mut flag_to_check_key0_remains = false;
        for (pub_key, _) in identities {
            if pub_key == key0 {
                flag_to_check_key0_remains = true;
            }
            assert_ne!(pub_key, key1);
        }
        assert!(flag_to_check_key0_remains, "An address that should have only lost network was removed entirely");
        fs::remove_dir_all(dbname).unwrap();
    }
}

