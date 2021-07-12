use sled::{Tree, IVec};
use parity_scale_codec::{Encode, Decode};
use definitions::{metadata::{AddressBookEntry, MetaValues, NameVersioned, VersionDecoded}, network_specs::ChainSpecsToSend};
use meta_reading::decode_metadata::get_meta_const;

use super::metadata_shortcut::{MetaShortCut, MetaSpecsShortCut, meta_shortcut, meta_specs_shortcut};
use super::output_prep::{load_meta_print, add_network_print, print_it};

/// Function to read network metadata entries existing in the metadata tree of the database
/// into MetaValues vector, and clear the metadata tree after reading.

pub fn read_metadata_database (metadata: &Tree) -> Result<Vec<MetaValues>, Box<dyn std::error::Error>> {
    
    let mut out: Vec<MetaValues> = Vec::new();
    
    for x in metadata.iter() {
    
        if let Ok((key, value)) = x {
    
        // decode what is in the key
            let name_versioned = match NameVersioned::decode(&mut &key[..]) {
                Ok(a) => a,
                Err(_) => return Err(Box::from("Database got corrupted. Unable to decode versioned name.")),
            };
    
        // check the database for corruption
            let version_vector = match get_meta_const(&value.to_vec()) {
                Ok(a) => a,
                Err(_) => return Err(Box::from("Database got corrupted. Unable to get version constant from the stored metadata.")),
            };   
            let version = match VersionDecoded::decode(&mut &version_vector[..]) {
                Ok(a) => a,
                Err(_) => return Err(Box::from("Database got corrupted. Unable to decode metadata version constant.")),
            };
            if (version.specname != name_versioned.name)||(version.spec_version != name_versioned.version) {return Err(Box::from("Database got corrupted. Specs from encoded metadata do not match the values in the versioned name."))}
        
        // prepare output
            let new = MetaValues {
                name: name_versioned.name,
                version: name_versioned.version,
                meta: value.to_vec(),
            };
            out.push(new);
        }
    }
    
    metadata.clear()?;
    
    Ok(out)
    
}


/// Struct used to sort the metadata entries:
/// newer has newest MetaValues entry from the database,
/// older has older MetaValues entry from the database

pub struct SortedMetaValues {
    pub newer: Vec<MetaValues>,
    pub older: Vec<MetaValues>,
}


/// Function to sort the metavalues into set of newest ones and set of older ones,
/// with maximum one older version for each of the networks;
/// at the moment it is agreed to have no more than two entries for each of the networks,
/// function throws error if finds the third one

pub fn sort_metavalues (meta_values: Vec<MetaValues>) -> Result<SortedMetaValues, String> {
    let mut newer: Vec<MetaValues> = Vec::new();
    let mut older: Vec<MetaValues> = Vec::new();
    for x in meta_values.iter() {
        let mut found_in_new = false;
        let mut num_new = None;
        for (i, y) in newer.iter().enumerate() {
            if x.name == y.name {
                for z in older.iter() {
                    if x.name == z.name {return Err(format!("Database corrupted. More than two metadata entries for network {}", x.name))}
                }
                found_in_new = true;
                if x.version < y.version {
                    let to_push = MetaValues {
                        name: x.name.to_string(),
                        version: x.version,
                        meta: x.meta.to_vec(),
                    };
                    older.push(to_push);
                }
                else {
                    if x.version == y.version {return Err(format!("Database corrupted. Same version {} is saved for {} two times.", x.version, x.name))}
                    else {
                        num_new = Some(i);
                    }
                }
            break;
            }
        }
        if !found_in_new {
            let to_push = MetaValues {
                name: x.name.to_string(),
                version: x.version,
                meta: x.meta.to_vec(),
            };
            newer.push(to_push);
        }
        if let Some(i) = num_new {
            older.push(newer.remove(i));
            let to_push = MetaValues {
                name: x.name.to_string(),
                version: x.version,
                meta: x.meta.to_vec(),
            };
            newer.push(to_push);
        }
    }
    Ok(SortedMetaValues{
        newer,
        older,
    })
}


