//! Helpers
use db_handling::identities::{
    AddrInfo, ExportAddrs, ExportAddrsV2, SeedInfo, TransactionBulk, TransactionBulkV1,
};
use parity_scale_codec::Encode;
use qrcode_rtx::transform_into_qr_apng;
use serde_json::{map::Map, value::Value};
use sled::Batch;
use sp_core::H256;
use std::path::Path;
use std::{cmp::Ordering, convert::TryInto};

use constants::{ADDRESS_BOOK, COLOR, METATREE, META_HISTORY, SECONDARY_COLOR, SPECSTREEPREP};
use db_handling::{
    db_transactions::TrDbHot,
    helpers::{make_batch_clear_tree, open_tree},
};
use definitions::{
    crypto::Encryption,
    helpers::unhex,
    keyring::{AddressBookKey, MetaKey, NetworkSpecsKey},
    metadata::{AddressBookEntry, MetaHistoryEntry, MetaValues},
    network_specs::NetworkSpecs,
    qr_transfers::{ContentAddSpecs, ContentLoadMeta},
};

use crate::error::{Changed, Error, NotHexActive, Result, SpecsError};
use crate::fetch_metadata::{fetch_info, fetch_info_with_network_specs, fetch_meta_at_block};
use crate::interpret_specs::{check_specs, interpret_properties, TokenFetch};
use crate::parser::{Goal, Token};

/// Get [`AddressBookEntry`] from the database for given address book title.
pub fn get_address_book_entry(database: &sled::Db, title: &str) -> Result<AddressBookEntry> {
    let address_book = open_tree(database, ADDRESS_BOOK)?;
    match address_book.get(AddressBookKey::from_title(title).key())? {
        Some(a) => Ok(AddressBookEntry::from_entry_with_title(title, &a)?),
        None => Err(Error::NotFound(title.to_string())),
    }
}

/// Get [`NetworkSpecs`] from the database for given address book title.
pub fn network_specs_from_title(database: &sled::Db, title: &str) -> Result<NetworkSpecs> {
    network_specs_from_entry(database, &get_address_book_entry(database, title)?)
}

/// Get [`NetworkSpecs`] corresponding to the given [`AddressBookEntry`].
///
/// Entries in [`ADDRESS_BOOK`] and [`SPECSTREEPREP`] trees for any network can
/// be added and removed only simultaneously.
// TODO consider combining those, key would be address book title, network specs
// key will stay only in cold database then?
pub fn network_specs_from_entry(
    database: &sled::Db,
    address_book_entry: &AddressBookEntry,
) -> Result<NetworkSpecs> {
    let network_specs_key = NetworkSpecsKey::from_parts(
        &address_book_entry.genesis_hash,
        &address_book_entry.encryption,
    );
    let network_specs = get_network_specs_to_send(database, &network_specs_key)?;
    if network_specs.name != address_book_entry.name {
        return Err(Error::AddressBookSpecsName {
            address_book_name: address_book_entry.name.to_string(),
            specs_name: network_specs.name,
        });
    }
    Ok(network_specs)
}

/// Try to get network specs [`NetworkSpecs`] from the hot database.
///
/// If the [`NetworkSpecsKey`] and associated [`NetworkSpecs`] are not
/// found in the [`SPECSTREEPREP`], the result is `Ok(None)`.
pub fn try_get_network_specs_to_send(
    database: &sled::Db,
    network_specs_key: &NetworkSpecsKey,
) -> Result<Option<NetworkSpecs>> {
    let chainspecs = open_tree(database, SPECSTREEPREP)?;
    match chainspecs.get(network_specs_key.key())? {
        Some(specs_encoded) => Ok(Some(NetworkSpecs::from_entry_with_key_checked(
            network_specs_key,
            specs_encoded,
        )?)),
        None => Ok(None),
    }
}

/// Get network specs [`NetworkSpecs`] from the hot database.
///
/// Network specs here are expected to be found, not finding them results in an
/// error.
pub fn get_network_specs_to_send(
    database: &sled::Db,
    network_specs_key: &NetworkSpecsKey,
) -> Result<NetworkSpecs> {
    match try_get_network_specs_to_send(database, network_specs_key)? {
        Some(a) => Ok(a),
        None => Err(Error::NetworkSpecs(network_specs_key.to_owned())),
    }
}

