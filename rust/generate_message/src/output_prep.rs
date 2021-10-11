use constants::{ADD, LOAD, SPECS};
use definitions::{network_specs::ChainSpecsToSend, qr_transfers::{ContentLoadMeta, ContentAddNetwork, ContentAddSpecs}};
//use parity_scale_codec::Encode;
use anyhow;

use crate::metadata_shortcut::{MetaShortCut, MetaSpecsShortCut};
use crate::error::Error;


/// Function to print in standardly named file a plaintext output ready for signing
/// for `load_meta` type of message.
/// Input is MetaShortCut.
pub fn load_meta_print (shortcut: &MetaShortCut) -> anyhow::Result<()> {
    let filename = format!("{}_{}V{}", LOAD, shortcut.meta_values.name, shortcut.meta_values.version);
    let content = ContentLoadMeta::generate(&shortcut.meta_values.meta, &shortcut.genesis_hash);
    match content.write(&filename) {
        Ok(_) => Ok(()),
        Err(e) => return Err(Error::InputOutputError(e).show()),
    }
}


/// Function to print in standardly named file a plaintext output ready for signing
/// for `add_network` type of message.
/// Input is MetaSpecsShortCut.
pub fn add_network_print (shortcut: &MetaSpecsShortCut) -> anyhow::Result<()> {
    let filename = format!("{}_{}V{}", ADD, shortcut.meta_values.name, shortcut.meta_values.version);
    let content = ContentAddNetwork::generate(&shortcut.meta_values.meta, &shortcut.specs);
    match content.write(&filename) {
        Ok(_) => Ok(()),
        Err(e) => return Err(Error::InputOutputError(e).show()),
    }
}


/// Function to print `add_specs`
pub fn print_specs (network_specs: &ChainSpecsToSend) -> anyhow::Result<()> {
    let filename = format!("{}_{}_{}", SPECS, network_specs.name, network_specs.encryption.show());
    let content = ContentAddSpecs::generate(network_specs);
    match content.write(&filename) {
        Ok(_) => Ok(()),
        Err(e) => return Err(Error::InputOutputError(e).show()),
    }
}
