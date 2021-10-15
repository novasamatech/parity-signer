use sled::{Db, Tree, Batch, open, IVec};
use anyhow;
use constants::{GENERALVERIFIER, SETTREE, VERIFIERS};
use definitions::{crypto::Encryption, metadata::VersionDecoded, keyring::{AddressKey, NetworkSpecsKey, VerifierKey}, network_specs::{ChainSpecs, CurrentVerifier, Verifier}, users::{AddressDetails}};
use meta_reading::decode_metadata::get_meta_const;
use parity_scale_codec::{Decode, Encode};

use crate::error::{Error, NotDecodeable, NotFound, NotHex};

/// Wrapper for `open` with crate error
pub fn open_db (database_name: &str) -> anyhow::Result<Db> {
    match open(database_name) {
        Ok(x) => Ok(x),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Wrapper for `open_tree` with crate error
pub fn open_tree (database: &Db, tree_name: &[u8]) -> anyhow::Result<Tree> {
    match database.open_tree(tree_name) {
        Ok(x) => Ok(x),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Wrapper to assemble a Batch for removing all elements of a tree
/// (to add into transaction where clear_tree should be)
pub fn make_batch_clear_tree (database_name: &str, tree_name: &[u8]) -> anyhow::Result<Batch> {
    let database = open_db(database_name)?;
    let tree = open_tree(&database, tree_name)?;
    let mut out = Batch::default();
    for x in tree.iter() {
        if let Ok((key, _)) = x {out.remove(key)}
    }
    Ok(out)
}
/*
/// Wrapper for `flush` with crate error
pub fn flush_db (database: &Db) -> anyhow::Result<()> {
    match database.flush() {
        Ok(_) => Ok(()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Wrapper for `clear` with crate error
pub fn clear_tree(tree: &Tree) -> anyhow::Result<()> {
    match tree.clear() {
        Ok(()) => Ok(()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Wrapper for `insert` with crate error, not catching previous value during insertion
pub fn insert_into_tree(key: Vec<u8>, value: Vec<u8>, tree: &Tree) -> anyhow::Result<()> {
    match tree.insert(key, value) {
        Ok(_) => Ok(()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Wrapper for `remove` with crate error, not catching previous value during removal
pub fn remove_from_tree(key: Vec<u8>, tree: &Tree) -> anyhow::Result<()> {
    match tree.remove(key) {
        Ok(_) => Ok(()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}
*/
/// Function to decode hex encoded &str into Vec<u8>,
/// `what` is enum of possible NotHex failures
pub fn unhex(hex_entry: &str, what: NotHex) -> anyhow::Result<Vec<u8>> {
    let hex_entry = {
        if hex_entry.starts_with("0x") {&hex_entry[2..]}
        else {hex_entry}
    };
    match hex::decode(hex_entry) {
        Ok(x) => Ok(x),
        Err(_) => return Err(Error::NotHex(what).show()),
    }
}

/// Function to get SCALE encoded network specs entry by given network_key, decode it
/// as ChainSpecs, and check for genesis hash mismatch. Is used forrom cold database
pub fn get_and_decode_chain_specs(chainspecs: &Tree, network_specs_key: &NetworkSpecsKey) -> anyhow::Result<ChainSpecs> {
    match chainspecs.get(network_specs_key.key()) {
        Ok(Some(chain_specs_encoded)) => decode_chain_specs(chain_specs_encoded, network_specs_key),
        Ok(None) => return Err(Error::NotFound(NotFound::NetworkSpecsKey).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Function to decode SCALE encoded network specs into ChainSpecs,
/// and check for genesis hash mismatch
pub fn decode_chain_specs(chain_specs_encoded: IVec, network_specs_key: &NetworkSpecsKey) -> anyhow::Result<ChainSpecs> {
    match <ChainSpecs>::decode(&mut &chain_specs_encoded[..]) {
        Ok(a) => {
            if network_specs_key != &NetworkSpecsKey::from_parts(&a.genesis_hash.to_vec(), &a.encryption) {return Err(Error::NetworkSpecsKeyMismatch.show())}
            Ok(a)
        },
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::ChainSpecs).show()),
    }
}

/// Function to decode SCALE encoded network specs into ChainSpecs,
/// and check for genesis hash mismatch
pub fn decode_address_details(address_details_encoded: IVec) -> anyhow::Result<AddressDetails> {
    match <AddressDetails>::decode(&mut &address_details_encoded[..]) {
        Ok(a) => Ok(a),
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::AddressDetails).show()),
    }
}

/// Function to check metadata vector from the database, and output if it's ok
pub fn check_metadata(meta: Vec<u8>, network_name: &str, network_version: u32) -> anyhow::Result<Vec<u8>> {
    let version_vector = match get_meta_const(&meta) {
        Ok(a) => a,
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::Metadata).show()),
    };
    let version = match VersionDecoded::decode(&mut &version_vector[..]) {
        Ok(a) => a,
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::Version).show()),
    };
    if version.specname != network_name {return Err(Error::MetadataNameMismatch.show())}
    if version.spec_version != network_version {return Err(Error::MetadataVersionMismatch.show())}
    Ok(meta)
}

