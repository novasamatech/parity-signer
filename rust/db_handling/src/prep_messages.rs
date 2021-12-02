use constants::SPECSTREE;
use definitions::{error::{ErrorSigner, ErrorSource, NotFoundSigner, Signer}, network_specs::NetworkSpecs, qr_transfers::ContentLoadTypes};

use crate::helpers::{open_db, open_tree, get_types};


/// Function to get types info from the database.
/// Gets used both on the Active side (when preparing messages containing `load_types` payload)
/// and on the Signer side (when preparing SufficientCrypto export qr code for `load_types` payload)
pub fn prep_types<T: ErrorSource> (database_name: &str) -> Result<ContentLoadTypes, T::Error> {
    Ok(ContentLoadTypes::generate(&get_types::<T>(database_name)?))
}

/// Function to get genesis hash from the Signer database searching by network name.
/// Gets used only on Signer side when preparing SufficientCrypto export qr code for `load_metadata` payload
pub fn get_genesis_hash (network_name: &str, database_name: &str) -> Result<[u8;32], ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let chainspecs = open_tree::<Signer>(&database, SPECSTREE)?;
    let mut found_genesis_hash = None;
    for x in chainspecs.iter() {
        if let Ok(a) = x {
            let network_specs = NetworkSpecs::from_entry_checked::<Signer>(a)?;
            if network_specs.name == network_name {
                found_genesis_hash = Some(network_specs.genesis_hash);
                break;
            }
        }
    }
    match found_genesis_hash {
        Some(a) => Ok(a),
        None => return Err(ErrorSigner::NotFound(NotFoundSigner::NetworkSpecsForName(network_name.to_string()))),
    }
}
