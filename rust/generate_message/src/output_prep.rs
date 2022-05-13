//! Export `add_specs` and `load_metadata` payloads into files for signing
use constants::{LOAD, SPECS};
use definitions::{
    error_active::ErrorActive,
    network_specs::NetworkSpecsToSend,
    qr_transfers::{ContentAddSpecs, ContentLoadMeta},
};

use crate::metadata_shortcut::MetaShortCut;

/// Write to file raw bytes payload of `load_metadata` update
///
/// Resulting file, located in dedicated [`FOLDER`](constants::FOLDER), could be
/// used to generate data signature and to produce updates.
pub fn load_meta_print(shortcut: &MetaShortCut) -> Result<(), ErrorActive> {
    let filename = format!(
        "{}_{}V{}",
        LOAD, shortcut.meta_values.name, shortcut.meta_values.version
    );
    let content = ContentLoadMeta::generate(&shortcut.meta_values.meta, &shortcut.genesis_hash);
    content.write(&filename)
}

/// Write to file raw bytes payload of `add_specs` update
///
/// Resulting file, located in dedicated [`FOLDER`](constants::FOLDER), could be
/// used to generate data signature and to produce updates.
pub fn print_specs(network_specs: &NetworkSpecsToSend) -> Result<(), ErrorActive> {
    let filename = format!(
        "{}_{}_{}",
        SPECS,
        network_specs.name,
        network_specs.encryption.show()
    );
    let content = ContentAddSpecs::generate(network_specs);
    content.write(&filename)
}
