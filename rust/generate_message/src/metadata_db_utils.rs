use constants::{HOT_DB_NAME, METATREE};
use db_handling::{
    db_transactions::TrDbHot,
    helpers::{make_batch_clear_tree, open_db, open_tree},
};
use definitions::{
    error::{Active, DatabaseActive, ErrorActive, Fetch},
    keyring::MetaKey,
    metadata::MetaValues,
};

/// Function to read network metadata entries existing in the metadata tree of the database
/// into MetaValues vector, and clear the metadata tree after reading.
fn read_metadata_database() -> Result<Vec<MetaValues>, ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let metadata = open_tree::<Active>(&database, METATREE)?;
    let mut out: Vec<MetaValues> = Vec::new();
    for x in metadata.iter().flatten() {
        out.push(MetaValues::from_entry_checked::<Active>(x)?)
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

/// Function to sort the metavalues into set of newest ones and set of older ones,
/// with maximum one older version for each of the networks;
/// at the moment it is agreed to have no more than two entries for each of the networks,
/// function throws error if finds the third one
fn sort_metavalues(meta_values: Vec<MetaValues>) -> Result<SortedMetaValues, ErrorActive> {
    let mut newer: Vec<MetaValues> = Vec::new();
    let mut older: Vec<MetaValues> = Vec::new();
    for x in meta_values.iter() {
        let mut found_in_new = false;
        let mut num_new = None;
        for (i, y) in newer.iter().enumerate() {
            if x.name == y.name {
                for z in older.iter() {
                    if x.name == z.name {
                        return Err(ErrorActive::Database(
                            DatabaseActive::HotDatabaseMetadataOverTwoEntries {
                                name: x.name.to_string(),
                            },
                        ));
                    }
                }
                found_in_new = true;
                match x.version.cmp(&y.version) {
                    std::cmp::Ordering::Less => {
                        older.push(x.to_owned());
                    }
                    std::cmp::Ordering::Equal => {
                        return Err(ErrorActive::Database(
                            DatabaseActive::HotDatabaseMetadataSameVersionTwice {
                                name: x.name.to_string(),
                                version: x.version,
                            },
                        ));
                    }
                    std::cmp::Ordering::Greater => {
                        num_new = Some(i);
                    }
                }

                break;
            }
        }
        if !found_in_new {
            newer.push(x.to_owned());
        }
        if let Some(i) = num_new {
            older.push(newer.remove(i));
            newer.push(x.to_owned());
        }
    }
    Ok(SortedMetaValues { newer, older })
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
pub fn add_new(
    new: &MetaValues,
    sorted: &SortedMetaValues,
) -> Result<UpdSortedMetaValues, ErrorActive> {
    let mut upd_done = false;
    let mut num_new = None;
    let mut num_old = None;
    let mut found_in_newer = false;
    for (i, x) in sorted.newer.iter().enumerate() {
        if new.name == x.name {
            found_in_newer = true;
            match new.version.cmp(&x.version) {
                std::cmp::Ordering::Less => {
                    return Err(ErrorActive::Fetch(Fetch::EarlierVersion {
                        name: x.name.to_string(),
                        old_version: x.version,
                        new_version: new.version,
                    }));
                }
                std::cmp::Ordering::Equal => {
                    if new.meta != x.meta {
                        let mut sus1: Vec<u8> = Vec::new();
                        let mut sus2: Vec<u8> = Vec::new();
                        for a in 0..x.meta.len() {
                            if new.meta[a] != x.meta[a] {
                                println!("Suspicious number {}", a);
                                sus1.push(new.meta[a]);
                                sus2.push(x.meta[a]);
                            }
                        }
                        println!("new: {:?}, in db: {:?}", sus1, sus2);
                        return Err(ErrorActive::Fetch(Fetch::SameVersionDifferentMetadata {
                            name: new.name.to_string(),
                            version: new.version,
                        }));
                    }
                }
                std::cmp::Ordering::Greater => {
                    num_new = Some(i);
                    for (j, y) in sorted.older.iter().enumerate() {
                        if x.name == y.name {
                            num_old = Some(j);
                            break;
                        }
                    }
                }
            }
        }
    }
    let mut sorted_output = SortedMetaValues {
        newer: sorted.newer.to_vec(),
        older: sorted.older.to_vec(),
    };
    if !found_in_newer {
        upd_done = true;
        sorted_output.newer.push(new.to_owned());
    } else {
        if let Some(j) = num_old {
            upd_done = true;
            sorted_output.older.remove(j);
        }
        if let Some(i) = num_new {
            upd_done = true;
            sorted_output.older.push(sorted_output.newer.remove(i));
            sorted_output.newer.push(new.to_owned());
        }
    }
    Ok(UpdSortedMetaValues {
        sorted: sorted_output,
        upd_done,
    })
}

/// Function to collect metadata from metadata tree of the database, clear that tree,
/// and sort the metadata into newer and older subsets
pub fn prepare_metadata() -> Result<SortedMetaValues, ErrorActive> {
    let known_metavalues = read_metadata_database()?;
    sort_metavalues(known_metavalues)
}

/// Function to write sorted metadata into the database
pub fn write_metadata(sorted_meta_values: SortedMetaValues) -> Result<(), ErrorActive> {
    let mut metadata_batch = make_batch_clear_tree::<Active>(HOT_DB_NAME, METATREE)?;
    let mut all_meta = sorted_meta_values.newer;
    all_meta.extend_from_slice(&sorted_meta_values.older);
    for x in all_meta.iter() {
        let meta_key = MetaKey::from_parts(&x.name, x.version);
        metadata_batch.insert(meta_key.key(), x.meta.to_vec());
    }
    TrDbHot::new()
        .set_metadata(metadata_batch)
        .apply(HOT_DB_NAME)
}
