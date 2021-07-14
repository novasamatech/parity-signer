use frame_metadata::{RuntimeMetadataV12};
use meta_reading::decode_metadata::{get_meta_const_light};
use definitions::{constants::{GENERALVERIFIER, TRANSACTION, TYPES}, network_specs::{ChainSpecs, Verifier, NetworkKey}, metadata::{NameVersioned, VersionDecoded}, types::TypeEntry};
use parity_scale_codec::{Decode, Encode};
use sled::{Db, open, Tree};

use super::error::{Error, DatabaseError, SystemError};

/// Function searches for full metadata for certain chain name and version in metadata database tree.
/// Checks that found full metadata indeed corresponds to the queried name and version;
/// in case of successful find produces a tuple of corresponding RuntimeMetadataV12 and Option<u32>;
/// Option is None if the version of chain is the latest known one,
/// and Some(latest_version) if there are later versions available.

pub fn find_meta(chain_name: &str, version: u32, metadata: &Tree) -> Result<(RuntimeMetadataV12, Option<u32>), Error> {
    
    let mut meta = None;
    let mut other = false;
    let mut latest_version = version;
    
    for x in metadata.scan_prefix(chain_name.encode()) {
        let (name, meta_found) = match x {
            Ok(t) => t,
            Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
        };
        let versioned_name = match <NameVersioned>::decode(&mut &name[..]) {
            Ok(t) => t,
            Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedVersName)),
        };
        if versioned_name.version == version {meta = Some(meta_found)}
        else {
            other = true;
            if versioned_name.version > latest_version {latest_version = versioned_name.version}
        }
    }
    
    match meta {
        Some(m) => {
            if !m.starts_with(&vec![109, 101, 116, 97]) {return Err(Error::SystemError(SystemError::NotMeta))}
            if m[4] < 12 {
                return Err(Error::SystemError(SystemError::MetaVersionBelow12));
            }
            let data_back = RuntimeMetadataV12::decode(&mut &m[5..]);
            match data_back {
                Ok(metadata) => {
                // check if the name and version are same in metadata, i.e. the database is not damaged
                    match get_meta_const_light(&metadata) {
                        Ok(x) => {
                            match VersionDecoded::decode(&mut &x[..]) {
                                Ok(y) => {
                                    if (y.spec_version != version) || (y.specname != chain_name) {return Err(Error::SystemError(SystemError::MetaMismatch))}
                                },
                                Err(_) => return Err(Error::SystemError(SystemError::VersionNotDecodeable))
                            }
                        },
                        Err(_) => return Err(Error::SystemError(SystemError::NoVersion))
                    };
                    if version < latest_version {
                        Ok((metadata, Some(latest_version)))
                    }
                    else {Ok((metadata, None))}
                },
                Err(_) => return Err(Error::SystemError(SystemError::UnableToDecodeMeta)),
            }
        },
        None => {
            if other {return Err(Error::DatabaseError(DatabaseError::NoMetaThisVersion))}
            else {return Err(Error::DatabaseError(DatabaseError::NoMetaAtAll))}
        },
    }
}


/// Function to search for network_key (genesis_hash at the moment) in chainspecs database tree
pub fn get_chainspecs (network_key: &NetworkKey, chainspecs: &Tree) -> Result<ChainSpecs, Error> {
    match chainspecs.get(network_key) {
        Ok(chainspecs_db_reply) => {
            match chainspecs_db_reply {
                Some(x) => {
                // some entry found for this network_key
                    match <ChainSpecs>::decode(&mut &x[..]) {
                        Ok(y) => Ok(y),
                        Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedChainSpecs)),
                    }
                },
                None => {
                // no entry exists
                    return Err(Error::DatabaseError(DatabaseError::NoNetwork))
                },
            }
        },
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}


/// function to search database for the TypeEntry vector
pub fn get_types (settings: &Tree) -> Result<Vec<TypeEntry>, Error> {
    match settings.get(TYPES) {
        Ok(types_db_reply) => {
            match types_db_reply {
                Some(a) => {
                    match <Vec<TypeEntry>>::decode(&mut &a[..]) {
                        Ok(x) => {
                            if x.len()==0 {return Err(Error::DatabaseError(DatabaseError::NoTypes))}
                            Ok(x)
                        },
                        Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedTypesDatabase)),
                    }
                },
                None => return Err(Error::DatabaseError(DatabaseError::NoTypes)),
            }
        },
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}


/// function to get general verifier from the database

pub fn get_general_verifier (settings: &Tree) -> Result<Verifier, Error> {
    
    match settings.get(GENERALVERIFIER) {
        Ok(reply) => {
            match reply {
                Some(a) => {
                    match <Verifier>::decode(&mut &a[..]) {
                        Ok(x) => Ok(x),
                        Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedGeneralVerifier)),
                    }
                },
                None => return Err(Error::DatabaseError(DatabaseError::NoGeneralVerifier)),
            }
        },
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
}


/// function to clear all previous (if any) saves in the database
pub fn purge (dbname: &str) -> Result<(), Error> {
    
    let database: Db = match open(dbname) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    
    match database.drop_tree(TRANSACTION) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    
    Ok(())
    
}
