use sled::{Db, Tree, Batch, open};
use anyhow;
use constants::{ADDRTREE, DANGER, GENERALVERIFIER, METATREE, SETTREE, SPECSTREE, TYPES, VERIFIERS};
use definitions::{danger::DangerRecord, error::{DatabaseSigner, EntryDecodingSigner, ErrorSigner, ErrorSource, NotFoundSigner, Signer}, keyring::{AddressKey, MetaKey, MetaKeyPrefix, NetworkSpecsKey, VerifierKey}, metadata::MetaValues, network_specs::{CurrentVerifier, NetworkSpecs, ValidCurrentVerifier, Verifier}, types::TypeEntry, users::{AddressDetails}};
use parity_scale_codec::{Decode, Encode};

/// Wrapper for `open`
pub fn open_db <T: ErrorSource> (database_name: &str) -> Result<Db, T::Error> {
    match open(database_name) {
        Ok(x) => Ok(x),
        Err(e) => return Err(<T>::db_internal(e)),
    }
}

/// Wrapper for `open_tree`
pub fn open_tree <T: ErrorSource> (database: &Db, tree_name: &[u8]) -> Result<Tree, T::Error> {
    match database.open_tree(tree_name) {
        Ok(x) => Ok(x),
        Err(e) => return Err(<T>::db_internal(e)),
    }
}

/// Wrapper to assemble a Batch for removing all elements of a tree
/// (to add into transaction where clear_tree should be)
pub fn make_batch_clear_tree <T: ErrorSource> (database_name: &str, tree_name: &[u8]) -> Result<Batch, T::Error> {
    let database = open_db::<T>(database_name)?;
    let tree = open_tree::<T>(&database, tree_name)?;
    let mut out = Batch::default();
    for x in tree.iter() {
        if let Ok((key, _)) = x {out.remove(key)}
    }
    Ok(out)
}

/// Function to try and get from the Signer database the _valid_ current verifier for network using VerifierKey
pub fn try_get_valid_current_verifier (verifier_key: &VerifierKey, database_name: &str) -> Result<Option<ValidCurrentVerifier>, ErrorSigner> {
    let general_verifier = get_general_verifier(database_name)?;
    let database = open_db::<Signer>(&database_name)?;
    let verifiers = open_tree::<Signer>(&database, VERIFIERS)?;
    match verifiers.get(verifier_key.key()) {
        Ok(Some(verifier_encoded)) => match <CurrentVerifier>::decode(&mut &verifier_encoded[..]) {
            Ok(a) => {
                match a {
                    CurrentVerifier::Valid(b) => {
                        if let ValidCurrentVerifier::Custom(ref custom_verifier) = b {
                            if (custom_verifier == &general_verifier)&&(general_verifier != Verifier(None)) {return Err(ErrorSigner::Database(DatabaseSigner::CustomVerifierIsGeneral(verifier_key.to_owned())))}
                        }
                        Ok(Some(b))
                    },
                    CurrentVerifier::Dead => return Err(ErrorSigner::DeadVerifier(verifier_key.to_owned())),
                }
            },
            Err(_) => return Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::CurrentVerifier(verifier_key.to_owned())))),
        },
        Ok(None) => {
            if let Some(network_specs_key) = genesis_hash_in_specs(verifier_key, &database)? {return Err(ErrorSigner::Database(DatabaseSigner::UnexpectedGenesisHash{verifier_key: verifier_key.to_owned(), network_specs_key}))}
            Ok(None)
        },
        Err(e) => return Err(<Signer>::db_internal(e)),
    }
}

/// Function to get from the Signer database the current verifier for network using VerifierKey, returns error if not found
pub fn get_valid_current_verifier (verifier_key: &VerifierKey, database_name: &str) -> Result<ValidCurrentVerifier, ErrorSigner> {
    match try_get_valid_current_verifier(verifier_key, database_name)? {
        Some(a) => Ok(a),
        None => return Err(ErrorSigner::NotFound(NotFoundSigner::CurrentVerifier(verifier_key.to_owned()))),
    }
}

/// Function to search for genesis hash corresponding to a given verifier key
/// in SPECSTREE of the Signer database
pub fn genesis_hash_in_specs (verifier_key: &VerifierKey, database: &Db) -> Result<Option<NetworkSpecsKey>, ErrorSigner> {
    let genesis_hash = verifier_key.genesis_hash();
    let chainspecs = open_tree::<Signer>(&database, SPECSTREE)?;
    let mut out = None;
    for x in chainspecs.iter() {
        if let Ok((network_specs_key_vec, network_specs_encoded)) = x {
            let network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
            let network_specs = NetworkSpecs::from_entry_with_key_checked::<Signer>(&network_specs_key, network_specs_encoded)?;
            if network_specs.genesis_hash.to_vec() == genesis_hash {
                out = Some(network_specs_key);
                break;
            }
        }
    }
    Ok(out)
}

