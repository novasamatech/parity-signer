use constants::SPECSTREE;
use definitions::{error::ErrorSource, network_specs::NetworkSpecs};

use crate::helpers::{open_db, open_tree};

/// Function to get network specs for all networks in Signer database.
/// Applicable only to the Signer database, but could be used also on Active side.
pub fn get_all_networks<T: ErrorSource> (database_name: &str) -> Result<Vec<NetworkSpecs>, T::Error> {
    let database = open_db::<T>(database_name)?;
    let chainspecs = open_tree::<T>(&database, SPECSTREE)?;
    let mut out: Vec<NetworkSpecs> = Vec::new();
    for x in chainspecs.iter() {if let Ok(a) = x {out.push(NetworkSpecs::from_entry_checked::<T>(a)?)}}
    Ok(out)
}

