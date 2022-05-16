//! Managing network metadata in hot database
//!
//! Hot database stores metadata entries in [`METATREE`] tree. Even though there
//! is `remove` command allowing user to remove metadata entries, at the
//! moment the maximum number of the metadata entries in the database for any
//! given network is two.
//!
//! Hot database gets the metadata entries only through rpc calls. The metadata
//! could be in the database only if there are also the network specs entry
//! [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend) in
//! [`SPECSTREE`](constants::SPECSTREE) and
//! [`AddressBookEntry`](definitions::metadata::AddressBookEntry) in
//! [`ADDRESS_BOOK`](constants::ADDRESS_BOOK).
use std::cmp::Ordering;

use constants::{HOT_DB_NAME, METATREE};
use db_handling::{
    db_transactions::TrDbHot,
    helpers::{make_batch_clear_tree, open_db, open_tree},
};
use definitions::{
    error_active::{Active, DatabaseActive, ErrorActive, Fetch},
    keyring::MetaKey,
    metadata::MetaValues,
};

/// Read all network metadata entries from the database
fn read_metadata_database() -> Result<Vec<MetaValues>, ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let metadata = open_tree::<Active>(&database, METATREE)?;
    let mut out: Vec<MetaValues> = Vec::new();
    for x in metadata.iter().flatten() {
        out.push(MetaValues::from_entry_checked::<Active>(x)?)
    }
    Ok(out)
}

/// Sorted metadata entries
pub struct SortedMetaValues {
    /// Set of the metadata entries with latest version known to the database.
    pub newer: Vec<MetaValues>,

    /// Other metadata entries. Since there are maximum two entries allowed,
    /// this set contains at most one entry for each network.
    pub older: Vec<MetaValues>,
}

/// Sort the metadata entries form the database into sets of newer and older, by
/// metadata version.
fn sort_metavalues(meta_values: Vec<MetaValues>) -> Result<SortedMetaValues, ErrorActive> {
    // newer metadata set, i.e. with higher version for given network
    let mut newer: Vec<MetaValues> = Vec::new();

    // older metadata set
    let mut older: Vec<MetaValues> = Vec::new();

    // scan through all available metadata and collect `newer` and `older` sets
    for x in meta_values.iter() {
        // flag to indicate that network has metadata entry in already collected
        // `newer` set
        let mut found_in_new = false;

        // entry number that should be removed from `newer` set, not necessarily
        // invoked for every true `found_in_new`
        let mut num_new = None;

        // search for the network name in already collected elements of `newer`
        // set
        for (i, y) in newer.iter().enumerate() {
            if x.name == y.name {
                // search for the network name in already collected elements of
                // `older` set; should not find any;
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

                // where the entry goes, based on the version
                match x.version.cmp(&y.version) {
                    // `x` entry goes to `older`
                    Ordering::Less => older.push(x.to_owned()),

                    // same version?!
                    Ordering::Equal => {
                        return Err(ErrorActive::Database(
                            DatabaseActive::HotDatabaseMetadataSameVersionTwice {
                                name: x.name.to_string(),
                                version: x.version,
                            },
                        ))
                    }

                    // `x` entry goes to `newer` and replaces `y` entry, `y`
                    // entry goes to `older`
                    Ordering::Greater => num_new = Some(i),
                }

                break;
            }
        }

        // no metadata entry in `newer`, simply add to `newer`
        if !found_in_new {
            newer.push(x.to_owned());
        }

        // already had metadata entry with older version in `newer` set;
        //
        // move existing entry to `older`, then add freshly found entry to
        // `newer`
        if let Some(i) = num_new {
            older.push(newer.remove(i));
            newer.push(x.to_owned());
        }
    }
    Ok(SortedMetaValues { newer, older })
}

/// Possibly updated sorted metadata entries
pub struct UpdSortedMetaValues {
    /// Sorted metadata entries, after attempt to add a new entry.
    pub sorted: SortedMetaValues,

    /// Flag to indicate if the entry was added.
    pub upd_done: bool,
}

/// Add new [`MetaValues`] entry to [`SortedMetaValues`]
///
/// If the fetched metadata is good and has later version than the ones in
/// [`SortedMetaValues`], it is added to `newer` set, any previous value from
/// `newer` is moved to `older`. If there was any value in `older`, it gets
/// kicked out.
pub fn add_new(
    new: &MetaValues,
    sorted: &SortedMetaValues,
) -> Result<UpdSortedMetaValues, ErrorActive> {
    // flag to indicate that updates were done, i.e. the database entries should
    // be rewritten
    let mut upd_done = false;

    // entry number to remove from `newer` set, and put into `older` set, if any
    let mut num_new = None;

    // entry number to remove from `older` set, if any
    let mut num_old = None;

    // flag to indicate that the number was found in `newer` set
    let mut found_in_newer = false;

    // search for entry with same name through `newer` existing entries
    for (i, x) in sorted.newer.iter().enumerate() {
        if new.name == x.name {
            found_in_newer = true;
            match new.version.cmp(&x.version) {
                // earlier metadata should not be fetched through rpc call;
                //
                // version downgrades happened, but these should always be
                // double checked before being accepted;
                //
                // earlier metadata could be retrieved from an outdated `.wasm`
                // file - no reason to accept it either;
                Ordering::Less => {
                    return Err(ErrorActive::Fetch(Fetch::EarlierVersion {
                        name: x.name.to_string(),
                        old_version: x.version,
                        new_version: new.version,
                    }))
                }

                // same version, no updates;
                //
                // check that metadata is exactly the same, different metadata
                // under same version is an error;
                Ordering::Equal => {
                    if new.meta != x.meta {
                        // metadata comparing, hopefully never to be needed
                        // again
                        //
                        // prints the difference for user to check
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

                // fetched newer metadata
                Ordering::Greater => {
                    // found entry in `newer` to move into `older`
                    num_new = Some(i);

                    // check if there is entry in `older` to be kicked
                    // altogether
                    for (j, y) in sorted.older.iter().enumerate() {
                        if x.name == y.name {
                            // found entry in `older` to be removed
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

    // no entries found in `newer`, i.e. no entries at all
    if !found_in_newer {
        upd_done = true;

        // push received entry into `newer`
        sorted_output.newer.push(new.to_owned());
    } else {
        // remove known entry from `older` if needed
        if let Some(j) = num_old {
            upd_done = true;
            sorted_output.older.remove(j);
        }

        // move known entry from `newer` to `older` , push received entry into
        // `newer`
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

/// Collect and sort metadata from [`METATREE`](constants::METATREE) tree of the
/// hot database
pub fn prepare_metadata() -> Result<SortedMetaValues, ErrorActive> {
    let known_metavalues = read_metadata_database()?;
    sort_metavalues(known_metavalues)
}

/// Clear [`METATREE`](constants::METATREE) tree of the hot database and write
/// new sorted metadata into it
pub fn write_metadata(sorted_meta_values: SortedMetaValues) -> Result<(), ErrorActive> {
    let mut metadata_batch = make_batch_clear_tree::<Active>(HOT_DB_NAME, METATREE)?;
    let mut all_meta = sorted_meta_values.newer;
    all_meta.extend_from_slice(&sorted_meta_values.older);
    for x in all_meta.iter() {
        let meta_key = MetaKey::from_parts(&x.name, x.version);
        metadata_batch.insert(meta_key.key(), &x.meta[..]);
    }
    TrDbHot::new()
        .set_metadata(metadata_batch)
        .apply(HOT_DB_NAME)
}
