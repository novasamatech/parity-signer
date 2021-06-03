use hex;
use regex::Regex;
use sp_core::{Pair, sr25519};
use ethsign::{keyfile::Crypto, Protected};
use serde_json;

fn main() {
    let mock_action_line = r#""action":{"type":"sign_transaction","payload":{"author":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","encrypted_seed":"{\"cipher\":\"aes-128-ctr\",\"cipherparams\":{\"iv\":\"e6cab0784f978c02c32fad9a284dbb6d\"},\"ciphertext\":\"9d69b246bbdeb0ca0393a2f33cf9ac14b1392b118d5591ef49cc7950d84b5a56e5e0f233cc6cc47407d8e0820fe0f794bd915bf4000b25c8410b251c8a7f4597c19d38dae2\",\"kdf\":\"pbkdf2\",\"kdfparams\":{\"c\":10240,\"dklen\":32,\"prf\":\"hmac-sha256\",\"salt\":\"2375ff331156311830759bfa6aa02d146036222b492d4a8f0bb7ae5959865ee4\"},\"mac\":\"65656a24bd60819cdfff4f8f7deac8151655ff8d9403037137e8acd6dc167445\"}","derivation_path":"//Alice","has_password":"false","name":"","network":"westend","version":"9010","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","prefix":"42","transaction":"ac0403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480f00c06e31d91001","crypto":"sr25519"}}"#;
    let pin = "000000";
    let pwd_entry = "";
    
    
    let re = Regex::new(r#"(?i)"action":\{"type":"sign_transaction","payload":\{.*?"encrypted_seed":(?P<seed>.*?),"derivation_path":"(?P<path>.*?)","has_password":"(?P<has_pwd>(true|false))","name":"(?P<name>.*?)".*"transaction":"(?P<transaction>([a-f0-9][a-f0-9])*)","crypto":"sr25519"\}\}"#).unwrap();
    
    let caps = re.captures(&mock_action_line).unwrap();
    let seed = caps.name("seed").unwrap().as_str().to_string();
    let has_pwd: bool = caps["has_pwd"].parse().unwrap();
    let path = caps.name("path").unwrap().as_str().to_string();
    
    let transaction_hex = caps.name("transaction").unwrap().as_str().to_string();
    let transaction = hex::decode(&transaction_hex).unwrap();
    
    let password = Protected::new(pin.as_bytes());
    
    let pwd = {
        if has_pwd {Some(pwd_entry)}
        else {None}
    };

    let seed_to_use: String = serde_json::from_str(&seed).unwrap();

    let crypto: Crypto = serde_json::from_str(&seed_to_use).unwrap();

    let decrypted = crypto.decrypt(&password).unwrap();

    let words = String::from_utf8(decrypted).unwrap();
//    println!("{}", words);


    let full_line = format!("{}{}", words, path);
    
    let got_pair = sr25519::Pair::from_string(&full_line, pwd).unwrap();
    
    let signature = got_pair.sign(&transaction[..]);
    println!("{:?}", signature);
    
}

