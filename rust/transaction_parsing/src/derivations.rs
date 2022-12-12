use db_handling::identities::{ExportAddrs, ExportAddrsV1};

use definitions::derivations::{DerivedKeyPreview, SeedKeysPreview};

use definitions::{helpers::unhex, navigation::TransactionCardSet};
use parity_scale_codec::Decode;
use std::path::Path;

use crate::cards::Card;
use crate::error::Result;
use crate::TransactionAction;

pub fn process_derivations<P>(data_hex: &str, _db_path: P) -> Result<TransactionAction>
where
    P: AsRef<Path>,
{
    let data = unhex(data_hex)?;
    let export_info = <ExportAddrs>::decode(&mut &data[3..])?;
    let import_info = prepare_derivations_preview(export_info);
    let derivations_card = Card::Derivations(&import_info).card(&mut 0, 0);
    Ok(TransactionAction::Derivations {
        content: Box::new(TransactionCardSet {
            importing_derivations: Some(vec![derivations_card]),
            ..Default::default()
        }),
    })
}

pub fn prepare_derivations_preview(export_info: ExportAddrs) -> Vec<SeedKeysPreview> {
    match export_info {
        ExportAddrs::V1(v1) => prepare_derivations_v1(v1),
    }
}

fn prepare_derivations_v1(export_info: ExportAddrsV1) -> Vec<SeedKeysPreview> {
    export_info
        .addrs
        .into_iter()
        .map(|addr| SeedKeysPreview {
            name: addr.name,
            multisigner: addr.multisigner,
            derived_keys: addr
                .derived_keys
                .iter()
                .map(|addr_info| DerivedKeyPreview {
                    address: addr_info.address.clone(),
                    derivation_path: addr_info.derivation_path.clone(),
                    encryption: addr_info.encryption,
                    genesis_hash: addr_info.genesis_hash,
                })
                .collect(),
        })
        .collect()
}
