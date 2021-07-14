use sled::{Db, Tree, open};
use parity_scale_codec::{Encode, Decode};
use definitions::{constants::{SPECSTREE, SPECSTREEPREP}, defaults::{get_default_chainspecs, get_default_chainspecs_to_send}, network_specs::{ChainSpecs, generate_network_key}};

/// Fetch 1 network from database by genesis hash
pub fn get_network (database_name: &str, genesis_hash: &str) -> Result<ChainSpecs, Box<dyn std::error::Error>> {
    let database: Db = open(database_name)?;
    let chainspecs: Tree = database.open_tree(SPECSTREE)?;
    let network_key = generate_network_key(&hex::decode(genesis_hash)?);

    match chainspecs.get(network_key) {
        Ok(Some(a)) => Ok(<ChainSpecs>::decode(&mut &a[..])?),
        Ok(None) => return Err(Box::from("Network not found")),
        Err(e) => return Err(Box::from(e)),
    }
}

/// Fetch all saved networks
pub fn get_all_networks (database_name: &str) -> Result<Vec<ChainSpecs>, Box<dyn std::error::Error>> {
    let database: Db = open(database_name)?;
    let chainspecs: Tree = database.open_tree(SPECSTREE)?;

    match chainspecs
        .iter()
        .collect::<Result<Vec<_>,_>>()?
        .into_iter()
        .map(|(_id, record)| <ChainSpecs>::decode(&mut &record[..]))
        .collect::<Result<Vec<_>,_>>()
        {
            Ok(a) => Ok(a),
            Err(e) => return Err(Box::from(e)),
        }
}


/// Function to populate cold database with default network specs ChainSpecs
pub fn load_chainspecs (database_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(database_name)?;
    let chainspecs: Tree = database.open_tree(SPECSTREE)?;
    chainspecs.clear()?;
    
    let specs_vec = get_default_chainspecs();
    
    for x in specs_vec.iter() {
        let network_key = generate_network_key(&x.genesis_hash.to_vec());
        chainspecs.insert(network_key, x.encode())?;
    }
    
    database.flush()?;
    Ok(())
    
}


/// Function to populate hot database with default network specs ChainSpecsToSend
pub fn load_chainspecs_to_send (database_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(database_name)?;
    let chainspecs: Tree = database.open_tree(SPECSTREEPREP)?;
    chainspecs.clear()?;
    
    let specs_vec = get_default_chainspecs_to_send();
    
    for x in specs_vec.iter() {
        let network_key = generate_network_key(&x.genesis_hash.to_vec());
        chainspecs.insert(network_key, x.encode())?;
    }
    
    database.flush()?;
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
