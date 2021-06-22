use generate_metadata_message::*;

fn main() {
    let crypto_used = CryptoUsed::None;
    generate_defaults(&crypto_used).unwrap();
}

