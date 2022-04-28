use parity_scale_codec::Encode;
use sled::Batch;

use constants::{ADDRTREE, SPECSTREE};
use definitions::{
    error_signer::{ErrorSigner, Signer},
    helpers::multisigner_to_public,
    history::{Event, IdentityHistory, MetaValuesDisplay, NetworkSpecsDisplay},
    keyring::{AddressKey, MetaKey, NetworkSpecsKey, VerifierKey},
    network_specs::{CurrentVerifier, NetworkSpecs, ValidCurrentVerifier, Verifier},
    users::AddressDetails,
};

use crate::db_transactions::TrDbCold;
use crate::helpers::{
    get_general_verifier, get_meta_values_by_name, get_meta_values_by_name_version,
    get_network_specs, get_valid_current_verifier, open_db, open_tree,
};
use crate::manage_history::events_to_batch;

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

    let verifier_key = VerifierKey::from_parts(&network_specs.genesis_hash);
    let valid_current_verifier = get_valid_current_verifier(&verifier_key, database_name)?;

    // modify verifier as needed
    if let ValidCurrentVerifier::Custom(ref a) = valid_current_verifier {
        match a {
            Verifier(None) => (),
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
        events.push(Event::MetadataRemoved(MetaValuesDisplay::get(meta_values)));
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
                events.push(Event::NetworkSpecsRemoved(NetworkSpecsDisplay::get(
                    &x_network_specs,
                    &valid_current_verifier,
                    &general_verifier,
                )));
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
                        &network_specs.genesis_hash,
                    );
                    events.push(Event::IdentityRemoved(identity_history));
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
/// network version, and has no information about the [`Encryption`] algorithm
/// supported by the network. Therefore if the network supports more than one
/// encryption algorithm, removing metadata for one will affect all encryptions.
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
        vec![Event::MetadataRemoved(meta_values_display)],
    )?;
    TrDbCold::new()
        .set_metadata(meta_batch) // remove metadata
        .set_history(history_batch) // add corresponding history
        .apply::<Signer>(database_name)
}
