//! Helpers
use parity_scale_codec::Encode;
use serde_json::{map::Map, value::Value};
use sled::Batch;
use sp_core::H256;
use std::{cmp::Ordering, convert::TryInto};

use constants::{
    add_specs, load_metadata, ADDRESS_BOOK, COLOR, EXPORT_FOLDER, HOT_DB_NAME, METATREE,
    META_HISTORY, SECONDARY_COLOR, SPECSTREEPREP,
};
use db_handling::{
    db_transactions::TrDbHot,
    helpers::{make_batch_clear_tree, open_db, open_tree},
};
use definitions::{
    crypto::Encryption,
    error::ErrorSource,
    error_active::{
        Active, Changed, DatabaseActive, ErrorActive, Fetch, IncomingMetadataSourceActiveStr,
        InputActive, MismatchActive, NotFoundActive, NotHexActive, SpecsError,
    },
    helpers::unhex,
    keyring::{AddressBookKey, MetaKey, NetworkSpecsKey},
    metadata::{AddressBookEntry, MetaHistoryEntry, MetaValues},
    network_specs::NetworkSpecsToSend,
    qr_transfers::{ContentAddSpecs, ContentLoadMeta},
};

use crate::fetch_metadata::{fetch_info, fetch_info_with_network_specs, fetch_meta_at_block};
use crate::interpret_specs::{check_specs, interpret_properties, TokenFetch};
use crate::parser::Token;

/// Get [`AddressBookEntry`] from the database for given address book title.
pub fn get_address_book_entry(title: &str) -> Result<AddressBookEntry, ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let address_book = open_tree::<Active>(&database, ADDRESS_BOOK)?;
    match address_book.get(AddressBookKey::from_title(title).key()) {
        Ok(Some(a)) => AddressBookEntry::from_entry_with_title(title, &a),
        Ok(None) => Err(ErrorActive::NotFound(NotFoundActive::AddressBookEntry {
            title: title.to_string(),
        })),
        Err(e) => Err(<Active>::db_internal(e)),
    }
}

/// Get [`NetworkSpecsToSend`] from the database for given address book title.
pub fn network_specs_from_title(title: &str) -> Result<NetworkSpecsToSend, ErrorActive> {
    network_specs_from_entry(&get_address_book_entry(title)?)
}

/// Get [`NetworkSpecsToSend`] corresponding to the given [`AddressBookEntry`].
///
/// Entries in [`ADDRESS_BOOK`] and [`SPECSTREEPREP`] trees for any network can
/// be added and removed only simultaneously.
// TODO consider combining those, key would be address book title, network specs
// key will stay only in cold database then?
pub fn network_specs_from_entry(
    address_book_entry: &AddressBookEntry,
) -> Result<NetworkSpecsToSend, ErrorActive> {
    let network_specs_key = NetworkSpecsKey::from_parts(
        &address_book_entry.genesis_hash,
        &address_book_entry.encryption,
    );
    let network_specs = get_network_specs_to_send(&network_specs_key)?;
    if network_specs.name != address_book_entry.name {
        return Err(ErrorActive::Database(DatabaseActive::Mismatch(
            MismatchActive::AddressBookSpecsName {
                address_book_name: address_book_entry.name.to_string(),
                specs_name: network_specs.name,
            },
        )));
    }
    Ok(network_specs)
}

/// Try to get network specs [`NetworkSpecsToSend`] from the hot database.
///
/// If the [`NetworkSpecsKey`] and associated [`NetworkSpecsToSend`] are not
/// found in the [`SPECSTREEPREP`], the result is `Ok(None)`.
pub fn try_get_network_specs_to_send(
    network_specs_key: &NetworkSpecsKey,
) -> Result<Option<NetworkSpecsToSend>, ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let chainspecs = open_tree::<Active>(&database, SPECSTREEPREP)?;
    match chainspecs.get(network_specs_key.key()) {
        Ok(Some(specs_encoded)) => Ok(Some(NetworkSpecsToSend::from_entry_with_key_checked(
            network_specs_key,
            specs_encoded,
        )?)),
        Ok(None) => Ok(None),
        Err(e) => Err(<Active>::db_internal(e)),
    }
}

