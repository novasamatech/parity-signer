//! All db handling related to seeds and addresses
//! seed phrases should be stored in hw encrypted by
//! best available tool and here they are only processed in plaintext.
//! Zeroization is mostly delegated to os

#[cfg(feature = "signer")]
use bip39::{Language, Mnemonic, MnemonicType};
use lazy_static::lazy_static;
#[cfg(feature = "signer")]
use parity_scale_codec::Encode;
use regex::Regex;
#[cfg(any(feature = "active", feature = "signer"))]
use sled::Batch;
#[cfg(any(feature = "active", feature = "signer"))]
use sp_core::{ecdsa, ed25519, sr25519, Pair};
#[cfg(any(feature = "active", feature = "signer"))]
use sp_runtime::MultiSigner;
#[cfg(any(feature = "active", feature = "signer"))]
use zeroize::Zeroize;

#[cfg(any(feature = "active", feature = "signer"))]
use constants::ADDRTREE;
#[cfg(feature = "active")]
use constants::ALICE_SEED_PHRASE;
#[cfg(feature = "signer")]
use constants::TRANSACTION;

#[cfg(feature = "active")]
use defaults::default_chainspecs;

#[cfg(any(feature = "active", feature = "signer"))]
use definitions::{
    crypto::Encryption,
    error::{AddressGenerationCommon, ErrorSource},
    helpers::multisigner_to_public,
    history::{Event, IdentityHistory},
    keyring::{AddressKey, NetworkSpecsKey},
    network_specs::NetworkSpecs,
    users::AddressDetails,
};
#[cfg(feature = "signer")]
use definitions::{
    error::{AddressGeneration, SpecsKeySource},
    error_signer::{
        ErrorSigner, ExtraAddressGenerationSigner, InputSigner, InterfaceSigner, Signer,
    },
};
#[cfg(feature = "active")]
use definitions::{
    error_active::{Active, ErrorActive, InputActive},
    qr_transfers::ContentDerivations,
};

#[cfg(any(feature = "active", feature = "signer"))]
use crate::{
    db_transactions::TrDbCold,
    helpers::{make_batch_clear_tree, open_db, open_tree, upd_id_batch},
    manage_history::events_to_batch,
    network_details::get_all_networks,
};
#[cfg(feature = "signer")]
use crate::{
    db_transactions::TrDbColdDerivations,
    helpers::{get_address_details, get_network_specs},
    interface_signer::addresses_set_seed_name_network,
};

lazy_static! {
// stolen from sp_core
// removed seed phrase part
// last '+' used to be '*', but empty password is an error
    static ref REG_PATH: Regex = Regex::new(r"^(?P<path>(//?[^/]+)*)(///(?P<password>.+))?$").expect("known value");
}

/// Get all identities from database.
/// Function gets used only on the Signer side.
#[cfg(feature = "signer")]
pub fn get_all_addresses(
    database_name: &str,
) -> Result<Vec<(MultiSigner, AddressDetails)>, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let identities = open_tree::<Signer>(&database, ADDRTREE)?;
    let mut out: Vec<(MultiSigner, AddressDetails)> = Vec::new();
    for (address_key_vec, address_entry) in identities.iter().flatten() {
        let address_key = AddressKey::from_ivec(&address_key_vec);
        let (multisigner, address_details) =
            AddressDetails::process_entry_with_key_checked::<Signer>(&address_key, address_entry)?;
        out.push((multisigner, address_details));
    }
    Ok(out)
}

/// Filter identities by given seed_name.
/// Function gets used only on the Signer side.
#[cfg(feature = "signer")]
pub fn get_addresses_by_seed_name(
    database_name: &str,
    seed_name: &str,
) -> Result<Vec<(MultiSigner, AddressDetails)>, ErrorSigner> {
    Ok(get_all_addresses(database_name)?
        .into_iter()
        .filter(|(_, address_details)| address_details.seed_name == seed_name)
        .collect())
}

