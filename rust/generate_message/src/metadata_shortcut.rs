use sled::Tree;
use meta_reading::{decode_metadata::decode_version, fetch_metadata::{fetch_info, fetch_info_with_chainspecs}, interpret_chainspecs::interpret_properties};
use constants::{COLOR, SECONDARY_COLOR};
use definitions::{crypto::Encryption, metadata::MetaValues, network_specs::ChainSpecsToSend};
use std::convert::TryInto;
use db_handling::{helpers::unhex, error::NotHex};
use anyhow;

use crate::error::{Error, NotDecodeable};
use crate::helpers::{genesis_hash_in_hot_db, filter_address_book_by_url, process_indices};


/// Struct to store MetaValues and genesis hash for network
pub struct MetaShortCut {
    pub meta_values: MetaValues,
    pub genesis_hash: [u8; 32],
}

/// Function to process address as &str, fetch metadata and genesis hash for it,
/// and output MetaShortCut value in case of success
pub fn meta_shortcut (address: &str) -> anyhow::Result<MetaShortCut> {

    let new_info = match fetch_info(address) {
        Ok(a) => a,
        Err(e) => return Err(Error::FetchFailed{address: address.to_string(), error: e.to_string()}.show()),
    };
    let genesis_hash = get_genesis_hash(&new_info.genesis_hash)?;
    let meta_values = match decode_version(&new_info.meta) {
        Ok(a) => a,
        Err(e) => return Err(Error::NotDecodeable(NotDecodeable::FetchedMetadata{address: address.to_string(), error: e.to_string()}).show())
    };
    Ok(MetaShortCut{
        meta_values,
        genesis_hash,
    })
}


/// Struct to store MetaValues, genesis hash, and ChainSpecsToSend for network
pub struct MetaSpecsShortCut {
    pub meta_values: MetaValues,
    pub specs: ChainSpecsToSend,
    pub update: bool, // flag to indicate that the database has no exact entry created
}


/// Function to process address as &str, fetch metadata, genesis hash, and chainspecs
/// for it, and output MetaSpecsShortCut value in case of success
pub fn meta_specs_shortcut (address: &str, address_book: &Tree, chainspecs: &Tree, encryption: Encryption) -> anyhow::Result<MetaSpecsShortCut> {

    let entries = filter_address_book_by_url(address, address_book)?;
    let new_info = match fetch_info_with_chainspecs(address) {
        Ok(a) => a,
        Err(e) => return Err(Error::FetchFailed{address: address.to_string(), error: e.to_string()}.show()),
    };
    let genesis_hash = get_genesis_hash(&new_info.genesis_hash)?;
    let meta_values = match decode_version(&new_info.meta) {
        Ok(a) => a,
        Err(e) => return Err(Error::NotDecodeable(NotDecodeable::FetchedMetadata{address: address.to_string(), error: e.to_string()}).show())
    };
    let new_properties = match interpret_properties(&new_info.properties) {
        Ok(a) => a,
        Err(e) => return Err(Error::BadNetworkProperties{address: address.to_string(), error: e.to_string()}.show()),
    };
    if entries.len() == 0 {
        if genesis_hash_in_hot_db (genesis_hash, chainspecs)? {return Err(Error::NoEntriesExpected(address.to_string()).show())}
        let specs = ChainSpecsToSend {
            base58prefix: new_properties.base58prefix,
            color: COLOR.to_string(),
            decimals: new_properties.decimals,
            encryption,
            genesis_hash,
            logo: meta_values.name.to_string(),
            name: meta_values.name.to_string(),
            path_id: format!("//{}", meta_values.name),
            secondary_color: SECONDARY_COLOR.to_string(),
            title: format!("{}-{}", meta_values.name, encryption.show()),
            unit: new_properties.unit.to_string(),
        };
        Ok(MetaSpecsShortCut{
            meta_values,
            specs,
            update: true,
        })
    }
    else {
        let (specs, update) = process_indices(&entries, chainspecs, encryption)?;
        if specs.base58prefix != new_properties.base58prefix {return Err(Error::Base58Changed(address.to_string()).show())}
        if specs.decimals != new_properties.decimals {return Err(Error::DecimalsChanged(address.to_string()).show())}
        if specs.unit != new_properties.unit {return Err(Error::UnitChanged(address.to_string()).show())}
        if specs.name != meta_values.name {return Err(Error::NameChanged(address.to_string()).show())}
        // ChainSpecsToSend are good, can use them
        Ok(MetaSpecsShortCut{
            meta_values,
            specs,
            update,
        })
    }
}

/// Helper function to interpret freshly fetched genesis hash
fn get_genesis_hash (fetched_genesis_hash: &str) -> anyhow::Result<[u8; 32]> {
    let genesis_hash_vec = unhex(fetched_genesis_hash, NotHex::GenesisHash)?;
    let out: [u8; 32] = match genesis_hash_vec.try_into() {
        Ok(a) => a,
        Err(_) => return Err(Error::UnexpectedGenesisHashFormat.show())
    };
    Ok(out)
}
