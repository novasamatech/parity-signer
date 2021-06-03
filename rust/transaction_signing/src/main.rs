use transaction_signing::*;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mock_action_line = r#"{"author":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","encrypted_seed":"{\"cipher\":\"aes-128-ctr\",\"cipherparams\":{\"iv\":\"e6cab0784f978c02c32fad9a284dbb6d\"},\"ciphertext\":\"9d69b246bbdeb0ca0393a2f33cf9ac14b1392b118d5591ef49cc7950d84b5a56e5e0f233cc6cc47407d8e0820fe0f794bd915bf4000b25c8410b251c8a7f4597c19d38dae2\",\"kdf\":\"pbkdf2\",\"kdfparams\":{\"c\":10240,\"dklen\":32,\"prf\":\"hmac-sha256\",\"salt\":\"2375ff331156311830759bfa6aa02d146036222b492d4a8f0bb7ae5959865ee4\"},\"mac\":\"65656a24bd60819cdfff4f8f7deac8151655ff8d9403037137e8acd6dc167445\"}","derivation_path":"//Alice","has_password":"false","name":"","network":"westend","version":"9030","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","prefix":"42","transaction":"a40400008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48070010a5d4e8b5018501004623000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423ece699db44863f18ac2a5a51dc798c7bca9e93b10c999a4e64c9afde1370cb8f9","crypto":"sr25519"}"#;
        //r#"{"author": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "crypto": "ed25519", "derivation_path": "//Alice", "encrypted_seed": "{\"cipher\":\"aes-128-ctr\",\"cipherparams\":{\"iv\":\"e6cab0784f978c02c32fad9a284dbb6d\"},\"ciphertext\":\"9d69b246bbdeb0ca0393a2f33cf9ac14b1392b118d5591ef49cc7950d84b5a56e5e0f233cc6cc47407d8e0820fe0f794bd915bf4000b25c8410b251c8a7f4597c19d38dae2\",\"kdf\":\"pbkdf2\",\"kdfparams\":{\"c\":10240,\"dklen\":32,\"prf\":\"hmac-sha256\",\"salt\":\"2375ff331156311830759bfa6aa02d146036222b492d4a8f0bb7ae5959865ee4\"},\"mac\":\"65656a24bd60819cdfff4f8f7deac8151655ff8d9403037137e8acd6dc167445\"}", "genesis_hash": "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e", "has_password": "false", "name": "", "network": "westend", "prefix": "42", "transaction": "a40400008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48070010a5d4e8b5018501004623000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423ece699db44863f18ac2a5a51dc798c7bca9e93b10c999a4e64c9afde1370cb8f9", "version": "9030"}"#;
    let pin = "000000";
    let pwd_entry = "";
    let signature = create_signature(&mock_action_line, pin, pwd_entry)?;
    println!("{}", signature);
    Ok(())
}

