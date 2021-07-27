use definitions::{constants::{METATREE, SETTREE, SPECSTREE, TYPES}, metadata::{MetaValues, NameVersioned}, network_specs::{ChainSpecsToSend}, types::TypeEntry};
use parity_scale_codec::{Decode, Encode};
use anyhow;

use crate::error::{Error, NotFound, NotDecodeable};
use crate::helpers::{open_db, open_tree, decode_chain_specs, check_metadata};


/// Function to get types info from the database

pub fn prep_types (database_name: &str) -> anyhow::Result<Vec<u8>> {
    
    let database = open_db(database_name)?;
    let settings = open_tree(&database, SETTREE)?;
    
    let types_info = match settings.get(TYPES) {
        Ok(Some(a)) => a.to_vec(),
        Ok(None) => return Err(Error::NotFound(NotFound::Types).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    match <Vec<TypeEntry>>::decode(&mut &types_info[..]) {
        Ok(_) => Ok(types_info),
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::Types).show()),
    }
}


/// Function to get encoded ChainSpecsToSend from the cold (!!!) database searching by network name
/// !!! for cold db only !!!
/// Cuts off the verifier and order, used for preparation of messages

pub fn get_network_specs (network_name: &str, database_name: &str) -> anyhow::Result<Vec<u8>> {
    
    let database = open_db(database_name)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    
    let mut found_network_specs = None;
    
    for x in chainspecs.iter() {
        if let Ok((network_key, network_specs_encoded)) = x {
            let network_specs = decode_chain_specs(network_specs_encoded, &network_key.to_vec())?;
            if network_specs.name == network_name {
                found_network_specs = Some(network_specs);
                break;
            }
        }
    }
    match found_network_specs {
        Some(network_specs) => {
            let network_specs_to_send = ChainSpecsToSend {
                base58prefix: network_specs.base58prefix,
                color: network_specs.color,
                decimals: network_specs.decimals,
                genesis_hash: network_specs.genesis_hash,
                logo: network_specs.logo,
                name: network_specs.name,
                path_id: network_specs.path_id,
                secondary_color: network_specs.secondary_color,
                title: network_specs.title,
                unit: network_specs.unit,
            };
            Ok(network_specs_to_send.encode())
        },
        None => return Err(Error::NotFound(NotFound::NetworkSpecs(network_name.to_string())).show()),
    }
    
}


/// Function to get genesis hash from the database searching by network name, for cold db only.

pub fn get_genesis_hash (network_name: &str, database_name: &str) -> anyhow::Result<Vec<u8>> {
    
    let database = open_db(database_name)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    
    let mut found_genesis_hash = None;
    
    for x in chainspecs.iter() {
        if let Ok((network_key, network_specs_encoded)) = x {
            let network_specs = decode_chain_specs(network_specs_encoded, &network_key.to_vec())?;
            if network_specs.name == network_name {
                found_genesis_hash = Some(network_specs.genesis_hash.to_vec());
                break;
            }
        }
    }
    match found_genesis_hash {
        Some(a) => Ok(a),
        None => return Err(Error::NotFound(NotFound::NetworkSpecs(network_name.to_string())).show()),
    }
    
}


/// Function to get metadata from the database searching by network name and version

pub fn get_metadata (network_name: &str, network_version: u32, database_name: &str) -> anyhow::Result<Vec<u8>> {
    
    let database = open_db(database_name)?;
    let metadata = open_tree(&database, METATREE)?;
    
    let versioned_name = NameVersioned {
        name: network_name.to_string(),
        version: network_version,
    };
    
    match metadata.get(versioned_name.encode()) {
        Ok(Some(meta)) => {
            check_metadata(meta.to_vec(), &versioned_name)
        },
        Ok(None) => return Err(Error::NotFound(NotFound::NameVersioned(versioned_name)).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
    
}


/// Function to get LATEST metadata from the database searching by network name

pub fn get_latest_metadata (network_name: &str, database_name: &str) -> anyhow::Result<Vec<u8>> {
    
    let database = open_db(database_name)?;
    let metadata = open_tree(&database, METATREE)?;
    
    let mut latest_version_meta_values: Option<MetaValues> = None;
    
    for x in metadata.scan_prefix(network_name.encode()) {
        if let Ok((versioned_name_encoded, meta)) = x {
            let versioned_name = match <NameVersioned>::decode(&mut &versioned_name_encoded[..]) {
                Ok(a) => a,
                Err(_) => return Err(Error::NotDecodeable(NotDecodeable::NameVersioned).show()),
            };
            let meta = check_metadata(meta.to_vec(), &versioned_name)?;
            
            latest_version_meta_values = match latest_version_meta_values {
                Some(a) => {
                    if a.version < versioned_name.version {
                        Some (MetaValues {
                            name: network_name.to_string(),
                            version: versioned_name.version,
                            meta,
                        })
                    }
                    else {Some(a)}
                },
                None => {
                    Some (MetaValues {
                        name: network_name.to_string(),
                        version: versioned_name.version,
                        meta,
                    })
                }
            };
        }
    }
    
    match latest_version_meta_values {
        Some(a) => Ok(a.meta),
        None => return Err(Error::NotFound(NotFound::MetaFromName(network_name.to_string())).show()),
    }
}


/// Function to get contents for load_metadata message from the database

pub fn prep_load_metadata (network_name: &str, network_version: u32, database_name: &str) -> anyhow::Result<Vec<u8>> {
    let metadata_vector = get_metadata (network_name, network_version, database_name)?;
    let genesis_hash_vector = get_genesis_hash (network_name, database_name)?;
    Ok([metadata_vector, genesis_hash_vector].concat())
}


/// Function to get contents for load_metadata message from the database

pub fn prep_add_network_versioned (network_name: &str, network_version: u32, database_name: &str) -> anyhow::Result<Vec<u8>> {
    let metadata_vector = get_metadata (network_name, network_version, database_name)?;
    let encoded_network_specs_vector = get_network_specs (network_name, database_name)?;
    Ok([metadata_vector, encoded_network_specs_vector].concat())
}


/// Function to get contents for load_metadata message from the database

pub fn prep_add_network_latest (network_name: &str, database_name: &str) -> anyhow::Result<Vec<u8>> {
    let metadata_vector = get_latest_metadata (network_name, database_name)?;
    let encoded_network_specs_vector = get_network_specs (network_name, database_name)?;
    Ok([metadata_vector, encoded_network_specs_vector].concat())
}