/// Struct to store sorted metavalues and a flag indicating if the entry was added

pub struct UpdSortedMetaValues {
    pub sorted: SortedMetaValues,
    pub upd_done: bool,
}


/// Function to add new MetaValues entry to SortedMetaValues
/// If the fetched metadata is good and has later version than the ones in SortedMetaValues,
/// it is added to newer group of metavalues, any previous value from newer is moved to older,
/// if there was any value in older, it gets kicked out.
/// flag upd_done indicates if any update was done to the SortedMetaValues

pub fn add_new (new: &MetaValues, mut sorted: SortedMetaValues) -> Result<UpdSortedMetaValues, String> {
    let mut upd_done = false;
    let mut num_new = None;
    let mut num_old = None;
    let mut found_in_newer = false;
    for (i, x) in sorted.newer.iter().enumerate() {
        if &new.name == &x.name {
            found_in_newer = true;
            if new.version < x.version {return Err(format!("Error for {}. Fetched earlier version.", new.name))}
            else {
                if new.version == x.version {
                    if new.meta != x.meta {return Err(format!("Error for {}. Same version {} has different associated metadata.", new.name, new.version))}
                }
                else {
                    num_new = Some(i);
                    for (j, y) in sorted.older.iter().enumerate() {
                        if &x.name == &y.name {
                            num_old = Some(j);
                            break;
                        }
                    }
                }
            }
        }
    }
    let to_push_new = MetaValues {
        name: new.name.to_string(),
        version: new.version,
        meta: new.meta.to_vec(),
    };
    if !found_in_newer {
        upd_done = true;
        sorted.newer.push(to_push_new);
    }
    else {
        if let Some(j) = num_old {
            upd_done = true;
            sorted.older.remove(j);
        }
        if let Some(i) = num_new {
            upd_done = true;
            sorted.older.push(sorted.newer.remove(i));
            sorted.newer.push(to_push_new);
        }
    }
    Ok(UpdSortedMetaValues{
        sorted,
        upd_done,
    })
}


/// Function to search through the database to get network specs from the network name.
///
/// Encoded name is key in address_book tree of the database. Genesis hash is one of the fields in value.
/// Also, genesis hash is key in chainspecs tree of the database. Network name is one of the fields in value.
///
/// Database is searched and checked, first in address_book tree, then in chainspecs tree.
/// The manner of database filling suggests that all networks in metadata tree
/// should have corresponding entries in chainspecs tree and in address_book tree.
/// Also, entries in address_book and chainspecs are appearing always together, so the situation where
/// network is only in address_book or in chainspecs are database corruption cases.

pub fn specs_from_name (name: &str, address_book: &Tree, chainspecs: &Tree) -> Result<ChainSpecsToSend, Box<dyn std::error::Error>> {
    
    let entry_encoded = match address_book.get(name.encode())? {
        Some(a) => a,
        None => return Err(Box::from("No address_book entry.")),
    };
    let entry = match <AddressBookEntry>::decode(&mut &entry_encoded[..]) {
        Ok(a) => a,
        Err(_) => return Err(Box::from("Address_book entry not decodeable.")),
    };
    if name != entry.name {return Err(Box::from("Address_book entry corrupted. Name mismatch."))}
    
    let network_specs_encoded = match chainspecs.get(entry.genesis_hash.to_vec())? {
        Some(a) => a,
        None => return Err(Box::from("No chainspecs entry.")),
    };
    let network_specs = match <ChainSpecsToSend>::decode(&mut &network_specs_encoded[..]) {
        Ok(a) => a,
        Err(_) => return Err(Box::from("Chainspecs entry not decodeable.")),
    };
    if network_specs.genesis_hash != entry.genesis_hash {return Err(Box::from("Database corrupted. Genesis hash mismatch."))}
    if network_specs.name != name {return Err(Box::from("Database corrupted. Name mismatch."))}
    
    Ok(network_specs)
}


