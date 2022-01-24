use constants::{COLOR, SECONDARY_COLOR};
use definitions::{crypto::Encryption, error::{Active, Changed, DatabaseActive, ErrorActive, Fetch, IncomingMetadataSourceActiveStr, NotHexActive}, helpers::unhex, metadata::MetaValues, network_specs::NetworkSpecsToSend};
use std::convert::TryInto;

use crate::fetch_metadata::{fetch_info, fetch_info_with_network_specs};
use crate::helpers::{genesis_hash_in_hot_db, filter_address_book_by_url, process_indices};
use crate::interpret_specs::interpret_properties;


/// Struct to store MetaValues and genesis hash for network
pub struct MetaShortCut {
    pub meta_values: MetaValues,
    pub genesis_hash: [u8; 32],
}

/// Function to process address as &str, fetch metadata and genesis hash for it,
/// and output MetaShortCut value in case of success
pub fn meta_shortcut (address: &str) -> Result<MetaShortCut, ErrorActive> {
    let new_info = match fetch_info(address) {
        Ok(a) => a,
        Err(e) => return Err(ErrorActive::Fetch(Fetch::Failed{url: address.to_string(), error: e.to_string()})),
    };
    let genesis_hash = get_genesis_hash(address, &new_info.genesis_hash)?;
    let meta_values = MetaValues::from_str_metadata(&new_info.meta, IncomingMetadataSourceActiveStr::Fetch{url: address.to_string()})?;
    Ok(MetaShortCut{
        meta_values,
        genesis_hash,
    })
}

/// Struct to store MetaValues, genesis hash, and NetworkSpecsToSend for network
pub struct MetaSpecsShortCut {
    pub meta_values: MetaValues,
    pub specs: NetworkSpecsToSend,
    pub update: bool, // flag to indicate that the database has no exact entry created
}

/// Function to process address as &str, fetch metadata, genesis hash, and network specs
/// for it, and output MetaSpecsShortCut value in case of success
pub fn meta_specs_shortcut (address: &str, encryption: Encryption) -> Result<MetaSpecsShortCut, ErrorActive> {

    let entries = filter_address_book_by_url(address)?;
    let new_info = match fetch_info_with_network_specs(address) {
        Ok(a) => a,
        Err(e) => return Err(ErrorActive::Fetch(Fetch::Failed{url: address.to_string(), error: e.to_string()})),
    };
    let genesis_hash = get_genesis_hash(address, &new_info.genesis_hash)?;
    let meta_values = MetaValues::from_str_metadata(&new_info.meta, IncomingMetadataSourceActiveStr::Fetch{url: address.to_string()})?;
    let new_properties = match interpret_properties(&new_info.properties) {
        Ok(a) => a,
        Err(error) => return Err(ErrorActive::Fetch(Fetch::FaultySpecs{url: address.to_string(), error})),
    };
    if entries.len() == 0 {
        if genesis_hash_in_hot_db (genesis_hash)? {return Err(ErrorActive::Database(DatabaseActive::NewAddressKnownGenesisHash{url: address.to_string(), genesis_hash}))}
        let specs = NetworkSpecsToSend {
            base58prefix: new_properties.base58prefix,
            color: COLOR.to_string(),
            decimals: new_properties.decimals,
            encryption: encryption.clone(),
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
        let (specs, update) = process_indices(&entries, encryption)?;
        let url = address.to_string();
        if specs.base58prefix != new_properties.base58prefix {return Err(ErrorActive::Fetch(Fetch::ValuesChanged{url, what: Changed::Base58Prefix{old: specs.base58prefix, new: new_properties.base58prefix}}))}
        if specs.decimals != new_properties.decimals {return Err(ErrorActive::Fetch(Fetch::ValuesChanged{url, what: Changed::Decimals{old: specs.decimals, new: new_properties.decimals}}))}
        if specs.unit != new_properties.unit {return Err(ErrorActive::Fetch(Fetch::ValuesChanged{url, what: Changed::Unit{old: specs.unit.to_string(), new: new_properties.unit.to_string()}}))}
        if specs.name != meta_values.name {return Err(ErrorActive::Fetch(Fetch::ValuesChanged{url, what: Changed::Name{old: specs.name.to_string(), new: meta_values.name.to_string()}}))}
        // NetworkSpecsToSend are good, can use them
        Ok(MetaSpecsShortCut{
            meta_values,
            specs,
            update,
        })
    }
}

/// Helper function to interpret freshly fetched genesis hash
fn get_genesis_hash (address: &str, fetched_genesis_hash: &str) -> Result<[u8; 32], ErrorActive> {
    let genesis_hash_vec = unhex::<Active>(fetched_genesis_hash, NotHexActive::FetchedGenesisHash{url: address.to_string()})?;
    let out: [u8; 32] = match genesis_hash_vec.try_into() {
        Ok(a) => a,
        Err(_) => return Err(ErrorActive::Fetch(Fetch::UnexpectedFetchedGenesisHashFormat{value: fetched_genesis_hash.to_string()})),
    };
    Ok(out)
}
