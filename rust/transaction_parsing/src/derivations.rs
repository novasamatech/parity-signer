use db_handling::identities::{ExportAddrs, ExportAddrsV1};

use definitions::derivations::{DerivedKeyPreview, SeedKeysPreview};

use db_handling::helpers::get_network_specs;
use definitions::helpers::{base58_or_eth_to_multisigner, make_identicon_from_multisigner};
use definitions::keyring::NetworkSpecsKey;
use definitions::{helpers::unhex, navigation::TransactionCardSet};
use parity_scale_codec::Decode;
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
    let import_info = prepare_derivations_preview(db_path, export_info);
    let derivations_card = Card::Derivations(&import_info).card(&mut 0, 0);
    Ok(TransactionAction::Derivations {
        content: Box::new(TransactionCardSet {
            importing_derivations: Some(vec![derivations_card]),
            ..Default::default()
        }),
    })
}

pub fn prepare_derivations_preview<P>(db_path: P, export_info: ExportAddrs) -> Vec<SeedKeysPreview>
where
    P: AsRef<Path>,
{
    match export_info {
        ExportAddrs::V1(v1) => prepare_derivations_v1(db_path, v1),
    }
}

fn prepare_derivations_v1<P>(db_path: P, export_info: ExportAddrsV1) -> Vec<SeedKeysPreview>
where
    P: AsRef<Path>,
{
    export_info
        .addrs
        .into_iter()
        .map(|addr| SeedKeysPreview {
            name: addr.name,
            multisigner: addr.multisigner,
            derived_keys: addr
                .derived_keys
                .iter()
                .map(|addr_info| {
                    let multisigner =
                        base58_or_eth_to_multisigner(&addr_info.address, addr_info.encryption)
                            .unwrap();
                    let identicon = make_identicon_from_multisigner(
                        &multisigner,
                        addr_info.encryption.identicon_style(),
                    );
                    let network_specs_key =
                        NetworkSpecsKey::from_parts(&addr_info.genesis_hash, &addr_info.encryption);
                    let network_title = get_network_specs(&db_path, &network_specs_key)
                        .map(|specs| specs.specs.title)
                        .unwrap_or_else(|_| "Unknown".to_string());
                    DerivedKeyPreview {
                        address: addr_info.address.clone(),
                        derivation_path: addr_info.derivation_path.clone(),
                        identicon,
                        has_pwd: false,
                        genesis_hash: addr_info.genesis_hash,
                        encryption: addr_info.encryption,
                        network_title,
                    }
                })
                .collect(),
        })
        .collect()
}
