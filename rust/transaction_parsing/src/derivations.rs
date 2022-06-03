use db_handling::{
    db_transactions::TrDbColdDerivations, helpers::try_get_network_specs,
    identities::check_derivation_set,
};
use definitions::{
    error_signer::{ErrorSigner, NotFoundSigner, NotHexSigner, Signer},
    helpers::unhex,
    keyring::NetworkSpecsKey,
    navigation::TransactionCardSet,
    qr_transfers::ContentDerivations,
};

use crate::cards::Card;
use crate::TransactionAction;

pub fn process_derivations(
    data_hex: &str,
    database_name: &str,
) -> Result<TransactionAction, ErrorSigner> {
    let data = unhex::<Signer>(data_hex, NotHexSigner::InputContent)?;
    let content_derivations = ContentDerivations::from_slice(&data[3..]);
    let (encryption, genesis_hash, derivations) =
        content_derivations.encryption_genhash_derivations()?;
    let network_specs_key = NetworkSpecsKey::from_parts(&genesis_hash, &encryption);
    match try_get_network_specs(database_name, &network_specs_key)? {
        Some(network_specs) => {
            check_derivation_set(&derivations)?;
            let checksum = TrDbColdDerivations::generate(&derivations, &network_specs)
                .store_and_get_checksum(database_name)?;
            let derivations_card = Card::Derivations(&derivations).card(&mut 0, 0);
            let network_info = network_specs;
            Ok(TransactionAction::Derivations {
                content: TransactionCardSet {
                    importing_derivations: Some(vec![derivations_card]),
                    ..Default::default()
                },
                network_info,
                checksum,
                network_specs_key,
            })
        }
        None => Err(ErrorSigner::NotFound(
            NotFoundSigner::NetworkForDerivationsImport {
                genesis_hash,
                encryption,
            },
        )),
    }
}
