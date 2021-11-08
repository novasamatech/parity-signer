use sled::{Db, Tree, open, IVec};
use hex;
use definitions::network_specs::{Verifier, generate_verifier_key};
use parity_scale_codec::Decode;

use crate::error::{Error, BadInputData, DatabaseError};

/// Wrapper for `open` with crate error (card)
pub fn open_db (database_name: &str) -> Result<Db, Error> {
    match open(database_name) {
        Ok(x) => Ok(x),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}

/// Wrapper for `open_tree` with crate error (card)
pub fn open_tree (database: &Db, tree_name: &[u8]) -> Result<Tree, Error> {
    match database.open_tree(tree_name) {
        Ok(x) => Ok(x),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}

/// Wrapper for `drop_tree` with crate error (card)
pub fn drop_tree (database: &Db, tree_name: &[u8]) -> Result<(), Error> {
    match database.drop_tree(tree_name) {
        Ok(_) => Ok(()),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}

/// Wrapper for `flush` with crate error (card)
pub fn flush_db (database: &Db) -> Result<(), Error> {
    match database.flush() {
        Ok(_) => Ok(()),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}

/// Wrapper for `insert` with crate error (card), not catching previous value during insertion
pub fn insert_into_tree(key: Vec<u8>, value: Vec<u8>, tree: &Tree) -> Result<(), Error> {
    match tree.insert(key, value) {
        Ok(_) => Ok(()),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}

/// Wrapper for `checksum` for database, with crate error (card)
pub fn get_checksum(database: &Db) -> Result<u32, Error> {
    match database.checksum() {
        Ok(x) => Ok(x),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}

/// Wrapper for `get` with crate error (card)
pub fn get_from_tree(key: &Vec<u8>, tree: &Tree) -> Result<Option<IVec>, Error> {
    match tree.get(key) {
        Ok(x) => Ok(x),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}

/// Function to decode hex string (possibly with `0x` start) into Vec<u8>, with crate error (card)
pub fn unhex(hex_entry: &str) -> Result<Vec<u8>, Error> {
    let hex_entry = {
        if hex_entry.starts_with("0x") {&hex_entry[2..]}
        else {hex_entry}
    };
    match hex::decode(hex_entry) {
        Ok(x) => Ok(x),
        Err(_) => return Err(Error::BadInputData(BadInputData::NotHex)),
    }
}

/// Function to get verifier for network with given genesis hash, with crate error (card)
pub fn get_verifier (genesis_hash: [u8; 32], verifiers: &Tree) -> Result<Verifier, Error> {
    match verifiers.get(&generate_verifier_key(&genesis_hash.to_vec())) {
        Ok(Some(verifier_encoded)) => match <Verifier>::decode(&mut &verifier_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedNetworkVerifier)),
        },
        Ok(None) => return Err(Error::DatabaseError(DatabaseError::NoNetworkVerifier(genesis_hash))),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}
