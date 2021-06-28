use hex;
use sled::{Db, Tree, open};
use std::convert::TryInto;
use parity_scale_codec::Encode;

pub mod address_book;
    use address_book::{AddressBookEntry, get_default_address_book};
mod constants;
    use constants::{METATREEPREP, SPECSTREEPREP};
pub mod decode_metadata;
    use decode_metadata::decode_version;
pub mod interpret_chainspecs;
    use interpret_chainspecs::interpret_chainspecs;
pub mod fetch_metadata;
    use fetch_metadata::{fetch_info, fetch_info_with_chainspecs};
mod write_metadata;
    use write_metadata::{NameVersioned, read_metadata_database, sort_metavalues, SortedMetaValues, add_new, write_metadata_database};


/// Function to process single AddressBookEntry, fetch metadata for it,
/// and prepare concatenated metadata and genesis hash, all to be signed by subkey tool

pub fn meta_for_signing (x: AddressBookEntry) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let new_info = fetch_info(x.address)?;
    let genesis_hash_fetched = match &new_info.genesis_hash[..2] {
        "0x" => hex::decode(&new_info.genesis_hash[2..])?,
        _ => hex::decode(&new_info.genesis_hash)?,
    };
    if genesis_hash_fetched != x.genesis_hash {return Err(Box::from("Genesis hash has changed."))}
    let meta_values = decode_version(&new_info.meta)?;
    if meta_values.name != x.name {return Err(Box::from("Network name different in metadata and in address book."))}
    Ok([meta_values.meta, x.genesis_hash.to_vec()].concat())
}

pub fn default_full_run (database_name: &str, logfile_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let address_book = get_default_address_book();
    
    let database: Db = open(database_name)?;
    let metadata: Tree = database.open_tree(METATREEPREP)?;
    
    let known_metavalues = read_metadata_database(&metadata)?;
    
    database.drop_tree(METATREEPREP)?;
    database.flush()?;
    
    let mut sorted_known_metavalues = sort_metavalues(known_metavalues)?;
    for x in address_book.iter() {
        sorted_known_metavalues = match process_single_entry(x, sorted_known_metavalues) {
            Ok(a) => a,
            Err(e) => {
                let err_line = format!("Error processing {}. {}", x.name, e);
                let err: Box<dyn std::error::Error> = From::from(err_line);
                return Err(err)
            },
        }
    }
    let mut all_metavalues = sorted_known_metavalues.newer;
    all_metavalues.append(&mut sorted_known_metavalues.older);
    
    let metadata: Tree = database.open_tree(METATREEPREP)?;
    
    write_metadata_database (&metadata, all_metavalues, logfile_name)?;
    
    database.flush()?;
    
    Ok(())
}


pub fn process_single_entry (x: &AddressBookEntry, sorted_known_metavalues: SortedMetaValues) -> Result<SortedMetaValues, Box<dyn std::error::Error>> {
    let new_info = fetch_info(x.address)?;
    let genesis_hash_fetched = match &new_info.genesis_hash[..2] {
        "0x" => hex::decode(&new_info.genesis_hash[2..])?,
        _ => hex::decode(&new_info.genesis_hash)?,
    };
    if genesis_hash_fetched != x.genesis_hash {return Err(Box::from("Genesis hash has changed."))}
    let new = decode_version(&new_info.meta)?;
    let upd_sorted = add_new(&new, sorted_known_metavalues)?;
    if upd_sorted.upd_done {
    // make new file with name containing network specname and spec_version,
    // and having inside &[u8] of concatenated meta and genesis hash
        let filename = format!("tests/files_for_signing/{}{}", new.name, new.version);
        let contents = [new.meta, genesis_hash_fetched].concat();
        std::fs::write(&filename, &contents)?;
    }
    Ok(upd_sorted.sorted)
}


pub fn process_single_entry_with_chainspecs (x: &AddressBookEntry, sorted_known_metavalues: SortedMetaValues, chainspecs: &Tree) -> Result<SortedMetaValues, Box<dyn std::error::Error>> {
    let new_info_with_chainspecs = fetch_info_with_chainspecs(x.address)?;
    let genesis_hash_fetched = match &new_info_with_chainspecs.genesis_hash[..2] {
        "0x" => hex::decode(&new_info_with_chainspecs.genesis_hash[2..])?,
        _ => hex::decode(&new_info_with_chainspecs.genesis_hash)?,
    };
    if genesis_hash_fetched != x.genesis_hash {return Err(Box::from("Genesis hash has changed."))}
    let new = decode_version(&new_info_with_chainspecs.meta)?;
    if new.name != x.name {return Err(Box::from("Network name different in metadata and in address book."))}
    let new_chain_specs_encoded = interpret_chainspecs(&new_info_with_chainspecs.properties, x.genesis_hash, &new.name)?;
    match chainspecs.get(&genesis_hash_fetched)? {
        Some(known_specs_encoded) => {
            if known_specs_encoded == new_chain_specs_encoded {()}
            else {return Err(Box::from("Network specs saved in database are different from the freshly fetched ones."))}
        },
        None => {
            let filename = format!("tests/files_for_signing/chainspecs_{}", new.name);
            std::fs::write(&filename, &new_chain_specs_encoded)?;
            chainspecs.insert(&genesis_hash_fetched.to_vec(), new_chain_specs_encoded)?;
        }
    }
    let upd_sorted = add_new(&new, sorted_known_metavalues)?;
    if upd_sorted.upd_done {
    // make new file with name containing network specname and spec_version,
    // and having inside &[u8] of concatenated meta and genesis hash
        let filename = format!("tests/files_for_signing/metadata_{}{}", new.name, new.version);
        let contents = [new.meta, genesis_hash_fetched].concat();
        std::fs::write(&filename, &contents)?;
    }
    Ok(upd_sorted.sorted)
}


