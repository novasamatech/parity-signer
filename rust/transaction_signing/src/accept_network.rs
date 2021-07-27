use anyhow;
use definitions::{network_specs::{ChainSpecs, generate_network_key}, constants::{ADDNETWORK, ADDRTREE, GENERALVERIFIER, METATREE, SETTREE, SPECSTREE, TRANSACTION}, transactions::AddNetworkDb};
use parity_scale_codec::{Decode, Encode};
use db_handling::helpers::{open_db, open_tree, flush_db, insert_into_tree, decode_address_details};

use crate::error::{Error, ActionFailure};
use crate::helpers::verify_checksum;

/// function to add approved network to the database;
/// flag upd_general indicates if general verifier should be updated as well

pub fn add_network (database_name: &str, checksum: u32, upd_general: bool) -> anyhow::Result<String> {
    
    let database = open_db(database_name)?;
    verify_checksum(&database, checksum)?;
    let transaction = open_tree(&database, TRANSACTION)?;
    
    let action = match transaction.remove(ADDNETWORK) {
        Ok(Some(encoded_action)) => match <AddNetworkDb>::decode(&mut &encoded_action[..]) {
            Ok(x) => x,
            Err(_) => return Err(Error::BadActionDecode(ActionFailure::AddNetwork).show()),
        },
        Ok(None) => return Err(Error::NoAction(ActionFailure::AddNetwork).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    flush_db(&database)?;
    
    let metadata = open_tree(&database, METATREE)?;
    insert_into_tree(action.versioned_name, action.meta, &metadata)?;
    flush_db(&database)?;
    
// updating general verifier if requested
    if upd_general {
        let settings = open_tree(&database, SETTREE)?;
        insert_into_tree(GENERALVERIFIER.to_vec(), action.verifier.encode(), &settings)?;
        flush_db(&database)?;
    }

// creating chainspecs tree entry
    let chainspecs = open_tree(&database, SPECSTREE)?;
    
    let order = chainspecs.len() as u8;
    let network_key = generate_network_key(&action.chainspecs.genesis_hash.to_vec());
    let new_chainspecs = ChainSpecs {
        base58prefix: action.chainspecs.base58prefix,
        color: action.chainspecs.color,
        decimals: action.chainspecs.decimals,
        genesis_hash: action.chainspecs.genesis_hash,
        logo: action.chainspecs.logo,
        name: action.chainspecs.name,
        order,
        path_id: action.chainspecs.path_id,
        secondary_color: action.chainspecs.secondary_color,
        title: action.chainspecs.title,
        unit: action.chainspecs.unit,
        verifier: action.verifier,
    };
    insert_into_tree(network_key.to_vec(), new_chainspecs.encode(), &chainspecs)?;
    flush_db(&database)?;

// adding network in network_id vector of all existing identities records
// with default path "", and no password (has_pwd = false)
    let identities = open_tree(&database, ADDRTREE)?;
    
    for x in identities.iter() {
        if let Ok((key, value)) = x {
            let mut address_details = decode_address_details(value)?;
            if (address_details.path.as_str() == "") && !address_details.has_pwd {
                address_details.network_id.push(network_key.to_vec());
                insert_into_tree(key.to_vec(), address_details.encode(), &identities)?;
            }
        }
    }
    flush_db(&database)?;
    
    if upd_general {Ok(String::from("Network successfully added. General verifier successfully updated."))}
    else {Ok(String::from("Network successfully added."))}
}