/// Update the database after `add-specs` run.
///
/// Inputs `&str` URL address that was used for RPC calls and already completed
/// [`NetworkSpecs`].
///
/// Adds simultaneously [`AddressBookEntry`] to [`ADDRESS_BOOK`] and
/// [`NetworkSpecs`] to [`SPECSTREEPREP`].
///
/// Key for [`AddressBookEntry`] is the network address book title. It always
/// has format `<network_name>-<network_encryption>`.
pub fn db_upd_network(
    database: &sled::Db,
    address: &str,
    network_specs: &NetworkSpecs,
) -> Result<()> {
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
        encryption: network_specs.encryption,
        def: false,
    }
    .encode();
    let mut address_book_batch = Batch::default();
    address_book_batch.insert(address_book_new_key.key(), address_book_new_entry_encoded);
    TrDbHot::new()
        .set_address_book(address_book_batch)
        .set_network_specs_prep(network_specs_prep_batch)
        .apply(database)?;
    Ok(())
}

/// Process error depending on pass errors flag `-s`.
pub fn error_occured(e: Error, pass_errors: bool) -> Result<()> {
    if pass_errors {
        println!("Error encountered. {e} Skipping it.");
        Ok(())
    } else {
        Err(e)
    }
}

/// Content to print during `load-metadata<-k/-p/-t>` processing.
pub enum Write {
    /// all payloads, `-t` key or no setting key was used
    All,

    /// only new payloads, `-k` setting key was used
    OnlyNew,

    /// no payloads, `-p` setting key was used
    None,
}

/// Get all [`ADDRESS_BOOK`] entries with address book titles.
pub fn address_book_content(database: &sled::Db) -> Result<Vec<(String, AddressBookEntry)>> {
    let address_book = open_tree(database, ADDRESS_BOOK)?;
    let mut out: Vec<(String, AddressBookEntry)> = Vec::new();
    for x in address_book.iter().flatten() {
        out.push(AddressBookEntry::process_entry(x)?)
    }
    Ok(out)
}

