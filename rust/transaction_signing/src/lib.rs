use anyhow;
use db_handling::db_transactions::{TrDbColdStub};
use definitions::error::{ErrorSigner, InterfaceSigner};

mod sign_message;
    use sign_message::{sufficient_crypto_load_types, sufficient_crypto_load_metadata, sufficient_crypto_add_specs};
mod sign_transaction;
    use sign_transaction::create_signature_png;
mod tests;


pub fn handle_stub (checksum_str: &str, database_name: &str) -> anyhow::Result<()> {
    let checksum = checksum(checksum_str)?;
    TrDbColdStub::from_storage(&database_name, checksum).map_err(|e| e.anyhow())?
        .apply(database_name).map_err(|e| e.anyhow())
}

pub fn handle_sign (checksum_str: &str, seed_phrase: &str, pwd_entry: &str, user_comment: &str, database_name: &str) -> anyhow::Result<String> {
    let checksum = checksum(checksum_str)?;
    create_signature_png(seed_phrase, pwd_entry, user_comment, database_name, checksum).map_err(|e| e.anyhow())
}

pub fn sign_load_types (address_key_hex: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    sufficient_crypto_load_types (address_key_hex, database_name, seed_phrase, pwd_entry).map_err(|e| e.anyhow())
}

pub fn sign_load_metadata (network_name: &str, network_version: u32, address_key_hex: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    sufficient_crypto_load_metadata (network_name, network_version, address_key_hex, database_name, seed_phrase, pwd_entry).map_err(|e| e.anyhow())
}

pub fn sign_add_specs (network_specs_key_hex: &str, address_key_hex: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> anyhow::Result<String> {
    sufficient_crypto_add_specs (network_specs_key_hex, address_key_hex, database_name, seed_phrase, pwd_entry).map_err(|e| e.anyhow())
}

fn checksum (checksum_str: &str) -> anyhow::Result<u32> {
    match checksum_str.parse() {
        Ok(a) => Ok(a),
        Err(_) => return Err(ErrorSigner::Interface(InterfaceSigner::ChecksumNotU32).anyhow()),
    }
}