/// Generate random phrase with given number of words.
/// Function gets used only on the Signer side.
/// Open to user interface.
#[cfg(feature = "signer")]
pub fn generate_random_phrase(words_number: u32) -> Result<String, ErrorSigner> {
    let mnemonic_type = match MnemonicType::for_word_count(words_number as usize) {
        Ok(a) => a,
        Err(e) => {
            return Err(ErrorSigner::AddressGeneration(AddressGeneration::Extra(
                ExtraAddressGenerationSigner::RandomPhraseGeneration(e),
            )))
        }
    };
    let mnemonic = Mnemonic::new(mnemonic_type, Language::English);
    Ok(mnemonic.into_phrase())
}

/// Create address from seed and path and insert it into the database.
/// Helper gets used both on Active side (when generating test cold database with well-known addresses)
/// and on Signer side (when real addresses are actually created by the user).
#[cfg(any(feature = "active", feature = "signer"))]
pub(crate) fn create_address<T: ErrorSource>(
    database_name: &str,
    input_batch_prep: &[(AddressKey, AddressDetails)],
    path: &str,
    network_specs: &NetworkSpecs,
    seed_name: &str,
    seed_phrase: &str,
) -> Result<(Vec<(AddressKey, AddressDetails)>, Vec<Event>), T::Error> {
    let mut output_batch_prep = input_batch_prep.to_vec();
    let mut output_events: Vec<Event> = Vec::new();
    let network_specs_key =
        NetworkSpecsKey::from_parts(&network_specs.genesis_hash, &network_specs.encryption);

    let mut full_address = seed_phrase.to_owned() + path;
    let multisigner = match network_specs.encryption {
        Encryption::Ed25519 => match ed25519::Pair::from_string(&full_address, None) {
            Ok(a) => {
                full_address.zeroize();
                MultiSigner::Ed25519(a.public())
            }
            Err(e) => {
                full_address.zeroize();
                return Err(<T>::address_generation_common(
                    AddressGenerationCommon::SecretString(e),
                ));
            }
        },
        Encryption::Sr25519 => match sr25519::Pair::from_string(&full_address, None) {
            Ok(a) => {
                full_address.zeroize();
                MultiSigner::Sr25519(a.public())
            }
            Err(e) => {
                full_address.zeroize();
                return Err(<T>::address_generation_common(
                    AddressGenerationCommon::SecretString(e),
                ));
            }
        },
        Encryption::Ecdsa => match ecdsa::Pair::from_string(&full_address, None) {
            Ok(a) => {
                full_address.zeroize();
                MultiSigner::Ecdsa(a.public())
            }
            Err(e) => {
                full_address.zeroize();
                return Err(<T>::address_generation_common(
                    AddressGenerationCommon::SecretString(e),
                ));
            }
        },
    };

    let public_key = multisigner_to_public(&multisigner);
    let address_key = AddressKey::from_multisigner(&multisigner);

    let (cropped_path, has_pwd) = match REG_PATH.captures(path) {
        Some(caps) => match caps.name("path") {
            Some(a) => (a.as_str(), caps.name("password").is_some()),
            None => ("", caps.name("password").is_some()),
        },
        None => ("", false),
    };

    let identity_history = IdentityHistory::get(
        seed_name,
        &network_specs.encryption,
        &public_key,
        cropped_path,
        &network_specs.genesis_hash,
    );
    output_events.push(Event::IdentityAdded(identity_history));

    let mut number_in_current = None;

    // check if the same address key already participates in current database transaction
    for (i, (x_address_key, x_address_details)) in output_batch_prep.iter().enumerate() {
        if (x_address_key == &address_key)
            && !x_address_details.network_id.contains(&network_specs_key)
        {
            number_in_current = Some(i);
            break;
        }
    }
    match number_in_current {
        Some(i) => {
            let mut mod_entry = output_batch_prep.remove(i);
            mod_entry.1.network_id.push(network_specs_key);
            output_batch_prep.push(mod_entry);
            Ok((output_batch_prep, output_events))
        }
        None => {
            let database = open_db::<T>(database_name)?;
            let identities = open_tree::<T>(&database, ADDRTREE)?;
            // This address might be already created; maybe we just need to allow its use in another network?
            match identities.get(address_key.key()) {
                Ok(Some(address_entry)) => {
                    let mut address_details = AddressDetails::from_entry_with_key_checked::<T>(
                        &address_key,
                        address_entry,
                    )?;
                    if address_details.path != cropped_path {
                        return Err(<T>::address_generation_common(
                            AddressGenerationCommon::KeyCollision {
                                seed_name: address_details.seed_name,
                            },
                        ));
                    }
                    if !address_details.network_id.contains(&network_specs_key) {
                        address_details.network_id.push(network_specs_key);
                        output_batch_prep.push((address_key, address_details));
                        Ok((output_batch_prep, output_events))
                    } else {
                        Err(<T>::address_generation_common(
                            AddressGenerationCommon::DerivationExists(
                                multisigner,
                                address_details,
                                network_specs_key,
                            ),
                        ))
                    }
                }
                Ok(None) => {
                    let address_details = AddressDetails {
                        seed_name: seed_name.to_string(),
                        path: cropped_path.to_string(),
                        has_pwd,
                        network_id: vec![network_specs_key],
                        encryption: network_specs.encryption.to_owned(),
                    };
                    output_batch_prep.push((address_key, address_details));
                    Ok((output_batch_prep, output_events))
                }
                Err(e) => Err(<T>::db_internal(e)),
            }
        }
    }
}