/// Function to search through the database to get genesis hash from the network name.
///
/// Since same type of checks is needed as in specs_from_name anyway,
/// this is just a small extension of specs_from_name

fn gen_hash_from_name (name: &str, address_book: &Tree, chainspecs: &Tree) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    
    let network_specs = specs_from_name (name, address_book, chainspecs)?;
    Ok(network_specs.genesis_hash)
    
}


/// Function to process single metadata tree entry to generate `load_meta` message
/// without rpc calls and without updating the database (-f key)

pub fn load_f_from_metadata_entry ((versioned_name_encoded, meta): (IVec, IVec), address_book: &Tree, chainspecs: &Tree) -> Result<(), Box<dyn std::error::Error>> {
    
    let versioned_name = <NameVersioned>::decode(&mut &versioned_name_encoded[..])?;
    let genesis_hash = gen_hash_from_name (&versioned_name.name, address_book, chainspecs)?;
    let shortcut = MetaShortCut {
        meta_values: MetaValues {
            name: versioned_name.name,
            version: versioned_name.version,
            meta: meta.to_vec(),
        },
        genesis_hash,
    };
    load_meta_print(&shortcut)
    
}


/// Function to process single metadata tree entry to generate `add_network` message
/// without rpc calls and without updating the database (-f key)

pub fn add_f_from_metadata_entry ((versioned_name_encoded, meta): (IVec, IVec), address_book: &Tree, chainspecs: &Tree) -> Result<(), Box<dyn std::error::Error>> {
    
    let versioned_name = <NameVersioned>::decode(&mut &versioned_name_encoded[..])?;
    let specs = specs_from_name (&versioned_name.name, address_book, chainspecs)?;
    let shortcut = MetaSpecsShortCut {
        meta_values: MetaValues {
            name: versioned_name.name,
            version: versioned_name.version,
            meta: meta.to_vec(),
        },
        specs,
        def: false,
    };
    add_network_print(&shortcut)
    
}


/// Function to process network name to generate `load_meta` message
/// without rpc calls and without updating the database (-f key)

pub fn load_f_from_name (name: &str, address_book: &Tree, chainspecs: &Tree, metadata: &Tree) -> Result<(), Box<dyn std::error::Error>> {
    
    let genesis_hash = gen_hash_from_name (&name, address_book, chainspecs)?;
    let mut found_name = false;
    for x in metadata.scan_prefix(name.encode()) {
        if let Ok((versioned_name_encoded, meta)) = x {
            found_name = true;
            let versioned_name = <NameVersioned>::decode(&mut &versioned_name_encoded[..])?;
            let shortcut = MetaShortCut {
                meta_values: MetaValues {
                    name: name.to_string(),
                    version: versioned_name.version,
                    meta: meta.to_vec(),
                },
                genesis_hash,
            };
            load_meta_print(&shortcut)?
        }
    }
    if !found_name {return Err(Box::from(format!("No network named {} found in the metadata database.", name)))}
    Ok(())
}


/// Function to process network name to generate `add_network` message
/// without rpc calls and without updating the database (-f key)

pub fn add_f_from_name (name: &str, address_book: &Tree, chainspecs: &Tree, metadata: &Tree) -> Result<(), Box<dyn std::error::Error>> {
    
    let specs = specs_from_name (&name, address_book, chainspecs)?;
    let mut found_name = false;
    for x in metadata.scan_prefix(name.encode()) {
        if let Ok((versioned_name_encoded, meta)) = x {
            found_name = true;
            let versioned_name = <NameVersioned>::decode(&mut &versioned_name_encoded[..])?;
            let shortcut = MetaSpecsShortCut {
                meta_values: MetaValues {
                    name: name.to_string(),
                    version: versioned_name.version,
                    meta: meta.to_vec(),
                },
                specs: ChainSpecsToSend {
                    base58prefix: specs.base58prefix,
                    color: specs.color.to_string(),
                    decimals: specs.decimals,
                    genesis_hash: specs.genesis_hash,
                    logo: specs.logo.to_string(),
                    name: specs.name.to_string(),
                    path_id: specs.path_id.to_string(),
                    secondary_color: specs.secondary_color.to_string(),
                    title: specs.title.to_string(),
                    unit: specs.unit.to_string(),
                },
                def: false,
            };
            add_network_print(&shortcut)?
        }
    }
    if !found_name {return Err(Box::from(format!("No network named {} found in the metadata database.", name)))}
    Ok(())
}


