use transaction_parsing::cards::Action;

mod accept_metadata;
    use accept_metadata::{accept_metadata, add_meta_verifier};
mod accept_network;
    use accept_network::add_network;
mod accept_types;
    use accept_types::{accept_types, add_general_verifier};
mod interpretation;
    use interpretation::interpret_action;
mod sign_transaction;
    use sign_transaction::create_signature;
mod tests;

/// Function process action card from RN.

pub fn handle_action (action_line: &str, seed_phrase: &str, pwd_entry: &str, dbname: &str) -> Result<String, Box<dyn std::error::Error>> {

    let action = interpret_action (action_line)?;
    
    match action {
        Action::SignTransaction(checksum) => create_signature(seed_phrase, pwd_entry, dbname, checksum),
        Action::LoadMetadata(checksum) => accept_metadata(dbname, checksum, false),
        Action::AddMetadataVerifier(checksum) => add_meta_verifier(dbname, checksum, false),
        Action::LoadTypes(checksum) => accept_types(dbname, checksum),
        Action::AddGeneralVerifier(checksum) => add_general_verifier(dbname, checksum),
        Action::AddTwoVerifiers(checksum) => add_meta_verifier(dbname, checksum, true),
        Action::LoadMetadataAndAddGeneralVerifier(checksum) => accept_metadata (dbname, checksum, true),
        Action::AddNetwork(checksum) => add_network (dbname, checksum, false),
        Action::AddNetworkAndAddGeneralVerifier(checksum) => add_network (dbname, checksum, true),
    }
}
