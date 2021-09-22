use sled::{Db, Tree, open, IVec};
use anyhow;
use definitions::{metadata::{NameVersioned, VersionDecoded}, network_specs::{ChainSpecs, NetworkKey, generate_network_key}, users::AddressDetails};
use meta_reading::decode_metadata::get_meta_const;
use parity_scale_codec::Decode;

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

/// Wrapper for `drop_tree` with crate error
pub fn drop_tree (database: &Db, tree_name: &[u8]) -> anyhow::Result<()> {
    match database.drop_tree(tree_name) {
        Ok(_) => Ok(()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

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
pub fn get_and_decode_chain_specs(chainspecs: &Tree, network_key: &NetworkKey) -> anyhow::Result<ChainSpecs> {
    match chainspecs.get(network_key) {
        Ok(Some(chain_specs_encoded)) => decode_chain_specs(chain_specs_encoded, network_key),
        Ok(None) => return Err(Error::NotFound(NotFound::NetworkKey).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Function to decode SCALE encoded network specs into ChainSpecs,
/// and check for genesis hash mismatch
pub fn decode_chain_specs(chain_specs_encoded: IVec, network_key: &NetworkKey) -> anyhow::Result<ChainSpecs> {
    match <ChainSpecs>::decode(&mut &chain_specs_encoded[..]) {
        Ok(a) => {
            if &generate_network_key(&a.genesis_hash.to_vec()) != network_key {return Err(Error::GenesisHashMismatch.show())}
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
pub fn check_metadata(meta: Vec<u8>, versioned_name: &NameVersioned) -> anyhow::Result<Vec<u8>> {
    let version_vector = match get_meta_const(&meta.to_vec()) {
        Ok(a) => a,
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::Metadata).show()),
    };
    let version = match VersionDecoded::decode(&mut &version_vector[..]) {
        Ok(a) => a,
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::Version).show()),
    };
    if version.specname != versioned_name.name {return Err(Error::MetadataNameMismatch.show())}
    if version.spec_version != versioned_name.version {return Err(Error::MetadataVersionMismatch.show())}
    Ok(meta)
}