/// Get all [`ADDRESS_BOOK`] entries with address book titles, for given URL
/// address.
pub fn filter_address_book_by_url(
    database: &sled::Db,
    address: &str,
) -> Result<Vec<(String, AddressBookEntry)>> {
    let mut out: Vec<(String, AddressBookEntry)> = Vec::new();
    let mut found_name = None;
    for (title, address_book_entry) in address_book_content(database)?.into_iter() {
        if address_book_entry.address == address {
            found_name = match found_name {
                Some(name) => {
                    if name == address_book_entry.name {
                        Some(name)
                    } else {
                        return Err(Error::TwoNamesForUrl {
                            url: address.to_string(),
                        });
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
pub fn genesis_hash_in_hot_db(
    database: &sled::Db,
    genesis_hash: H256,
) -> Result<Option<AddressBookEntry>> {
    let mut out = None;
    for (_, address_book_entry) in address_book_content(database)?.into_iter() {
        if address_book_entry.genesis_hash == genesis_hash {
            out = Some(address_book_entry);
            break;
        }
    }
    Ok(out)
}

/// Check if [`ADDRESS_BOOK`] has entries with given `name` and title other than
/// `except_title`.
pub fn is_specname_in_db(database: &sled::Db, name: &str, except_title: &str) -> Result<bool> {
    let address_book = open_tree(database, ADDRESS_BOOK)?;
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
pub fn meta_history_content(database: &sled::Db) -> Result<Vec<MetaHistoryEntry>> {
    let meta_history = open_tree(database, META_HISTORY)?;
    let mut out: Vec<MetaHistoryEntry> = Vec::new();
    for x in meta_history.iter().flatten() {
        out.push(MetaHistoryEntry::from_entry(x)?)
    }
    Ok(out)
}

/// [`MetaValues`] with corresponding block hash at the time of fetch, if
/// available.
///
/// Block hash may be missing if the metadata was extracted from `.wasm` file.
#[derive(Clone)]
pub struct MetaValuesStamped {
    pub meta_values: MetaValues,
    pub at_block_hash: Option<H256>,
}

/// Collect all [`MetaValuesStamped`] from the hot database.
pub fn read_metadata_database(database: &sled::Db) -> Result<Vec<MetaValuesStamped>> {
    let metadata = open_tree(database, METATREE)?;
    let meta_history = open_tree(database, META_HISTORY)?;
    let mut out: Vec<MetaValuesStamped> = Vec::new();
    for x in metadata.iter().flatten() {
        let meta_values = MetaValues::from_entry_checked(x)?;
        let meta_key = MetaKey::from_parts(&meta_values.name, meta_values.version);
        let at_block_hash = match meta_history.get(meta_key.key())? {
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

/// [`MetaValuesStamped`] sorted into sets of newer and older, by metadata
/// version.
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
fn sort_metavalues(meta_values: Vec<MetaValuesStamped>) -> Result<SortedMetaValues> {
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
                        return Err(Error::HotDatabaseMetadataOverTwoEntries {
                            name: x.meta_values.name.to_string(),
                        });
                    }
                }

                found_in_new = true;

                // where the entry goes, based on the version
                match x.meta_values.version.cmp(&y.meta_values.version) {
                    // `x` entry goes to `older`
                    Ordering::Less => older.push(x.to_owned()),

                    // same version?!
                    Ordering::Equal => {
                        return Err(Error::HotDatabaseMetadataSameVersionTwice {
                            name: x.meta_values.name.to_string(),
                            version: x.meta_values.version,
                        })
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
pub fn add_new_metadata(new: &MetaValuesStamped, sorted: &mut SortedMetaValues) -> Result<bool> {
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
                // earlier metadata should not be fetched through RPC call;
                //
                // version downgrades happened, but these should always be
                // double checked before being accepted;
                //
                // earlier metadata could be retrieved from an outdated `.wasm`
                // file - no reason to accept it either;
                Ordering::Less => {
                    return Err(Error::EarlierVersion {
                        name: x.meta_values.name.to_string(),
                        old_version: x.meta_values.version,
                        new_version: new.meta_values.version,
                    })
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
                                println!("Suspicious number {a}");
                                sus1.push(new.meta_values.meta[a]);
                                sus2.push(x.meta_values.meta[a]);
                            }
                        }
                        println!("new: {sus1:?}, in db: {sus2:?}");

                        return Err(Error::SameVersionDifferentMetadata {
                            name: new.meta_values.name.to_string(),
                            version: new.meta_values.version,
                            block_hash_in_db: x.at_block_hash,
                            block_hash_in_fetch: new.at_block_hash,
                        });
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
pub fn prepare_metadata(database: &sled::Db) -> Result<SortedMetaValues> {
    let known_metavalues = read_metadata_database(database)?;
    sort_metavalues(known_metavalues)
}

/// Update the database after `load-metadata` run.
///
/// Clear [`METATREE`] tree of the hot database and write new metadata set in
/// it.
///
/// Update [`META_HISTORY`] tree.
pub fn db_upd_metadata(database: &sled::Db, sorted_meta_values: SortedMetaValues) -> Result<()> {
    let mut metadata_batch = make_batch_clear_tree(database, METATREE)?;
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
        .apply(database)?;

    Ok(())
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

/// Get network information through RPC calls at `address` and interpret it into
/// [`MetaFetched`].
pub fn meta_fetch(address: &str) -> Result<MetaFetched> {
    let new_info = fetch_info(address)?;

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
    let meta_values = MetaValues::from_str_metadata(&new_info.meta)?;
    Ok(MetaFetched {
        meta_values,
        block_hash,
        genesis_hash,
    })
}

/// Get network metadata file from given URL address at specified block.
///
/// For investigating silent metadata update cases.
///
/// Inputs `&str` address and hexadecimal `&str` block hash.
///
/// Fetched network metadata, processes it, and outputs file
/// `<network_name><metadata_version>_<block_hash>` with hexadecimal
/// metadata in [`EXPORT_FOLDER`](constants::EXPORT_FOLDER).
///
/// Command line to get metadata at block:
///
/// `meta_at_block -u <network_url_address> -block <block_hash>`
pub fn debug_meta_at_block<P>(address: &str, hex_block_hash: &str, export_dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let block_hash = get_hash(hex_block_hash, Hash::BlockEntered)?;
    let meta_fetched = fetch_meta_at_block(address, block_hash)?;
    let meta_values = MetaValues::from_str_metadata(&meta_fetched)?;
    let filename = format!(
        "{}{}_{}",
        meta_values.name,
        meta_values.version,
        hex::encode(block_hash)
    );
    let f_path = export_dir.as_ref().join(filename);
    std::fs::write(f_path, hex::encode(meta_values.meta))?;

    Ok(())
}

pub fn generate_key_info_export_to_qr<P: AsRef<Path>>(
    output_name: P,
    chunk_size: u16,
    fps: u16,
    keys_num: usize,
) -> Result<()> {
    use sp_keyring::sr25519::Keyring;
    use sp_runtime::MultiSigner;

    let multisigner = MultiSigner::from(Keyring::Alice.public());
    let name = "a very long key name a very long key name".to_owned();

    let derived_keys: Vec<AddrInfo> = (0..keys_num)
        .map(|num| AddrInfo {
            address: "0xdeadbeefdeadbeefdeadbeef".to_string(),
            derivation_path: Some(format!("//this//is//a//path//{num}")),
            encryption: Encryption::Sr25519,
            genesis_hash: H256::default(),
        })
        .collect();

    let seed_info = SeedInfo {
        name,
        multisigner,
        derived_keys,
    };
    let export_addrs_v2 = ExportAddrsV2::new(seed_info);
    let export_addrs = ExportAddrs::V2(export_addrs_v2);

    let export_addrs_encoded = [&[0x53, 0xff, 0xde], export_addrs.encode().as_slice()].concat();

    generate_qr_code(&export_addrs_encoded, chunk_size, fps, output_name)
}

/// Generate Bulk Transaction Signing payload for testing
pub fn generate_bulk_transaction_qr<P: AsRef<Path>>(
    dst_file: P,
    tx_count: usize,
    chunk_size: u16,
    from: String,
    output_format: Goal,
) -> Result<()> {
    let encoded_transactions = (0..tx_count).map(|_| {

let line = "0102".to_string() + 
    &from +
"a80403004adb5312dbd3a1bc28610deb1fb631110bbcccb6e7fb97e18501509d5565ea760b00a014e3322615010000682400000e000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e2faa9938ec10c2627d0fd5d20214aba3bf281e94a6a8f50d19b4e0ce3b253d65e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        hex::decode(line).unwrap()
    })
    .collect();
    let v1_bulk = TransactionBulkV1 {
        encoded_transactions,
    };
    let bulk = TransactionBulk::V1(v1_bulk);

    let payload = [&[0x53, 0xff, 0x04], bulk.encode().as_slice()].concat();

    match output_format {
        Goal::Qr => generate_qr_code(&payload, chunk_size, 8, dst_file)?,
        Goal::Text => std::fs::write(dst_file, hex::encode(payload))?,
        Goal::Both => todo!(),
    };

    Ok(())
}

/// Generate with data into a specified file.
pub fn generate_qr_code<P: AsRef<Path>>(
    input: &[u8],
    chunk_size: u16,
    fps: u16,
    output_name: P,
) -> Result<()> {
    transform_into_qr_apng(input, chunk_size, fps, output_name).map_err(Error::Qr)
}

/// Fetch data and assemble [`NetworkSpecs`] with only URL address and
/// user-entered data.
///
/// Database is not addressed. For `-d` content key.
pub fn specs_agnostic(
    address: &str,
    encryption: Encryption,
    optional_token_override: Option<Token>,
    optional_signer_title_override: Option<String>,
) -> Result<NetworkSpecs> {
    let fetch = common_specs_fetch(address)?;

    // `NetworkProperties` checked and processed
    let new_properties = interpret_properties(
        &fetch.properties,
        fetch.meta_values.optional_base58prefix,
        optional_token_override,
    )?;

    let title = optional_signer_title_override.unwrap_or(format!(
        "{}-{}",
        fetch.meta_values.name,
        encryption.show()
    ));

    // `NetworkSpecs` is constructed with fetched and user-entered values
    // and with default colors.
    Ok(NetworkSpecs {
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

/// Update [`NetworkSpecs`] already existing in the database with
/// **exactly same** encryption.
///
/// Could be used to overwrite token (if possible for the network) or the Vault
/// display title. If no title override is used, the title remains as it was.
///
/// Output flag indicates if the value has changed, and the database entry
/// should be updated.
pub fn update_known_specs(
    address: &str,
    specs: &mut NetworkSpecs,
    optional_signer_title_override: Option<String>,
    optional_token_override: Option<Token>,
) -> Result<bool> {
    let mut update_done = known_specs_processing(address, specs, optional_token_override)?;

    if let Some(title) = optional_signer_title_override {
        if specs.title != title {
            specs.title = title;
            update_done = true;
        }
    }
    Ok(update_done)
}

/// Modify [`NetworkSpecs`] existing in the database **only** with
/// different encryption.
///
/// New data always will be added to the database unless errors occur.
///
/// Function inputs:
///
/// - `&str` address to make RPC calls
/// - `NetworkSpecs` as they were found in the database, to be modified
/// here
/// - new `Encryption` to apply to `encryption` and `title` (if no title
/// override was entered) fields of the `NetworkSpecs`
/// - optional title override
/// - optional token override
pub fn update_modify_encryption_specs(
    address: &str,
    specs: &mut NetworkSpecs,
    encryption: &Encryption,
    optional_signer_title_override: Option<String>,
    optional_token_override: Option<Token>,
) -> Result<()> {
    known_specs_processing(address, specs, optional_token_override)?;

    specs.title =
        optional_signer_title_override.unwrap_or(format!("{}-{}", specs.name, encryption.show()));

    encryption.clone_into(&mut specs.encryption);

    Ok(())
}

/// `add_specs` payload data, after processing similar for entirely new and
/// partially known data.
///
/// Metadata and genesis hash are interpreted, properties remain raw.
struct CommonSpecsFetch {
    genesis_hash: H256,
    meta_values: MetaValues,
    properties: Map<String, Value>,
}

/// Fetch `add_specs` update payload data, process metadata and genesis hash.
///
/// Processing of propertoes depends on what is done to the fetch results and
/// the contents of the database.
fn common_specs_fetch(address: &str) -> Result<CommonSpecsFetch> {
    // actual fetch
    let new_info = fetch_info_with_network_specs(address)?;

    // genesis hash in proper format
    let genesis_hash = get_hash(
        &new_info.genesis_hash,
        Hash::Genesis {
            url: address.to_string(),
        },
    )?;

    // `MetaValues` are needed to get network name and (optionally) base58
    // prefix
    let meta_values = MetaValues::from_str_metadata(&new_info.meta)?;

    Ok(CommonSpecsFetch {
        genesis_hash,
        meta_values,
        properties: new_info.properties,
    })
}

/// Check known [`NetworkSpecs`] with network data fetched and apply token
/// override.
///
/// This is a helper function for `add-specs` runs with `-n` reference key, i.e.
/// for cases when *some* network specs entry already exists in the database.
///
/// Input [`NetworkSpecs`] is the entry from the database to which the
/// encryption and title overrides could be applied.
///
/// Function inputs `address` at which RPC calls are made, network specs
/// `NetworkSpecs` from the database, and user-entered optional override
/// for `Token`.
///
/// Output is flag indicating if the network specs have been changed. This flag
/// would be needed only in case the encryption is not modified outside of this
/// function.
fn known_specs_processing(
    address: &str,
    specs: &mut NetworkSpecs,
    optional_token_override: Option<Token>,
) -> Result<bool> {
    let mut update_done = false;
    let url = address.to_string();

    let fetch = common_specs_fetch(address)?;

    let (base58prefix, token_fetch) =
        check_specs(&fetch.properties, fetch.meta_values.optional_base58prefix)?;

    // check that base58 prefix did not change
    if specs.base58prefix != base58prefix {
        return Err(Error::ValuesChanged {
            url,
            what: Changed::Base58Prefix {
                old: specs.base58prefix,
                new: base58prefix,
            },
        });
    }

    // check that genesis hash did not change
    if specs.genesis_hash != fetch.genesis_hash {
        return Err(Error::ValuesChanged {
            url,
            what: Changed::GenesisHash {
                old: specs.genesis_hash,
                new: fetch.genesis_hash,
            },
        });
    }

    // check that name did not change
    if specs.name != fetch.meta_values.name {
        return Err(Error::ValuesChanged {
            url,
            what: Changed::Name {
                old: specs.name.to_string(),
                new: fetch.meta_values.name,
            },
        });
    }

    // check that token did not change or could be overridden
    match token_fetch {
        // single token, no override was or is possible, must match
        TokenFetch::Single(token) => {
            // check that decimals value did not change
            if specs.decimals != token.decimals {
                return Err(Error::ValuesChanged {
                    url,
                    what: Changed::Decimals {
                        old: specs.decimals,
                        new: token.decimals,
                    },
                });
            }

            // check that unit did not change
            if specs.unit != token.unit {
                return Err(Error::ValuesChanged {
                    url,
                    what: Changed::Unit {
                        old: specs.unit.to_string(),
                        new: token.unit,
                    },
                });
            }

            // override is not allowed
            if optional_token_override.is_some() {
                return Err(Error::FaultySpecs {
                    url,
                    error: SpecsError::OverrideIgnoredSingle,
                });
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
                return Err(Error::ValuesChanged {
                    url,
                    what: Changed::DecimalsBecameNone {
                        old: specs.decimals,
                    },
                });
            }

            // only unit `UNIT` possible, check that unit did not change
            if specs.unit != "UNIT" {
                return Err(Error::ValuesChanged {
                    url,
                    what: Changed::UnitBecameNone {
                        old: specs.unit.to_string(),
                    },
                });
            }

            // override is not allowed
            if optional_token_override.is_some() {
                return Err(Error::FaultySpecs {
                    url,
                    error: SpecsError::OverrideIgnoredNone,
                });
            }
        }
    }

    Ok(update_done)
}

/// The type of hash processed. Determines the error.
enum Hash {
    BlockEntered,
    BlockFetched { url: String },
    Genesis { url: String },
}

/// Transform hash from hexadecimal string into `H256` format
///
/// Inputs hexadecimal `input_hash` and hash type/source `Hash` (used to
/// produce error in case the `input_hash` format is incorrect).
fn get_hash(input_hash: &str, what: Hash) -> Result<H256> {
    let _not_hex = match what {
        Hash::BlockEntered => NotHexActive::EnteredBlock,
        Hash::BlockFetched { ref url } => NotHexActive::FetchedBlock {
            url: url.to_string(),
        },
        Hash::Genesis { ref url } => NotHexActive::FetchedGenesis {
            url: url.to_string(),
        },
    };
    let hash_vec = unhex(input_hash)?;
    let out: [u8; 32] = match hash_vec.try_into() {
        Ok(a) => a,
        Err(_) => {
            return match what {
                Hash::BlockEntered => Err(Error::BlockHashLength),
                Hash::BlockFetched { url: _ } => Err(Error::UnexpectedFetchedBlockHashFormat {
                    value: input_hash.to_string(),
                }),
                Hash::Genesis { url: _ } => Err(Error::UnexpectedFetchedGenesisHashFormat {
                    value: input_hash.to_string(),
                }),
            }
        }
    };
    Ok(out.into())
}

/// Write to file `load_metadata` update payload as raw bytes.
///
/// Resulting file, located in dedicated [`FOLDER`](constants::FOLDER), could be
/// used to generate data signature and to produce updates.
pub fn load_metadata_print<P>(shortcut: &MetaShortCut, files_dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let file_name = format!(
        "sign_me_load_metadata_{}V{}",
        shortcut.meta_values.name, shortcut.meta_values.version
    );
    let file_path = files_dir.as_ref().join(file_name);
    let content = ContentLoadMeta::generate(&shortcut.meta_values.meta, &shortcut.genesis_hash);
    content.write(file_path)?;
    Ok(())
}

/// Write to file `add_specs` update payload as raw bytes.
///
/// Resulting file, located in dedicated directory (by default, [`FOLDER`](constants::FOLDER)), could be
/// used to generate data signature and to produce updates.
pub fn add_specs_print<P>(network_specs: &NetworkSpecs, files_dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let file_name = format!(
        "sign_me_add_specs_{}_{}",
        network_specs.name,
        network_specs.encryption.show()
    );
    let file_path = files_dir.as_ref().join(file_name);
    let content = ContentAddSpecs::generate(network_specs);
    content.write(file_path)?;
    Ok(())
}
