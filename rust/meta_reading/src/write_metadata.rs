use sled::Tree;
use parity_scale_codec::{Encode, Decode};
use parity_scale_codec_derive;
use std::fs::OpenOptions;
use std::io::prelude::*;

use super::decode_metadata::{get_meta_const, MetaValues, VersionDecoded};

/// Struct used to store the network metadata name and version in the database

#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub struct NameVersioned {
    pub name: String,
    pub version: u32,
}


/// Function to read network metadata entries existing in the database into MetaValues vector,
/// and remove the database tree after reading.

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
                Err(_) => return Err(Box::from("Database got corrupted. Unable to get version from the stored metadata.")),
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
    
    Ok(out)
    
}


/// Struct used to sort the metadata entries:
/// newer has newest MetaValues entry from the database,
/// older has older MetaValues entry from the database

pub struct SortedMetaValues {
    pub newer: Vec<MetaValues>,
    pub older: Vec<MetaValues>,
}


/// Function to sort the metavalues,
/// since at the moment agreed to have no more than two entries,
/// throws error if finds the third one

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
    if !found_in_newer {
        upd_done = true;
        let to_push_new = MetaValues {
            name: new.name.to_string(),
            version: new.version,
            meta: new.meta.to_vec(),
        };
        sorted.newer.push(to_push_new)
    }
    if let Some(j) = num_old {
        upd_done = true;
        sorted.older.remove(j);
    }
    if let Some(i) = num_new {
        upd_done = true;
        sorted.older.push(sorted.newer.remove(i));
    }
    Ok(UpdSortedMetaValues{
        sorted,
        upd_done,
    })
}


/// Function to update the database with modified entries,
/// also creates log file

pub fn write_metadata_database (metadata: &Tree, meta_values: Vec<MetaValues>, logfile_name: &str) -> Result <(), Box<dyn std::error::Error>> {
    
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(logfile_name)?;
        
    for x in meta_values.iter() {
        let string_for_log = format!("{}{}", x.name, x.version);
        let versioned_name = NameVersioned {
            name: x.name.to_string(),
            version: x.version,
        };
        metadata.insert(versioned_name.encode(), x.meta.to_vec())?;
        writeln!(file, "{}", string_for_log)?;
    }
    
    Ok(())

}
