use constants::{METATREE, SPECSTREE};
use db_handling::helpers::{get_types, open_db, open_tree};
use definitions::{
    crypto::Encryption,
    error::MetadataError,
    helpers::unhex,
    keyring::{MetaKey, MetaKeyPrefix, NetworkSpecsKey},
    metadata::{MetaSetElement, MetaValues},
    network_specs::{NetworkSpecs, NetworkSpecsToSend, ShortSpecs},
};
use frame_metadata::RuntimeMetadata;
use parser::{method::OlderMeta, MetadataBundle};
use sp_core::{ecdsa, ed25519, sr25519, H256};
use sp_runtime::MultiSigner;
use std::convert::TryInto;

use crate::error::{Error, Result};

/// Function to get the network specs from the database
/// by network name and encryption
pub(crate) fn specs_by_name(
    network_name: &str,
    encryption: &Encryption,
    database_name: &str,
) -> Result<NetworkSpecs> {
    let database = open_db(database_name)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    let mut found_network_specs = None;
    for x in chainspecs.iter().flatten() {
        let network_specs = NetworkSpecs::from_entry_checked(x)?;
        if (network_specs.name == network_name) && (&network_specs.encryption == encryption) {
            match found_network_specs {
                Some(_) => {
                    return Err(Error::SpecsCollision {
                        name: network_name.to_string(),
                        encryption: encryption.to_owned(),
                    })
                }
                None => {
                    found_network_specs = Some(network_specs);
                }
            }
        }
    }
    match found_network_specs {
        Some(a) => Ok(a),
        None => Err(Error::HistoryNetworkSpecs {
            name: network_name.to_string(),
            encryption: encryption.to_owned(),
        }),
    }
}

pub fn find_meta_set(short_specs: &ShortSpecs, database_name: &str) -> Result<Vec<MetaSetElement>> {
    let database = open_db(database_name)?;
    let metadata = open_tree(&database, METATREE)?;
    let mut out: Vec<MetaSetElement> = Vec::new();
    let meta_key_prefix = MetaKeyPrefix::from_name(&short_specs.name);
    for x in metadata.scan_prefix(meta_key_prefix.prefix()).flatten() {
        let new_element = MetaSetElement::from_entry(x)?;
        if let Some(found_now) = new_element.optional_base58prefix() {
            if found_now != short_specs.base58prefix {
                return Err(MetadataError::Base58PrefixSpecsMismatch {
                    specs: short_specs.base58prefix,
                    meta: found_now,
                }
                .into());
            }
        }
        out.push(new_element);
    }
    out.sort_by_key(|b| std::cmp::Reverse(b.version()));
    Ok(out)
}

pub fn bundle_from_meta_set_element<'a>(
    meta_set_element: &'a MetaSetElement,
    database_name: &'a str,
) -> Result<MetadataBundle<'a>> {
    match meta_set_element.runtime_metadata() {
        RuntimeMetadata::V12(ref meta_v12) => Ok(MetadataBundle::Older {
            older_meta: OlderMeta::V12(meta_v12),
            types: get_types(database_name)?,
            network_version: meta_set_element.version(),
        }),
        RuntimeMetadata::V13(ref meta_v13) => Ok(MetadataBundle::Older {
            older_meta: OlderMeta::V13(meta_v13),
            types: get_types(database_name)?,
            network_version: meta_set_element.version(),
        }),
        RuntimeMetadata::V14(ref meta_v14) => Ok(MetadataBundle::Sci {
            meta_v14,
            network_version: meta_set_element.version(),
        }),
        _ => Err(MetadataError::VersionIncompatible.into()),
    }
}

pub fn accept_meta_values(meta_values: &MetaValues, database_name: &str) -> Result<bool> {
    let meta_key = MetaKey::from_parts(&meta_values.name, meta_values.version);
    let database = open_db(database_name)?;
    let metadata = open_tree(&database, METATREE)?;
    match metadata.get(meta_key.key())? {
        Some(a) => {
            if a == meta_values.meta {
                Ok(false)
            } else {
                Err(Error::SameNameVersionDifferentMeta {
                    name: meta_values.name.to_string(),
                    version: meta_values.version,
                })
            }
        }
        None => Ok(true),
    }
}

/// Function to check if the chaispecs are already in the database
pub fn specs_are_new(new: &NetworkSpecsToSend, database_name: &str) -> Result<bool> {
    let network_specs_key = NetworkSpecsKey::from_parts(&new.genesis_hash, &new.encryption);
    let database = open_db(database_name)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    match chainspecs.get(network_specs_key.key())? {
        Some(encoded_known_network_specs) => {
            let old = NetworkSpecs::from_entry_with_key_checked(
                &network_specs_key,
                encoded_known_network_specs,
            )?;
            if (old.base58prefix != new.base58prefix)
                | (old.decimals != new.decimals)
                | (old.encryption != new.encryption)
                | (old.name != new.name)
                | (old.unit != new.unit)
            {
                return Err(Error::ImportantSpecsChanged(network_specs_key));
            }
            let is_known = (old.color == new.color)
                && (old.logo == new.logo)
                && (old.path_id == new.path_id)
                && (old.secondary_color == new.secondary_color)
                && (old.title == new.title);
            Ok(!is_known)
        }
        None => Ok(true),
    }
}

/// function to process hex data and get from it author_public_key, encryption,
/// data to process (either transaction to parse or message to decode),
/// and network specs key
pub fn multisigner_msg_genesis_encryption(
    data_hex: &str,
) -> Result<(MultiSigner, Vec<u8>, H256, Encryption)> {
    let data = unhex(data_hex)?;
    let (multi_signer, data, encryption) = match &data_hex[2..4] {
        "00" => match data.get(3..35) {
            Some(a) => (
                MultiSigner::Ed25519(ed25519::Public::from_raw(
                    a.try_into().expect("static length"),
                )),
                &data[35..],
                Encryption::Ed25519,
            ),
            None => return Err(Error::TooShort),
        },
        "01" => match data.get(3..35) {
            Some(a) => (
                MultiSigner::Sr25519(sr25519::Public::from_raw(
                    a.try_into().expect("static length"),
                )),
                &data[35..],
                Encryption::Sr25519,
            ),
            None => return Err(Error::TooShort),
        },
        "02" => match data.get(3..36) {
            Some(a) => (
                MultiSigner::Ecdsa(ecdsa::Public::from_raw(
                    a.try_into().expect("static length"),
                )),
                &data[36..],
                Encryption::Ecdsa,
            ),
            None => return Err(Error::TooShort),
        },
        _ => return Err(Error::EncryptionNotSupported(data_hex[2..4].to_string())),
    };
    if data.len() < 32 {
        return Err(Error::TooShort);
    }
    // network genesis hash
    let raw_hash: [u8; 32] = data[data.len() - 32..]
        .try_into()
        .map_err(|_| Error::TooShort)?;
    let genesis_hash_vec = H256::from(raw_hash);
    let msg = data[..data.len() - 32].to_vec();
    Ok((multi_signer, msg, genesis_hash_vec, encryption))
}
