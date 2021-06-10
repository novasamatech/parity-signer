use sled::{Db, Tree, open};
use parity_scale_codec::Encode;
use parity_scale_codec_derive;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub struct ChainSpecs {
    pub name: String,
    pub base58prefix: u8,
    pub decimals: u8,
    pub unit: String,
    pub deleted: bool,
    pub protocol: String,
    pub secondary_color: String,
    pub color: String,
    pub logo: u8,
    pub order: u8,
    pub path_id: String,
    pub title: String,    
}

lazy_static! {
    static ref REG_CHAINSPECS: Regex = Regex::new(r#"(?i)\["(0x)?(?P<gen_hash>[0-9a-f]{64})",\{"deleted":(?P<deleted>false|true),"protocol":"(?P<protocol>.*?)","secondaryColor":"(?P<secondary_color>#[0-9a-z]+)","color":"(?P<color>#[0-9a-z]+)","decimals":(?P<decimals>[0-9]+),"genesisHash":"(0x)?(?P<gen_hash_again>[0-9a-f]{64})","logo":(?P<logo>[0-9]+),[^]]*"specName":"(?P<name>[^"]+)"[^]]*"order":(?P<order>[0-9]+),"pathId":"(?P<path_id>.*?)","prefix":(?P<prefix>[0-9]+),"title":"(?P<title>.*?)","unit":"(?P<unit>[a-z]+)""#).unwrap();
}

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