/// Function to process single AddressBookEntry, run meta_shortcut on the address,
/// do checks for database consistency, and
/// feed good result to load_meta_print

fn sign_me_prep_load_meta (x: &AddressBookEntry) -> Result<(), Box<dyn std::error::Error>> {
    let shortcut = meta_shortcut(&x.address)?;
    if shortcut.genesis_hash != x.genesis_hash {return Err(Box::from("Genesis hash has changed."))}
    if shortcut.meta_values.name != x.name {return Err(Box::from("Network name different in metadata and in address book."))}
    load_meta_print(&shortcut)
}


/// Function to process single AddressBookEntry, run meta_specs_shortcut on the address,
/// do checks for database consistency, and
/// feed good result to add_network_print

fn sign_me_prep_add_network (x: &AddressBookEntry, chainspecs: &Tree) -> Result<(), Box<dyn std::error::Error>> {
    let shortcut = meta_specs_shortcut(&x.address, chainspecs)?;
    if shortcut.specs.genesis_hash != x.genesis_hash {return Err(Box::from("Genesis hash has changed."))}
    if shortcut.meta_values.name != x.name {return Err(Box::from("Network name different in metadata and in address book."))}
    add_network_print(&shortcut)
}


/// Function to process single &str address, run meta_shortcut on the address, and
/// feed good result to load_meta_print

pub fn sign_me_prep_load_meta_from_address (address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let shortcut = meta_shortcut(address)?;
    load_meta_print(&shortcut)
}


/// Function to process single &str address, run meta_specs_shortcut on the address, and
/// feed good result to add_network_print

pub fn sign_me_prep_add_network_from_address (address: &str, chainspecs: &Tree) -> Result<(), Box<dyn std::error::Error>> {
    let shortcut = meta_specs_shortcut(address, chainspecs)?;
    add_network_print(&shortcut)
}


/// Function to process single address_book tree entry
/// through rpc call but without updating the database (-d key).
/// flag = true corresponds to generating `load_meta` message
/// flag = false corresponds to generating `add_network` message

pub fn do_d_from_address_book_entry ((name_encoded, entry_encoded): (IVec, IVec), flag: bool, chainspecs: &Tree) -> Result<(), Box<dyn std::error::Error>> {
    
    let entry = <AddressBookEntry>::decode(&mut &entry_encoded[..])?;
    let name = <String>::decode(&mut &name_encoded[..])?;
    if name != entry.name {return Err(Box::from("Database corrupted. Name mismatch in address book."))}
    if flag {sign_me_prep_load_meta (&entry)}
    else {sign_me_prep_add_network (&entry, chainspecs)}
    
}


/// Function to process network name to generate `load_meta` message
/// through rpc call but without updating the database (-d key).
/// flag = true corresponds to generating `load_meta` message
/// flag = false corresponds to generating `add_network` message

pub fn do_d_from_name (name: &str, flag: bool, address_book: &Tree, chainspecs: &Tree) -> Result<(), Box<dyn std::error::Error>> {
    
    match address_book.get(name.encode())? {
        Some(entry_encoded) =>{
            let entry = <AddressBookEntry>::decode(&mut &entry_encoded[..])?;
            if name != entry.name {return Err(Box::from("Database corrupted. Name mismatch in address book."))}
            if flag {sign_me_prep_load_meta (&entry)}
            else {sign_me_prep_add_network (&entry, chainspecs)}
        },
        None => {return Err(Box::from(format!("No address book entry found for network {}.", name)))},
    }
}