/// Function to get general Verifier from the Signer database
/// Note that not finding general verifier is always an error.
pub fn get_general_verifier (database_name: &str) -> Result<Verifier, ErrorSigner> {
    let database = open_db::<Signer>(&database_name)?;
    let settings = open_tree::<Signer>(&database, SETTREE)?;
    match settings.get(GENERALVERIFIER.to_vec()) {
        Ok(Some(verifier_encoded)) => match <Verifier>::decode(&mut &verifier_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => return Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::GeneralVerifier))),
        },
        Ok(None) => return Err(ErrorSigner::NotFound(NotFoundSigner::GeneralVerifier)),
        Err(e) => return Err(<Signer>::db_internal(e)),
    }
}

/// Function to display general Verifier from the Signer database
pub fn display_general_verifier (database_name: &str) -> anyhow::Result<String> {
    match get_general_verifier(database_name) {
        Ok(general_verifier) => Ok(general_verifier.show_card()),
        Err(e) => return Err(e.anyhow()),
    }
}

/// Function to try and get types information from the database
/// Applicable to both Active side and Signer side
pub fn try_get_types <T: ErrorSource> (database_name: &str) -> Result<Option<Vec<TypeEntry>>, T::Error> {
    let database = open_db::<T>(&database_name)?;
    let settings = open_tree::<T>(&database, SETTREE)?;
    match settings.get(TYPES) {
        Ok(Some(types_info_encoded)) => {
            match <Vec<TypeEntry>>::decode(&mut &types_info_encoded[..]) {
                Ok(a) => Ok(Some(a)),
                Err(_) => return Err(<T>::faulty_database_types()),
            }
        },
        Ok(None) => Ok(None),
        Err(e) => return Err(<T>::db_internal(e)),
    }
}

/// Function to get types information from the database, returns error if not found
/// Applicable to both Active side and Signer side
pub fn get_types <T: ErrorSource> (database_name: &str) -> Result<Vec<TypeEntry>, T::Error> {
    match try_get_types::<T>(database_name)? {
        Some(a) => Ok(a),
        None => return Err(<T>::types_not_found()),
    }
}

/// Function to try and get network specs from the Signer database
pub fn try_get_network_specs (database_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<Option<NetworkSpecs>, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let chainspecs = open_tree::<Signer>(&database, SPECSTREE)?;
    match chainspecs.get(network_specs_key.key()) {
        Ok(Some(network_specs_encoded)) => Ok(Some(NetworkSpecs::from_entry_with_key_checked::<Signer>(network_specs_key, network_specs_encoded)?)),
        Ok(None) => Ok(None),
        Err(e) => return Err(<Signer>::db_internal(e)),
    }
}

