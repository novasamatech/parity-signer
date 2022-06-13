//! Utils to display the database content and to verify default metadata files
use blake2_rfc::blake2b::blake2b;

use constants::HOT_DB_NAME;
use db_handling::helpers::try_get_meta_values_by_name_version;
use definitions::{
    error_active::{Active, Check, ErrorActive, IncomingMetadataSourceActiveStr},
    metadata::{AddressBookEntry, MetaValues},
    network_specs::NetworkSpecsToSend,
};

use crate::helpers::{
    address_book_content, meta_history_content, network_specs_from_entry, network_specs_from_title,
    read_metadata_database,
};

/// Display all metadata currenty stored in the hot database
///
/// Function prints for each entry in hot database
/// [`METATREE`](constants::METATREE) tree:
///
/// - network name
/// - network version
/// - hexadecimal metadata hash
///
/// It could be called by:
///
/// `$ cargo run show -metadata`
///
/// When generated, hot database has no metadata entries. All entries are
/// expected to appear as a result of rpc calls. Network name and version
/// combination is unique identifier of the metadata. Hashes could be used to
/// compare metadata contents.
pub fn show_metadata() -> Result<(), ErrorActive> {
    let meta_values_stamped_set = read_metadata_database()?;
    if meta_values_stamped_set.is_empty() {
        println!("Database has no metadata entries.");
        return Ok(());
    }
    println!("Database has metadata information for following networks:\n");
    for x in meta_values_stamped_set.iter() {
        let block_hash_insert = match x.at_block_hash {
            Some(h) => format!("fetched at block hash {}", hex::encode(h)),
            None => String::from("no block hash on record"),
        };
        println!(
            "{} {}, metadata hash {}, {}",
            x.meta_values.name,
            x.meta_values.version,
            hash_string(&x.meta_values.meta),
            block_hash_insert
        );
    }
    Ok(())
}

/// Show current state of the hot database address book
///
/// Function prints for each entry in hot database
/// [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) tree:
///
/// - address book title for the network, used only to distinguish between
/// address book entries
/// - url address at which rpc calls are made for the network
/// - network encryption
/// - additional marker that the network is a default one
/// - network title as it will be displayed in Signer, from
/// [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend)
///
/// It could be called by:
///
/// `$ cargo run show -networks`
///
/// When generated, hot database has address book entries for Polkadot, Kusama,
/// and Westend. Other entries appear as a result of the database usage.
/// Address book title is `<network name>` for default network and
/// `<network name>-<encryption>` for non-default networks.
pub fn show_networks() -> Result<(), ErrorActive> {
    let address_book_set = address_book_content()?;
    if address_book_set.is_empty() {
        println!("Address book is empty.");
        return Ok(());
    }
    struct SetPart {
        title: String,
        address_book_entry: AddressBookEntry,
        network_specs: NetworkSpecsToSend,
    }
    let mut set: Vec<SetPart> = Vec::new();
    for (title, address_book_entry) in address_book_set.into_iter() {
        let network_specs = network_specs_from_entry(&address_book_entry)?;
        set.push(SetPart {
            title,
            address_book_entry,
            network_specs,
        })
    }
    println!("Address book has entries for following networks:\n");
    for x in set.iter() {
        if x.address_book_entry.def {
            println!(
                "{} at {}, encryption {} (default), Signer display title {}",
                x.title,
                x.address_book_entry.address,
                x.address_book_entry.encryption.show(),
                x.network_specs.title,
            );
        } else {
            println!(
                "{} at {}, encryption {}, Signer display title {}",
                x.title,
                x.address_book_entry.address,
                x.address_book_entry.encryption.show(),
                x.network_specs.title,
            );
        }
    }
    Ok(())
}

/// Check metadata file
///
/// Function asserts that:
///
/// - the file contains valid metadata, with retrievable network name and
/// version
/// - if the metadata for same network name and version is in the hot database,
/// it completely matches the one from the file
///
/// Function could be used to check release metadata files in `defaults` crate.
pub fn check_file(path: String) -> Result<(), ErrorActive> {
    let meta_str = match std::fs::read_to_string(&path) {
        Ok(a) => a,
        Err(e) => {
            return Err(ErrorActive::Check {
                filename: path,
                check: Check::MetadataFile(e),
            })
        }
    };

    // `MetaValues` from metadata in file
    let from_file = MetaValues::from_str_metadata(
        meta_str.trim(),
        IncomingMetadataSourceActiveStr::Check { filename: path },
    )?;

    match try_get_meta_values_by_name_version::<Active>(
        HOT_DB_NAME,
        &from_file.name,
        from_file.version,
    )? {
        // network metadata for same network name and version is in the database
        Some(from_database) => {
            if from_database.meta == from_file.meta {
                // metadata matches
                println!(
                    "{}{}, metadata hash {}, in the database",
                    from_file.name,
                    from_file.version,
                    hash_string(&from_file.meta)
                )
            } else {
                // metadata does not match, this could indicate a serious issue
                println!(
                    "{}{}, metadata hash {}, same version metadata in the database has different hash {}",
                    from_file.name,
                    from_file.version,
                    hash_string(&from_file.meta),
                    hash_string(&from_database.meta)
                )
            }
        }

        // network metadata is not in the database
        None => {
            println!(
                "{}{}, metadata hash {}, not in the database",
                from_file.name,
                from_file.version,
                hash_string(&from_file.meta)
            )
        }
    }
    Ok(())
}

/// Hash metadata and produce hash hexadecimal string
fn hash_string(meta: &[u8]) -> String {
    hex::encode(blake2b(32, &[], meta).as_bytes())
}

/// Show network specs for user-entered address book title
pub fn show_specs(title: String) -> Result<(), ErrorActive> {
    let specs = network_specs_from_title(&title)?;
    println!(
        "address book title: {}\nbase58 prefix: {}\ncolor: {}\ndecimals: {}\nencryption: {}\ngenesis_hash: {}\nlogo: {}\nname: {}\npath_id: {}\nsecondary_color: {}\ntitle: {}\nunit: {}",
        title,
        specs.base58prefix,
        specs.color,
        specs.decimals,
        specs.encryption.show(),
        hex::encode(specs.genesis_hash),
        specs.logo,
        specs.name,
        specs.path_id,
        specs.secondary_color,
        specs.title,
        specs.unit
    );
    Ok(())
}

/// Show metadata fetch history from `META_HISTORY` tree
pub fn show_block_history() -> Result<(), ErrorActive> {
    let meta_history_set = meta_history_content()?;
    if meta_history_set.is_empty() {
        println!("Database has no metadata fetch history entries on record.");
        return Ok(());
    }
    println!("Database has following metadata fetch history:\n");
    for x in meta_history_set.iter() {
        println!(
            "{} {}, fetched at block {}",
            x.name,
            x.version,
            hex::encode(x.block_hash),
        );
    }
    Ok(())
}
