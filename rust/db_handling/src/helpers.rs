use parity_scale_codec::Decode;
#[cfg(any(feature = "active", feature = "signer"))]
use parity_scale_codec::Encode;
use sled::{Db, Tree, Batch, open};

use constants::{METATREE, SETTREE, TYPES};
#[cfg(feature = "signer")]
use constants::{ADDRTREE, DANGER, GENERALVERIFIER, SPECSTREE, VERIFIERS};

use definitions::{error::ErrorSource, keyring::{MetaKey, MetaKeyPrefix}, metadata::MetaValues, types::TypeEntry};
#[cfg(feature = "signer")]
use definitions::{danger::DangerRecord, error_signer::{DatabaseSigner, EntryDecodingSigner, ErrorSigner, NotFoundSigner, Signer}, keyring::{NetworkSpecsKey, VerifierKey}, network_specs::{CurrentVerifier, NetworkSpecs, ValidCurrentVerifier, Verifier}};
#[cfg(any(feature = "active", feature = "signer"))]
use definitions::{keyring::AddressKey, users::AddressDetails};

/// Wrapper for `open`
pub fn open_db <T: ErrorSource> (database_name: &str) -> Result<Db, T::Error> {
    open(database_name).map_err(<T>::db_internal)
}

/// Wrapper for `open_tree`
pub fn open_tree <T: ErrorSource> (database: &Db, tree_name: &[u8]) -> Result<Tree, T::Error> {
    database.open_tree(tree_name).map_err(<T>::db_internal)
}

/// Wrapper to assemble a Batch for removing all elements of a tree
/// (to add into transaction where clear_tree should be)
pub fn make_batch_clear_tree <T: ErrorSource> (database_name: &str, tree_name: &[u8]) -> Result<Batch, T::Error> {
    let database = open_db::<T>(database_name)?;
    let tree = open_tree::<T>(&database, tree_name)?;
    let mut out = Batch::default();
    for (key, _) in tree.iter().flatten() {out.remove(key)}
    Ok(out)
}

/// Function to try and get from the Signer database the _valid_ current verifier for network using VerifierKey
#[cfg(feature = "signer")]
pub fn try_get_valid_current_verifier (verifier_key: &VerifierKey, database_name: &str) -> Result<Option<ValidCurrentVerifier>, ErrorSigner> {
    let general_verifier = get_general_verifier(database_name)?;
    let database = open_db::<Signer>(database_name)?;
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
                    CurrentVerifier::Dead => Err(ErrorSigner::DeadVerifier(verifier_key.to_owned())),
                }
            },
            Err(_) => Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::CurrentVerifier(verifier_key.to_owned())))),
        },
        Ok(None) => {
            if let Some((network_specs_key, _)) = genesis_hash_in_specs(verifier_key, &database)? {return Err(ErrorSigner::Database(DatabaseSigner::UnexpectedGenesisHash{verifier_key: verifier_key.to_owned(), network_specs_key}))}
            Ok(None)
        },
        Err(e) => Err(<Signer>::db_internal(e)),
    }
}

/// Function to get from the Signer database the current verifier for network using VerifierKey, returns error if not found
#[cfg(feature = "signer")]
pub fn get_valid_current_verifier (verifier_key: &VerifierKey, database_name: &str) -> Result<ValidCurrentVerifier, ErrorSigner> {
    try_get_valid_current_verifier(verifier_key, database_name)?
        .ok_or_else(|| ErrorSigner::NotFound(NotFoundSigner::CurrentVerifier(verifier_key.to_owned())))
}