/// Create addresses for all default paths in all default networks, and insert them in the database
#[cfg(any(feature = "active", feature = "signer"))]
fn populate_addresses<T: ErrorSource>(
    database_name: &str,
    entry_batch: Batch,
    seed_name: &str,
    seed_phrase: &str,
    roots: bool,
) -> Result<(Batch, Vec<Event>), T::Error> {
    // TODO: check zeroize
    let mut identity_adds: Vec<(AddressKey, AddressDetails)> = Vec::new();
    let mut current_events: Vec<Event> = Vec::new();
    let specs_set = get_all_networks::<T>(database_name)?;
    for network_specs in specs_set.iter() {
        if roots {
            let (adds, events) = create_address::<T>(
                database_name,
                &identity_adds,
                "",
                network_specs,
                seed_name,
                seed_phrase,
            )?;
            identity_adds = adds;
            current_events.extend_from_slice(&events);
        }
        if let Ok((adds, events)) = create_address::<T>(
            database_name,
            &identity_adds,
            &network_specs.path_id,
            network_specs,
            seed_name,
            seed_phrase,
        ) {
            identity_adds = adds;
            current_events.extend_from_slice(&events);
        }
    }
    Ok((upd_id_batch(entry_batch, identity_adds), current_events))
}

/// Generate new seed and populate all known networks with default accounts
#[cfg(feature = "signer")]
pub fn try_create_seed(
    seed_name: &str,
    seed_phrase: &str,
    roots: bool,
    database_name: &str,
) -> Result<(), ErrorSigner> {
    let mut events: Vec<Event> = vec![Event::SeedCreated(seed_name.to_string())];
    let (id_batch, add_events) = populate_addresses::<Signer>(
        database_name,
        Batch::default(),
        seed_name,
        seed_phrase,
        roots,
    )?;
    events.extend_from_slice(&add_events);
    TrDbCold::new()
        .set_addresses(id_batch) // add addresses just made in populate_addresses
        .set_history(events_to_batch::<Signer>(database_name, events)?) // add corresponding history
        .apply::<Signer>(database_name)
}

/// Function removes address by multisigner identifier and and network id
/// Function removes network_key from network_id vector for database record with address_key corresponding to given public key
#[cfg(feature = "signer")]
pub fn remove_key(
    database_name: &str,
    multisigner: &MultiSigner,
    network_specs_key: &NetworkSpecsKey,
) -> Result<(), ErrorSigner> {
    remove_keys_set(database_name, &[multisigner.to_owned()], network_specs_key)
}

