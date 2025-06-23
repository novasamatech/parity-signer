//! Common helper functions for database operations
use definitions::crypto::Encryption;
use parity_scale_codec::Decode;
#[cfg(feature = "active")]
use parity_scale_codec::Encode;
use sled::{Batch, Db, Tree};
use sp_core::H256;

use constants::{ADDRTREE, DANGER, GENERALVERIFIER, SCHEMA_VERSION, VERIFIERS};
use constants::{METATREE, SETTREE, SPECSTREE, TYPES};

use definitions::network_specs::NetworkSpecs;
use definitions::schema_version::SchemaVersion;
use definitions::{
    danger::DangerRecord,
    helpers::multisigner_to_public,
    history::{Event, IdentityHistory, MetaValuesDisplay, NetworkSpecsDisplay, TypesDisplay},
    keyring::{NetworkSpecsKey, VerifierKey},
    network_specs::{CurrentVerifier, ValidCurrentVerifier, Verifier},
};
use definitions::{
    keyring::MetaKey, metadata::MetaValues, network_specs::OrderedNetworkSpecs,
    qr_transfers::ContentLoadTypes, types::TypeEntry,
};
#[cfg(feature = "active")]
use definitions::{
    keyring::{AddressKey, MetaKeyPrefix},
    users::AddressDetails,
};
use sp_runtime::MultiSigner;

use crate::identities::find_address_details_for_multisigner;
use crate::{Error, Result};

#[cfg(feature = "active")]
use crate::db_transactions::TrDbCold;
use crate::manage_history::events_to_batch;

/// Open a tree in the database.
///
/// Wrapper for `open_tree()` method for database [`Db`] from [`sled`].
/// Input is `&[u8]` tree identifier.
pub fn open_tree(database: &Db, tree_name: &[u8]) -> Result<Tree> {
    Ok(database.open_tree(tree_name)?)
}

/// Check database schema version.
/// Prevents app stash when user upgrades the app without re-install.
pub fn assert_db_version(database: &Db) -> Result<()> {
    let expected = *SchemaVersion::current();
    let settings = open_tree(database, SETTREE)?;
    let ivec = settings
        .get(SCHEMA_VERSION)?
        .ok_or(Error::DbSchemaMismatch { expected, found: 0 })?;
    let found = *SchemaVersion::from_ivec(&ivec)?;
    if expected != found {
        return Err(Error::DbSchemaMismatch { expected, found });
    }
    Ok(())
}

/// Assemble a [`Batch`] that removes all elements from a tree.
pub fn make_batch_clear_tree(database: &sled::Db, tree_name: &[u8]) -> Result<Batch> {
    let tree = open_tree(database, tree_name)?;
    let mut out = Batch::default();
    for (key, _) in tree.iter().flatten() {
        out.remove(key)
    }
    Ok(out)
}

/// Get [`OrderedNetworkSpecs`] for all networks from the cold database.
///
/// Function is used both on active and Vault side, but only for the cold
/// database.
pub fn get_all_networks(database: &sled::Db) -> Result<Vec<OrderedNetworkSpecs>> {
    let chainspecs = open_tree(database, SPECSTREE)?;
    let mut out: Vec<OrderedNetworkSpecs> = Vec::new();
    for x in chainspecs.iter().flatten() {
        out.push(OrderedNetworkSpecs::from_entry_checked(x)?)
    }
    Ok(out)
}

