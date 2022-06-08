//! Common helper functions for database operations

use parity_scale_codec::Decode;
#[cfg(any(feature = "active", feature = "signer"))]
use parity_scale_codec::Encode;
use sled::{open, Batch, Db, Tree};

#[cfg(feature = "signer")]
use constants::{ADDRTREE, DANGER, GENERALVERIFIER, VERIFIERS};
use constants::{METATREE, SETTREE, SPECSTREE, TYPES};

#[cfg(feature = "active")]
use definitions::error_active::{Active, ErrorActive};
#[cfg(feature = "signer")]
use definitions::{
    danger::DangerRecord,
    error_signer::{DatabaseSigner, EntryDecodingSigner, ErrorSigner, NotFoundSigner, Signer},
    helpers::multisigner_to_public,
    history::{Event, IdentityHistory, MetaValuesDisplay, NetworkSpecsDisplay, TypesDisplay},
    keyring::{NetworkSpecsKey, VerifierKey},
    network_specs::{CurrentVerifier, ValidCurrentVerifier, Verifier},
};
use definitions::{
    error::ErrorSource, keyring::MetaKey, metadata::MetaValues, network_specs::NetworkSpecs,
    qr_transfers::ContentLoadTypes, types::TypeEntry,
};
#[cfg(any(feature = "active", feature = "signer"))]
use definitions::{
    keyring::{AddressKey, MetaKeyPrefix},
    users::AddressDetails,
};

#[cfg(any(feature = "active", feature = "signer"))]
use crate::db_transactions::TrDbCold;
#[cfg(feature = "signer")]
use crate::manage_history::events_to_batch;

/// Open a database.
///
/// Wrapper for [`open`] from [`sled`]. Input is database location path.
pub fn open_db<T: ErrorSource>(database_name: &str) -> Result<Db, T::Error> {
    open(database_name).map_err(<T>::db_internal)
}

/// Open a tree in the database.
///
/// Wrapper for `open_tree()` method for database [`Db`] from [`sled`].
/// Input is `&[u8]` tree identifier.
pub fn open_tree<T: ErrorSource>(database: &Db, tree_name: &[u8]) -> Result<Tree, T::Error> {
    database.open_tree(tree_name).map_err(<T>::db_internal)
}

/// Assemble a [`Batch`] that removes all elements from a tree.
pub fn make_batch_clear_tree<T: ErrorSource>(
    database_name: &str,
    tree_name: &[u8],
) -> Result<Batch, T::Error> {
    let database = open_db::<T>(database_name)?;
    let tree = open_tree::<T>(&database, tree_name)?;
    let mut out = Batch::default();
    for (key, _) in tree.iter().flatten() {
        out.remove(key)
    }
    Ok(out)
}

/// Get [`NetworkSpecs`] for all networks from the cold database.
///
/// Function is used both on active and Signer side, but only for the cold
/// database.
pub fn get_all_networks<T: ErrorSource>(
    database_name: &str,
) -> Result<Vec<NetworkSpecs>, T::Error> {
    let database = open_db::<T>(database_name)?;
    let chainspecs = open_tree::<T>(&database, SPECSTREE)?;
    let mut out: Vec<NetworkSpecs> = Vec::new();
    for x in chainspecs.iter().flatten() {
        out.push(NetworkSpecs::from_entry_checked::<T>(x)?)
    }
    Ok(out)
}

