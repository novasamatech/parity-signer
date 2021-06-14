use sled::{Db, Tree, open};
use parity_scale_codec::{Encode, Decode};
use parity_scale_codec_derive;

use super::db_utils;
use super::constants::get_default_chainspecs;

//TODO: rename fields to make them more clear
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode, PartialEq, Debug)]
pub struct ChainSpecs {
    pub base58prefix: u8,
    pub color: String,
    pub decimals: u8,
    pub genesis_hash: [u8; 32],
    pub logo: String,
    pub name: String,
    pub order: u8,
    pub path_id: String,
    pub secondary_color: String,
    pub title: String,
    pub unit: String,
    //TODO: add metadata signature parameters
}

/// Fetch 1 network from database by genesis hash
pub fn get_network (database_name: &str, genesis_hash: &str) -> Result<ChainSpecs, Box<dyn std::error::Error>> {
    let database: Db = open(database_name)?;
    let chainspecs: Tree = database.open_tree(b"chainspecs")?;
    let chain_id = hex::decode(genesis_hash)?;

    match chainspecs.get(chain_id) {
        Ok(Some(a)) => Ok(<ChainSpecs>::decode(&mut &a[..])?),
        Ok(None) => return Err(Box::from("Network not found")),
        Err(e) => return Err(Box::from(e)),
    }
}

/// Fetch all saved networks
pub fn get_all_networks (database_name: &str) -> Result<Vec<ChainSpecs>, Box<dyn std::error::Error>> {
    let database: Db = open(database_name)?;
    let chainspecs: Tree = database.open_tree(b"chainspecs")?;

    /*
    for Ok((_Id, record)) in chainspecs.iter() {
        match <ChainSpecs>::decode(&mut &record[..]) {
            Ok(a) => networks.push(a),
            Err(e) => return Err(Box::from(e)),
        }
    }
    return Ok(networks);
    */

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

/// Add network
pub fn add_network (database_name: &str, genesis_hash: &str, specs: &ChainSpecs) -> Result<(), Box<dyn std::error::Error>> {
    let database: Db = open(database_name)?;
    let chainspecs: Tree = database.open_tree(b"chainspecs")?;
    let chain_id = hex::decode(genesis_hash)?;

    chainspecs.insert(chain_id, specs.encode())?;

    db_utils::db_flush_check(&database)
}

/// Remove network
pub fn remove_network (_database_name: &str, _genesis_hash: &str) -> Result<(), Box<dyn std::error::Error>> {
    //TODO
    return Ok(());
}

///Function to initially populate network specs database
pub fn load_chainspecs (database_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(database_name)?;
    let chainspecs: Tree = database.open_tree(b"chainspecs")?;
    chainspecs.clear()?;
    
    for record in get_default_chainspecs() {
        let genesis_hash = record.genesis_hash;
        chainspecs.insert(genesis_hash, record.encode())?;
    }
    
    database.flush()?;
    Ok(())
    
}

#[cfg(test)]
mod tests {
    use super::*;
    static TESTDB: &str = "tests/testdb";

    #[test]
    fn test_add_fetch_remove_network(){
        let testspecs = get_default_chainspecs();
        let genhash0 = hex::encode(testspecs[0].genesis_hash);
        add_network(TESTDB, &genhash0, &testspecs[0]);
        let fetched_network = get_network(TESTDB, &genhash0).unwrap();
        assert_eq!(fetched_network, testspecs[0]);
        remove_network(TESTDB, &genhash0);
        //mustfail fetch
        let database: Db = open(TESTDB).unwrap();
        db_utils::db_flush_check(&database);
    }

    #[test]
    fn get_constants() {
        let _test = get_default_chainspecs();
    }
}
