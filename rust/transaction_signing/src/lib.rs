use sled::{Db, open};

mod accept_metadata;
    use accept_metadata::{accept_metadata, add_meta_verifier};
mod accept_types;
    use accept_types::{accept_types, add_types_verifier};
mod interpretation;
    use interpretation::{get_checksum, get_action_type, ActionType};
mod sign_transaction;
    use sign_transaction::create_signature;

/// Function process action card from RN.
/// Currently supported action type cards are:
/// - sign_transaction: in case of success, creates signature for transaction
/// - load_metadata: in case of success, loads the received metadata in the database

pub fn handle_action (action_line: &str, pin: &str, pwd_entry: &str, dbname: &str) -> Result<String, Box<dyn std::error::Error>> {

// get checksum from action line
    let checksum = get_checksum(action_line)?;

// open the actual database and get the actual checksum
    let database: Db = open(dbname)?;
    let real_checksum = database.checksum()?;
    
    if checksum != real_checksum {return Err(Box::from("Database checksum mismatch."))}

// get action type from action line
    let action_type = get_action_type(action_line)?;
    
    match action_type {
        ActionType::SignTransaction => create_signature(pin, pwd_entry, database),
        ActionType::LoadMetadata => accept_metadata(database),
        ActionType::AddMetadataVerifier => add_meta_verifier(database),
        ActionType::LoadTypes => accept_types(database),
        ActionType::AddTypesVerifier => add_types_verifier(database),
    }
}