/// Function removes a set of addresses within one network by set of multisigner identifier and and network id
/// Function removes network_key from network_id vector for a defined set of database records
#[cfg(feature = "signer")]
pub fn remove_keys_set(
    database_name: &str,
    multiselect: &[MultiSigner],
    network_specs_key: &NetworkSpecsKey,
) -> Result<(), ErrorSigner> {
    let mut id_batch = Batch::default();
    let mut events: Vec<Event> = Vec::new();
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    for multisigner in multiselect.iter() {
        let public_key = multisigner_to_public(multisigner);
        let address_key = AddressKey::from_multisigner(multisigner);
        let mut address_details = get_address_details(database_name, &address_key)?;
        let identity_history = IdentityHistory::get(
            &address_details.seed_name,
            &network_specs.encryption,
            &public_key,
            &address_details.path,
            &network_specs.genesis_hash,
        );
        events.push(Event::IdentityRemoved(identity_history));
        address_details.network_id = address_details
            .network_id
            .into_iter()
            .filter(|id| id != network_specs_key)
            .collect();
        if address_details.network_id.is_empty() {
            id_batch.remove(address_key.key())
        } else {
            id_batch.insert(address_key.key(), address_details.encode())
        }
    }
    TrDbCold::new()
        .set_addresses(id_batch) // modify existing address entries
        .set_history(events_to_batch::<Signer>(database_name, events)?) // add corresponding history
        .apply::<Signer>(database_name)
}

/// Add a bunch of new derived addresses, N+1, N+2, etc.
#[cfg(feature = "signer")]
pub fn create_increment_set(
    increment: u32,
    multisigner: &MultiSigner,
    network_specs_key: &NetworkSpecsKey,
    seed_phrase: &str,
    database_name: &str,
) -> Result<(), ErrorSigner> {
    let address_details =
        get_address_details(database_name, &AddressKey::from_multisigner(multisigner))?;
    let existing_identities = addresses_set_seed_name_network(
        database_name,
        &address_details.seed_name,
        network_specs_key,
    )?;
    let mut last_index = 0;
    for (_, details) in existing_identities.iter() {
        if let Some(("", suffix)) = details.path.split_once(&address_details.path) {
            if let Some(could_be_number) = suffix.get(2..) {
                if let Ok(index) = could_be_number.parse::<u32>() {
                    last_index = std::cmp::max(index + 1, last_index);
                }
            }
        }
    }
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let mut identity_adds: Vec<(AddressKey, AddressDetails)> = Vec::new();
    let mut current_events: Vec<Event> = Vec::new();
    for i in 0..increment {
        let path = address_details.path.to_string() + "//" + &(last_index + i).to_string();
        let (adds, events) = create_address::<Signer>(
            database_name,
            &identity_adds,
            &path,
            &network_specs,
            &address_details.seed_name,
            seed_phrase,
        )?;
        identity_adds = adds;
        current_events.extend_from_slice(&events);
    }
    let id_batch = upd_id_batch(Batch::default(), identity_adds);
    TrDbCold::new()
        .set_addresses(id_batch) // add created address
        .set_history(events_to_batch::<Signer>(database_name, current_events)?) // add corresponding history
        .apply::<Signer>(database_name)
}

/// Check derivation format and determine whether there is a password
#[cfg(feature = "signer")]
pub fn is_passworded(path: &str) -> Result<bool, ErrorSigner> {
    match REG_PATH.captures(path) {
        Some(caps) => Ok(caps.name("password").is_some()),
        None => Err(ErrorSigner::AddressGeneration(AddressGeneration::Extra(
            ExtraAddressGenerationSigner::InvalidDerivation,
        ))),
    }
}

/// Enum to describe derivation proposed
#[cfg(feature = "signer")]
pub enum DerivationCheck {
    BadFormat, // bad format, still gets json back into UI
    Password,  // passworded, no dynamic checking for collisions
    NoPassword(Option<(MultiSigner, AddressDetails)>), // no password and optional collision
}

/// Function to check if proposed address collides,
/// i.e. if the proposed path for given seed name and network specs key already exists
#[cfg(feature = "signer")]
pub fn derivation_check(
    seed_name: &str,
    path: &str,
    network_specs_key: &NetworkSpecsKey,
    database_name: &str,
) -> Result<DerivationCheck, ErrorSigner> {
    match is_passworded(path) {
        Ok(true) => Ok(DerivationCheck::Password),
        Ok(false) => {
            let mut found_collision = None;
            for (multisigner, address_details) in get_all_addresses(database_name)?.into_iter() {
                if (address_details.seed_name == seed_name)
                    && (address_details.path == path)
                    && (address_details.network_id.contains(network_specs_key))
                    && (!address_details.has_pwd)
                {
                    found_collision = Some((multisigner, address_details));
                    break;
                }
            }
            Ok(DerivationCheck::NoPassword(found_collision))
        }
        Err(_) => Ok(DerivationCheck::BadFormat),
    }
}

