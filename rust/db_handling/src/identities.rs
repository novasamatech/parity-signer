use sled::{Db, Tree, open};
use parity_scale_codec::{Decode, Encode};
use parity_scale_codec_derive;

#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub struct AddressDetails {
    pub name_for_seed: String,
    pub path: String,
    pub has_pwd: bool,
    pub name: String,
    pub network_id: String,
}

///get all seed names to display selector

pub fn get_seed_names_list (database_name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let database: Db = open(database_name)?;
    let seeds: Tree = database.open_tree(b"seeds")?;
    match seeds
        .iter()
        .collect::<Result<Vec<_>,_>>()?
        .into_iter()
        .map(|(key, _)| <String>::decode(&mut &key[..]))
        .collect::<Result<Vec<_>,_>>()
        {
            Ok(a) => Ok(a),
            Err(e) => return Err(Box::from(e)),
        }
}

///get all identities within given seed and network
pub fn get_relevant_identities (seed_name: &str, genesis_hash: &str, database_name: &str) -> Result<Vec<AddressDetails>, Box<dyn std::error::Error>> {
    let database: Db = open(database_name)?;
    let identities: Tree = database.open_tree(b"addresses")?;
    Ok(identities
        .iter()
        .collect::<Result<Vec<_>,_>>()?
        .into_iter()
        .map(|(_, value)| <AddressDetails>::decode(&mut &value[..]))
        .collect::<Result<Vec<_>,_>>()?
        .into_iter()
        .filter(|identity| (identity.network_id == genesis_hash) && (identity.name_for_seed == seed_name))
        .collect())
}

