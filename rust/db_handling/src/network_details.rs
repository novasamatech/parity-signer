use constants::SPECSTREE;
use definitions::{error::{ErrorSigner, Signer}, network_specs::{NetworkSpecs}};

use crate::helpers::{open_db, open_tree};

/// Function to get network specs for all networks in Signer database.
/// Applicable only to the Signer side.
pub fn get_all_networks (database_name: &str) -> Result<Vec<NetworkSpecs>, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let chainspecs = open_tree::<Signer>(&database, SPECSTREE)?;
    let mut out: Vec<NetworkSpecs> = Vec::new();
    for x in chainspecs.iter() {if let Ok(a) = x {out.push(NetworkSpecs::from_entry_checked::<Signer>(a)?)}}
    Ok(out)
}

