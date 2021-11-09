use constants::SPECSTREE;
use definitions::{keyring::{NetworkSpecsKey}, network_specs::{ChainSpecs}};
use anyhow;
use hex;

use crate::error::NotHex;
use crate::helpers::{open_db, open_tree, unhex, get_and_decode_chain_specs, decode_chain_specs};

/// Fetch ChainSpecs for 1 network from cold database by network key (genesis hash and encryption)
pub (crate) fn get_network (database_name: &str, network_specs_key_string: &str) -> anyhow::Result<ChainSpecs> {
    let database = open_db(database_name)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    let network_specs_key = NetworkSpecsKey::from_vec(&unhex(network_specs_key_string, NotHex::NetworkSpecsKey)?);
    get_and_decode_chain_specs(&chainspecs, &network_specs_key)
}

/// Print network details for 1 network from cold database by genesis hash
pub fn print_network (database_name: &str, network_specs_key_string: &str) -> anyhow::Result<String> {
    let network_specs = get_network (database_name, network_specs_key_string)?;
    Ok(format!("{{\"color\":\"{}\",\"logo\":\"{}\",\"secondaryColor\":\"{}\",\"title\":\"{}\"}}", network_specs.color, network_specs.logo, network_specs.secondary_color, network_specs.title))
}

/// Fetch ChainSpecs for all saved networks
pub (crate) fn get_all_networks (database_name: &str) -> anyhow::Result<Vec<ChainSpecs>> {
    let database = open_db(database_name)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    let mut out: Vec<ChainSpecs> = Vec::new();
    for x in chainspecs.iter() {
        if let Ok((network_specs_key_vec, network_specs_encoded)) = x {
            let new = decode_chain_specs(network_specs_encoded, &NetworkSpecsKey::from_vec(&network_specs_key_vec.to_vec()))?;
            out.push(new);
        }
    }
    Ok(out)
}

/// Print details for all saved networks
pub fn print_all_networks (database_name: &str) -> anyhow::Result<String> {
    let network_specs_vec = get_all_networks (database_name)?;
    let mut out = String::from("[");
    for (i, x) in network_specs_vec.iter().enumerate() {
        if i>0 {out.push_str(",");}
        out.push_str(&format!("{{\"key\":\"{}\",\"color\":\"{}\",\"logo\":\"{}\",\"order\":\"{}\",\"secondaryColor\":\"{}\",\"title\":\"{}\"}}", hex::encode(NetworkSpecsKey::from_parts(&x.genesis_hash.to_vec(), &x.encryption).key()), x.color, x.logo, x.order, x.secondary_color, x.title));
    }
    out.push_str("]");
    Ok(out)
}
