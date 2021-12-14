use db_handling::db_transactions::{TrDbColdStub};
use definitions::error::ErrorSigner;

pub mod sign_message;
mod sign_transaction;
    use sign_transaction::create_signature_png;
mod tests;


pub fn handle_stub (checksum: u32, database_name: &str) -> Result<(), ErrorSigner> {
    TrDbColdStub::from_storage(&database_name, checksum)?
        .apply(database_name)
}

pub fn handle_sign (checksum: u32, seed_phrase: &str, pwd_entry: &str, user_comment: &str, database_name: &str) -> Result<String, ErrorSigner> {
    create_signature_png(seed_phrase, pwd_entry, user_comment, database_name, checksum)
}


