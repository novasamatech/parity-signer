use db_handling::{db_transactions::TrDbColdDerivations, helpers::try_get_network_specs, identities::check_derivation_set};
use definitions::{error::{ErrorSigner, NotFoundSigner, NotHexSigner, Signer}, helpers::unhex, keyring::NetworkSpecsKey, qr_transfers::ContentDerivations};

use crate::Action;
use crate::cards::Card;

pub fn process_derivations(data_hex: &str, database_name: &str) -> Result<Action, ErrorSigner> {
    let data = unhex::<Signer>(&data_hex, NotHexSigner::InputContent)?;
    let content_derivations = ContentDerivations::from_slice(&data[3..]);
    let (encryption, genesis_hash, derivations) = content_derivations.encryption_genhash_derivations()?;
    let network_specs_key = NetworkSpecsKey::from_parts(&genesis_hash.to_vec(), &encryption);
    match try_get_network_specs(database_name, &network_specs_key)? {
        Some(network_specs) => {
            check_derivation_set(&derivations)?;
            let checksum = TrDbColdDerivations::generate(&derivations, &network_specs).store_and_get_checksum(&database_name)?;
            let derivations_card = Card::Derivations(&derivations).card(&mut 0, 0);
            let network_info = format!("\"network_title\":\"{}\",\"network_logo\":\"{}\"", network_specs.title, network_specs.logo);
            Ok(Action::Derivations{content: format!("\"importing_derivations\":[{}]", derivations_card), network_info, checksum})
        },
        None => return Err(ErrorSigner::NotFound(NotFoundSigner::NetworkForDerivationsImport{genesis_hash, encryption})),
    }
}
