//! Hexadecimal strings with identicons and qr codes data, as encountered in
//! test jsons throughout the workspace

/// Empty 30x30 transparent png image,
/// used in cases when identicon generation failed or public key does not exist
pub fn empty_png() -> String {
    hex::encode(include_bytes!("empty_png"))
}

/// Real Parity verifier identicon
pub fn real_parity_verifier() -> String {
    hex::encode(include_bytes!("real_parity_verifier"))
}

/// Identicon for Alice root key, Sr25519 encryption
pub fn alice_sr_root() -> String {
    hex::encode(include_bytes!("alice_sr_root"))
}

/// Identicon for Alice key with derivation "//0", Sr25519 encryption
pub fn alice_sr_0() -> String {
    hex::encode(include_bytes!("alice_sr_0"))
}

/// Identicon for Alice key with derivation "//1", Sr25519 encryption
pub fn alice_sr_1() -> String {
    hex::encode(include_bytes!("alice_sr_1"))
}

/// Identicon for Alice key with derivation "//Alice", Sr25519 encryption
pub fn alice_sr_alice() -> String {
    hex::encode(include_bytes!("alice_sr_alice"))
}

/// Identicon for Alice key with derivation "//kusama", Sr25519 encryption
pub fn alice_sr_kusama() -> String {
    hex::encode(include_bytes!("alice_sr_kusama"))
}

/// Identicon for Alice key with derivation "//polkadot", Sr25519 encryption
pub fn alice_sr_polkadot() -> String {
    hex::encode(include_bytes!("alice_sr_polkadot"))
}

/// Identicon for Alice key with derivation "//westend", Sr25519 encryption
pub fn alice_sr_westend() -> String {
    hex::encode(include_bytes!("alice_sr_westend"))
}

/// Identicon for Alice key with derivation "//westend//0", Sr25519 encryption
pub fn alice_sr_westend_0() -> String {
    hex::encode(include_bytes!("alice_sr_westend_0"))
}

/// Identicon for Alice key with derivation "//westend//1", Sr25519 encryption
pub fn alice_sr_westend_1() -> String {
    hex::encode(include_bytes!("alice_sr_westend_1"))
}

/// Identicon for Alice key with derivation "//westend//2", Sr25519 encryption
pub fn alice_sr_westend_2() -> String {
    hex::encode(include_bytes!("alice_sr_westend_2"))
}

/// Identicon for Alice key with derivation "//secret///abracadabra", Sr25519
/// encryption
pub fn alice_sr_secret_abracadabra() -> String {
    hex::encode(include_bytes!("alice_sr_secret_abracadabra"))
}

/// Identicon for Alice key with derivation "//secret//path///multipass", Sr25519
/// encryption
pub fn alice_sr_secret_path_multipass() -> String {
    hex::encode(include_bytes!("alice_sr_secret_path_multipass"))
}

/// Identicon for Alice key with derivation "//Alice/secret//secret", Sr25519
/// encryption
pub fn alice_sr_alice_secret_secret() -> String {
    hex::encode(include_bytes!("alice_sr_alice_secret_secret"))
}

/// Identicon for Alice key with derivation "//Alice/westend", Sr25519
/// encryption
pub fn alice_sr_alice_westend() -> String {
    hex::encode(include_bytes!("alice_sr_alice_westend"))
}

/// Identicon for kusama9130 metadata hash
pub fn kusama_9130() -> String {
    hex::encode(include_bytes!("kusama_9130"))
}

/// Identicon for kusama9151 metadata hash
pub fn kusama_9151() -> String {
    hex::encode(include_bytes!("kusama_9151"))
}

/// Identicon for westend9000 metadata hash
pub fn westend_9000() -> String {
    hex::encode(include_bytes!("westend_9000"))
}

/// Identicon for westend9010 metadata hash
pub fn westend_9010() -> String {
    hex::encode(include_bytes!("westend_9010"))
}

/// Identicon for westend9070 metadata hash
pub fn westend_9070() -> String {
    hex::encode(include_bytes!("westend_9070"))
}

/// Identicon for westend9111 metadata hash
pub fn westend_9111() -> String {
    hex::encode(include_bytes!("westend_9111"))
}

/// Identicon for westend9122 metadata hash
pub fn westend_9122() -> String {
    hex::encode(include_bytes!("westend_9122"))
}

/// Identicon for westend9150 metadata hash
pub fn westend_9150() -> String {
    hex::encode(include_bytes!("westend_9150"))
}

/// Identicon for dock31 metadata hash
pub fn dock_31() -> String {
    hex::encode(include_bytes!("dock_31"))
}

/// Identicon for shell200 metadata hash
pub fn shell_200() -> String {
    hex::encode(include_bytes!("shell_200"))
}

/// Identicon for test address id_01
pub fn id_01() -> String {
    hex::encode(include_bytes!("id_01"))
}

/// Identicon for test address id_02
pub fn id_02() -> String {
    hex::encode(include_bytes!("id_02"))
}

/// Identicon for test address id_03
pub fn id_03() -> String {
    hex::encode(include_bytes!("id_03"))
}

/// Identicon for test address id_04
pub fn id_04() -> String {
    hex::encode(include_bytes!("id_04"))
}

/// Identicon for test address id_05
pub fn id_05() -> String {
    hex::encode(include_bytes!("id_05"))
}

/// Identicon for hash of types information
pub fn types_known() -> String {
    hex::encode(include_bytes!("types_known"))
}

/// Identicon for hash of unknown to the database types information
pub fn types_unknown() -> String {
    hex::encode(include_bytes!("types_unknown"))
}

/// Identicon for Alice key with derivation "//Bob", aka Bob, Sr25519 encryption
pub fn bob() -> String {
    hex::encode(include_bytes!("bob"))
}

/// Identicon for empty vector hashed, value encountered in card tests
pub fn empty_vec_hash_pic() -> String {
    hex::encode(include_bytes!("empty_vec_hash_pic"))
}

/// Identicon for Alice key with derivation "//Alice", Ed25519 encryption
pub fn ed() -> String {
    hex::encode(include_bytes!("ed"))
}

/// Export qr code for root Alice address in westend network
pub fn alice_westend_root_qr() -> String {
    hex::encode(include_bytes!("alice_westend_root_qr"))
}

/// Export qr code for Alice key with "//westend" derivation in westend network
pub fn alice_westend_westend_qr() -> String {
    hex::encode(include_bytes!("alice_westend_westend_qr"))
}

/// Export qr code for Alice key with "//Alice" derivation in westend network
pub fn alice_westend_alice_qr() -> String {
    hex::encode(include_bytes!("alice_westend_alice_qr"))
}

/// Export qr code for Alice key with "//Alice/secret//secret" derivation in
/// westend network
pub fn alice_westend_alice_secret_secret_qr() -> String {
    hex::encode(include_bytes!("alice_westend_alice_secret_secret_qr"))
}

/// Export qr code for Alice key with "//polkadot" derivation in polkadot
/// network
pub fn alice_polkadot_qr() -> String {
    hex::encode(include_bytes!("alice_polkadot_qr"))
}