/// Get network specs [`NetworkSpecsToSend`] from the hot database.
///
/// Network specs here are expected to be found, not finding them results in an
/// error.
pub fn get_network_specs_to_send(
    network_specs_key: &NetworkSpecsKey,
) -> Result<NetworkSpecsToSend, ErrorActive> {
    match try_get_network_specs_to_send(network_specs_key)? {
        Some(a) => Ok(a),
        None => Err(ErrorActive::NotFound(NotFoundActive::NetworkSpecsToSend(
            network_specs_key.to_owned(),
        ))),
    }
}

/// Update the database after `add_specs` run.
///
/// Inputs `&str` url address that was used for rpc calls and already completed
/// [`NetworkSpecsToSend`].
///
/// Adds simultaneously [`AddressBookEntry`] to [`ADDRESS_BOOK`] and
/// [`NetworkSpecsToSend`] to [`SPECSTREEPREP`].
///
/// Key for [`AddressBookEntry`] is the network address book title. It always
/// has format <network_name>-<network_encryption>.
pub fn db_upd_network(
    address: &str,
    network_specs: &NetworkSpecsToSend,
) -> Result<(), ErrorActive> {
    let mut network_specs_prep_batch = Batch::default();
    network_specs_prep_batch.insert(
        NetworkSpecsKey::from_parts(&network_specs.genesis_hash, &network_specs.encryption).key(),
        network_specs.encode(),
    );
    let address_book_new_key = AddressBookKey::from_title(&format!(
        "{}-{}",
        network_specs.name,
        network_specs.encryption.show()
    ));
    let address_book_new_entry_encoded = AddressBookEntry {
        name: network_specs.name.to_string(),
        genesis_hash: network_specs.genesis_hash,
        address: address.to_string(),
        encryption: network_specs.encryption.clone(),
        def: false,
    }
    .encode();
    let mut address_book_batch = Batch::default();
    address_book_batch.insert(address_book_new_key.key(), address_book_new_entry_encoded);
    TrDbHot::new()
        .set_address_book(address_book_batch)
        .set_network_specs_prep(network_specs_prep_batch)
        .apply(HOT_DB_NAME)
}

/// Process error depending on pass errors flag `-s`.
pub fn error_occured(e: ErrorActive, pass_errors: bool) -> Result<(), ErrorActive> {
    if pass_errors {
        println!("Error encountered. {} Skipping it.", e);
        Ok(())
    } else {
        Err(e)
    }
}

/// Content to print during `load_metadata <-k/-p/-t>` processing.
pub enum Write {
    /// all payloads, `-t` key or no setting key was used
    All,

    /// only new payloads, `-k` setting key was used
    OnlyNew,

    /// no payloads, `-p` setting key was used    
    None,
}

/// Get all [`ADDRESS_BOOK`] entries with address book titles.
pub fn address_book_content() -> Result<Vec<(String, AddressBookEntry)>, ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let address_book = open_tree::<Active>(&database, ADDRESS_BOOK)?;
    let mut out: Vec<(String, AddressBookEntry)> = Vec::new();
    for x in address_book.iter().flatten() {
        out.push(AddressBookEntry::process_entry(x)?)
    }
    Ok(out)
}

/// Get all [`ADDRESS_BOOK`] entries with address book titles, for given url
/// address.
pub fn filter_address_book_by_url(
    address: &str,
) -> Result<Vec<(String, AddressBookEntry)>, ErrorActive> {
    let mut out: Vec<(String, AddressBookEntry)> = Vec::new();
    let mut found_name = None;
    for (title, address_book_entry) in address_book_content()?.into_iter() {
        if address_book_entry.address == address {
            found_name = match found_name {
                Some(name) => {
                    if name == address_book_entry.name {
                        Some(name)
                    } else {
                        return Err(ErrorActive::Database(DatabaseActive::TwoNamesForUrl {
                            url: address.to_string(),
                        }));
                    }
                }
                None => Some(address_book_entry.name.to_string()),
            };
            out.push((title, address_book_entry))
        }
    }
    Ok(out)
}

