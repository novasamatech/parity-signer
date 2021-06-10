use sled::{Db, Tree, open};
use parity_scale_codec::Encode;
use parity_scale_codec_derive;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub struct AddressDetails {
    pub name_for_seed: String,
    pub path: String,
    pub has_pwd: bool,
    pub name: String,
    pub network_path_id: String,
}

lazy_static! {
    static ref REG_SEED: Regex = Regex::new(r#""encryptedSeed":(?P<seed>.*?),"addresses":\[(?P<addresses>(\[[^]]+\](,)?)+)\],"meta":\[(?P<meta>(\[[^]]+\](,)?)+)\],"name":"(?P<name>.*?)""#).unwrap();
    static ref REG_ADDRESSES: Regex = Regex::new(r#"(?i)\["(?P<path>\S*?)",\{"address":"(?P<base58>[0-9a-z]+)",.*?"hasPassword":(?P<has_pwd>(true|false)),"name":"(?P<name>.*?)","networkPathId":"(?P<network_path_id>.*?)".*?\}\]"#).unwrap();
}

pub fn load_users (database_name: &str, identities: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(database_name)?;
    let seeds: Tree = database.open_tree(b"seeds")?;
    let addresses: Tree = database.open_tree(b"addresses")?;
    
    for caps1 in REG_SEED.captures_iter(identities) {
        let name = caps1["name"].to_string();
        let encrypted_seed = caps1["seed"].to_string();
        
        seeds.insert(name.encode(), encrypted_seed.encode())?;
        
        let meta = caps1["meta"].to_string();
                
        for caps2 in REG_ADDRESSES.captures_iter(&meta) {
            let base58address = caps2["base58"].to_string();
            let address_details = AddressDetails {
                name_for_seed: (&name).to_string(),
                path: caps2["path"].to_string(),
                has_pwd: caps2["has_pwd"].parse()?,
                name: caps2["name"].to_string(),
                network_path_id: caps2["network_path_id"].to_string(),
            };
            addresses.insert(base58address.encode(), address_details.encode())?;
        }
        database.flush()?;
    }
    database.flush()?;
    Ok(())
}
