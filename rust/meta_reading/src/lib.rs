use hex;

mod address_book;
    use address_book::{AddressBookEntry, get_default_address_book};
mod constants;
mod decode_metadata;
    use decode_metadata::decode_version;
mod fetch_metadata;
    use fetch_metadata::fetch_info;
mod write_metadata;
    use write_metadata::{read_metadata_database, sort_metavalues, SortedMetaValues, add_new, write_metadata_database};


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

pub fn full_run (database_name: &str, logfile_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let address_book = get_default_address_book();
    let known_metavalues = read_metadata_database(database_name)?;
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
    write_metadata_database (database_name, all_metavalues, logfile_name)?;
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
