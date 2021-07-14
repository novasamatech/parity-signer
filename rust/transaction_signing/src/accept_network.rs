use sled::{Db, open, Tree};
use definitions::{network_specs::{ChainSpecs, generate_network_key}, constants::{ADDNETWORK, GENERALVERIFIER, METATREE, SETTREE, SPECSTREE, TRANSACTION}, transactions::AddNetworkDb};
use parity_scale_codec::{Decode, Encode};

/// function to add approved network to the database;
/// flag upd_general indicates if general verifier should be updated as well

pub fn add_network (dbname: &str, checksum: u32, upd_general: bool) -> Result<String, Box<dyn std::error::Error>> {
    
    let database: Db = open(dbname)?;
    let real_checksum = database.checksum()?;
    
    if checksum != real_checksum {return Err(Box::from("Database checksum mismatch."))}
    
    let transaction: Tree = database.open_tree(TRANSACTION)?;
    let action = match transaction.remove(ADDNETWORK)? {
        Some(x) => {<AddNetworkDb>::decode(&mut &x[..])?},
        None => {return Err(Box::from("No approved network information found."))}
    };
    database.flush()?;
    
    let metadata: Tree = database.open_tree(METATREE)?;
    metadata.insert(action.versioned_name, action.meta)?;
    database.flush()?;
    
    if upd_general {
        let settings: Tree = database.open_tree(SETTREE)?;
        settings.insert(GENERALVERIFIER, action.verifier.encode())?;
        database.flush()?;
    }
    
    let chainspecs: Tree = database.open_tree(SPECSTREE)?;
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
    chainspecs.insert(network_key, new_chainspecs.encode())?;
    database.flush()?;
    
    if upd_general {Ok(String::from("Network successfully added. General verifier successfully updated."))}
    else {Ok(String::from("Network successfully added."))}
}

