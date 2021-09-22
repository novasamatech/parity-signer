use definitions::{constants::{ADD, ADDDEF, LOAD}, metadata::MetaValues};
use parity_scale_codec::Encode;

use super::metadata_shortcut::{MetaShortCut, MetaSpecsShortCut};


/// Function to print in standardly named file a plaintext output ready for signing
/// for `load_meta` type of message.
/// Input is MetaShortCut.

pub fn load_meta_print (shortcut: &MetaShortCut) -> Result<(), Box<dyn std::error::Error>> {
    
    let filename = format!("{}_{}V{}", LOAD, shortcut.meta_values.name, shortcut.meta_values.version);
    let contents = [shortcut.meta_values.meta.to_vec(), shortcut.genesis_hash.to_vec()].concat();
    std::fs::write(&filename, &contents)?;
    Ok(())
    
}


/// Function to print in standardly named file a plaintext output ready for signing
/// for `load_meta` type of message.
/// Input is MetaSpecsShortCut.

pub fn load_meta_print_specs (shortcut_specs: &MetaSpecsShortCut) -> Result<(), Box<dyn std::error::Error>> {
    
    let shortcut = MetaShortCut {
        meta_values: MetaValues {
            name: shortcut_specs.meta_values.name.to_string(),
            version: shortcut_specs.meta_values.version,
            meta: shortcut_specs.meta_values.meta.to_vec(),
        },
        genesis_hash: shortcut_specs.specs.genesis_hash,
    };
    load_meta_print (&shortcut)
    
}


/// Function to print in standardly named file a plaintext output ready for signing
/// for `add_network` type of message.
/// Input is MetaSpecsShortCut.

pub fn add_network_print (shortcut: &MetaSpecsShortCut) -> Result<(), Box<dyn std::error::Error>> {
    
    let filename = {
        if shortcut.def {format!("{}_{}V{}", ADDDEF, shortcut.meta_values.name, shortcut.meta_values.version)} // fresh fetch and some defaults were used
        else {format!("{}_{}V{}", ADD, shortcut.meta_values.name, shortcut.meta_values.version)} // chainspecs from the database were used
    };
    let contents = [shortcut.meta_values.meta.encode(), shortcut.specs.encode()].concat();
    std::fs::write(&filename, &contents)?;
    Ok(())
    
}


/// Function to print `load_meta` (flag = true) or `add_network` (flag = false)

pub fn print_it (flag: bool, shortcut: &MetaSpecsShortCut) -> Result<(), Box<dyn std::error::Error>> {
    
    if flag {load_meta_print_specs(&shortcut)}
    else {add_network_print(&shortcut)}
    
}