/// Function to find encryption aldorithm corresponding to network with known network key
pub fn check_encryption (chainspecs: &Tree, network_specs_key: &NetworkSpecsKey) -> anyhow::Result<()> {
    let from_specs = get_and_decode_chain_specs(chainspecs, network_specs_key)?.encryption;
    let (_, from_key) = reverse_network_specs_key(network_specs_key)?;
    if from_specs == from_key {Ok(())}
    else {return Err(Error::EncryptionMismatchNetwork.show())}
}

/// Function to generate address key with crate error
pub fn generate_address_key (public: &Vec<u8>, encryption: &Encryption) -> anyhow::Result<AddressKey> {
    match AddressKey::from_parts(public, encryption) {
        Ok(a) => Ok(a),
        Err(e) => return Err(Error::AddressKey(e.to_string()).show()),
    }
}

/// Function to produce public key and encryption from AddressKey
pub fn reverse_address_key (address_key: &AddressKey) -> anyhow::Result<(Vec<u8>, Encryption)> {
    match address_key.public_key_encryption(){
        Ok(a) => Ok(a),
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::AddressKey).show())
    }
}

/// Helper function to get genesis hash and encryption from network key
pub fn reverse_network_specs_key (network_specs_key: &NetworkSpecsKey) -> anyhow::Result<(Vec<u8>, Encryption)> {
    match network_specs_key.genesis_hash_encryption() {
        Ok(a) => Ok(a),
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::NetworkSpecsKey).show()),
    }
}
/*
/// Function to determine if there are entries with similar genesis hash left in the database;
/// searches through chainspecs tree of the cold database for the given genesis hash
pub fn genesis_hash_in_cold_db (genesis_hash: [u8; 32], chainspecs: &Tree) -> anyhow::Result<bool> {
    let mut out = false;
    for x in chainspecs.iter() {
        if let Ok((network_specs_key_vec, chain_specs_encoded)) = x {
            let network_specs = decode_chain_specs(chain_specs_encoded, &NetworkSpecsKey::from_vec(&network_specs_key_vec.to_vec()))?;
            if network_specs.genesis_hash == genesis_hash {
                out = true;
                break;
            }
        }
    }
    Ok(out)
}
*/
/// Function to get Verifier from the database for network using VerifierKey
pub fn get_current_verifier (verifier_key: &VerifierKey, database_name: &str) -> anyhow::Result<CurrentVerifier> {
    let database = open_db(&database_name)?;
    let verifiers = open_tree(&database, VERIFIERS)?;
    match verifiers.get(verifier_key.key()) {
        Ok(Some(verifier_encoded)) => match <CurrentVerifier>::decode(&mut &verifier_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => return Err(Error::NotDecodeable(NotDecodeable::CurrentVerifier).show()),
        },
        Ok(None) => return Err(Error::NotFound(NotFound::CurrentVerifier).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Function to get general Verifier from the database
pub fn get_general_verifier (database_name: &str) -> anyhow::Result<Verifier> {
    let database = open_db(&database_name)?;
    let settings = open_tree(&database, SETTREE)?;
    match settings.get(GENERALVERIFIER.to_vec()) {
        Ok(Some(verifier_encoded)) => match <Verifier>::decode(&mut &verifier_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => return Err(Error::NotDecodeable(NotDecodeable::GeneralVerifier).show()),
        },
        Ok(None) => return Err(Error::NotFound(NotFound::GeneralVerifier).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Function to modify existing batch for ADDRTREE with incoming vector of additions
pub (crate) fn upd_id_batch (mut batch: Batch, adds: Vec<(AddressKey, AddressDetails)>) -> Batch {
    for (address_key, address_details) in adds.iter() {batch.insert(address_key.key(), address_details.encode());}
    batch
}

/// Function to verify checksum
pub fn verify_checksum (database: &Db, checksum: u32) -> anyhow::Result<()> {
    let real_checksum = match database.checksum() {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    if checksum != real_checksum {return Err(Error::ChecksumMismatch.show())}
    Ok(())
}

