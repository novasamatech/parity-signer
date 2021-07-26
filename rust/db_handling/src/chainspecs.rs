use sled::{Db, Tree, open};
use parity_scale_codec::{Encode, Decode};
use definitions::{constants::{SPECSTREE, SPECSTREEPREP}, defaults::{get_default_chainspecs, get_default_chainspecs_to_send}, network_specs::{ChainSpecs, generate_network_key}};
use anyhow;

use super::error::{Error, NotHex, NotDecodeable, NotFound};

/// Fetch ChainSpecs for 1 network from database by genesis hash
pub fn get_network (database_name: &str, genesis_hash: &str) -> anyhow::Result<ChainSpecs> {
    let database: Db = match open(database_name) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    let chainspecs: Tree = match database.open_tree(SPECSTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let unhex_genesis_hash = match hex::decode(genesis_hash) {
        Ok(x) => x,
        Err(_) => return Err(Error::NotHex(NotHex::GenesisHash).show()),
    };
    
    let network_key = generate_network_key(&unhex_genesis_hash);

    match chainspecs.get(&network_key) {
        Ok(a) => match a {
            Some(chain_specs_encoded) => match <ChainSpecs>::decode(&mut &chain_specs_encoded[..]) {
                Ok(b) => {
                    if generate_network_key(&b.genesis_hash.to_vec()) != network_key {return Err(Error::GenesisHashMismatch.show())}
                    Ok(b)
                },
                Err(_) => return Err(Error::NotDecodeable(NotDecodeable::ChainSpecs).show()),
            },
            None => return Err(Error::NotFound(NotFound::NetworkKey).show()),
        },
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}

/// Fetch ChainSpecs for all saved networks
pub fn get_all_networks (database_name: &str) -> anyhow::Result<Vec<ChainSpecs>> {
    
    let database: Db = match open(database_name) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    let chainspecs: Tree = match database.open_tree(SPECSTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let mut out: Vec<ChainSpecs> = Vec::new();

    for x in chainspecs.iter() {
        if let Ok((network_key, network_specs_encoded)) = x {
            match <ChainSpecs>::decode(&mut &network_specs_encoded[..]) {
                Ok(network_specs) => {
                    if generate_network_key(&network_specs.genesis_hash.to_vec()) != network_key.to_vec() {return Err(Error::GenesisHashMismatch.show())}
                    out.push(network_specs)
                },
                Err(_) => return Err(Error::NotDecodeable(NotDecodeable::ChainSpecs).show()),
            }
        }
    }
    Ok(out)
}


/// Function to populate cold database with default network specs ChainSpecs
pub fn load_chainspecs (database_name: &str) -> anyhow::Result<()> {
    
    let database: Db = match open(database_name) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    let chainspecs: Tree = match database.open_tree(SPECSTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    match chainspecs.clear() {
        Ok(()) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let specs_vec = get_default_chainspecs();
    
    for x in specs_vec.iter() {
        let network_key = generate_network_key(&x.genesis_hash.to_vec());
        match chainspecs.insert(network_key, x.encode()) {
            Ok(_) => (),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
    }
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    Ok(())
    
}


/// Function to populate hot database with default network specs ChainSpecsToSend
pub fn load_chainspecs_to_send (database_name: &str) -> anyhow::Result<()> {
    
    let database: Db = match open(database_name) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    let chainspecs: Tree = match database.open_tree(SPECSTREEPREP) {
        Ok(x) => x,
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    match chainspecs.clear() {
        Ok(()) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    let specs_vec = get_default_chainspecs_to_send();
    
    for x in specs_vec.iter() {
        let network_key = generate_network_key(&x.genesis_hash.to_vec());
        match chainspecs.insert(network_key, x.encode()) {
            Ok(_) => (),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        };
    }
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    };
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_constants() {
        let _test = get_default_chainspecs();
    }
}