/// Try to get [`ValidCurrentVerifier`] from the Signer database for a network,
/// using [`VerifierKey`].
///
/// If the network is not known to the database, i.e. there is no verifier data
/// and network genesis hash is not encountered elsewhere in the database,
/// result is `Ok(None)`.
///
/// Note that `CurrentVerifier::Dead` or damaged verifier data result in
/// errors.
#[cfg(feature = "signer")]
pub fn try_get_valid_current_verifier(
    verifier_key: &VerifierKey,
    database_name: &str,
) -> Result<Option<ValidCurrentVerifier>, ErrorSigner> {
    let general_verifier = get_general_verifier(database_name)?;
    let database = open_db::<Signer>(database_name)?;
    let verifiers = open_tree::<Signer>(&database, VERIFIERS)?;
    match verifiers.get(verifier_key.key()) {
        // verifier entry is found
        Ok(Some(verifier_encoded)) => match <CurrentVerifier>::decode(&mut &verifier_encoded[..]) {
            // verifier could be decoded
            Ok(a) => match a {
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
                            return Err(ErrorSigner::Database(
                                DatabaseSigner::CustomVerifierIsGeneral(verifier_key.to_owned()),
                            ));
                        }
                    }
                    Ok(Some(b))
                }
                // verifier is dead, network could not be used
                CurrentVerifier::Dead => Err(ErrorSigner::DeadVerifier(verifier_key.to_owned())),
            },
            // verifier entry is damaged
            Err(_) => Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(
                EntryDecodingSigner::CurrentVerifier(verifier_key.to_owned()),
            ))),
        },
        // no verifier for network in the database
        Ok(None) => {
            // `VerifierKey` is formed from network genesis hash.
            // When the entries are added in the database, network specs could
            // be added only if the verifier is already in the database or is
            // added at the same time.
            // If the genesis hash is found in network specs, but no verifier
            // entry exists, it indicated that the database is corrupted.
            if let Some((network_specs_key, _)) = genesis_hash_in_specs(verifier_key, &database)? {
                return Err(ErrorSigner::Database(
                    DatabaseSigner::UnexpectedGenesisHash {
                        verifier_key: verifier_key.to_owned(),
                        network_specs_key,
                    },
                ));
            }
            Ok(None)
        }
        // database own errors
        Err(e) => Err(<Signer>::db_internal(e)),
    }
}

/// Get [`ValidCurrentVerifier`] from the Signer database for a network, using
/// [`VerifierKey`].
///
/// Entry here is expected to be in the database, failure to find it results in
/// an error.
#[cfg(feature = "signer")]
pub fn get_valid_current_verifier(
    verifier_key: &VerifierKey,
    database_name: &str,
) -> Result<ValidCurrentVerifier, ErrorSigner> {
    try_get_valid_current_verifier(verifier_key, database_name)?.ok_or_else(|| {
        ErrorSigner::NotFound(NotFoundSigner::CurrentVerifier(verifier_key.to_owned()))
    })
}

/// Search for network genesis hash in [`NetworkSpecs`] entries in [`SPECSTREE`]
/// of the Signer database.
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
#[cfg(feature = "signer")]
pub fn genesis_hash_in_specs(
    verifier_key: &VerifierKey,
    database: &Db,
) -> Result<Option<(NetworkSpecsKey, NetworkSpecs)>, ErrorSigner> {
    let genesis_hash = verifier_key.genesis_hash();
    let chainspecs = open_tree::<Signer>(database, SPECSTREE)?;
    let mut specs_set: Vec<(NetworkSpecsKey, NetworkSpecs)> = Vec::new();
    let mut found_permanent_specs: Option<(u16, String)> = None;
    for (network_specs_key_vec, network_specs_encoded) in chainspecs.iter().flatten() {
        let network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
        let network_specs = NetworkSpecs::from_entry_with_key_checked::<Signer>(
            &network_specs_key,
            network_specs_encoded,
        )?;
        if network_specs.genesis_hash.as_bytes() == &genesis_hash[..] {
            found_permanent_specs = match found_permanent_specs {
                Some((base58prefix, name)) => {
                    if base58prefix == network_specs.base58prefix {
                        if name == network_specs.name {
                            Some((base58prefix, name))
                        } else {
                            return Err(ErrorSigner::Database(
                                DatabaseSigner::DifferentNamesSameGenesisHash {
                                    name1: name,
                                    name2: network_specs.name.to_string(),
                                    genesis_hash: network_specs.genesis_hash,
                                },
                            ));
                        }
                    } else {
                        return Err(ErrorSigner::Database(
                            DatabaseSigner::DifferentBase58Specs {
                                genesis_hash: network_specs.genesis_hash,
                                base58_1: base58prefix,
                                base58_2: network_specs.base58prefix,
                            },
                        ));
                    }
                }
                None => Some((network_specs.base58prefix, network_specs.name.to_string())),
            };
            specs_set.push((network_specs_key, network_specs))
        }
    }
    specs_set.sort_by(|a, b| a.1.order.cmp(&b.1.order));
    match specs_set.get(0) {
        Some(a) => Ok(Some(a.to_owned())),
        None => Ok(None),
    }
}

