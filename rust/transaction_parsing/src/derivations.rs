use db_handling::identities::{get_all_addresses, is_passworded};
use std::collections::{HashMap, HashSet};

use definitions::derivations::{
    DerivedKeyError, DerivedKeyInfo, DerivedKeyPreview, ExportAddrs, ExportAddrsContainer,
    ExportAddrsV1, SeedKeysPreview, SeedKeysPreviewSummary,
};

use db_handling::helpers::get_network_specs;
use definitions::crypto::Encryption;
use definitions::helpers::{
    base58_to_multisigner, make_identicon_from_multisigner, print_multisigner_as_base58_or_eth,
};
use definitions::keyring::NetworkSpecsKey;
use definitions::{helpers::unhex, navigation::TransactionCardSet};
use parity_scale_codec::Decode;
use sp_core::crypto::Zeroize;
use sp_core::{ecdsa, ed25519, sr25519, Pair};
use sp_runtime::MultiSigner;
use std::path::Path;

use crate::cards::Card;
use crate::error::Result;
use crate::{Error, TransactionAction};

pub fn process_derivations<P>(data_hex: &str, _db_path: P) -> Result<TransactionAction>
where
    P: AsRef<Path>,
{
    let data = unhex(data_hex)?;
    let export_info = <ExportAddrs>::decode(&mut &data[3..])?;
    let derivations_card = Card::Derivations(&export_info.into()).card(&mut 0, 0);
    Ok(TransactionAction::Derivations {
        content: Box::new(TransactionCardSet {
            importing_derivations: Some(vec![derivations_card]),
            ..Default::default()
        }),
    })
}

pub fn prepare_derivations_preview<P>(
    db_path: P,
    export_info: ExportAddrsContainer,
    seeds: HashMap<String, String>,
) -> Result<SeedKeysPreviewSummary>
where
    P: AsRef<Path>,
{
    match export_info {
        ExportAddrsContainer::V1 { data: v1 } => prepare_derivations_v1(db_path, v1, seeds),
    }
}