/// Function to cut the password to be verified later in UI.
/// Expects password, if sees no password, returns error.
#[cfg(feature = "signer")]
pub fn cut_path(path: &str) -> Result<(String, String), ErrorSigner> {
    match REG_PATH.captures(path) {
        Some(caps) => {
            let cropped_path = match caps.name("path") {
                Some(a) => a.as_str().to_string(),
                None => "".to_string(),
            };
            match caps.name("password") {
                Some(pwd) => Ok((cropped_path, pwd.as_str().to_string())),
                None => Err(ErrorSigner::Interface(InterfaceSigner::LostPwd)),
            }
        }
        None => Err(ErrorSigner::AddressGeneration(AddressGeneration::Extra(
            ExtraAddressGenerationSigner::InvalidDerivation,
        ))),
    }
}

/// Generate new identity (api for create_address())
/// Function is open to user interface
#[cfg(feature = "signer")]
pub fn try_create_address(
    seed_name: &str,
    seed_phrase: &str,
    path: &str,
    network_specs_key: &NetworkSpecsKey,
    database_name: &str,
) -> Result<(), ErrorSigner> {
    match derivation_check(seed_name, path, network_specs_key, database_name)? {
        DerivationCheck::BadFormat => Err(ErrorSigner::AddressGeneration(
            AddressGeneration::Extra(ExtraAddressGenerationSigner::InvalidDerivation),
        )),
        DerivationCheck::NoPassword(Some((multisigner, address_details))) => Err(
            <Signer>::address_generation_common(AddressGenerationCommon::DerivationExists(
                multisigner,
                address_details,
                network_specs_key.to_owned(),
            )),
        ),
        _ => {
            let network_specs = get_network_specs(database_name, network_specs_key)?;
            let (adds, events) = create_address::<Signer>(
                database_name,
                &Vec::new(),
                path,
                &network_specs,
                seed_name,
                seed_phrase,
            )?;
            let id_batch = upd_id_batch(Batch::default(), adds);
            TrDbCold::new()
                .set_addresses(id_batch) // add created address
                .set_history(events_to_batch::<Signer>(database_name, events)?) // add corresponding history
                .apply::<Signer>(database_name)
        }
    }
}

/// Function to generate identities batch with Alice information
#[cfg(feature = "active")]
pub fn generate_test_identities(database_name: &str) -> Result<(), ErrorActive> {
    let (id_batch, events) = {
        let entry_batch = make_batch_clear_tree::<Active>(database_name, ADDRTREE)?;
        let mut events = vec![Event::IdentitiesWiped];
        let (mut id_batch, new_events) = populate_addresses::<Active>(
            database_name,
            entry_batch,
            "Alice",
            ALICE_SEED_PHRASE,
            true,
        )?;
        events.extend_from_slice(&new_events);
        for network_specs in default_chainspecs().iter() {
            if (network_specs.name == "westend")
                && (network_specs.encryption == Encryption::Sr25519)
            {
                let (adds, updated_events) = create_address::<Active>(
                    database_name,
                    &Vec::new(),
                    "//Alice",
                    network_specs,
                    "Alice",
                    ALICE_SEED_PHRASE,
                )?;
                id_batch = upd_id_batch(id_batch, adds);
                events.extend_from_slice(&updated_events);
            }
        }
        (id_batch, events)
    };
    TrDbCold::new()
        .set_addresses(id_batch) // add created address
        .set_history(events_to_batch::<Active>(database_name, events)?) // add corresponding history
        .apply::<Active>(database_name)
}