/// Function to search for genesis hash corresponding to a given verifier key
/// in SPECSTREE of the Signer database
/// If there are more than one network corresponding to the same genesis hash,
/// outputs network specs key for the network with the lowest order
#[cfg(feature = "signer")]
pub fn genesis_hash_in_specs (verifier_key: &VerifierKey, database: &Db) -> Result<Option<(NetworkSpecsKey, NetworkSpecs)>, ErrorSigner> {
    let genesis_hash = verifier_key.genesis_hash();
    let chainspecs = open_tree::<Signer>(database, SPECSTREE)?;
    let mut specs_set: Vec<(NetworkSpecsKey, NetworkSpecs)> = Vec::new();
    let mut found_base58prefix = None;
    for (network_specs_key_vec, network_specs_encoded) in chainspecs.iter().flatten() {
        let network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
        let network_specs = NetworkSpecs::from_entry_with_key_checked::<Signer>(&network_specs_key, network_specs_encoded)?;
        if network_specs.genesis_hash == genesis_hash[..] {
            found_base58prefix = match found_base58prefix {
                Some(base58prefix) => {
                    if base58prefix == network_specs.base58prefix {Some(base58prefix)}
                    else {return Err(ErrorSigner::Database(DatabaseSigner::DifferentBase58Specs{genesis_hash: network_specs.genesis_hash, base58_1: base58prefix, base58_2: network_specs.base58prefix}))}
                },
                None => Some(network_specs.base58prefix),
            };
            specs_set.push((network_specs_key, network_specs))
        }
    }
    specs_set.sort_by(|a, b| a.1.order.cmp(&b.1.order));
    match specs_set.get(0) {
        Some(a) => Ok(Some(a.to_owned())),
        None => Ok(None),
    } 
}

/// Function to get general Verifier from the Signer database
/// Note that not finding general verifier is always an error.
#[cfg(feature = "signer")]
pub fn get_general_verifier (database_name: &str) -> Result<Verifier, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let settings = open_tree::<Signer>(&database, SETTREE)?;
    match settings.get(GENERALVERIFIER) {
        Ok(Some(verifier_encoded)) => match <Verifier>::decode(&mut &verifier_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::GeneralVerifier))),
        },
        Ok(None) => Err(ErrorSigner::NotFound(NotFoundSigner::GeneralVerifier)),
        Err(e) => Err(<Signer>::db_internal(e)),
    }
}

/// Function to display general Verifier from the Signer database
#[cfg(feature = "signer")]
pub fn display_general_verifier (database_name: &str) -> Result<String, ErrorSigner> {
    Ok(get_general_verifier(database_name)?.show_card())
}

/// Function to try and get types information from the database
/// Applicable to both Active side and Signer side
pub fn try_get_types <T: ErrorSource> (database_name: &str) -> Result<Option<Vec<TypeEntry>>, T::Error> {
    let database = open_db::<T>(database_name)?;
    let settings = open_tree::<T>(&database, SETTREE)?;
    match settings.get(TYPES) {
        Ok(Some(types_info_encoded)) => {
            match <Vec<TypeEntry>>::decode(&mut &types_info_encoded[..]) {
                Ok(a) => Ok(Some(a)),
                Err(_) => Err(<T>::faulty_database_types()),
            }
        },
        Ok(None) => Ok(None),
        Err(e) => Err(<T>::db_internal(e)),
    }
}

/// Function to get types information from the database, returns error if not found
/// Applicable to both Active side and Signer side
pub fn get_types <T: ErrorSource> (database_name: &str) -> Result<Vec<TypeEntry>, T::Error> {
    try_get_types::<T>(database_name)?
        .ok_or_else(|| <T>::types_not_found())
}

/// Function to try and get network specs from the Signer database
#[cfg(feature = "signer")]
pub fn try_get_network_specs (database_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<Option<NetworkSpecs>, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let chainspecs = open_tree::<Signer>(&database, SPECSTREE)?;
    match chainspecs.get(network_specs_key.key()) {
        Ok(Some(network_specs_encoded)) => Ok(Some(NetworkSpecs::from_entry_with_key_checked::<Signer>(network_specs_key, network_specs_encoded)?)),
        Ok(None) => Ok(None),
        Err(e) => Err(<Signer>::db_internal(e)),
    }
}

