#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

use sp_runtime::MultiSigner;

use db_handling::db_transactions::TrDbColdStub;
use definitions::{keyring::NetworkSpecsKey, navigation::MSCContent, users::AddressDetails};

mod sign_message;
use sign_message::{
    sufficient_crypto_add_specs, sufficient_crypto_load_metadata, sufficient_crypto_load_types,
};
mod sign_transaction;
use sign_transaction::create_signature_png;
#[cfg(test)]
mod tests;

mod error;
pub use error::{Error, Result};

pub fn handle_stub(checksum: u32, database_name: &str) -> Result<()> {
    Ok(TrDbColdStub::from_storage(database_name, checksum)?.apply(database_name)?)
}

pub fn handle_sign(
    checksum: u32,
    seed_phrase: &str,
    pwd_entry: &str,
    user_comment: &str,
    database_name: &str,
) -> Result<Vec<u8>> {
    create_signature_png(
        seed_phrase,
        pwd_entry,
        user_comment,
        database_name,
        checksum,
    )
}

///Possible content to generate sufficient crypto for
#[derive(Debug, Clone)]
pub enum SufficientContent {
    AddSpecs(NetworkSpecsKey),
    LoadMeta(NetworkSpecsKey, u32),
    LoadTypes,
}

pub fn sign_content(
    multisigner: &MultiSigner,
    address_details: &AddressDetails,
    content: SufficientContent,
    database_name: &str,
    seed_phrase: &str,
    pwd_entry: &str,
) -> Result<(Vec<u8>, MSCContent)> {
    match content {
        SufficientContent::AddSpecs(network_specs_key) => sufficient_crypto_add_specs(
            &network_specs_key,
            multisigner,
            address_details,
            database_name,
            seed_phrase,
            pwd_entry,
        ),
        SufficientContent::LoadMeta(network_specs_key, version) => sufficient_crypto_load_metadata(
            &network_specs_key,
            version,
            multisigner,
            address_details,
            database_name,
            seed_phrase,
            pwd_entry,
        ),
        SufficientContent::LoadTypes => sufficient_crypto_load_types(
            multisigner,
            address_details,
            database_name,
            seed_phrase,
            pwd_entry,
        ),
    }
}