/// Try to get [`ValidCurrentVerifier`] from the Vault database for a network,
/// using [`VerifierKey`].
///
/// If the network is not known to the database, i.e. there is no verifier data
/// and network genesis hash is not encountered elsewhere in the database,
/// result is `Ok(None)`.
///
/// Note that `CurrentVerifier::Dead` or damaged verifier data result in
/// errors.
pub fn try_get_valid_current_verifier(
    database: &sled::Db,
    verifier_key: &VerifierKey,
) -> Result<Option<ValidCurrentVerifier>> {
    let general_verifier = get_general_verifier(database)?;
    let verifiers = open_tree(database, VERIFIERS)?;
    match verifiers.get(verifier_key.key())? {
        // verifier entry is found
        Some(verifier_encoded) => {
            match <CurrentVerifier>::decode(&mut &verifier_encoded[..])? {
                // verifier is a valid one
                CurrentVerifier::Valid(b) => {
                    // Custom verifier ([`Verifier`]) can never be entered in
                    // the database with same value as database general verifier
                    // ([`Verifier`]), unless both values are `None`.
                    // If such entry is found, it indicates that the database is
                    // corrupted.
                    if let ValidCurrentVerifier::Custom {
                        v: ref custom_verifier,
                    } = b
                    {
                        if (custom_verifier == &general_verifier)
                            && (general_verifier != Verifier { v: None })
                        {
                            return Err(Error::CustomVerifierIsGeneral(verifier_key.to_owned()));
                        }
                    }
                    Ok(Some(b))
                }
            }
        }
        // no verifier for network in the database
        None => {
            // `VerifierKey` is formed from network genesis hash.
            // When the entries are added in the database, network specs could
            // be added only if the verifier is already in the database or is
            // added at the same time.
            // If the genesis hash is found in network specs, but no verifier
            // entry exists, it indicated that the database is corrupted.
            if let Some(specs_invariants) =
                genesis_hash_in_specs(database, verifier_key.genesis_hash())?
            {
                return Err(Error::UnexpectedGenesisHash {
                    name: specs_invariants.name,
                    genesis_hash: specs_invariants.genesis_hash,
                });
            }
            Ok(None)
        }
    }
}

/// Get [`ValidCurrentVerifier`] from the Vault database for a network, using
/// [`VerifierKey`].
///
/// Entry here is expected to be in the database, failure to find it results in
/// an error.
pub fn get_valid_current_verifier(
    database: &sled::Db,
    verifier_key: &VerifierKey,
) -> Result<ValidCurrentVerifier> {
    try_get_valid_current_verifier(database, verifier_key)?
        .ok_or_else(|| Error::NoValidCurrentVerifier(verifier_key.clone()))
}

/// Specs invariants that are expected to stay unchanged for the network over
/// time and can not be different for same genesis hash and different encryption
/// algorithms.
pub struct SpecsInvariants {
    pub base58prefix: u16,

    /// network with lowest order, for correct navigation when updating the
    /// network metadata
    pub first_network_specs_key: NetworkSpecsKey,
    pub genesis_hash: H256,
    pub name: String,
}

/// Search for network genesis hash in [`OrderedNetworkSpecs`] entries in [`SPECSTREE`]
/// of the Vault database.
///
/// Genesis hash is calculated from network [`VerifierKey`] input.
// TODO too convoluted, historically so; replace VerifierKey with genesis hash;
// fixes needed in `add_specs`, `load_metadata` and
// `try_get_valid_current_verifier` function above
///
/// If there are more than one network corresponding to the same genesis hash,
/// outputs network specs key for the network with the lowest order.
///
/// If there are several entries with same genesis hash, all of them must have
/// identical base58 prefix and network name. Network name is, and base58 prefix
/// could be a part of the network metadata, and therefore must not depend on
/// encryption used.
pub fn genesis_hash_in_specs(
    database: &sled::Db,
    genesis_hash: H256,
) -> Result<Option<SpecsInvariants>> {
    let chainspecs = open_tree(database, SPECSTREE)?;
    let mut specs_set: Vec<(NetworkSpecsKey, OrderedNetworkSpecs)> = Vec::new();
    let mut found_permanent_specs: Option<(u16, String)> = None;
    for (network_specs_key_vec, network_specs_encoded) in chainspecs.iter().flatten() {
        let network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
        let network_specs = OrderedNetworkSpecs::from_entry_with_key_checked(
            &network_specs_key,
            network_specs_encoded,
        )?;
        if network_specs.specs.genesis_hash.as_bytes() == &genesis_hash[..] {
            found_permanent_specs = match found_permanent_specs {
                Some((base58prefix, name)) => {
                    if base58prefix == network_specs.specs.base58prefix {
                        if name == network_specs.specs.name {
                            Some((base58prefix, name))
                        } else {
                            return Err(Error::DifferentNamesSameGenesisHash {
                                name1: name,
                                name2: network_specs.specs.name.to_string(),
                                genesis_hash: network_specs.specs.genesis_hash,
                            });
                        }
                    } else {
                        return Err(Error::DifferentBase58Specs {
                            genesis_hash: network_specs.specs.genesis_hash,
                            base58_1: base58prefix,
                            base58_2: network_specs.specs.base58prefix,
                        });
                    }
                }
                None => Some((
                    network_specs.specs.base58prefix,
                    network_specs.specs.name.to_string(),
                )),
            };
            specs_set.push((network_specs_key, network_specs))
        }
    }
    specs_set.sort_by(|a, b| a.1.order.cmp(&b.1.order));
    match specs_set.first() {
        Some(a) => Ok(Some(SpecsInvariants {
            base58prefix: a.1.specs.base58prefix,
            first_network_specs_key: a.0.to_owned(),
            genesis_hash,
            name: a.1.specs.name.to_string(),
        })),
        None => Ok(None),
    }
}

