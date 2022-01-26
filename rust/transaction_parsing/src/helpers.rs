use constants::{METATREE, SPECSTREE};
use db_handling::helpers::{open_db, open_tree, get_types};
use definitions::{crypto::Encryption, error::{DatabaseSigner, ErrorSigner, ErrorSource, InputSigner, MetadataError, MetadataSource, NotFoundSigner, NotHexSigner, Signer}, helpers::unhex, keyring::{MetaKey, MetaKeyPrefix, NetworkSpecsKey}, metadata::{MetaValues, MetaSetElement}, network_specs::{NetworkSpecs, NetworkSpecsToSend, ShortSpecs}};
use frame_metadata::RuntimeMetadata;
use parser::{MetadataBundle, method::OlderMeta};
use sp_core::{ed25519, sr25519, ecdsa};
use sp_runtime::MultiSigner;
use std::convert::TryInto;

/// Function to get the network specs from the database
/// by network name and encryption
pub (crate) fn specs_by_name (network_name: &str, encryption: &Encryption, database_name: &str) -> Result<NetworkSpecs, ErrorSigner> {
    let database = open_db::<Signer>(&database_name)?;
    let chainspecs = open_tree::<Signer>(&database, SPECSTREE)?;
    let mut found_network_specs = None;
    for x in chainspecs.iter() {
        if let Ok(a) = x {
            let network_specs = NetworkSpecs::from_entry_checked::<Signer>(a)?;
            if (network_specs.name == network_name)&&(&network_specs.encryption == encryption) {
                match found_network_specs {
                    Some(_) => return Err(ErrorSigner::Database(DatabaseSigner::SpecsCollision{name: network_name.to_string(), encryption: encryption.to_owned()})),
                    None => {found_network_specs = Some(network_specs);},
                }
            }
        }
    }
    match found_network_specs {
        Some(a) => Ok(a),
        None => return Err(ErrorSigner::NotFound(NotFoundSigner::HistoryNetworkSpecs{name: network_name.to_string(), encryption: encryption.to_owned()})),
    }
}

pub fn find_meta_set(short_specs: &ShortSpecs, database_name: &str) -> Result<Vec<MetaSetElement>, ErrorSigner> {
    let database = open_db::<Signer>(&database_name)?;
    let metadata = open_tree::<Signer>(&database, METATREE)?;
    let mut out: Vec<MetaSetElement> = Vec::new();
    let meta_key_prefix = MetaKeyPrefix::from_name(&short_specs.name);
    for x in metadata.scan_prefix(meta_key_prefix.prefix()) {
        if let Ok(a) = x {
            let new_element = MetaSetElement::from_entry(a)?;
            if let Some(found_now) = new_element.optional_base58prefix {
                if found_now != short_specs.base58prefix {return Err(<Signer>::faulty_metadata(MetadataError::Base58PrefixSpecsMismatch{specs: short_specs.base58prefix, meta: found_now}, MetadataSource::Database{name: short_specs.name.to_string(), version: new_element.version}))}
            }
            out.push(new_element);
        }
    }
    out.sort_by(|a, b| b.version.cmp(&a.version));
    Ok(out)
}

pub fn bundle_from_meta_set_element <'a> (meta_set_element: &'a MetaSetElement, database_name: &'a str) -> Result<MetadataBundle <'a>, ErrorSigner> {
    match meta_set_element.runtime_metadata {
        RuntimeMetadata::V12(ref meta_v12) => Ok(MetadataBundle::Older{older_meta: OlderMeta::V12(&meta_v12), types: get_types::<Signer>(database_name)?, network_version: meta_set_element.version}),
        RuntimeMetadata::V13(ref meta_v13) => Ok(MetadataBundle::Older{older_meta: OlderMeta::V13(&meta_v13), types: get_types::<Signer>(database_name)?, network_version: meta_set_element.version}),
        RuntimeMetadata::V14(ref meta_v14) => Ok(MetadataBundle::Sci{meta_v14: &meta_v14, network_version: meta_set_element.version}),
        _ => return Err(<Signer>::faulty_metadata(MetadataError::VersionIncompatible, MetadataSource::Database{name: meta_set_element.name.to_string(), version: meta_set_element.version})),
    }
}

