use frame_metadata::{RuntimeMetadataV12};
use meta_reading::{get_meta_const_light, decode_version};
use db_handling::{metadata::NameVersioned};
use parity_scale_codec::{Decode, Encode};
use sled::Tree;

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
            if m[0] < 12 {
                return Err(Error::SystemError(SystemError::MetaVersionBelow12));
            }
            let data_back = RuntimeMetadataV12::decode(&mut &m[1..]);
            match data_back {
                Ok(metadata) => {
                // check if the name and version are same in metadata, i.e. the database is not damaged
                    match get_meta_const_light(&metadata) {
                        Ok(x) => {
                            let check = decode_version(x);
                            if (check.spec_version != version) || (check.specname != chain_name) {return Err(Error::SystemError(SystemError::MetaMismatch))}
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

