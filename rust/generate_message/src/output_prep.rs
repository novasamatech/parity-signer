use constants::{LOAD, SPECS};
use definitions::{
    error_active::ErrorActive,
    network_specs::NetworkSpecsToSend,
    qr_transfers::{ContentAddSpecs, ContentLoadMeta},
};

use crate::metadata_shortcut::MetaShortCut;

/// Function to print in standardly named file a plaintext output ready for signing
/// for `load_meta` type of message.
/// Input is MetaShortCut.
pub fn load_meta_print(shortcut: &MetaShortCut) -> Result<(), ErrorActive> {
    let filename = format!(
        "{}_{}V{}",
        LOAD, shortcut.meta_values.name, shortcut.meta_values.version
    );
    let content = ContentLoadMeta::generate(&shortcut.meta_values.meta, &shortcut.genesis_hash);
    content.write(&filename)
}

/// Function to print `add_specs`
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
