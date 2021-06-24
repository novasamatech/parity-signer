use transaction_signing::*;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dbname = "../db_handling/tests/signer_database";
    let mock_action_line = r#"{"type":"sign_transaction","checksum":"3479856399","has_password":"false"}"#;
    let pin = "000000";
    let pwd_entry = "jaskier";
    let signature = handle_action(&mock_action_line, pin, pwd_entry, dbname)?;
    println!("{}", signature);
    Ok(())
}