/// Get general verifier [`Verifier`] from the Vault database.
///
/// Vault works only with an initiated database, i.e. the one with general
/// verifier set up. Failure to find general verifier is always an error.
pub fn get_general_verifier(database: &sled::Db) -> Result<Verifier> {
    let settings = open_tree(database, SETTREE)?;
    let verifier_encoded = settings
        .get(GENERALVERIFIER)?
        .ok_or(Error::GeneralVerifierNotFound)?;
    Ok(<Verifier>::decode(&mut &verifier_encoded[..])?)
}

/// Try to get types information from the database.
///
/// If no types information is found, result is `Ok(None)`.
pub fn try_get_types(database: &sled::Db) -> Result<Option<Vec<TypeEntry>>> {
    let settings = open_tree(database, SETTREE)?;
    let res = settings
        .get(TYPES)?
        .map(|types_info_encoded| <Vec<TypeEntry>>::decode(&mut &types_info_encoded[..]))
        .transpose()?;

    Ok(res)
}

/// Get types information as `Vec<TypeEntry>` from the database.
///
/// Types data is expected to be found, for example, in:
///
/// - hot database, from which the types data could not be removed using
///   standard operations
/// - cold database, when transactions made using `RuntimeMetadata` `V12` or `V13` are
///   being decoded
///
/// Not finding types data results in an error.
pub fn get_types(database: &sled::Db) -> Result<Vec<TypeEntry>> {
    try_get_types(database)?.ok_or(Error::TypesNotFound)
}

/// Get types information as [`ContentLoadTypes`] from the database.
///
/// Function prepares types information in qr payload format.
///
/// Is used on the active side when preparing `load_types` qr payload and in
/// Vault when preparing `SufficientCrypto` export qr code for `load_types`
/// payload.
///
/// Not finding types data results in an error.
pub fn prep_types(database: &sled::Db) -> Result<ContentLoadTypes> {
    Ok(ContentLoadTypes::generate(&get_types(database)?))
}

/// Try to get network specs [`OrderedNetworkSpecs`] from the Vault database.
///
/// If the [`NetworkSpecsKey`] and associated [`OrderedNetworkSpecs`] are not found in
/// the [`SPECSTREE`], the result is `Ok(None)`.
pub fn try_get_network_specs(
    database: &sled::Db,
    network_specs_key: &NetworkSpecsKey,
) -> Result<Option<OrderedNetworkSpecs>> {
    let chainspecs = open_tree(database, SPECSTREE)?;
    Ok(chainspecs
        .get(network_specs_key.key())?
        .map(|network_specs_encoded| {
            OrderedNetworkSpecs::from_entry_with_key_checked(
                network_specs_key,
                network_specs_encoded,
            )
        })
        .transpose()?)
}

/// Get network specs [`OrderedNetworkSpecs`] from the Vault database.
///
/// Network specs here are expected to be found, not finding them results in an
/// error.
pub fn get_network_specs(
    database: &sled::Db,
    network_specs_key: &NetworkSpecsKey,
) -> Result<OrderedNetworkSpecs> {
    try_get_network_specs(database, network_specs_key)?
        .ok_or_else(|| Error::NetworkSpecsNotFound(network_specs_key.clone()))
}

/// Try to get [`AddressDetails`] from the Vault database, using
/// [`AddressKey`].
///
/// If no entry with provided [`AddressKey`] is found, the result is `Ok(None)`.
pub fn try_get_address_details(
    database: &sled::Db,
    address_key: &AddressKey,
) -> Result<Option<AddressDetails>> {
    let identities = open_tree(database, ADDRTREE)?;
    identities
        .get(address_key.key())?
        .map(|address_details_encoded| -> Result<AddressDetails> {
            Ok(AddressDetails::from_entry_with_key_checked(
                address_key,
                address_details_encoded,
            )?)
        })
        .transpose()
}

