use sled::Tree;
use meta_reading::{decode_metadata::decode_version, fetch_metadata::{fetch_info, fetch_info_with_chainspecs}, interpret_chainspecs::interpret_properties};
use hex;
use parity_scale_codec::{Decode};
use definitions::{constants::{COLOR, SECONDARY_COLOR}, metadata::{MetaValues}, network_specs::ChainSpecsToSend};
use std::convert::TryInto;


/// Struct to store MetaValues and genesis hash for network

pub struct MetaShortCut {
    pub meta_values: MetaValues,
    pub genesis_hash: [u8; 32],
}

/// Function to process address as &str, fetch metadata and genesis hash for it,
/// and output MetaShortCut value in case of success

pub fn meta_shortcut (address: &str) -> Result <MetaShortCut, Box<dyn std::error::Error>> {

    let new_info = fetch_info(address)?;
    let genesis_hash: [u8; 32] = match &new_info.genesis_hash[..2] {
        "0x" => {
            let a = hex::decode(&new_info.genesis_hash[2..])?;
            match a.try_into() {
                Ok(b) => b,
                Err(_) => return Err(Box::from("Unexpected genesis hash format")),
            }
        },
        _ => {
            let a = hex::decode(&new_info.genesis_hash)?;
            match a.try_into() {
                Ok(b) => b,
                Err(_) => return Err(Box::from("Unexpected genesis hash format")),
            }
        },
    };
    let meta_values = decode_version(&new_info.meta)?;
    Ok(MetaShortCut{
        meta_values,
        genesis_hash,
    })
}


/// Struct to store MetaValues, genesis hash, and ChainSpecsToSend for network

pub struct MetaSpecsShortCut {
    pub meta_values: MetaValues,
    pub specs: ChainSpecsToSend,
    pub def: bool, // flag to indicate use of defaults
}


/// Function to process address as &str, fetch metadata, genesis hash, and chainspecs
/// for it, and output MetaSpecsShortCut value in case of success

pub fn meta_specs_shortcut (address: &str, chainspecs: &Tree) -> Result <MetaSpecsShortCut, Box<dyn std::error::Error>> {

    let new_info = fetch_info_with_chainspecs(address)?;
    let genesis_hash: [u8; 32] = match &new_info.genesis_hash[..2] {
        "0x" => {
            let a = hex::decode(&new_info.genesis_hash[2..])?;
            match a.try_into() {
                Ok(b) => b,
                Err(_) => return Err(Box::from("Unexpected genesis hash format")),
            }
        },
        _ => {
            let a = hex::decode(&new_info.genesis_hash)?;
            match a.try_into() {
                Ok(b) => b,
                Err(_) => return Err(Box::from("Unexpected genesis hash format")),
            }
        },
    };
    let meta_values = decode_version(&new_info.meta)?;
    
    let new_properties = interpret_properties(&new_info.properties)?;
    
    match chainspecs.get(genesis_hash.to_vec()) {
        Ok(z) => {
            match z {
                Some(a) => {
                // found corresponding ChainSpecsToSend in the database
                    let specs = <ChainSpecsToSend>::decode(&mut &a[..])?;
                    let def = false;
                    if (specs.base58prefix == new_properties.base58prefix)&&(specs.decimals == new_properties.decimals)&&(specs.unit == new_properties.unit)&&(specs.name == meta_values.name)&&(specs.genesis_hash == genesis_hash) {
                    // ChainSpecsToSend are good, can use them
                        Ok(MetaSpecsShortCut{
                            meta_values,
                            specs,
                            def,
                        })
                    }
                    else {
                        if specs.genesis_hash != genesis_hash {return Err(Box::from("Error is fetched genesis hash."))}
                        else {
                            if specs.name == meta_values.name {return Err(Box::from("Error is fetched network name."))}
                            else {return Err(Box::from("Error is fetched network specs."))}
                        }
                    }
                },
                None => {
                // not cound corresponding ChainSpecsToSend in the database, use default values to fill in the missing parts
                    let specs = ChainSpecsToSend {
                        base58prefix: new_properties.base58prefix,
                        color: COLOR.to_string(),
                        decimals: new_properties.decimals,
                        genesis_hash,
                        logo: meta_values.name.to_string(),
                        name: meta_values.name.to_string(),
                        path_id: format!("//{}", meta_values.name),
                        secondary_color: SECONDARY_COLOR.to_string(),
                        title: meta_values.name.to_string(),
                        unit: new_properties.unit.to_string(),
                    };
                    let def = true;
                    Ok(MetaSpecsShortCut{
                        meta_values,
                        specs,
                        def
                    })
                },
            }
        },
        Err(e) => {
            let err_text = format!("Database error. {}", e);
            let err: Box<dyn std::error::Error> = From::from(err_text);
            return Err(err)
        },
    }
}