/// Function to get network specs from the Signer database, returns error if not found
#[cfg(feature = "signer")]
pub fn get_network_specs (database_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<NetworkSpecs, ErrorSigner> {
    try_get_network_specs(database_name, network_specs_key)?
        .ok_or_else(|| ErrorSigner::NotFound(NotFoundSigner::NetworkSpecs(network_specs_key.to_owned())))
}

/// Function to try and get address details from the Signer database
#[cfg(feature = "signer")]
pub fn try_get_address_details (database_name: &str, address_key: &AddressKey) -> Result<Option<AddressDetails>, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let identities = open_tree::<Signer>(&database, ADDRTREE)?;
    match identities.get(address_key.key()) {
        Ok(Some(address_details_encoded)) => Ok(Some(AddressDetails::from_entry_with_key_checked::<Signer>(address_key, address_details_encoded)?)),
        Ok(None) => Ok(None),
        Err(e) => Err(<Signer>::db_internal(e)),
    }
}

/// Function to get address details from the Signer database, returns error if not found
#[cfg(feature = "signer")]
pub fn get_address_details (database_name: &str, address_key: &AddressKey) -> Result<AddressDetails, ErrorSigner> {
    try_get_address_details(database_name, address_key)?
        .ok_or_else(|| ErrorSigner::NotFound(NotFoundSigner::AddressDetails(address_key.to_owned())))
}

/// Function to collect MetaValues corresponding to given network name.
/// Applicable to both Active side and Signer side
pub fn get_meta_values_by_name <T: ErrorSource> (database_name: &str, network_name: &str) -> Result<Vec<MetaValues>, T::Error> {
    let database = open_db::<T>(database_name)?;
    let metadata = open_tree::<T>(&database, METATREE)?;
    let mut out: Vec<MetaValues> = Vec::new();
    let meta_key_prefix = MetaKeyPrefix::from_name(network_name);
    for x in metadata.scan_prefix(meta_key_prefix.prefix()).flatten() {
        let meta_values = MetaValues::from_entry_checked::<T>(x)?;
        if meta_values.name == network_name {out.push(meta_values)}
    }
    Ok(out)
}

/// Function to get MetaValues corresponding to given network name and version.
/// Returns error if finds nothing.
/// Applicable to both Active side and Signer side.
pub fn get_meta_values_by_name_version <T: ErrorSource> (database_name: &str, network_name: &str, network_version: u32) -> Result<MetaValues, T::Error> {
    let database = open_db::<T>(database_name)?;
    let metadata = open_tree::<T>(&database, METATREE)?;
    let meta_key = MetaKey::from_parts(network_name, network_version);
    match metadata.get(meta_key.key()) {
        Ok(Some(meta)) => MetaValues::from_entry_name_version_checked::<T>(network_name, network_version, meta),
        Ok(None) => Err(<T>::metadata_not_found(network_name.to_string(), network_version)),
        Err(e) => Err(<T>::db_internal(e)),
    }
}

/// Function to modify existing batch for ADDRTREE with incoming vector of additions
#[cfg(any(feature = "active", feature = "signer"))]
pub (crate) fn upd_id_batch (mut batch: Batch, adds: Vec<(AddressKey, AddressDetails)>) -> Batch {
    for (address_key, address_details) in adds.iter() {batch.insert(address_key.key(), address_details.encode());}
    batch
}

/// Function to verify checksum in Signer database
#[cfg(feature = "signer")]
pub fn verify_checksum (database: &Db, checksum: u32) -> Result<(), ErrorSigner> {
    let real_checksum = match database.checksum() {
        Ok(x) => x,
        Err(e) => return Err(<Signer>::db_internal(e)),
    };
    if checksum != real_checksum {return Err(ErrorSigner::Database(DatabaseSigner::ChecksumMismatch))}
    Ok(())
}

/// Function to get the danger status from the Signer database.
/// Function interacts with user interface.
#[cfg(feature = "signer")]
pub fn get_danger_status(database_name: &str) -> Result<bool, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let settings = open_tree::<Signer>(&database, SETTREE)?;
    match settings.get(DANGER) {
        Ok(Some(a)) => DangerRecord::from_ivec(&a).device_was_online(),
        Ok(None) => Err(ErrorSigner::NotFound(NotFoundSigner::DangerStatus)),
        Err(e) => Err(<Signer>::db_internal(e)),
    }
}