fn prepare_derivations_v1<P>(
    db_path: P,
    export_info: ExportAddrsV1,
    seeds: HashMap<String, String>,
) -> Result<SeedKeysPreviewSummary>
where
    P: AsRef<Path>,
{
    let mut seeds_preview = vec![];
    let mut sr25519_signers = HashMap::new();
    let mut ed25519_signers = HashMap::new();
    let mut ecdsa_signers = HashMap::new();

    let mut existing_addresses: HashSet<String> = HashSet::new();
    for (m, addr) in get_all_addresses(&db_path)? {
        for spec_key in addr.network_id {
            if let Ok(specs) = get_network_specs(&db_path, &spec_key) {
                existing_addresses.insert(print_multisigner_as_base58_or_eth(
                    &m,
                    Some(specs.specs.base58prefix),
                    addr.encryption,
                ));
            }
        }
    }

    for (k, v) in &seeds {
        let sr25519_public = sr25519::Pair::from_phrase(v, None).unwrap().0.public();
        let ed25519_public = ed25519::Pair::from_phrase(v, None).unwrap().0.public();
        let ecdsa_public = ecdsa::Pair::from_phrase(v, None).unwrap().0.public();
        sr25519_signers.insert(sr25519_public, k);
        ed25519_signers.insert(ed25519_public, k);
        ecdsa_signers.insert(ecdsa_public, k);
    }

    for seed_info in export_info.addrs {
        let mut importable_keys = vec![];
        let mut already_existing_keys = vec![];
        let mut non_importable_keys = vec![];

        let seed_name = match seed_info.multisigner {
            MultiSigner::Sr25519(p) => sr25519_signers.get(&p),
            MultiSigner::Ed25519(p) => ed25519_signers.get(&p),
            MultiSigner::Ecdsa(p) => ecdsa_signers.get(&p),
        };
        let seed_name = if let Some(seed_name) = seed_name {
            seed_name
        } else {
            seeds_preview.push(SeedKeysPreview {
                name: seed_info.name,
                multisigner: seed_info.multisigner,
                importable_keys,
                already_existing_keys,
                non_importable_keys,
                is_key_set_missing: true,
            });
            continue;
        };
        let seed_phrase = if let Some(seed_phrase) = seeds.get(seed_name.as_str()) {
            seed_phrase
        } else {
            continue;
        };
        for key_info in seed_info.derived_keys {
            let path = key_info.derivation_path.clone().unwrap_or_default();
            if is_passworded(&path).is_err() {
                non_importable_keys.push(DerivedKeyInfo {
                    derivation_path: path,
                    key_error: DerivedKeyError::BadFormat,
                });
                continue;
            }
            let network_specs_key =
                NetworkSpecsKey::from_parts(&key_info.genesis_hash, &key_info.encryption);
            let specs = if let Ok(specs) = get_network_specs(&db_path, &network_specs_key) {
                specs.specs
            } else {
                non_importable_keys.push(DerivedKeyInfo {
                    derivation_path: path,
                    key_error: DerivedKeyError::NetworkMissing,
                });
                continue;
            };
            let network_title = specs.title;

            // create fixed-length string to avoid reallocation
            let mut full_address = String::with_capacity(seed_phrase.len() + path.len());
            full_address.push_str(seed_phrase);
            full_address.push_str(&path);

            let multisigner_pwdless = match key_info.encryption {
                Encryption::Ed25519 => match ed25519::Pair::from_string(&full_address, None) {
                    Ok(a) => {
                        full_address.zeroize();
                        MultiSigner::Ed25519(a.public())
                    }
                    Err(e) => {
                        full_address.zeroize();
                        return Err(Error::SecretStringError(e));
                    }
                },
                Encryption::Sr25519 => match sr25519::Pair::from_string(&full_address, None) {
                    Ok(a) => {
                        full_address.zeroize();
                        MultiSigner::Sr25519(a.public())
                    }
                    Err(e) => {
                        full_address.zeroize();
                        return Err(Error::SecretStringError(e));
                    }
                },
                Encryption::Ecdsa | Encryption::Ethereum => {
                    match ecdsa::Pair::from_string(&full_address, None) {
                        Ok(a) => {
                            full_address.zeroize();
                            MultiSigner::Ecdsa(a.public())
                        }
                        Err(e) => {
                            full_address.zeroize();
                            return Err(Error::SecretStringError(e));
                        }
                    }
                }
            };
            let ss58_pwdless = print_multisigner_as_base58_or_eth(
                &multisigner_pwdless,
                Some(specs.base58prefix),
                key_info.encryption,
            );
            let has_pwd = ss58_pwdless != key_info.address;
            let multisigner = if has_pwd {
                if let Ok(m) = base58_to_multisigner(&key_info.address, &key_info.encryption) {
                    m
                } else {
                    continue;
                }
            } else {
                multisigner_pwdless
            };

            let identicon = make_identicon_from_multisigner(
                &multisigner,
                key_info.encryption.identicon_style(),
            );

            if existing_addresses.contains(&key_info.address) {
                already_existing_keys.push(DerivedKeyPreview {
                    address: key_info.address.clone(),
                    derivation_path: path,
                    identicon,
                    has_pwd,
                    genesis_hash: key_info.genesis_hash,
                    encryption: key_info.encryption,
                    network_title,
                });
                continue;
            };

            importable_keys.push(DerivedKeyPreview {
                address: key_info.address.clone(),
                derivation_path: path,
                identicon,
                has_pwd,
                genesis_hash: key_info.genesis_hash,
                encryption: key_info.encryption,
                network_title,
            })
        }
        seeds_preview.push(SeedKeysPreview {
            name: (*seed_name).clone(),
            multisigner: seed_info.multisigner,
            importable_keys,
            already_existing_keys,
            non_importable_keys,
            is_key_set_missing: false,
        })
    }
    Ok(SeedKeysPreviewSummary {
        already_existing_key_count: seeds_preview
            .iter()
            .flat_map(|s| &s.already_existing_keys)
            .count() as u32,
        seed_keys: seeds_preview,
    })
}