/// Function to remove all identities associated with given seen_name
/// Function is open to the user interface
#[cfg(feature = "signer")]
pub fn remove_seed(database_name: &str, seed_name: &str) -> Result<(), ErrorSigner> {
    let mut identity_batch = Batch::default();
    let mut events: Vec<Event> = Vec::new();
    let id_set = get_addresses_by_seed_name(database_name, seed_name)?;
    for (multisigner, address_details) in id_set.iter() {
        let address_key = AddressKey::from_multisigner(multisigner);
        identity_batch.remove(address_key.key());
        let public_key = multisigner_to_public(multisigner);
        for network_specs_key in address_details.network_id.iter() {
            let (genesis_hash_vec, _) = network_specs_key.genesis_hash_encryption::<Signer>(
                SpecsKeySource::AddrTree(address_key.to_owned()),
            )?;
            let identity_history = IdentityHistory::get(
                seed_name,
                &address_details.encryption,
                &public_key,
                &address_details.path,
                &genesis_hash_vec,
            );
            events.push(Event::IdentityRemoved(identity_history));
        }
    }
    TrDbCold::new()
        .set_addresses(identity_batch) // modify addresses
        .set_history(events_to_batch::<Signer>(database_name, events)?) // add corresponding history
        .apply::<Signer>(database_name)
}

/// Function to import derivations without password into Signer from qr code
/// i.e. create new (address key + address details) entry,
/// or add network specs key to the existing one
#[cfg(feature = "signer")]
pub fn import_derivations(
    checksum: u32,
    seed_name: &str,
    seed_phrase: &str,
    database_name: &str,
) -> Result<(), ErrorSigner> {
    let content_derivations = TrDbColdDerivations::from_storage(database_name, checksum)?;
    let network_specs = content_derivations.network_specs();
    let mut adds: Vec<(AddressKey, AddressDetails)> = Vec::new();
    let mut events: Vec<Event> = Vec::new();
    for path in content_derivations.checked_derivations().iter() {
        match create_address::<Signer>(
            database_name,
            &adds,
            path,
            network_specs,
            seed_name,
            seed_phrase,
        ) {
            Ok((mod_adds, mod_events)) => {
                adds = mod_adds;
                events.extend_from_slice(&mod_events);
            }
            Err(ErrorSigner::AddressGeneration(AddressGeneration::Common(
                AddressGenerationCommon::DerivationExists(_, _, _),
            ))) => (),
            Err(e) => return Err(e),
        }
    }
    let identity_batch = upd_id_batch(Batch::default(), adds);
    let transaction_batch = make_batch_clear_tree::<Signer>(database_name, TRANSACTION)?;
    TrDbCold::new()
        .set_addresses(identity_batch) // modify addresses
        .set_history(events_to_batch::<Signer>(database_name, events)?) // add corresponding history
        .set_transaction(transaction_batch) // clear transaction tree
        .apply::<Signer>(database_name)
}

/// Function to check derivations before offering user to import them
#[cfg(feature = "signer")]
pub fn check_derivation_set(derivations: &[String]) -> Result<(), ErrorSigner> {
    for path in derivations.iter() {
        match REG_PATH.captures(path) {
            Some(caps) => {
                if caps.name("password").is_some() {
                    return Err(ErrorSigner::Input(InputSigner::OnlyNoPwdDerivations));
                }
            }
            None => {
                return Err(ErrorSigner::Input(InputSigner::InvalidDerivation(
                    path.to_string(),
                )))
            }
        }
    }
    Ok(())
}

/// Function to prepare derivations export using regex and a text provided by the user
#[cfg(feature = "active")]
pub fn prepare_derivations_export(
    encryption: &Encryption,
    genesis_hash: &[u8; 32],
    content: &str,
) -> Result<ContentDerivations, ErrorActive> {
    let mut derivations: Vec<String> = Vec::new();
    let content_set: Vec<&str> = content.trim().split('\n').collect();
    for path in content_set.iter() {
        if let Some(caps) = REG_PATH.captures(path) {
            if let Some(p) = caps.name("path") {
                if caps.name("password").is_none() {
                    derivations.push(p.as_str().to_string())
                }
            }
        }
    }
    if derivations.is_empty() {
        return Err(ErrorActive::Input(InputActive::NoValidDerivationsToExport));
    } else {
        println!(
            "Found and used {} valid password-free derivations:",
            derivations.len()
        );
        for x in derivations.iter() {
            println!("\"{}\"", x);
        }
    }
    Ok(ContentDerivations::generate(
        encryption,
        genesis_hash,
        &derivations,
    ))
}