/// Get general verifier [`Verifier`] from the Signer database.
///
/// Signer works only with an initiated database, i.e. the one with general
/// verifier set up. Failure to find general verifier is always an error.
#[cfg(feature = "signer")]
pub fn get_general_verifier(database_name: &str) -> Result<Verifier, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let settings = open_tree::<Signer>(&database, SETTREE)?;
    match settings.get(GENERALVERIFIER) {
        Ok(Some(verifier_encoded)) => match <Verifier>::decode(&mut &verifier_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => Err(ErrorSigner::Database(DatabaseSigner::EntryDecoding(
                EntryDecodingSigner::GeneralVerifier,
            ))),
        },
        Ok(None) => Err(ErrorSigner::NotFound(NotFoundSigner::GeneralVerifier)),
        Err(e) => Err(<Signer>::db_internal(e)),
    }
}

/// Function to try and get types information from the database
/// Applicable to both Active side and Signer side
pub fn try_get_types<T: ErrorSource>(
    database_name: &str,
) -> Result<Option<Vec<TypeEntry>>, T::Error> {
    let database = open_db::<T>(database_name)?;
    let settings = open_tree::<T>(&database, SETTREE)?;
    match settings.get(TYPES) {
        Ok(Some(types_info_encoded)) => {
            match <Vec<TypeEntry>>::decode(&mut &types_info_encoded[..]) {
                Ok(a) => Ok(Some(a)),
                Err(_) => Err(<T>::faulty_database_types()),
            }
        }
        Ok(None) => Ok(None),
        Err(e) => Err(<T>::db_internal(e)),
    }
}

/// Get types information as `Vec<TypeEntry>` from the database.
///
/// Types data is expected to be found, for example, in:
///
/// - hot database, from which the types data could not be removed using
/// standard operations
/// - cold database, when transactions made using RuntimeMetadata V12 or V13 are
/// being decoded
///
/// Not finding types data results in an error.
pub fn get_types<T: ErrorSource>(database_name: &str) -> Result<Vec<TypeEntry>, T::Error> {
    try_get_types::<T>(database_name)?.ok_or_else(|| <T>::types_not_found())
}

/// Get types information as [`ContentLoadTypes`] from the database.
///
/// Function prepares types information in qr payload format.
///
/// Is used on the active side when preparing `load_types` qr payload and in
/// Signer when preparing `SufficientCrypto` export qr code for `load_types`
/// payload.
///
/// Not finding types data results in an error.
pub fn prep_types<T: ErrorSource>(database_name: &str) -> Result<ContentLoadTypes, T::Error> {
    Ok(ContentLoadTypes::generate(&get_types::<T>(database_name)?))
}

/// Try to get network specs [`NetworkSpecs`] from the Signer database.
///
/// If the [`NetworkSpecsKey`] and associated [`NetworkSpecs`] are not found in
/// the [`SPECSTREE`], the result is `Ok(None)`.
#[cfg(feature = "signer")]
pub fn try_get_network_specs(
    database_name: &str,
    network_specs_key: &NetworkSpecsKey,
) -> Result<Option<NetworkSpecs>, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let chainspecs = open_tree::<Signer>(&database, SPECSTREE)?;
    match chainspecs.get(network_specs_key.key()) {
        Ok(Some(network_specs_encoded)) => Ok(Some(NetworkSpecs::from_entry_with_key_checked::<
            Signer,
        >(
            network_specs_key, network_specs_encoded
        )?)),
        Ok(None) => Ok(None),
        Err(e) => Err(<Signer>::db_internal(e)),
    }
}

/// Get network specs [`NetworkSpecs`] from the Signer database.
///
/// Network specs here are expected to be found, not finding them results in an
/// error.
#[cfg(feature = "signer")]
pub fn get_network_specs(
    database_name: &str,
    network_specs_key: &NetworkSpecsKey,
) -> Result<NetworkSpecs, ErrorSigner> {
    try_get_network_specs(database_name, network_specs_key)?.ok_or_else(|| {
        ErrorSigner::NotFound(NotFoundSigner::NetworkSpecs(network_specs_key.to_owned()))
    })
}