pub fn accept_meta_values (meta_values: &MetaValues, database_name: &str) -> Result<bool, ErrorSigner> {
    let meta_key = MetaKey::from_parts(&meta_values.name, meta_values.version);
    let database = open_db::<Signer>(&database_name)?;
    let metadata = open_tree::<Signer>(&database, METATREE)?;
    match metadata.get(meta_key.key()) {
        Ok(Some(a)) => {
            if a == meta_values.meta {Ok(false)}
            else {return Err(ErrorSigner::Input(InputSigner::SameNameVersionDifferentMeta{name: meta_values.name.to_string(), version: meta_values.version}))}
        },
        Ok(None) => Ok(true),
        Err(e) => return Err(<Signer>::db_internal(e)),
    }
}

/// Function to check if the chaispecs are already in the database
pub fn specs_are_new (new: &NetworkSpecsToSend, database_name: &str) -> Result<bool, ErrorSigner> {
    let network_specs_key = NetworkSpecsKey::from_parts(&new.genesis_hash.to_vec(), &new.encryption);
    let database = open_db::<Signer>(&database_name)?;
    let chainspecs = open_tree::<Signer>(&database, SPECSTREE)?;
    match chainspecs.get(network_specs_key.key()) {
        Ok(Some(encoded_known_network_specs)) => {
            let old = NetworkSpecs::from_entry_with_key_checked::<Signer>(&network_specs_key, encoded_known_network_specs)?;
            if (old.base58prefix != new.base58prefix)|(old.decimals != new.decimals)|(old.encryption != new.encryption)|(old.name != new.name)|(old.unit != new.unit) {return Err(ErrorSigner::Input(InputSigner::ImportantSpecsChanged(network_specs_key.to_owned())))}
            let is_known = (old.color == new.color) && (old.logo == new.logo) && (old.path_id == new.path_id) && (old.secondary_color == new.secondary_color) && (old.title == new.title);
            Ok(!is_known)
        },
        Ok(None) => Ok(true),
        Err(e) => return Err(<Signer>::db_internal(e)),
    }
}

/// function to process hex data and get from it author_public_key, encryption,
/// data to process (either transaction to parse or message to decode),
/// and network specs key
pub fn multisigner_msg_genesis_encryption (data_hex: &str) -> Result<(MultiSigner, Vec<u8>, Vec<u8>, Encryption), ErrorSigner> {
    let data = unhex::<Signer>(&data_hex, NotHexSigner::InputContent)?;
    let (multi_signer, data, encryption) = match &data_hex[2..4] {
        "00" => match data.get(3..35) {
            Some(a) => (MultiSigner::Ed25519(ed25519::Public::from_raw(a.to_vec().try_into().expect("static length"))), &data[35..], Encryption::Ed25519),
            None => return Err(ErrorSigner::Input(InputSigner::TooShort)),
        },
        "01" => match data.get(3..35) {
            Some(a) => (MultiSigner::Sr25519(sr25519::Public::from_raw(a.to_vec().try_into().expect("static length"))), &data[35..], Encryption::Sr25519),
            None => return Err(ErrorSigner::Input(InputSigner::TooShort)),
        },
        "02" => match data.get(3..36) {
            Some(a) => (MultiSigner::Ecdsa(ecdsa::Public::from_raw(a.to_vec().try_into().expect("static length"))), &data[36..], Encryption::Ecdsa),
            None => return Err(ErrorSigner::Input(InputSigner::TooShort)),
        },
        _ => return Err(ErrorSigner::Input(InputSigner::EncryptionNotSupported(data_hex[2..4].to_string()))),
    };
    if data.len()<32 {return Err(ErrorSigner::Input(InputSigner::TooShort))}
    let genesis_hash_vec = data[data.len()-32..].to_vec(); // network genesis hash
    let msg = data[..data.len()-32].to_vec();
    Ok((multi_signer, msg, genesis_hash_vec, encryption))
}

