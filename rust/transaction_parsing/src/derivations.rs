use db_handling::{
    db_transactions::TrDbColdDerivations, helpers::try_get_network_specs,
    identities::check_derivation_set,
};
use definitions::{
    helpers::unhex, keyring::NetworkSpecsKey, navigation::TransactionCardSet,
    qr_transfers::ContentDerivations,
};
use std::path::Path;

use crate::cards::Card;
use crate::error::{Error, Result};
use crate::TransactionAction;

pub fn process_derivations<P>(data_hex: &str, db_path: P) -> Result<TransactionAction>
where
    P: AsRef<Path>,
{
    let data = unhex(data_hex)?;
    let content_derivations = ContentDerivations::from_slice(&data[3..]);
    let (encryption, genesis_hash, derivations) =
        content_derivations.encryption_genhash_derivations()?;
    let network_specs_key = NetworkSpecsKey::from_parts(&genesis_hash, &encryption);
    let network_specs = try_get_network_specs(&db_path, &network_specs_key)?.ok_or(
        Error::NetworkForDerivationsImport {
            genesis_hash,
            encryption,
        },
    )?;
    check_derivation_set(&derivations)?;
    let checksum = TrDbColdDerivations::generate(&derivations, &network_specs)
        .store_and_get_checksum(&db_path)?;
    let derivations_card = Card::Derivations(&derivations).card(&mut 0, 0);
    let network_info = network_specs;
    Ok(TransactionAction::Derivations {
        content: Box::new(TransactionCardSet {
            importing_derivations: Some(vec![derivations_card]),
            ..Default::default()
        }),
        network_info: Box::new(network_info),
        checksum,
        network_specs_key,
    })
}
