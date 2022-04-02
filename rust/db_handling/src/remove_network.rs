use parity_scale_codec::Encode;
use sled::Batch;

use constants::{ADDRTREE, METATREE, SPECSTREE};
use definitions::{
    error::ErrorSource,
    error_signer::{ErrorSigner, NotFoundSigner, Signer},
    helpers::multisigner_to_public,
    history::{Event, IdentityHistory, MetaValuesDisplay, NetworkSpecsDisplay},
    keyring::{AddressKey, MetaKey, MetaKeyPrefix, NetworkSpecsKey, VerifierKey},
    network_specs::{CurrentVerifier, NetworkSpecs, ValidCurrentVerifier, Verifier},
    users::AddressDetails,
};

use crate::db_transactions::TrDbCold;
use crate::helpers::{
    get_general_verifier, get_network_specs, get_valid_current_verifier, open_db, open_tree,
};
use crate::manage_history::events_to_batch;

/// Function to remove the network with given NetworkSpecsKey from the database.
/// Removes network specs for all entries with same genesis hash.
/// Removes all metadata entries for corresponding network specname.
/// Removes all addresses corresponding to the networks removed (all encryptions).
/// If ValidCurrentVerifier is Custom(Verifier(None)), leaves it as that. If ValidCurrentVerifier is General, leaves it as General.
/// If ValidCurrentVerifier is Custom with some Verifier set, transforms CurrentVerifier from Valid into Dead to disable the network
/// permanently until Signer is wiped altogether.
/// Function is used only on the Signer side.
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

    {
        let database = open_db::<Signer>(database_name)?;
        let metadata = open_tree::<Signer>(&database, METATREE)?;
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
        // scan through metadata tree to mark for removal all networks with target name
        let meta_key_prefix = MetaKeyPrefix::from_name(&network_specs.name);
        for (meta_key_vec, meta_stored) in metadata.scan_prefix(meta_key_prefix.prefix()).flatten()
        {
            let meta_key = MetaKey::from_ivec(&meta_key_vec);
            meta_batch.remove(meta_key.key());
            if let Ok((name, version)) = meta_key.name_version::<Signer>() {
                let meta_values_display =
                    MetaValuesDisplay::from_storage(&name, version, meta_stored);
                events.push(Event::MetadataRemoved(meta_values_display));
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

pub fn remove_metadata(
    network_specs_key: &NetworkSpecsKey,
    network_version: u32,
    database_name: &str,
) -> Result<(), ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let meta_key = MetaKey::from_parts(&network_specs.name, network_version);
    let mut meta_batch = Batch::default();
    meta_batch.remove(meta_key.key());
    let history_batch =
        get_batch_remove_unchecked_meta(database_name, &network_specs.name, network_version)?;
    TrDbCold::new()
        .set_metadata(meta_batch) // remove metadata
        .set_history(history_batch) // add corresponding history
        .apply::<Signer>(database_name)
}

fn get_batch_remove_unchecked_meta(
    database_name: &str,
    network_name: &str,
    network_version: u32,
) -> Result<Batch, ErrorSigner> {
    let events = {
        let meta_key = MetaKey::from_parts(network_name, network_version);
        let database = open_db::<Signer>(database_name)?;
        let metadata = open_tree::<Signer>(&database, METATREE)?;
        match metadata.get(meta_key.key()) {
            Ok(Some(meta_stored)) => {
                let meta_values_display =
                    MetaValuesDisplay::from_storage(network_name, network_version, meta_stored);
                vec![Event::MetadataRemoved(meta_values_display)]
            }
            Ok(None) => {
                return Err(ErrorSigner::NotFound(NotFoundSigner::Metadata {
                    name: network_name.to_string(),
                    version: network_version,
                }))
            }
            Err(e) => return Err(<Signer>::db_internal(e)),
        }
    };
    events_to_batch::<Signer>(database_name, events)
}
