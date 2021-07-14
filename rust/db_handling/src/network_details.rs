use definitions::{constants::{METATREE, SPECSTREE}, metadata::{NameVersioned, VersionDecoded}, network_specs::{ChainSpecs, NetworkKey, generate_network_key}};
use meta_reading::decode_metadata::get_meta_const;
use sled::{Db, open, Tree};
use parity_scale_codec::{Decode, Encode};
use blake2_rfc::blake2b::blake2b;
use hex;

struct MetaPrint {
    spec_version: u32,
    metadata_hash: String,
}

pub fn get_network_details_by_key (network_key: NetworkKey, database_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    
    let database: Db = open(database_name)?;
    let metadata: Tree = database.open_tree(METATREE)?;
    let chainspecs: Tree = database.open_tree(SPECSTREE)?;
    
    match chainspecs.get(&network_key)? {
        Some(network_specs_encoded) => {
            let network_specs = match <ChainSpecs>::decode(&mut &network_specs_encoded[..]) {
                Ok(a) => a,
                Err(_) => return Err(Box::from("Network specs in the database are damaged, and could not be decoded.")),
            };
            if generate_network_key(&network_specs.genesis_hash.to_vec()) != network_key {
                return Err(Box::from("Network specs in the database are corrupted, genesis hash mismatch."))
            }
            let mut relevant_metadata: Vec<MetaPrint> = Vec::new();
            for x in metadata.scan_prefix(network_specs.name.encode()) {
                if let Ok((versioned_name_encoded, meta)) = x {
                    let versioned_name = match <NameVersioned>::decode(&mut &versioned_name_encoded[..]) {
                        Ok(a) => a,
                        Err(_) => return Err(Box::from("Network metadata record is damaged, and versioned name could not be decoded.")),
                    };
                    let version_vector = get_meta_const(&meta.to_vec())?;
                    let version = match VersionDecoded::decode(&mut &version_vector[..]) {
                        Ok(a) => a,
                        Err(_) => return Err(Box::from("Database records damaged. Network metadata version constant could not be decoded.")),
                    };
                    if version.specname != versioned_name.name {return Err(Box::from("Database records damaged. Name decoded from version constant does not match the name from database key."))}
                    if version.spec_version != versioned_name.version {return Err(Box::from("Database records damaged. Metadata version decoded from version constant does not match the version from database key."))}
                    let new = MetaPrint {
                        spec_version: versioned_name.version,
                        metadata_hash: hex::encode(blake2b(32, &[], &meta).as_bytes()),
                    };
                    relevant_metadata.push(new);
                }
            }
            if relevant_metadata.len() == 0 {return Err(Box::from(format!("No entries for network {} found in the metadata database.", network_specs.name)))}
            let chainspecs_print = format!("\"base58prefix\":\"{}\",\"color\":\"{}\",\"decimals\":\"{}\",\"genesis_hash\":\"{}\",\"logo\":\"{}\",\"name\":\"{}\",\"order\":\"{}\",\"path_id\":\"{}\",\"secondary_color\":\"{}\",\"title\":\"{}\",\"unit\":\"{}\",\"verifier\":{}", network_specs.base58prefix, network_specs.color, network_specs.decimals, hex::encode(network_specs.genesis_hash), network_specs.logo, network_specs.name, network_specs.order, network_specs.path_id, network_specs.secondary_color, network_specs.title, network_specs.unit, network_specs.verifier.show_card());
            let mut metadata_print = String::from("[");
            for (i,x) in relevant_metadata.iter().enumerate() {
                if i > 0 {metadata_print.push_str(",")}
                metadata_print.push_str(&format!("{{\"spec_version\":\"{}\",\"meta_hash\":\"{}\"}}", x.spec_version, x.metadata_hash));
            }
            metadata_print.push_str("]");
            Ok(format!("{{{},\"meta\":{}}}", chainspecs_print, metadata_print))
        },
        None => return Err(Box::from("Network key not found in chainspecs tree of the database")),
    }
}


pub fn get_network_details_by_hex (could_be_hex_gen_hash: &str, database_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    
    let hex_gen_hash = {
        if could_be_hex_gen_hash.starts_with("0x") {&could_be_hex_gen_hash[2..]}
        else {could_be_hex_gen_hash}
    };
    let network_key = generate_network_key(&hex::decode(hex_gen_hash)?);
    get_network_details_by_key (network_key, database_name)
    
}
