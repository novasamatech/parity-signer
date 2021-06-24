use generate_message::*;

fn main() {
    let crypto_used = CryptoUsed::None;
    let dbname = "../db_handling/tests/signer_database";
    generate_types_default(dbname, &crypto_used).unwrap();
//    generate_metadata_defaults(&crypto_used).unwrap();
}

