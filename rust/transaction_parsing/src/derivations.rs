use db_handling::identities::{get_all_addresses, is_passworded, ExportAddrs, ExportAddrsV1};

use definitions::derivations::{
    DerivedKeyError, DerivedKeyPreview, DerivedKeyStatus, SeedKeysPreview,
};

use db_handling::helpers::get_network_specs;
use definitions::helpers::{base58_or_eth_to_multisigner, make_identicon_from_multisigner};
use definitions::keyring::NetworkSpecsKey;
use definitions::{helpers::unhex, navigation::TransactionCardSet};
use parity_scale_codec::Decode;
use sp_runtime::MultiSigner;
use std::path::Path;

use crate::cards::Card;
use crate::error::Result;
use crate::TransactionAction;

pub fn process_derivations<P>(data_hex: &str, db_path: P) -> Result<TransactionAction>
where
    P: AsRef<Path>,
{
    let data = unhex(data_hex)?;
    let export_info = <ExportAddrs>::decode(&mut &data[3..])?;
    let import_info = prepare_derivations_preview(db_path, export_info)?;
    let derivations_card = Card::Derivations(&import_info).card(&mut 0, 0);
    Ok(TransactionAction::Derivations {
        content: Box::new(TransactionCardSet {
            importing_derivations: Some(vec![derivations_card]),
            ..Default::default()
        }),
    })
}

pub fn prepare_derivations_preview<P>(
    db_path: P,
    export_info: ExportAddrs,
) -> Result<Vec<SeedKeysPreview>>
where
    P: AsRef<Path>,
{
    match export_info {
        ExportAddrs::V1(v1) => prepare_derivations_v1(db_path, v1),
    }
}

fn prepare_derivations_v1<P>(db_path: P, export_info: ExportAddrsV1) -> Result<Vec<SeedKeysPreview>>
where
    P: AsRef<Path>,
{
    let mut result = Vec::new();
    for seed_info in export_info.addrs {
        let mut derived_keys = vec![];
        for addr_info in seed_info.derived_keys {
            let multisigner =
                base58_or_eth_to_multisigner(&addr_info.address, &addr_info.encryption)?;
            let identicon = make_identicon_from_multisigner(
                &multisigner,
                addr_info.encryption.identicon_style(),
            );
            let network_specs_key =
                NetworkSpecsKey::from_parts(&addr_info.genesis_hash, &addr_info.encryption);
            let network_title = get_network_specs(&db_path, &network_specs_key)
                .map(|specs| specs.specs.title)
                .ok();
            let path = addr_info.derivation_path.clone().unwrap_or_default();
            let status = get_derivation_status(
                &path,
                &network_title,
                &seed_info.multisigner,
                &multisigner,
                &db_path,
            )?;
            derived_keys.push(DerivedKeyPreview {
                address: addr_info.address.clone(),
                derivation_path: addr_info.derivation_path,
                identicon,
                has_pwd: None, // unknown at this point
                genesis_hash: addr_info.genesis_hash,
                encryption: addr_info.encryption,
                network_title,
                status,
            })
        }
        result.push(SeedKeysPreview {
            name: seed_info.name,
            multisigner: seed_info.multisigner,
            derived_keys,
        })
    }
    Ok(result)
}

fn get_derivation_status<P>(
    path: &str,
    network_title: &Option<String>,
    seed_multisigner: &MultiSigner,
    key_multisigner: &MultiSigner,
    db_path: P,
) -> Result<DerivedKeyStatus>
where
    P: AsRef<Path>,
{
    let mut seed_found = false;
    // FIXME: nested loop, should be optimized
    for (m, _) in get_all_addresses(db_path)?.into_iter() {
        if m == *seed_multisigner {
            seed_found = true;
        }
        if m == *key_multisigner {
            return Ok(DerivedKeyStatus::AlreadyExists);
        }
    }

    let mut errors = vec![];
    if is_passworded(path).is_err() {
        errors.push(DerivedKeyError::BadFormat);
    }
    if network_title.is_none() {
        errors.push(DerivedKeyError::NetworkMissing);
    }
    if !seed_found {
        errors.push(DerivedKeyError::KeySetMissing);
    }
    if !errors.is_empty() {
        return Ok(DerivedKeyStatus::Invalid { errors });
    }
    Ok(DerivedKeyStatus::Importable)
}