/// Function to insert information from MetaSpecsShortCut into database
/// using add_new function
///
/// flag = true corresponds to generating `load_meta` message
/// flag = false corresponds to generating `add_network` message
///
/// Can run with or without generating message
/// depending on output_all and output_only_new flags

pub fn insert_shortcut (shortcut: MetaSpecsShortCut, flag: bool, metadata: &Tree, output_all: bool, output_only_new: bool) -> Result<(), Box<dyn std::error::Error>> {
    
    let known_metavalues = read_metadata_database(metadata)?;
    metadata.clear()?;
    
    let mut sorted_known_metavalues = sort_metavalues(known_metavalues)?;
    let upd_sorted = add_new(&shortcut.meta_values, sorted_known_metavalues)?;
    if output_all {print_it(flag, &shortcut)?}
    else {
        if output_only_new {
            if upd_sorted.upd_done {print_it(flag, &shortcut)?}
        }
    }
    sorted_known_metavalues = upd_sorted.sorted;
    
    let mut all_meta = sorted_known_metavalues.newer;
    all_meta.extend(sorted_known_metavalues.older);
    
    for x in all_meta.iter() {
        let versioned_name = NameVersioned {
            name: x.name.to_string(),
            version: x.version,
        };
        metadata.insert(versioned_name.encode(), x.meta.to_vec())?;
    }
    
    Ok(())
   
}


/// Function to update metadata tree in the database through rpc call
/// starting with address_book entry.
/// 
/// flag = true corresponds to generating `load_meta` message
/// flag = false corresponds to generating `add_network` message
///
/// Can run with or without generating message
/// depending on output_all and output_only_new flags
///
/// Function is suitable for -k, -p and -t keys

pub fn upd_from_address_book_entry ((name_encoded, entry_encoded): (IVec, IVec), flag: bool, chainspecs: &Tree, metadata: &Tree, output_all: bool, output_only_new: bool) -> Result<(), Box<dyn std::error::Error>> {
    
    let entry = <AddressBookEntry>::decode(&mut &entry_encoded[..])?;
    let name = <String>::decode(&mut &name_encoded[..])?;
    if name != entry.name {return Err(Box::from("Database corrupted. Name mismatch in address book."))}
    let shortcut = meta_specs_shortcut(&entry.address, chainspecs)?;
    if shortcut.specs.genesis_hash != entry.genesis_hash {return Err(Box::from("Genesis hash has changed."))}
    if shortcut.meta_values.name != entry.name {return Err(Box::from("Network name different in metadata and in address book."))}
    
    insert_shortcut (shortcut, flag, metadata, output_all, output_only_new)
}


/// Function to update metadata tree in the database through rpc call
/// starting with network name.
/// 
/// flag = true corresponds to generating `load_meta` message
/// flag = false corresponds to generating `add_network` message
///
/// Can run with or without generating message
/// depending on output_all and output_only_new flags
///
/// Function is suitable for -k, -p and -t keys

pub fn upd_from_name (name: &str, flag: bool, address_book: &Tree, chainspecs: &Tree, metadata: &Tree, output_all: bool, output_only_new: bool) -> Result<(), Box<dyn std::error::Error>> {
    
    match address_book.get(name.encode())? {
        Some(entry_encoded) =>{
            let entry = <AddressBookEntry>::decode(&mut &entry_encoded[..])?;
            if name != entry.name {return Err(Box::from("Database corrupted. Name mismatch in address book."))}
            let shortcut = meta_specs_shortcut(&entry.address, chainspecs)?;
            if shortcut.specs.genesis_hash != entry.genesis_hash {return Err(Box::from("Genesis hash has changed."))}
            if shortcut.meta_values.name != entry.name {return Err(Box::from("Network name different in metadata and in address book."))}
            insert_shortcut (shortcut, flag, metadata, output_all, output_only_new)
        },
        None => {return Err(Box::from(format!("No address book entry found for network {}.", name)))},
    }
}