/// Try to get [`AddressDetails`] from the Vault database, using
/// [`AddressKey`].
///
/// If no entry with provided [`AddressKey`] is found, the result is `Ok(None)`.
pub fn try_get_address_details_by_multisigner(
    database: &sled::Db,
    multisigner: &MultiSigner,
    genesis_hash: &H256,
    encryption: &Encryption,
) -> Result<Option<AddressDetails>> {
    let maybe_address_details =
        find_address_details_for_multisigner(database, multisigner, vec![*genesis_hash])?;

    let network_specs_key = NetworkSpecsKey::from_parts(genesis_hash, encryption);

    let new_details = maybe_address_details.map(|address_details| AddressDetails {
        seed_name: address_details.seed_name,
        path: address_details.path,
        has_pwd: address_details.has_pwd,
        network_id: Some(network_specs_key),
        encryption: encryption.clone(),
        secret_exposed: address_details.secret_exposed,
    });

    Ok(new_details)
}

/// Get [`AddressDetails`] from the Vault database, using
/// [`AddressKey`].
///
/// Address is expected to exist, not finding it results in an error.
pub fn get_address_details(
    database: &sled::Db,
    address_key: &AddressKey,
) -> Result<AddressDetails> {
    try_get_address_details(database, address_key)?
        .ok_or_else(|| Error::AddressNotFound(address_key.clone()))
}

/// Get [`MetaValues`] set from Vault database, for networks with a given name.
///
/// The resulting set could be an empty one. It is used to display metadata
/// available for the network and to find the metadata to be deleted, when the
/// network gets deleted.
pub(crate) fn get_meta_values_by_name(
    database: &sled::Db,
    network_name: &str,
) -> Result<Vec<MetaValues>> {
    let metadata = open_tree(database, METATREE)?;
    let mut out: Vec<MetaValues> = Vec::new();
    let meta_key_prefix = MetaKeyPrefix::from_name(network_name);
    for x in metadata.scan_prefix(meta_key_prefix.prefix()).flatten() {
        let meta_values = MetaValues::from_entry_checked(x)?;
        if meta_values.name == network_name {
            out.push(meta_values)
        }
    }
    Ok(out)
}

/// Try to get [`MetaValues`], corresponding to given network name and version
/// from the database.
///
/// If no entry is found, the result is `Ok(None)`.
pub fn try_get_meta_values_by_name_version(
    database: &sled::Db,
    network_name: &str,
    network_version: u32,
) -> Result<Option<MetaValues>> {
    let metadata = open_tree(database, METATREE)?;
    let meta_key = MetaKey::from_parts(network_name, network_version);
    Ok(metadata
        .get(meta_key.key())?
        .map(|meta| -> Result<_> {
            Ok(Some(MetaValues::from_entry_name_version_checked(
                network_name,
                network_version,
                meta,
            )?))
        })
        .transpose()?
        .unwrap_or_default())
}

/// Get [`MetaValues`], corresponding to given network name and version, from
/// the database.
///
/// Entry is expected to be in the database, error is produced if it is not
/// found.
pub fn get_meta_values_by_name_version(
    database: &sled::Db,
    network_name: &str,
    network_version: u32,
) -> Result<MetaValues> {
    try_get_meta_values_by_name_version(database, network_name, network_version)?.ok_or(
        Error::MetaValuesNotFound {
            name: network_name.to_owned(),
            version: network_version,
        },
    )
}

/// Transfer metadata from the hot database into the cold one.
///
/// Function scans through [`METATREE`] tree of the hot database and transfers
/// into [`METATREE`] tree of the cold database the metadata entries for the
/// networks that already have the network specs [`OrderedNetworkSpecs`] entries in
/// [`SPECSTREE`] of the cold database.
///
/// Applicable only on the active side.
#[cfg(feature = "active")]
pub fn transfer_metadata_to_cold(database_hot: &sled::Db, database_cold: &sled::Db) -> Result<()> {
    let mut for_metadata = Batch::default();
    {
        let metadata_hot = open_tree(database_hot, METATREE)?;
        let chainspecs_cold = open_tree(database_cold, SPECSTREE)?;
        for x in chainspecs_cold.iter().flatten() {
            let network_specs = NetworkSpecs::from_entry_checked(x)?;
            for (key, value) in metadata_hot
                .scan_prefix(MetaKeyPrefix::from_name(&network_specs.name).prefix())
                .flatten()
            {
                for_metadata.insert(key, value)
            }
        }
    }
    TrDbCold::new()
        .set_metadata(for_metadata)
        .apply(database_cold)
}

