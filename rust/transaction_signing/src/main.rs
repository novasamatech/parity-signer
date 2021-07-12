use transaction_signing::*;
use definitions::constants::COLD_DB_NAME;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mock_action_line = r#"{"type":"sign_transaction","checksum":"3684856122"}"#;
    let pin = "000000";
    let pwd_entry = "jaskier";
    let result = handle_action(&mock_action_line, pin, pwd_entry, COLD_DB_NAME)?;
    println!("{}", result);
    Ok(())
}

