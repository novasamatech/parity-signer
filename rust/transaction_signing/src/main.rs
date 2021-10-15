use transaction_signing::handle_stub;
use constants::COLD_DB_NAME;


fn main() {
    let mock_stub = "3665731191";
//    let seed_phrase = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
//    let pwd_entry = "jaskier";
//    let user_comment = "";
    match handle_stub(&mock_stub, COLD_DB_NAME) {
        Ok(()) => println!("Success"),
        Err(e) => println!("Error. {}", e),
    }
}