/// Remove the network from the database.
///
/// Function inputs [`NetworkSpecsKey`] of the network, gets network genesis
/// hash and proceeds to act on **all** networks with same genesis hash.
///
/// Removing network is mostly an emergency tool and is not expected to be used
/// really often.
///
/// Removing a network means:
///
/// - Remove from [`SPECSTREE`] all [`OrderedNetworkSpecs`] that have genesis hash
///   associated with given `NetworkSpecsKey`
/// - Remove from [`METATREE`] all metadata entries corresponding to the network
///   name, as found in `OrderedNetworkSpecs`
/// - Remove from [`ADDRTREE`] all addresses in the networks being removed
/// - Modify `Verifier` data if necessary.
///
/// Note that if the network supports multiple encryption algorithms, the
/// removal of network with one of the encryptions will cause the networks
/// with other encryptions be removed as well.
pub fn remove_network(database: &Db, network_specs_key: &NetworkSpecsKey) -> Result<()> {
    let mut address_batch = Batch::default();
    let mut meta_batch = Batch::default();
    let mut network_specs_batch = Batch::default();
    let mut verifiers_batch = Batch::default();
    let mut events: Vec<Event> = Vec::new();

    let general_verifier = get_general_verifier(database)?;
    let network_specs = get_network_specs(database, network_specs_key)?;

    let verifier_key = VerifierKey::from_parts(network_specs.specs.genesis_hash);
    let valid_current_verifier = get_valid_current_verifier(database, &verifier_key)?;

    // modify verifier as needed
    if let ValidCurrentVerifier::Custom { ref v } = valid_current_verifier {
        match v {
            Verifier { v: None } => (),
            _ => {
                verifiers_batch.remove(verifier_key.key());
            }
        }
    }

    // scan through metadata tree to mark for removal all networks with target name
    for meta_values in get_meta_values_by_name(database, &network_specs.specs.name)?.iter() {
        let meta_key = MetaKey::from_parts(&meta_values.name, meta_values.version);
        meta_batch.remove(meta_key.key());
        events.push(Event::MetadataRemoved {
            meta_values_display: MetaValuesDisplay::get(meta_values),
        });
    }

    {
        let chainspecs = open_tree(database, SPECSTREE)?;
        let identities = open_tree(database, ADDRTREE)?;

        // scan through chainspecs tree to mark for removal all networks with target genesis hash
        let mut keys_to_wipe: Vec<NetworkSpecsKey> = Vec::new();
        for (network_specs_key_vec, entry) in chainspecs.iter().flatten() {
            let x_network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
            let mut x_network_specs =
                OrderedNetworkSpecs::from_entry_with_key_checked(&x_network_specs_key, entry)?;
            if x_network_specs.specs.genesis_hash == network_specs.specs.genesis_hash {
                network_specs_batch.remove(x_network_specs_key.key());
                events.push(Event::NetworkSpecsRemoved {
                    network_specs_display: NetworkSpecsDisplay::get(
                        &x_network_specs,
                        &valid_current_verifier,
                        &general_verifier,
                    ),
                });
                keys_to_wipe.push(x_network_specs_key);
            } else if x_network_specs.order > network_specs.order {
                x_network_specs.order -= 1;
                network_specs_batch.insert(x_network_specs_key.key(), x_network_specs.encode());
            }
        }

        // scan through address tree to clean up the network_key(s) from identities
        for (address_key_vec, entry) in identities.iter().flatten() {
            let address_key = AddressKey::from_ivec(&address_key_vec)?;
            let (multisigner, address_details) =
                AddressDetails::process_entry_checked((address_key_vec, entry))?;
            for key in keys_to_wipe.iter() {
                if address_details.network_id.as_ref() == Some(key) {
                    let identity_history = IdentityHistory::get(
                        &address_details.seed_name,
                        &address_details.encryption,
                        &multisigner_to_public(&multisigner),
                        &address_details.path,
                        network_specs.specs.genesis_hash,
                    );
                    events.push(Event::IdentityRemoved { identity_history });
                    address_batch.remove(address_key.key())
                } else {
                    address_batch.insert(address_key.key(), address_details.encode())
                }
            }
        }
    }
    TrDbCold::new()
        .set_addresses(address_batch) // upd addresses
        .set_history(events_to_batch(database, events)?) // add corresponding history
        .set_metadata(meta_batch) // upd metadata
        .set_network_specs(network_specs_batch) // upd network_specs
        .set_verifiers(verifiers_batch) // upd network_verifiers
        .apply(database)
}

