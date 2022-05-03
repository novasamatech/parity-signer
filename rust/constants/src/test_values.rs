/// Identicons and related pictures, vector of png data

/// Empty 30x30 transparent png image,
/// used in cases when identicon generation failed or public key does not exist
pub const fn empty_png() -> &'static [u8] {
    include_bytes!("empty_png")
}

/// Real Parity verifier identicon
pub const fn real_parity_verifier() -> &'static [u8] {
    include_bytes!("real_parity_verifier")
}

/// Alice root key identicon, Sr25519 encryption
pub const fn alice_sr_root() -> &'static [u8] {
    include_bytes!("alice_sr_root")
}

/// Alice key with derivation "//0", Sr25519 encryption
pub const fn alice_sr_0() -> &'static [u8] {
    include_bytes!("alice_sr_0")
}

/// Alice key with derivation "//1", Sr25519 encryption
pub const fn alice_sr_1() -> &'static [u8] {
    include_bytes!("alice_sr_1")
}

/// Alice key with derivation "//Alice", Sr25519 encryption
pub const fn alice_sr_alice() -> &'static [u8] {
    include_bytes!("alice_sr_alice")
}

/// Alice key with derivation "//kusama", Sr25519 encryption
pub const fn alice_sr_kusama() -> &'static [u8] {
    include_bytes!("alice_sr_kusama")
}

/// Alice key with derivation "//polkadot", Sr25519 encryption
pub const fn alice_sr_polkadot() -> &'static [u8] {
    include_bytes!("alice_sr_polkadot")
}

/// Alice key with derivation "//westend", Sr25519 encryption
pub const fn alice_sr_westend() -> &'static [u8] {
    include_bytes!("alice_sr_westend")
}

/// Alice key with derivation "//westend//0", Sr25519 encryption
pub const fn alice_sr_westend_0() -> &'static [u8] {
    include_bytes!("alice_sr_westend_0")
}

/// Alice key with derivation "//westend//1", Sr25519 encryption
pub const fn alice_sr_westend_1() -> &'static [u8] {
    include_bytes!("alice_sr_westend_1")
}

/// Alice key with derivation "//westend//2", Sr25519 encryption
pub const fn alice_sr_westend_2() -> &'static [u8] {
    include_bytes!("alice_sr_westend_2")
}

/// Alice key with derivation "//secret///abracadabra", Sr25519 encryption
pub const fn alice_sr_secret_abracadabra() -> &'static [u8] {
    include_bytes!("alice_sr_secret_abracadabra")
}

/// Alice key with derivation "//secret//path///multipass", Sr25519 encryption
pub const fn alice_sr_secret_path_multipass() -> &'static [u8] {
    include_bytes!("alice_sr_secret_path_multipass")
}

/// Alice key with derivation "//Alice/secret//secret", Sr25519 encryption
pub const fn alice_sr_alice_secret_secret() -> &'static [u8] {
    include_bytes!("alice_sr_alice_secret_secret")
}

/// Alice key with derivation "//Alice/westend", Sr25519 encryption
pub const fn alice_sr_alice_westend() -> &'static [u8] {
    include_bytes!("alice_sr_alice_westend")
}

/// Identicon for kusama9130 metadata hash
pub const fn kusama_9130() -> &'static [u8] {
    include_bytes!("kusama_9130")
}

/// Identicon for kusama9151 metadata hash
pub const fn kusama_9151() -> &'static [u8] {
    include_bytes!("kusama_9151")
}

/// Identicon for westend9000 metadata hash
pub const fn westend_9000() -> &'static [u8] {
    include_bytes!("westend_9000")
}

/// Identicon for westend9010 metadata hash
pub const fn westend_9010() -> &'static [u8] {
    include_bytes!("westend_9010")
}

/// Identicon for westend9070 metadata hash
pub const fn westend_9070() -> &'static [u8] {
    include_bytes!("westend_9070")
}

/// Identicon for westend9111 metadata hash
pub const fn westend_9111() -> &'static [u8] {
    include_bytes!("westend_9111")
}

/// Identicon for westend9122 metadata hash
pub const fn westend_9122() -> &'static [u8] {
    include_bytes!("westend_9122")
}

/// Identicon for westend9150 metadata hash
pub const fn westend_9150() -> &'static [u8] {
    include_bytes!("westend_9150")
}

/// Identicon for dock31 metadata hash
pub const fn dock_31() -> &'static [u8] {
    include_bytes!("dock_31")
}

/// Identicon for shell200 metadata hash
pub const fn shell_200() -> &'static [u8] {
    include_bytes!("shell_200")
}

/// Identicons for test addresses
pub const fn id_01() -> &'static [u8] {
    include_bytes!("id_01")
}
pub const fn id_02() -> &'static [u8] {
    include_bytes!("id_02")
}
pub const fn id_03() -> &'static [u8] {
    include_bytes!("id_03")
}
pub const fn id_04() -> &'static [u8] {
    include_bytes!("id_04")
}
pub const fn id_05() -> &'static [u8] {
    include_bytes!("id_05")
}

/// Identicon for hash of types information
pub const fn types_known() -> &'static [u8] {
    include_bytes!("types_known")
}

/// Identicon for hash of unknown to the database types information
pub const fn types_unknown() -> &'static [u8] {
    include_bytes!("types_unknown")
}

/// Identicon for Alice key with derivation "//Bob", aka Bob, Sr25519 encryption
pub const fn bob() -> &'static [u8] {
    include_bytes!("bob")
}

/// Identicon for empty vector hashed, value encountered in card tests
pub const fn empty_vec_hash_pic() -> &'static [u8] {
    include_bytes!("empty_vec_hash_pic")
}

/// Alice key with derivation "//Alice", Ed25519 encryption
pub const fn ed() -> &'static [u8] {
    include_bytes!("ed")
}

/// Export qr code for root Alice address in westend network
pub const fn alice_westend_root_qr() -> &'static [u8] {
    include_bytes!("alice_westend_root_qr")
}

/// Export qr code for Alice key with "//westend" derivation in westend network
pub const fn alice_westend_westend_qr() -> &'static [u8] {
    include_bytes!("alice_westend_westend_qr")
}

/// Export qr code for Alice key with "//Alice" derivation in westend network
pub const fn alice_westend_alice_qr() -> &'static [u8] {
    include_bytes!("alice_westend_alice_qr")
}

/// Export qr code for Alice key with "//Alice/secret//secret" derivation in westend network
pub const fn alice_westend_alice_secret_secret_qr() -> &'static [u8] {
    include_bytes!("alice_westend_alice_secret_secret_qr")
}

/// Export qr code for Alice //polkadot key in polkadot network
pub const fn alice_polkadot_qr() -> &'static [u8] {
    include_bytes!("alice_polkadot_qr")
}
