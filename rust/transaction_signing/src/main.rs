use transaction_signing::*;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dbname = "../db_handling/tests/signer_database";
    let mock_action_line = r#"{"checksum":"1076431204","has_password":"false"}"#;
    let pin = "000000";
    let pwd_entry = "jaskier";
    let signature = create_signature(&mock_action_line, pin, pwd_entry, dbname)?;
    println!("{}", signature);
    Ok(())
}

