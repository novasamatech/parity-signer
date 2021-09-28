use parity_scale_codec::Encode;
use constants::{SPECSTREE, SPECSTREEPREP};
use definitions::{defaults::{get_default_chainspecs, get_default_chainspecs_to_send}, network_specs::{ChainSpecs, generate_network_key}};
use anyhow;
use hex;

use crate::error::NotHex;
use crate::helpers::{open_db, open_tree, flush_db, clear_tree, insert_into_tree, unhex, get_and_decode_chain_specs, decode_chain_specs};

/// Fetch ChainSpecs for 1 network from cold database by genesis hash
pub fn get_network (database_name: &str, genesis_hash: &str) -> anyhow::Result<ChainSpecs> {
    let database = open_db(database_name)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    let network_key = generate_network_key(&unhex(genesis_hash, NotHex::GenesisHash)?);
    get_and_decode_chain_specs(&chainspecs, &network_key)
}

/// Print network details for 1 network from cold database by genesis hash
pub fn print_network (database_name: &str, genesis_hash: &str) -> anyhow::Result<String> {
    let network_specs = get_network (database_name, genesis_hash)?;
    Ok(format!("{{\"color\":\"{}\",\"logo\":\"{}\",\"secondaryColor\":\"{}\",\"title\":\"{}\"}}", network_specs.color, network_specs.logo, network_specs.secondary_color, network_specs.title))
}

/// Fetch ChainSpecs for all saved networks
pub fn get_all_networks (database_name: &str) -> anyhow::Result<Vec<ChainSpecs>> {
    let database = open_db(database_name)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    let mut out: Vec<ChainSpecs> = Vec::new();
    for x in chainspecs.iter() {
        if let Ok((network_key, network_specs_encoded)) = x {
            let new = decode_chain_specs(network_specs_encoded, &network_key.to_vec())?;
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
        out.push_str(&format!("{{\"key\":\"{}\",\"color\":\"{}\",\"logo\":\"{}\",\"order\":\"{}\",\"secondaryColor\":\"{}\",\"title\":\"{}\"}}", hex::encode(x.genesis_hash), x.color, x.logo, x.order, x.secondary_color, x.title));
    }
    out.push_str("]");
    Ok(out)
}

/// Function to populate cold database with default network specs ChainSpecs
pub fn load_chainspecs (database_name: &str) -> anyhow::Result<()> {
    
    let database = open_db(database_name)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    clear_tree(&chainspecs)?;
    
    let specs_vec = get_default_chainspecs();
    
    for x in specs_vec.iter() {
        let network_key = generate_network_key(&x.genesis_hash.to_vec());
        insert_into_tree(network_key, x.encode(), &chainspecs)?;
    }
    
    flush_db(&database)?;
    Ok(())
}


/// Function to populate hot database with default network specs ChainSpecsToSend
pub fn load_chainspecs_to_send (database_name: &str) -> anyhow::Result<()> {
    
    let database = open_db(database_name)?;
    let chainspecs = open_tree(&database, SPECSTREEPREP)?;
    clear_tree(&chainspecs)?;
    
    let specs_vec = get_default_chainspecs_to_send();
    
    for x in specs_vec.iter() {
        let network_key = generate_network_key(&x.genesis_hash.to_vec());
        insert_into_tree(network_key, x.encode(), &chainspecs)?;
    }
    
    flush_db(&database)?;
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