/// Function to process single &str address through rpc call
/// 
/// flag = true corresponds to generating `load_meta` message
/// flag = false corresponds to generating `add_network` message
///
/// Can run with or without generating message
/// depending on output_all and output_only_new flags
///
/// Function is suitable for -k, -p and -t keys

pub fn insert_from_address (flag: bool, address: &str, metadata: &Tree, chainspecs: &Tree, address_book: &Tree, output_all: bool, output_only_new: bool) -> Result<(), Box<dyn std::error::Error>> {
    
    let shortcut = meta_specs_shortcut(address, chainspecs)?;
    
    if shortcut.def {
    // genesis hash is not in the chainspecs tree of the database, network is new:
    // print file,
    // add information into chainspecs tree, metadata tree (no double-checking), address_book tree (no double-checking)
    
        if output_all | output_only_new {print_it(flag, &shortcut)?}
        
        chainspecs.insert(shortcut.specs.genesis_hash.to_vec(), shortcut.specs.encode())?;
        
        let versioned_name = NameVersioned {
            name: shortcut.meta_values.name.to_string(),
            version: shortcut.meta_values.version,
        };
        metadata.insert(versioned_name.encode(), shortcut.meta_values.meta.to_vec())?;
        
        let entry = AddressBookEntry {
            name: shortcut.meta_values.name.to_string(),
            genesis_hash: shortcut.specs.genesis_hash,
            address: address.to_string(),
        };
        address_book.insert(shortcut.meta_values.name.encode(), entry.encode())?;
        
    }
    else {
    // genesis hash is in the chainspecs tree of the database
    
    // checking metadata database
        let name_versioned = NameVersioned {
            name: shortcut.meta_values.name.to_string(),
            version: shortcut.meta_values.version,
        };
        match metadata.get(&name_versioned.encode())? {
            Some(known_meta) => {
                if known_meta == &shortcut.meta_values.meta {
                    if output_all {print_it(flag, &shortcut)?}
                }
                else {return Err(Box::from(format!("Error. Metadata for network {} version {} in database is different from the one just fetched.", shortcut.meta_values.name, shortcut.meta_values.version)));}
            },
            None => {
                let known_metavalues = read_metadata_database(metadata)?;
                metadata.clear()?;
                
                let sorted_known_metavalues = sort_metavalues(known_metavalues)?;
            
                let upd_sorted = add_new(&shortcut.meta_values, sorted_known_metavalues)?;
                
                if output_all {print_it(flag, &shortcut)?}
                else {
                    if output_only_new {
                        if upd_sorted.upd_done {print_it(flag, &shortcut)?}
                    }
                }
                
                let mut all_meta = upd_sorted.sorted.newer;
                all_meta.extend(upd_sorted.sorted.older);
    
                for x in all_meta.iter() {
                    let versioned_name = NameVersioned {
                        name: x.name.to_string(),
                        version: x.version,
                    };
                    metadata.insert(versioned_name.encode(), x.meta.to_vec())?;
                }
            },
        }
        
    // updating address book if needed (since the new address worked)
        match address_book.get(shortcut.meta_values.name.encode())? {
            Some(entry_encoded) => {
                let mut entry = <AddressBookEntry>::decode(&mut &entry_encoded[..])?;
                if &entry.address != address {
                    address_book.remove(shortcut.meta_values.name.encode())?;
                    entry.address = address.to_string();
                    address_book.insert(shortcut.meta_values.name.encode(), entry.encode())?;
                }
            },
            None => {
                let entry = AddressBookEntry {
                    name: shortcut.meta_values.name.to_string(),
                    genesis_hash: shortcut.specs.genesis_hash,
                    address: address.to_string(),
                };
                address_book.insert(shortcut.meta_values.name.encode(), entry.encode())?;
            }
        }
    }
    Ok(())
}



/// Function to process error depending on pass_errors flag

pub fn error_occured (e: Box<dyn std::error::Error>, pass_errors: bool) -> Result<(), Box<dyn std::error::Error>> {
    if pass_errors {Ok(println!("Error encountered. {} Skipping it.", e))}
    else {return Err(e)}
}



