use anyhow;
use db_handling::db_transactions::{TrDbColdStub};

mod error;
    use error::Error;
pub mod sign_message;
mod sign_transaction;
    use sign_transaction::create_signature_png;
//mod tests;


pub fn handle_stub (checksum_str: &str, database_name: &str) -> anyhow::Result<()> {
    let checksum = checksum(checksum_str)?;
    TrDbColdStub::from_storage(&database_name, checksum)?
        .apply(&database_name)
}

pub fn handle_sign (checksum_str: &str, seed_phrase: &str, pwd_entry: &str, user_comment: &str, database_name: &str) -> anyhow::Result<String> {
    let checksum = checksum(checksum_str)?;
    create_signature_png(seed_phrase, pwd_entry, user_comment, database_name, checksum)
}

fn checksum (checksum_str: &str) -> anyhow::Result<u32> {
    match checksum_str.parse() {
        Ok(a) => Ok(a),
        Err(_) => return Err(Error::ChecksumNotU32.show()),
    }
}