/// Remove the network metadata entry from the database.
///
/// Function inputs [`NetworkSpecsKey`] of the network and `u32` version of the
/// network metadata, and proceeds to remove a single metadata entry
/// corresponding to this version.
///
/// Metadata in the Vault database is determined by the network name and
/// network version, and has no information about the
/// [`Encryption`](definitions::crypto::Encryption) algorithm supported by the
/// network. Therefore if the network supports more than one encryption
/// algorithm, removing metadata for one will affect all encryptions.
pub fn remove_metadata(
    database: &sled::Db,
    network_specs_key: &NetworkSpecsKey,
    network_version: u32,
) -> Result<()> {
    let network_specs = get_network_specs(database, network_specs_key)?;
    let meta_key = MetaKey::from_parts(&network_specs.specs.name, network_version);
    let mut meta_batch = Batch::default();
    meta_batch.remove(meta_key.key());

    let meta_values =
        get_meta_values_by_name_version(database, &network_specs.specs.name, network_version)?;
    let meta_values_display = MetaValuesDisplay::get(&meta_values);
    let history_batch = events_to_batch(
        database,
        vec![Event::MetadataRemoved {
            meta_values_display,
        }],
    )?;
    TrDbCold::new()
        .set_metadata(meta_batch) // remove metadata
        .set_history(history_batch) // add corresponding history
        .apply(database)
}

/// User-initiated removal of the types information from the Vault database.
///
/// Types information is not necessary to process transactions in networks with
/// metadata having `RuntimeVersionV14`, as the types information after `V14`
/// is a part of the metadata itself.
///
/// Types information is installed in Vault by default and could be removed by
/// user manually, through this function.
///
/// Types information is verified by the general verifier. When the general
/// verifier gets changed, the types information is also removed from the
/// Vault database through so-called `GeneralHold` processing, to avoid
/// confusion regarding what data was verified by whom. Note that this situation
/// in **not** related to the `remove_types_info` function and is handled
/// elsewhere.
pub fn remove_types_info(database: &sled::Db) -> Result<()> {
    let mut settings_batch = Batch::default();
    settings_batch.remove(TYPES);
    let events: Vec<Event> = vec![Event::TypesRemoved {
        types_display: TypesDisplay::get(
            &ContentLoadTypes::generate(&get_types(database)?),
            &get_general_verifier(database)?,
        ),
    }];
    TrDbCold::new()
        .set_history(events_to_batch(database, events)?)
        // add history
        .set_settings(settings_batch)
        // upd settings
        .apply(database)
}

/// Modify existing batch for [`ADDRTREE`](constants::ADDRTREE) with incoming
/// vector of additions.
#[cfg(feature = "active")]
pub(crate) fn upd_id_batch(mut batch: Batch, adds: Vec<(AddressKey, AddressDetails)>) -> Batch {
    for (address_key, address_details) in adds.iter() {
        batch.insert(address_key.key(), address_details.encode());
    }
    batch
}

/// Verify checksum in Vault database.
///
/// Used in retrieving temporary stored data from
/// [`TRANSACTION`](constants::TRANSACTION) tree of the database.
// TODO Goes obsolete if the temporary storage goes.
pub(crate) fn verify_checksum(database: &Db, checksum: u32) -> Result<()> {
    let real_checksum = database.checksum()?;
    if checksum != real_checksum {
        return Err(Error::ChecksumMismatch);
    }
    Ok(())
}

/// Get the danger status from the Vault database.
///
/// Currently, the only flag contributing to the danger status is whether the
/// device was online. This may change eventually.
pub fn get_danger_status(database: &sled::Db) -> Result<bool> {
    let settings = open_tree(database, SETTREE)?;
    let a = settings.get(DANGER)?.ok_or(Error::DangerStatusNotFound)?;
    Ok(DangerRecord::from_ivec(&a).device_was_online()?)
}

pub fn validate_mnemonic(mnemonic: &str) -> bool {
    bip39::Mnemonic::validate(mnemonic, bip39::Language::English).is_ok()
}
