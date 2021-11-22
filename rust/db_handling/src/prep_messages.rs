use constants::{METATREE, SETTREE, SPECSTREE, TYPES};
use definitions::{keyring::{MetaKey, NetworkSpecsKey}, network_specs::{ChainSpecsToSend}, qr_transfers::{ContentLoadTypes, ContentLoadMeta, ContentAddSpecs}};
use anyhow;

use crate::error::{Error, NotFound, NotDecodeable, NotHex};
use crate::helpers::{open_db, open_tree, decode_chain_specs, check_metadata, unhex};


/// Function to get types info from the database
pub fn prep_types (database_name: &str) -> anyhow::Result<ContentLoadTypes> {
    let database = open_db(database_name)?;
    let settings = open_tree(&database, SETTREE)?;
    let types_info_encoded = match settings.get(TYPES) {
        Ok(Some(a)) => a.to_vec(),
        Ok(None) => return Err(Error::NotFound(NotFound::Types).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    let out = ContentLoadTypes::from_vec(&types_info_encoded);
    match out.types() {
        Ok(_) => Ok(out),
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::Types).show()),
    }
}

/// Function to get encoded ChainSpecsToSend from the cold (!!!) database searching by network name
/// !!! for cold db only !!!
/// Cuts off the verifier and order, used for preparation of messages
pub fn prep_network_specs (network_specs_key_hex: &str, database_name: &str) -> anyhow::Result<ContentAddSpecs> {
    let database = open_db(database_name)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    let network_specs_key = NetworkSpecsKey::from_vec(&unhex(network_specs_key_hex, NotHex::NetworkSpecsKey)?);
    let network_specs = match chainspecs.get(&network_specs_key.key()) {
        Ok(Some(network_specs_encoded)) => decode_chain_specs(network_specs_encoded, &network_specs_key)?,
        Ok(None) => return Err(Error::NotFound(NotFound::NetworkSpecsKey).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    let network_specs_to_send = ChainSpecsToSend {
        base58prefix: network_specs.base58prefix,
        color: network_specs.color,
        decimals: network_specs.decimals,
        encryption: network_specs.encryption,
        genesis_hash: network_specs.genesis_hash,
        logo: network_specs.logo,
        name: network_specs.name,
        path_id: network_specs.path_id,
        secondary_color: network_specs.secondary_color,
        title: network_specs.title,
        unit: network_specs.unit,
    };
    Ok(ContentAddSpecs::generate(&network_specs_to_send))
}


/// Function to get genesis hash from the database searching by network name, for cold db only.
pub fn get_genesis_hash (network_name: &str, database_name: &str) -> anyhow::Result<[u8;32]> {
    let database = open_db(database_name)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    let mut found_genesis_hash = None;
    for x in chainspecs.iter() {
        if let Ok((network_specs_key_vec, network_specs_encoded)) = x {
            let network_specs = decode_chain_specs(network_specs_encoded, &NetworkSpecsKey::from_vec(&network_specs_key_vec.to_vec()))?;
            if network_specs.name == network_name {
                found_genesis_hash = Some(network_specs.genesis_hash);
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
    let meta_key = MetaKey::from_parts(network_name, network_version);
    match metadata.get(meta_key.key()) {
        Ok(Some(meta)) => {check_metadata(meta.to_vec(), network_name, network_version)},
        Ok(None) => return Err(Error::NotFound(NotFound::NameVersioned{name: network_name.to_string(), version: network_version}).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Function to get contents for load_metadata message from the database
pub fn prep_load_metadata (network_name: &str, network_version: u32, database_name: &str) -> anyhow::Result<ContentLoadMeta> {
    let metadata_vector = get_metadata (network_name, network_version, database_name)?;
    let genesis_hash = get_genesis_hash (network_name, database_name)?;
    Ok(ContentLoadMeta::generate(&metadata_vector, &genesis_hash))
}

