use generate_metadata_message::*;

fn main() {
    let meta = String::from("1147658909fa189203");
    let gen_hash = String::from("67dddf2673b69e5f875f6f25277495834398eafd67f492e09f3f3345e003d1b5");
    let crypto_used = CryptoUsed::Sr25519 {pwd: None, full_line: String::from("bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice")};
    let line = create_metadata_transfer(meta, gen_hash, crypto_used).unwrap();
    println!("{}", line);
    

}