/// Try to get [`AddressDetails`] from the Signer database, using
/// [`AddressKey`].
///
/// If no entry with provided [`AddressKey`] is found, the result is `Ok(None)`.
#[cfg(feature = "signer")]
pub fn try_get_address_details(
    database_name: &str,
    address_key: &AddressKey,
) -> Result<Option<AddressDetails>, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let identities = open_tree::<Signer>(&database, ADDRTREE)?;
    match identities.get(address_key.key()) {
        Ok(Some(address_details_encoded)) => {
            Ok(Some(AddressDetails::from_entry_with_key_checked::<Signer>(
                address_key,
                address_details_encoded,
            )?))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(<Signer>::db_internal(e)),
    }
}

/// Get [`AddressDetails`] from the Signer database, using
/// [`AddressKey`].
///
/// Address is expected to exist, not finding it results in an error.
#[cfg(feature = "signer")]
pub fn get_address_details(
    database_name: &str,
    address_key: &AddressKey,
) -> Result<AddressDetails, ErrorSigner> {
    try_get_address_details(database_name, address_key)?.ok_or_else(|| {
        ErrorSigner::NotFound(NotFoundSigner::AddressDetails(address_key.to_owned()))
    })
}

/// Get [`MetaValues`] set from Signer database, for networks with a given name.
///
/// The resulting set could be an empty one. It is used to display metadata
/// available for the network and to find the metadata to be deleted, when the
/// network gets deleted.
#[cfg(feature = "signer")]
pub(crate) fn get_meta_values_by_name(
    database_name: &str,
    network_name: &str,
) -> Result<Vec<MetaValues>, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let metadata = open_tree::<Signer>(&database, METATREE)?;
    let mut out: Vec<MetaValues> = Vec::new();
    let meta_key_prefix = MetaKeyPrefix::from_name(network_name);
    for x in metadata.scan_prefix(meta_key_prefix.prefix()).flatten() {
        let meta_values = MetaValues::from_entry_checked::<Signer>(x)?;
        if meta_values.name == network_name {
            out.push(meta_values)
        }
    }
    Ok(out)
}

/// Get [`MetaValues`], corresponding to given network name and version, from
/// the database.
///
/// Entry is expected to be in the database, error is produced if it is not
/// found.
pub fn get_meta_values_by_name_version<T: ErrorSource>(
    database_name: &str,
    network_name: &str,
    network_version: u32,
) -> Result<MetaValues, T::Error> {
    let database = open_db::<T>(database_name)?;
    let metadata = open_tree::<T>(&database, METATREE)?;
    let meta_key = MetaKey::from_parts(network_name, network_version);
    match metadata.get(meta_key.key()) {
        Ok(Some(meta)) => {
            MetaValues::from_entry_name_version_checked::<T>(network_name, network_version, meta)
        }
        Ok(None) => Err(<T>::metadata_not_found(
            network_name.to_string(),
            network_version,
        )),
        Err(e) => Err(<T>::db_internal(e)),
    }
}