pub fn default_full_run_with_chainspecs (database_name: &str, logfile_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let address_book = get_default_address_book();
    
    let database: Db = open(database_name)?;
    let metadata: Tree = database.open_tree(METATREEPREP)?;
    let chainspecs: Tree = database.open_tree(SPECSTREEPREP)?;
    
    let known_metavalues = read_metadata_database(&metadata)?;
    
    database.drop_tree(METATREEPREP)?;
    database.flush()?;
    
    let mut sorted_known_metavalues = sort_metavalues(known_metavalues)?;
        
    for x in address_book.iter() {
        sorted_known_metavalues = match process_single_entry_with_chainspecs(x, sorted_known_metavalues, &chainspecs) {
            Ok(a) => a,
            Err(e) => {
                let err_line = format!("Error processing {}. {}", x.name, e);
                let err: Box<dyn std::error::Error> = From::from(err_line);
                return Err(err)
            },
        };
        database.flush()?;
    }
    let mut all_metavalues = sorted_known_metavalues.newer;
    all_metavalues.append(&mut sorted_known_metavalues.older);
    
    let metadata: Tree = database.open_tree(METATREEPREP)?;
    
    write_metadata_database (&metadata, all_metavalues, logfile_name)?;
    database.flush()?;
    Ok(())
}


pub fn add_from_address (address: &str, database_name: &str, logfile_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(database_name)?;
    let metadata: Tree = database.open_tree(METATREEPREP)?;
    let chainspecs: Tree = database.open_tree(SPECSTREEPREP)?;
    
    let new_info_with_chainspecs = fetch_info_with_chainspecs(address)?;
    let genesis_hash_fetched: [u8; 32] = match &new_info_with_chainspecs.genesis_hash[..2] {
        "0x" => {
            let a = hex::decode(&new_info_with_chainspecs.genesis_hash[2..])?;
            match a.try_into() {
                Ok(b) => b,
                Err(_) => return Err(Box::from("Unexpected genesis hash format")),
            }
        },
        _ => {
            let a = hex::decode(&new_info_with_chainspecs.genesis_hash)?;
            match a.try_into() {
                Ok(b) => b,
                Err(_) => return Err(Box::from("Unexpected genesis hash format")),
            }
        },
    };
    let new = decode_version(&new_info_with_chainspecs.meta)?;
    let new_chain_specs_encoded = interpret_chainspecs(&new_info_with_chainspecs.properties, genesis_hash_fetched, &new.name)?;
    match chainspecs.get(&genesis_hash_fetched)? {
        Some(known_specs_encoded) => {
            if known_specs_encoded == new_chain_specs_encoded {
            // database already contains info about the network
                let name_versioned = NameVersioned {
                    name: new.name.to_string(),
                    version: new.version,
                };
                match metadata.get(&name_versioned.encode())? {
                    Some(known_meta) => {
                        if known_meta == &new.meta {Ok(println!("Network {} version {} already has correct metadata in the database. No updates made.", new.name, new.version))}
                        else {
                            let err_text = format!("Error. Metadata for network {} version {} in database id different from the one just fetched.", new.name, new.version);
                            let e: Box<dyn std::error::Error> = From::from(err_text);
                            return Err(e);
                        }
                    },
                    None => {
            
                        let known_metavalues = read_metadata_database(&metadata)?;
            
                        database.drop_tree(METATREEPREP)?;
                        database.flush()?;
    
                        let sorted_known_metavalues = sort_metavalues(known_metavalues)?;
            
                        let mut upd_sorted = add_new(&new, sorted_known_metavalues)?;
                        if upd_sorted.upd_done {
                        // make new file with name containing network specname and spec_version,
                        // and having inside &[u8] of concatenated meta and genesis hash
                            let filename = format!("tests/files_for_signing/metadata_{}{}", new.name, new.version);
                            let contents = [new.meta, genesis_hash_fetched.to_vec()].concat();
                            std::fs::write(&filename, &contents)?;
                        }
            
                        let mut all_metavalues = upd_sorted.sorted.newer;
                        all_metavalues.append(&mut upd_sorted.sorted.older);
    
                        let metadata: Tree = database.open_tree(METATREEPREP)?;
    
                        write_metadata_database (&metadata, all_metavalues, logfile_name)?;
                        database.flush()?;
                        Ok(())
                    }
                }
            }
            else {return Err(Box::from("Network specs saved in database are different from the freshly fetched ones."))}
        },
        None => {
            let filename = format!("tests/files_for_signing/chainspecs_{}", new.name);
            std::fs::write(&filename, &new_chain_specs_encoded)?;
            chainspecs.insert(&genesis_hash_fetched.to_vec(), new_chain_specs_encoded)?;
            
        // make new file with name containing network specname and spec_version,
        // and having inside &[u8] of concatenated meta and genesis hash
            let filename = format!("tests/files_for_signing/metadata_{}{}", new.name, new.version);
            let contents = [(&new.meta).to_vec(), genesis_hash_fetched.to_vec()].concat();
            std::fs::write(&filename, &contents)?;
            
            let mut known_metavalues = read_metadata_database(&metadata)?;
            database.drop_tree(METATREEPREP)?;
            database.flush()?;
            
            known_metavalues.push(new);
            
            let metadata: Tree = database.open_tree(METATREEPREP)?;
    
            write_metadata_database (&metadata, known_metavalues, logfile_name)?;
            database.flush()?;
            Ok(())
            
        }
    }
}