/// Function to get network specs from the Signer database, returns error if not found
pub fn get_network_specs (database_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<NetworkSpecs, ErrorSigner> {
    match try_get_network_specs(database_name, network_specs_key)? {
        Some(a) => Ok(a),
        None => return Err(ErrorSigner::NotFound(NotFoundSigner::NetworkSpecs(network_specs_key.to_owned()))),
    }
}

/// Function to try and get address details from the Signer database
pub fn try_get_address_details (database_name: &str, address_key: &AddressKey) -> Result<Option<AddressDetails>, ErrorSigner> {
    let database = open_db::<Signer>(&database_name)?;
    let identities = open_tree::<Signer>(&database, ADDRTREE)?;
    match identities.get(address_key.key()) {
        Ok(Some(address_details_encoded)) => Ok(Some(AddressDetails::from_entry_with_key_checked::<Signer>(address_key, address_details_encoded)?)),
        Ok(None) => Ok(None),
        Err(e) => return Err(<Signer>::db_internal(e)),
    }
}

/// Function to get address details from the Signer database, returns error if not found
pub fn get_address_details (database_name: &str, address_key: &AddressKey) -> Result<AddressDetails, ErrorSigner> {
    match try_get_address_details(database_name, address_key)? {
        Some(a) => Ok(a),
        None => return Err(ErrorSigner::NotFound(NotFoundSigner::AddressDetails(address_key.to_owned()))),
    }
}

/// Function to collect MetaValues corresponding to given network name.
/// Applicable to both Active side and Signer side
pub fn get_meta_values_by_name <T: ErrorSource> (database_name: &str, network_name: &str) -> Result<Vec<MetaValues>, T::Error> {
    let database = open_db::<T>(&database_name)?;
    let metadata = open_tree::<T>(&database, METATREE)?;
    let mut out: Vec<MetaValues> = Vec::new();
    let meta_key_prefix = MetaKeyPrefix::from_name(&network_name);
    for x in metadata.scan_prefix(meta_key_prefix.prefix()) {
        if let Ok(a) = x {
            let meta_values = MetaValues::from_entry_checked::<T>(a)?;
            if meta_values.name == network_name {out.push(meta_values)}
        }
    }
    Ok(out)
}

/// Function to get MetaValues corresponding to given network name and version.
/// Returns error if finds nothing.
/// Applicable to both Active side and Signer side.
pub fn get_meta_values_by_name_version <T: ErrorSource> (database_name: &str, network_name: &str, network_version: u32) -> Result<MetaValues, T::Error> {
    let database = open_db::<T>(&database_name)?;
    let metadata = open_tree::<T>(&database, METATREE)?;
    let meta_key = MetaKey::from_parts(network_name, network_version);
    match metadata.get(meta_key.key()) {
        Ok(Some(meta)) => MetaValues::from_entry_name_version_checked::<T>(network_name, network_version, meta),
        Ok(None) => return Err(<T>::metadata_not_found(network_name.to_string(), network_version)),
        Err(e) => return Err(<T>::db_internal(e)),
    }
}

/// Function to modify existing batch for ADDRTREE with incoming vector of additions
pub (crate) fn upd_id_batch (mut batch: Batch, adds: Vec<(AddressKey, AddressDetails)>) -> Batch {
    for (address_key, address_details) in adds.iter() {batch.insert(address_key.key(), address_details.encode());}
    batch
}

/// Function to verify checksum in Signer database
pub fn verify_checksum (database: &Db, checksum: u32) -> Result<(), ErrorSigner> {
    let real_checksum = match database.checksum() {
        Ok(x) => x,
        Err(e) => return Err(<Signer>::db_internal(e)),
    };
    if checksum != real_checksum {return Err(ErrorSigner::Database(DatabaseSigner::ChecksumMismatch))}
    Ok(())
}

/// Function to get the danger status from the Signer database
fn get_danger_status(database_name: &str) -> Result<bool, ErrorSigner> {
    let database = open_db::<Signer>(&database_name)?;
    let settings = open_tree::<Signer>(&database, SETTREE)?;
    match settings.get(DANGER.to_vec()) {
        Ok(Some(a)) => DangerRecord::from_ivec(&a).device_was_online(),
        Ok(None) => return Err(ErrorSigner::NotFound(NotFoundSigner::DangerStatus)),
        Err(e) => return Err(<Signer>::db_internal(e)),
    }
}

/// Function to display the danger status from the database.
/// Function interacts with user interface.
pub fn display_danger_status(database_name: &str) -> anyhow::Result<bool> {
    get_danger_status(database_name).map_err(|e| e.anyhow())
}


#[cfg(test)]
mod tests {
    
    use super::*;
    
    use definitions::{keyring::VerifierKey, network_specs::{ValidCurrentVerifier, Verifier}};
    use hex;
    use std::fs;
    
    use crate::{cold_default::{populate_cold_no_metadata, reset_cold_database_no_addresses, signer_init_no_cert, signer_init_with_cert}, manage_history::{device_was_online, reset_danger_status_to_safe}};

    #[test]
    fn get_danger_status_properly () {
        let dbname = "for_tests/get_danger_status_properly";
        reset_cold_database_no_addresses(dbname, Verifier(None)).unwrap();
        signer_init_no_cert(dbname).unwrap();
        assert!(get_danger_status(dbname).unwrap() == false, "Expected danger status = false after the database initiation.");
        device_was_online(dbname).unwrap();
        assert!(get_danger_status(dbname).unwrap() == true, "Expected danger status = true after the reported exposure.");
        reset_danger_status_to_safe(dbname).unwrap();
        assert!(get_danger_status(dbname).unwrap() == false, "Expected danger status = false after the danger reset.");
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn display_general_verifier_properly() {
        let dbname = "for_tests/display_general_verifier_properly";
        reset_cold_database_no_addresses(dbname, Verifier(None)).unwrap();
        let print = display_general_verifier(dbname).unwrap();
        assert!(print == r#"{"hex":"","encryption":"none"}"#, "Got: {}", print);
        signer_init_with_cert(dbname).unwrap();
        let print = display_general_verifier(dbname).unwrap();
        assert!(print == r#"{"hex":"c46a22b9da19540a77cbde23197e5fd90485c72b4ecf3c599ecca6998f39bd57","encryption":"sr25519"}"#, "Got: {}", print);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn find_westend_verifier() {
        let dbname = "for_tests/find_westend_verifier";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let verifier_key = VerifierKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap());
        let westend_verifier = try_get_valid_current_verifier(&verifier_key, &dbname).unwrap();
        assert!(westend_verifier == Some(ValidCurrentVerifier::General));
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn not_find_mock_verifier() {
        let dbname = "for_tests/not_find_mock_verifier";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let verifier_key = VerifierKey::from_parts(&hex::decode("62bacaaa3d9bb01313bb882c23615aae6509ab2ef1e7e807581ee0b74c77416b").unwrap());
        match try_get_valid_current_verifier(&verifier_key, &dbname) {
            Ok(Some(_)) => panic!("Found network key that should not be in database."),
            Ok(None) => (),
            Err(e) => panic!("Error looking for mock verifier: {}", <Signer>::show(&e)),
        }
        fs::remove_dir_all(dbname).unwrap();
    }
}
