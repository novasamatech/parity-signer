#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

use sp_runtime::MultiSigner;

use db_handling::db_transactions::TrDbColdStub;
use definitions::{
    crypto::Encryption, keyring::NetworkSpecsKey, navigation::MSCContent, users::AddressDetails,
};

mod sign_message;
use sign_message::{
    sufficient_crypto_add_specs, sufficient_crypto_load_metadata, sufficient_crypto_load_types,
};
mod sign_transaction;
#[cfg(test)]
mod tests;

mod error;
pub use error::{Error, Result};

pub use sign_transaction::{create_signature, SignatureAndChecksum, SignatureType};

pub fn handle_stub(database: &sled::Db, checksum: u32) -> Result<()> {
    Ok(TrDbColdStub::from_storage(database, checksum)?.apply(database)?)
}

pub fn handle_sign(
    database: &sled::Db,
    checksum: u32,
    seed_phrase: &str,
    pwd_entry: &str,
    user_comment: &str,
    idx: usize,
    encryption: Encryption,
) -> Result<Vec<u8>> {
    create_signature(
        database,
        seed_phrase,
        pwd_entry,
        user_comment,
        checksum,
        idx,
        encryption,
    )
    .map(|s| s.to_string().as_bytes().to_vec())
}

///Possible content to generate sufficient crypto for
#[derive(Debug, Clone)]
pub enum SufficientContent {
    AddSpecs(NetworkSpecsKey),
    LoadMeta(NetworkSpecsKey, u32),
    LoadTypes,
}

pub fn sign_content(
    database: &sled::Db,
    multisigner: &MultiSigner,
    address_details: &AddressDetails,
    content: SufficientContent,
    seed_phrase: &str,
    pwd_entry: &str,
) -> Result<(Vec<u8>, MSCContent)> {
    match content {
        SufficientContent::AddSpecs(network_specs_key) => sufficient_crypto_add_specs(
            database,
            &network_specs_key,
            multisigner,
            address_details,
            seed_phrase,
            pwd_entry,
        ),
        SufficientContent::LoadMeta(network_specs_key, version) => sufficient_crypto_load_metadata(
            database,
            &network_specs_key,
            version,
            multisigner,
            address_details,
            seed_phrase,
            pwd_entry,
        ),
        SufficientContent::LoadTypes => sufficient_crypto_load_types(
            database,
            multisigner,
            address_details,
            seed_phrase,
            pwd_entry,
        ),
    }
}