/// Transfer metadata from the hot database into the cold one.
///
/// Function scans through [`METATREE`] tree of the hot database and transfers
/// into [`METATREE`] tree of the cold database the metadata entries for the
/// networks that already have the network specs [`NetworkSpecs`] entries in
/// [`SPECSTREE`] of the cold database.
///
/// Applicable only on the active side.
#[cfg(feature = "active")]
pub fn transfer_metadata_to_cold(
    database_name_hot: &str,
    database_name_cold: &str,
) -> Result<(), ErrorActive> {
    let mut for_metadata = Batch::default();
    {
        let database_hot = open_db::<Active>(database_name_hot)?;
        let metadata_hot = open_tree::<Active>(&database_hot, METATREE)?;
        let database_cold = open_db::<Active>(database_name_cold)?;
        let chainspecs_cold = open_tree::<Active>(&database_cold, SPECSTREE)?;
        for x in chainspecs_cold.iter().flatten() {
            let network_specs = NetworkSpecs::from_entry_checked::<Active>(x)?;
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
        .apply::<Active>(database_name_cold)
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
/// - Remove from [`SPECSTREE`] all [`NetworkSpecs`] that have genesis hash
/// associated with given `NetworkSpecsKey`
/// - Remove from [`METATREE`] all metadata entries corresponding to the network
/// name, as found in `NetworkSpecs`
/// - Remove from [`ADDRTREE`] all addresses in the networks being removed
/// - Modify `Verifier` data if necessary.
///
/// Removing the network **may result** in blocking the network altogether until
/// the Signer is reset. This happens only if the removed network
/// [`ValidCurrentVerifier`] was set to
/// `ValidCurrentVerifier::Custom(Verifier(Some(_)))` and is a security measure.
/// This should be used to deal with compromised `Custom` verifiers.
///
/// Compromised general verifier is a major disaster and will require Signer
/// reset in any case.
///
/// Removing the network verified by the general verifier **does not** block the
/// network.
///
/// Removing the network verified by
/// `ValidCurrentVerifier::Custom(Verifier(None))` **does not** block the
/// network.
///
/// Note that if the network supports multiple encryption algorithms, the
/// removal of network with one of the encryptions will cause the networks
/// with other encryptions be removed as well.
#[cfg(feature = "signer")]
pub fn remove_network(
    network_specs_key: &NetworkSpecsKey,
    database_name: &str,
) -> Result<(), ErrorSigner> {
    let mut address_batch = Batch::default();
    let mut meta_batch = Batch::default();
    let mut network_specs_batch = Batch::default();
    let mut verifiers_batch = Batch::default();
    let mut events: Vec<Event> = Vec::new();

    let general_verifier = get_general_verifier(database_name)?;
    let network_specs = get_network_specs(database_name, network_specs_key)?;

    let verifier_key = VerifierKey::from_parts(network_specs.genesis_hash);
    let valid_current_verifier = get_valid_current_verifier(&verifier_key, database_name)?;

    // modify verifier as needed
    if let ValidCurrentVerifier::Custom { ref v } = valid_current_verifier {
        match v {
            Verifier { v: None } => (),
            _ => {
                verifiers_batch.remove(verifier_key.key());
                verifiers_batch.insert(verifier_key.key(), (CurrentVerifier::Dead).encode());
            }
        }
    }

    // scan through metadata tree to mark for removal all networks with target name
    for meta_values in get_meta_values_by_name(database_name, &network_specs.name)?.iter() {
        let meta_key = MetaKey::from_parts(&meta_values.name, meta_values.version);
        meta_batch.remove(meta_key.key());
        events.push(Event::MetadataRemoved {
            meta_values_display: MetaValuesDisplay::get(meta_values),
        });
    }

    {
        let database = open_db::<Signer>(database_name)?;
        let chainspecs = open_tree::<Signer>(&database, SPECSTREE)?;
        let identities = open_tree::<Signer>(&database, ADDRTREE)?;

        // scan through chainspecs tree to mark for removal all networks with target genesis hash
        let mut keys_to_wipe: Vec<NetworkSpecsKey> = Vec::new();
        for (network_specs_key_vec, entry) in chainspecs.iter().flatten() {
            let x_network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
            let mut x_network_specs =
                NetworkSpecs::from_entry_with_key_checked::<Signer>(&x_network_specs_key, entry)?;
            if x_network_specs.genesis_hash == network_specs.genesis_hash {
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
            let address_key = AddressKey::from_ivec(&address_key_vec);
            let (multisigner, mut address_details) =
                AddressDetails::process_entry_checked::<Signer>((address_key_vec, entry))?;
            for key in keys_to_wipe.iter() {
                if address_details.network_id.contains(key) {
                    let identity_history = IdentityHistory::get(
                        &address_details.seed_name,
                        &address_details.encryption,
                        &multisigner_to_public(&multisigner),
                        &address_details.path,
                        network_specs.genesis_hash.as_bytes(),
                    );
                    events.push(Event::IdentityRemoved { identity_history });
                    address_details.network_id = address_details
                        .network_id
                        .into_iter()
                        .filter(|id| id != key)
                        .collect();
                }
            }
            if address_details.network_id.is_empty() {
                address_batch.remove(address_key.key())
            } else {
                address_batch.insert(address_key.key(), address_details.encode())
            }
        }
    }
    TrDbCold::new()
        .set_addresses(address_batch) // upd addresses
        .set_history(events_to_batch::<Signer>(database_name, events)?) // add corresponding history
        .set_metadata(meta_batch) // upd metadata
        .set_network_specs(network_specs_batch) // upd network_specs
        .set_verifiers(verifiers_batch) // upd network_verifiers
        .apply::<Signer>(database_name)
}

/// Remove the network metadata entry from the database.
///
/// Function inputs [`NetworkSpecsKey`] of the network and `u32` version of the
/// network metadata, and proceeds to remove a single metadata entry
/// corresponding to this version.
///
/// Metadata in the Signer database is determined by the network name and
/// network version, and has no information about the
/// [`Encryption`](definitions::crypto::Encryption) algorithm supported by the
/// network. Therefore if the network supports more than one encryption
/// algorithm, removing metadata for one will affect all encryptions.
#[cfg(feature = "signer")]
pub fn remove_metadata(
    network_specs_key: &NetworkSpecsKey,
    network_version: u32,
    database_name: &str,
) -> Result<(), ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let meta_key = MetaKey::from_parts(&network_specs.name, network_version);
    let mut meta_batch = Batch::default();
    meta_batch.remove(meta_key.key());

    let meta_values = get_meta_values_by_name_version::<Signer>(
        database_name,
        &network_specs.name,
        network_version,
    )?;
    let meta_values_display = MetaValuesDisplay::get(&meta_values);
    let history_batch = events_to_batch::<Signer>(
        database_name,
        vec![Event::MetadataRemoved {
            meta_values_display,
        }],
    )?;
    TrDbCold::new()
        .set_metadata(meta_batch) // remove metadata
        .set_history(history_batch) // add corresponding history
        .apply::<Signer>(database_name)
}

/// User-initiated removal of the types information from the Signer database.
///
/// Types information is not necessary to process transactions in networks with
/// metadata having `RuntimeVersionV14`, as the types information after `V14`
/// is a part of the metadata itself.
///
/// Types information is installed in Signer by default and could be removed by
/// user manually, through this function.
///
/// Types information is verified by the general verifier. When the general
/// verifier gets changed, the types information is also removed from the
/// Signer database through so-called `GeneralHold` processing, to avoid
/// confusion regarding what data was verified by whom. Note that this situation
/// in **not** related to the `remove_types_info` function and is handled
/// elsewhere.
#[cfg(feature = "signer")]
pub fn remove_types_info(database_name: &str) -> Result<(), ErrorSigner> {
    let mut settings_batch = Batch::default();
    settings_batch.remove(TYPES);
    let events: Vec<Event> = vec![Event::TypesRemoved {
        types_display: TypesDisplay::get(
            &ContentLoadTypes::generate(&get_types::<Signer>(database_name)?),
            &get_general_verifier(database_name)?,
        ),
    }];
    TrDbCold::new()
        .set_history(events_to_batch::<Signer>(database_name, events)?) // add history
        .set_settings(settings_batch) // upd settings
        .apply::<Signer>(database_name)
}

/// Modify existing batch for [`ADDRTREE`](constants::ADDRTREE) with incoming
/// vector of additions.
#[cfg(any(feature = "active", feature = "signer"))]
pub(crate) fn upd_id_batch(mut batch: Batch, adds: Vec<(AddressKey, AddressDetails)>) -> Batch {
    for (address_key, address_details) in adds.iter() {
        batch.insert(address_key.key(), address_details.encode());
    }
    batch
}

/// Verify checksum in Signer database.
///
/// Used in retrieving temporary stored data from
/// [`TRANSACTION`](constants::TRANSACTION) tree of the database.
// TODO Goes obsolete if the temporary storage goes.
#[cfg(feature = "signer")]
pub(crate) fn verify_checksum(database: &Db, checksum: u32) -> Result<(), ErrorSigner> {
    let real_checksum = match database.checksum() {
        Ok(x) => x,
        Err(e) => return Err(<Signer>::db_internal(e)),
    };
    if checksum != real_checksum {
        return Err(ErrorSigner::Database(DatabaseSigner::ChecksumMismatch));
    }
    Ok(())
}

/// Get the danger status from the Signer database.
///
/// Currently, the only flag contributing to the danger status is whether the
/// device was online. This may change eventually.
#[cfg(feature = "signer")]
pub fn get_danger_status(database_name: &str) -> Result<bool, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let settings = open_tree::<Signer>(&database, SETTREE)?;
    match settings.get(DANGER) {
        Ok(Some(a)) => DangerRecord::from_ivec(&a).device_was_online(),
        Ok(None) => Err(ErrorSigner::NotFound(NotFoundSigner::DangerStatus)),
        Err(e) => Err(<Signer>::db_internal(e)),
    }
}