/// Search for any [`ADDRESS_BOOK`] entry with given genesis hash.
pub fn genesis_hash_in_hot_db(genesis_hash: H256) -> Result<Option<AddressBookEntry>, ErrorActive> {
    let mut out = None;
    for (_, address_book_entry) in address_book_content()?.into_iter() {
        if address_book_entry.genesis_hash == genesis_hash {
            out = Some(address_book_entry);
            break;
        }
    }
    Ok(out)
}

/// Check if [`ADDRESS_BOOK`] has entries with given `name` and title other than
/// `except_title`.
pub fn is_specname_in_db(name: &str, except_title: &str) -> Result<bool, ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let address_book = open_tree::<Active>(&database, ADDRESS_BOOK)?;
    let mut out = false;
    for x in address_book.iter().flatten() {
        let (title, address_book_entry) = <AddressBookEntry>::process_entry(x)?;
        if (address_book_entry.name == name) && (title != except_title) {
            out = true;
            break;
        }
    }
    Ok(out)
}

/// Get all entries from `META_HISTORY`.
pub fn meta_history_content() -> Result<Vec<MetaHistoryEntry>, ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let meta_history = open_tree::<Active>(&database, META_HISTORY)?;
    let mut out: Vec<MetaHistoryEntry> = Vec::new();
    for x in meta_history.iter().flatten() {
        out.push(MetaHistoryEntry::from_entry(x)?)
    }
    Ok(out)
}

/// [`MetaValues`] with block hash at the time of fetch, if available.
///
/// Block hash may be missing if the metadata was extracted from `.wasm` file.
#[derive(Clone)]
pub struct MetaValuesStamped {
    pub meta_values: MetaValues,
    pub at_block_hash: Option<H256>,
}

/// Collect all [`MetaValuesStamped`] from the hot database.
pub fn read_metadata_database() -> Result<Vec<MetaValuesStamped>, ErrorActive> {
    let database = open_db::<Active>(HOT_DB_NAME)?;
    let metadata = open_tree::<Active>(&database, METATREE)?;
    let meta_history = open_tree::<Active>(&database, META_HISTORY)?;
    let mut out: Vec<MetaValuesStamped> = Vec::new();
    for x in metadata.iter().flatten() {
        let meta_values = MetaValues::from_entry_checked::<Active>(x)?;
        let meta_key = MetaKey::from_parts(&meta_values.name, meta_values.version);
        let at_block_hash = match meta_history
            .get(meta_key.key())
            .map_err(<Active>::db_internal)?
        {
            Some(meta_history_entry_encoded) => Some(
                MetaHistoryEntry::from_entry_with_key_parts(
                    &meta_values.name,
                    meta_values.version,
                    &meta_history_entry_encoded,
                )?
                .block_hash,
            ),
            None => None,
        };
        out.push(MetaValuesStamped {
            meta_values,
            at_block_hash,
        })
    }
    Ok(out)
}

/// Sorted [`MetaValuesStamped`] from the hot database.
pub struct SortedMetaValues {
    /// Set of the metadata entries with latest version known to the database.
    pub newer: Vec<MetaValuesStamped>,

    /// Other metadata entries. Since there are maximum two entries allowed,
    /// this set contains at most one entry for each network.
    pub older: Vec<MetaValuesStamped>,
}

