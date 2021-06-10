use sled::{Db, Tree, open};
use parity_scale_codec::{Encode, Decode};
use parity_scale_codec_derive;
use regex::Regex;
use lazy_static::lazy_static;

//TODO: rename fields to make them more clear
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub struct ChainSpecs {
    pub name: String,
    pub base58prefix: u8,
    pub decimals: u8,
    pub unit: String,
    //TODO: deleted should be removed
    pub deleted: bool,
    pub protocol: String,
    pub secondary_color: String,
    pub color: String,
    pub logo: u8,
    pub order: u8,
    pub path_id: String,
    pub title: String,
    //TODO: add metadata signature parameters
}

lazy_static! {
    static ref REG_CHAINSPECS: Regex = Regex::new(r#"(?i)\["(0x)?(?P<gen_hash>[0-9a-f]{64})",\{"deleted":(?P<deleted>false|true),"protocol":"(?P<protocol>.*?)","secondaryColor":"(?P<secondary_color>#[0-9a-z]+)","color":"(?P<color>#[0-9a-z]+)","decimals":(?P<decimals>[0-9]+),"genesisHash":"(0x)?(?P<gen_hash_again>[0-9a-f]{64})","logo":(?P<logo>[0-9]+),[^]]*"specName":"(?P<name>[^"]+)"[^]]*"order":(?P<order>[0-9]+),"pathId":"(?P<path_id>.*?)","prefix":(?P<prefix>[0-9]+),"title":"(?P<title>.*?)","unit":"(?P<unit>[a-z]+)""#).unwrap();
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
pub fn add_network (database_name: &str, genesis_hash: &str, specs: ChainSpecs) -> Result<(), Box<dyn std::error::Error>> {
    let database: Db = open(database_name)?;
    let chainspecs: Tree = database.open_tree(b"chainspecs")?;
    let chain_id = hex::decode(genesis_hash)?;

    chainspecs.insert(chain_id, specs.encode())?;

    database.flush();
    Ok(())
}

/// Remove network
pub fn remove_network (database_name: &str, genesis_hash: &str) -> Result<(), Box<dyn std::error::Error>> {
    //TODO
    return Ok(());
}

///Function to initially populate network specs database
pub fn load_chainspecs (database_name: &str, chain_specs: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(database_name)?;
    let chainspecs: Tree = database.open_tree(b"chainspecs")?;
    
    for caps in REG_CHAINSPECS.captures_iter(&chain_specs) {
        if caps["gen_hash"] == caps["gen_hash_again"] {
            let genesis_hash = hex::decode(&(caps["gen_hash"].to_string()))?;
            let specs = ChainSpecs {
                name: caps["name"].to_string(),
                base58prefix: caps["prefix"].parse()?,
                decimals: caps["decimals"].parse()?,
                unit: caps["unit"].to_string(),
                deleted: caps["deleted"].parse()?,
                protocol: caps["protocol"].to_string(),
                secondary_color: caps["secondary_color"].to_string(),
                color: caps["color"].to_string(),
                logo: caps["logo"].parse()?,
                order: caps["order"].parse()?,
                path_id: caps["path_id"].to_string(),
                title: caps["title"].to_string(),
            };
            chainspecs.insert(genesis_hash, specs.encode())?;
        }
    }
    
    database.flush()?;
    Ok(())
    
}

#[cfg(tests)]
mod tests {
    use super::*;
    static TESTDB: &str = "tests/testdb";
    static GENHASH1: &str = "1111111111111111111111111111111111111111111111111111111111111111";
    static GENHASH2: &str = "2222222222222222222222222222222222222222222222222222222222222222";
    static CHAIN1: ChainSpecs = ChainSpecs {
        name: "chain1".to_string(),
        base58prefix: 1,
        decimals: 1,
        unit: "UNIT1".to_string(),
        deleted: false,
        protocol: "Protocol1".to_string(),
        secondary_color: "purple".to_string(),
        color: "deep".to_string(),
        logo: 1,
        order: 1,
        path_id: "path1".to_string(),
        title: "Network1".to_string(),    
    };


    #[test]
    fn test_add_fetch_remove_network() {
        add_network(TESTDB, GENHASH1, CHAIN1)?;
        assert_eq!(get_network(TESTDB, GENHASH1), CHAIN1);
        remove_network(TESTDB, GENHASH1)?;
        //mustfail fetch
    }
}