/// Sort [`MetaValuesStamped`] into sets of newer and older, by metadata
/// version.
///
/// Database contains maximum two metadata entries for each network name, both
/// newer and older sets can contain at most one metadata [`MetaValuesStamped`].
fn sort_metavalues(meta_values: Vec<MetaValuesStamped>) -> Result<SortedMetaValues, ErrorActive> {
    // newer metadata set, i.e. with higher version for given network
    let mut newer: Vec<MetaValuesStamped> = Vec::new();

    // older metadata set
    let mut older: Vec<MetaValuesStamped> = Vec::new();

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
            if x.meta_values.name == y.meta_values.name {
                // search for the network name in already collected elements of
                // `older` set; should not find any;
                for z in older.iter() {
                    if x.meta_values.name == z.meta_values.name {
                        return Err(ErrorActive::Database(
                            DatabaseActive::HotDatabaseMetadataOverTwoEntries {
                                name: x.meta_values.name.to_string(),
                            },
                        ));
                    }
                }

                found_in_new = true;

                // where the entry goes, based on the version
                match x.meta_values.version.cmp(&y.meta_values.version) {
                    // `x` entry goes to `older`
                    Ordering::Less => older.push(x.to_owned()),

                    // same version?!
                    Ordering::Equal => {
                        return Err(ErrorActive::Database(
                            DatabaseActive::HotDatabaseMetadataSameVersionTwice {
                                name: x.meta_values.name.to_string(),
                                version: x.meta_values.version,
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

/// Try updating [`SortedMetaValues`] with new [`MetaValuesStamped`].
///
/// Outputs flag to indicate that the [`SortedMetaValues`] got updated.
///
/// If the fetched metadata is good and has later version than the ones in
/// [`SortedMetaValues`], it is added to `newer` set, any previous value from
/// `newer` is moved to `older`. If there was any value in `older`, it gets
/// kicked out.
///
/// If there was no block hash in hot database and the metadata did not change,
/// a new block hash could be added if it is known.
pub fn add_new(
    new: &MetaValuesStamped,
    sorted: &mut SortedMetaValues,
) -> Result<bool, ErrorActive> {
    // action to perform after sorting on found entry
    enum Found {
        DoNothing,
        Replace {
            move_from_newer: usize,
            remove_from_older: Option<usize>,
        },
        UpdateBlock {
            in_newer: usize,
        },
    }

    let mut similar_entries: Option<Found> = None;

    // search for entry with same name through `newer` existing entries
    for (i, x) in sorted.newer.iter().enumerate() {
        if new.meta_values.name == x.meta_values.name {
            similar_entries = match new.meta_values.version.cmp(&x.meta_values.version) {
                // earlier metadata should not be fetched through rpc call;
                //
                // version downgrades happened, but these should always be
                // double checked before being accepted;
                //
                // earlier metadata could be retrieved from an outdated `.wasm`
                // file - no reason to accept it either;
                Ordering::Less => {
                    return Err(ErrorActive::Fetch(Fetch::EarlierVersion {
                        name: x.meta_values.name.to_string(),
                        old_version: x.meta_values.version,
                        new_version: new.meta_values.version,
                    }))
                }

                // same version, no updates;
                //
                // check that metadata is exactly the same, different metadata
                // under same version is an error;
                Ordering::Equal => {
                    if new.meta_values.meta != x.meta_values.meta {
                        // metadata comparing, hopefully never to be needed
                        // again
                        //
                        // prints the difference for user to check
                        let mut sus1: Vec<u8> = Vec::new();
                        let mut sus2: Vec<u8> = Vec::new();
                        for a in 0..x.meta_values.meta.len() {
                            if new.meta_values.meta[a] != x.meta_values.meta[a] {
                                println!("Suspicious number {}", a);
                                sus1.push(new.meta_values.meta[a]);
                                sus2.push(x.meta_values.meta[a]);
                            }
                        }
                        println!("new: {:?}, in db: {:?}", sus1, sus2);

                        return Err(ErrorActive::Fetch(Fetch::SameVersionDifferentMetadata {
                            name: new.meta_values.name.to_string(),
                            version: new.meta_values.version,
                            block_hash_in_db: x.at_block_hash,
                            block_hash_in_fetch: new.at_block_hash,
                        }));
                    }
                    match x.at_block_hash {
                        Some(_) => Some(Found::DoNothing),
                        None => Some(Found::UpdateBlock { in_newer: i }),
                    }
                }

                // fetched newer metadata
                Ordering::Greater => {
                    let mut remove_from_older = None;

                    // check if there is entry in `older` to be kicked
                    // altogether
                    for (j, y) in sorted.older.iter().enumerate() {
                        if x.meta_values.name == y.meta_values.name {
                            // found entry in `older` to be removed
                            remove_from_older = Some(j);
                            break;
                        }
                    }
                    Some(Found::Replace {
                        move_from_newer: i,
                        remove_from_older,
                    })
                }
            };
            break;
        }
    }

    match similar_entries {
        Some(Found::DoNothing) => Ok(false),
        Some(Found::Replace {
            move_from_newer,
            remove_from_older,
        }) => {
            if let Some(j) = remove_from_older {
                sorted.older.remove(j);
            }
            sorted.older.push(sorted.newer.remove(move_from_newer));
            sorted.newer.push(new.to_owned());
            Ok(true)
        }
        Some(Found::UpdateBlock { in_newer }) => {
            sorted.newer[in_newer].at_block_hash = new.at_block_hash;
            Ok(false)
        }
        None => {
            sorted.newer.push(new.to_owned());
            Ok(true)
        }
    }
}

/// Collect and sort [`MetaValuesStamped`] from the hot database
pub fn prepare_metadata() -> Result<SortedMetaValues, ErrorActive> {
    let known_metavalues = read_metadata_database()?;
    sort_metavalues(known_metavalues)
}

/// Update the database after `load_metadata` run.
///
/// Clear [`METATREE`] tree of the hot database and write new metadata set in
/// it.
///
/// Update [`META_HISTORY`] tree.
pub fn db_upd_metadata(sorted_meta_values: SortedMetaValues) -> Result<(), ErrorActive> {
    let mut metadata_batch = make_batch_clear_tree::<Active>(HOT_DB_NAME, METATREE)?;
    let mut meta_history_batch = Batch::default();
    let mut all_meta = sorted_meta_values.newer;
    all_meta.extend_from_slice(&sorted_meta_values.older);
    for x in all_meta.iter() {
        let meta_key = MetaKey::from_parts(&x.meta_values.name, x.meta_values.version);
        metadata_batch.insert(meta_key.key(), &x.meta_values.meta[..]);
        if let Some(hash) = x.at_block_hash {
            meta_history_batch.insert(meta_key.key(), hash.encode());
        }
    }
    TrDbHot::new()
        .set_metadata(metadata_batch)
        .set_meta_history(meta_history_batch)
        .apply(HOT_DB_NAME)
}

/// Data needed to output `load_metadata` update payload file.
pub struct MetaShortCut {
    pub meta_values: MetaValues,
    pub genesis_hash: H256,
}

/// Fetched and interpreted data for `load_metadata` payload and database
/// update.
pub struct MetaFetched {
    pub meta_values: MetaValues,
    pub block_hash: H256,
    pub genesis_hash: H256,
}

impl MetaFetched {
    pub fn stamped(&self) -> MetaValuesStamped {
        MetaValuesStamped {
            meta_values: self.meta_values.to_owned(),
            at_block_hash: Some(self.block_hash),
        }
    }
    pub fn cut(&self) -> MetaShortCut {
        MetaShortCut {
            meta_values: self.meta_values.to_owned(),
            genesis_hash: self.genesis_hash,
        }
    }
}

/// Get network information through rpc calls at `address` and interpret it into
/// [`MetaFetched`].
pub fn meta_fetch(address: &str) -> Result<MetaFetched, ErrorActive> {
    let new_info = fetch_info(address).map_err(|e| {
        ErrorActive::Fetch(Fetch::Failed {
            url: address.to_string(),
            error: e.to_string(),
        })
    })?;

    let genesis_hash = get_hash(
        &new_info.genesis_hash,
        Hash::Genesis {
            url: address.to_string(),
        },
    )?;
    let block_hash = get_hash(
        &new_info.block_hash,
        Hash::BlockFetched {
            url: address.to_string(),
        },
    )?;
    let meta_values = MetaValues::from_str_metadata(
        &new_info.meta,
        IncomingMetadataSourceActiveStr::Fetch {
            url: address.to_string(),
            optional_block: None,
        },
    )?;
    Ok(MetaFetched {
        meta_values,
        block_hash,
        genesis_hash,
    })
}

/// Get network metadata file from given url address at specified block.
///
/// For investigating silent metadata update cases.
///
/// Inputs `&str` address and hexadecimal `&str` block hash.
///
/// Fetched network metadata, processes it, and outputs file
/// `<network_name><metadata_version>_<block_hash>` with hexadecimal
/// metadata in [`EXPORT_FOLDER`].
///
/// Command line to get metadata at block:
///
/// `meta_at_block -u <network_url_address> -block <block_hash>`
pub fn debug_meta_at_block(address: &str, hex_block_hash: &str) -> Result<(), ErrorActive> {
    let block_hash = get_hash(hex_block_hash, Hash::BlockEntered)?;
    let meta_fetched = fetch_meta_at_block(address, block_hash).map_err(|e| {
        ErrorActive::Fetch(Fetch::Failed {
            url: address.to_string(),
            error: e.to_string(),
        })
    })?;
    let meta_values = MetaValues::from_str_metadata(
        &meta_fetched,
        IncomingMetadataSourceActiveStr::Fetch {
            url: address.to_string(),
            optional_block: Some(block_hash),
        },
    )?;
    let filename = format!(
        "{}/{}{}_{}",
        EXPORT_FOLDER,
        meta_values.name,
        meta_values.version,
        hex::encode(block_hash)
    );
    match std::fs::write(&filename, hex::encode(meta_values.meta)) {
        Ok(_) => Ok(()),
        Err(e) => Err(ErrorActive::Output(e)),
    }
}

/// Prepare [`NetworkSpecsToSend`] with only url address and user-entered data.
///
/// Database is not addressed. For `-d` content key.
pub fn specs_agnostic(
    address: &str,
    encryption: Encryption,
    optional_token_override: Option<Token>,
    optional_signer_title_override: Option<String>,
) -> Result<NetworkSpecsToSend, ErrorActive> {
    let fetch = common_specs_fetch(address)?;

    // `NetworkProperties` checked and processed
    let new_properties = interpret_properties(
        &fetch.properties,
        fetch.meta_values.optional_base58prefix,
        optional_token_override,
    )
    .map_err(|error| {
        ErrorActive::Fetch(Fetch::FaultySpecs {
            url: address.to_string(),
            error,
        })
    })?;

    let title = optional_signer_title_override.unwrap_or(format!(
        "{}-{}",
        fetch.meta_values.name,
        encryption.show()
    ));

    // `NetworkSpecsToSend` is constructed with fetched and user-entered values
    // and with default colors.
    Ok(NetworkSpecsToSend {
        base58prefix: new_properties.base58prefix,
        color: COLOR.to_string(),
        decimals: new_properties.decimals,
        encryption,
        genesis_hash: fetch.genesis_hash,
        logo: fetch.meta_values.name.to_string(),
        name: fetch.meta_values.name.to_string(),
        path_id: format!("//{}", fetch.meta_values.name),
        secondary_color: SECONDARY_COLOR.to_string(),
        title,
        unit: new_properties.unit,
    })
}

/// Update [`NetworkSpecsToSend`] already existing in the database with exactly
/// same encryption.
///
/// Could be used to overwrite token (if possible for the network) or the Signer
/// display title.
///
/// Output flag indicates if the value has changed, and a new database entry
/// should be created.
pub fn update_known_specs(
    address: &str,
    specs: &mut NetworkSpecsToSend,
    optional_signer_title_override: Option<String>,
    optional_token_override: Option<Token>,
) -> Result<bool, ErrorActive> {
    let mut update_done = common_specs_processing(address, specs, optional_token_override)?;

    if let Some(title) = optional_signer_title_override {
        if specs.title != title {
            specs.title = title;
            update_done = true;
        }
    }
    Ok(update_done)
}

/// Make [`NetworkSpecsToSend`] from the known database with different
/// [`Encryption`].
///
/// Requires new `encryption`.
///
/// Accepts changes in token (if possible for the network) or the Signer
/// display title.
///
/// A new database entry is created in any case.
pub fn update_modify_encryption_specs(
    address: &str,
    specs: &mut NetworkSpecsToSend,
    encryption: &Encryption,
    optional_signer_title_override: Option<String>,
    optional_token_override: Option<Token>,
) -> Result<(), ErrorActive> {
    common_specs_processing(address, specs, optional_token_override)?;

    specs.title =
        optional_signer_title_override.unwrap_or(format!("{}-{}", specs.name, encryption.show()));

    specs.encryption = encryption.to_owned();

    Ok(())
}

/// Fetched network information: metadata and genesis hash are interpreted,
/// properties remain raw.
struct CommonSpecsFetch {
    genesis_hash: H256,
    meta_values: MetaValues,
    properties: Map<String, Value>,
}

/// Fetch network information for `add_specs` update payload, process metadata
/// and genesis hash.
///
/// Properties processing depends on what is done to the fetch results and what
/// is in the database.
fn common_specs_fetch(address: &str) -> Result<CommonSpecsFetch, ErrorActive> {
    // actual fetch
    let new_info = fetch_info_with_network_specs(address).map_err(|e| {
        ErrorActive::Fetch(Fetch::Failed {
            url: address.to_string(),
            error: e.to_string(),
        })
    })?;

    // genesis hash in proper format
    let genesis_hash = get_hash(
        &new_info.genesis_hash,
        Hash::Genesis {
            url: address.to_string(),
        },
    )?;

    // `MetaValues` are needed to get network name and (optionally) base58
    // prefix
    let meta_values = MetaValues::from_str_metadata(
        &new_info.meta,
        IncomingMetadataSourceActiveStr::Fetch {
            url: address.to_string(),
            optional_block: None,
        },
    )?;

    Ok(CommonSpecsFetch {
        genesis_hash,
        meta_values,
        properties: new_info.properties,
    })
}

/// Check and modify [`NetworkSpecsToSend`] entry with the network information
/// fetched and user token override.
///
/// This is a helper function for `add_specs` runs with `-n` reference key, i.e.
/// for cases when *some* network specs entry already exists in the database.
///
/// Input [`NetworkSpecsToSend`] may be the entry from the database or the
/// entry from the database modified with encryption override.
///
/// Function inputs `address` at which rpc calls are made, network specs
/// `NetworkSpecsToSend` from the database, and user-entered optional
/// override for `Token`.
///
/// Output is flag indicating if the network specs have been changed.
fn common_specs_processing(
    address: &str,
    specs: &mut NetworkSpecsToSend,
    optional_token_override: Option<Token>,
) -> Result<bool, ErrorActive> {
    let mut update_done = false;
    let url = address.to_string();

    let fetch = common_specs_fetch(address)?;

    let (base58prefix, token_fetch) =
        check_specs(&fetch.properties, fetch.meta_values.optional_base58prefix).map_err(
            |error| {
                ErrorActive::Fetch(Fetch::FaultySpecs {
                    url: address.to_string(),
                    error,
                })
            },
        )?;

    // check that base58 prefix did not change
    if specs.base58prefix != base58prefix {
        return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
            url,
            what: Changed::Base58Prefix {
                old: specs.base58prefix,
                new: base58prefix,
            },
        }));
    }

    // check that genesis hash did not change
    if specs.genesis_hash != fetch.genesis_hash {
        return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
            url,
            what: Changed::GenesisHash {
                old: specs.genesis_hash,
                new: fetch.genesis_hash,
            },
        }));
    }

    // check that name did not change
    if specs.name != fetch.meta_values.name {
        return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
            url,
            what: Changed::Name {
                old: specs.name.to_string(),
                new: fetch.meta_values.name,
            },
        }));
    }

    // check that token did not change or could be overridden
    match token_fetch {
        // single token, no override was or is possible, must match
        TokenFetch::Single(token) => {
            // check that decimals value did not change
            if specs.decimals != token.decimals {
                return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
                    url,
                    what: Changed::Decimals {
                        old: specs.decimals,
                        new: token.decimals,
                    },
                }));
            }

            // check that unit did not change
            if specs.unit != token.unit {
                return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
                    url,
                    what: Changed::Unit {
                        old: specs.unit.to_string(),
                        new: token.unit,
                    },
                }));
            }

            // override is not allowed
            if optional_token_override.is_some() {
                return Err(ErrorActive::Fetch(Fetch::FaultySpecs {
                    url,
                    error: SpecsError::OverrideIgnoredSingle,
                }));
            }
        }
        TokenFetch::Array { .. } => {
            // override is allowed
            if let Some(token) = optional_token_override {
                if specs.decimals != token.decimals {
                    specs.decimals = token.decimals;
                    update_done = true;
                }
                if specs.unit != token.unit {
                    specs.unit = token.unit;
                    update_done = true;
                }
            }
        }
        TokenFetch::None => {
            // only decimals `0` possible, check that decimals value did not
            // change
            if specs.decimals != 0 {
                return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
                    url,
                    what: Changed::DecimalsBecameNone {
                        old: specs.decimals,
                    },
                }));
            }

            // only unit `UNIT` possible, check that unit did not change
            if specs.unit != "UNIT" {
                return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
                    url,
                    what: Changed::UnitBecameNone {
                        old: specs.unit.to_string(),
                    },
                }));
            }

            // override is not allowed
            if optional_token_override.is_some() {
                return Err(ErrorActive::Fetch(Fetch::FaultySpecs {
                    url,
                    error: SpecsError::OverrideIgnoredNone,
                }));
            }
        }
    }

    Ok(update_done)
}

/// The kind of hash processed. Determines the error.
enum Hash {
    BlockEntered,
    BlockFetched { url: String },
    Genesis { url: String },
}

/// Transform hash from fetched hexadecimal string into `H256` format
///
/// Inputs url `address` from which the data was fetched, hexadecimal
/// `fetched_hash` and `Hash` used to produce correct error in case the format
/// is incorrect.
fn get_hash(fetched_hash: &str, what: Hash) -> Result<H256, ErrorActive> {
    let not_hex = match what {
        Hash::BlockEntered => NotHexActive::EnteredBlockHash,
        Hash::BlockFetched { ref url } => NotHexActive::FetchedBlockHash {
            url: url.to_string(),
        },
        Hash::Genesis { ref url } => NotHexActive::FetchedGenesisHash {
            url: url.to_string(),
        },
    };
    let hash_vec = unhex::<Active>(fetched_hash, not_hex)?;
    let out: [u8; 32] = match hash_vec.try_into() {
        Ok(a) => a,
        Err(_) => match what {
            Hash::BlockEntered => return Err(ErrorActive::Input(InputActive::BlockHashLength)),
            Hash::BlockFetched { url: _ } => {
                return Err(ErrorActive::Fetch(
                    Fetch::UnexpectedFetchedBlockHashFormat {
                        value: fetched_hash.to_string(),
                    },
                ))
            }
            Hash::Genesis { url: _ } => {
                return Err(ErrorActive::Fetch(
                    Fetch::UnexpectedFetchedGenesisHashFormat {
                        value: fetched_hash.to_string(),
                    },
                ))
            }
        },
    };
    Ok(out.into())
}

/// Write to file raw bytes payload of `load_metadata` update
///
/// Resulting file, located in dedicated [`FOLDER`](constants::FOLDER), could be
/// used to generate data signature and to produce updates.
pub fn load_meta_print(shortcut: &MetaShortCut) -> Result<(), ErrorActive> {
    let filename = format!(
        "{}_{}V{}",
        load_metadata(),
        shortcut.meta_values.name,
        shortcut.meta_values.version
    );
    let content = ContentLoadMeta::generate(&shortcut.meta_values.meta, &shortcut.genesis_hash);
    content.write(&filename)
}

/// Write to file raw bytes payload of `add_specs` update
///
/// Resulting file, located in dedicated [`FOLDER`](constants::FOLDER), could be
/// used to generate data signature and to produce updates.
pub fn print_specs(network_specs: &NetworkSpecsToSend) -> Result<(), ErrorActive> {
    let filename = format!(
        "{}_{}_{}",
        add_specs(),
        network_specs.name,
        network_specs.encryption.show()
    );
    let content = ContentAddSpecs::generate(network_specs);
    content.write(&filename)
}
