//! Hexadecimal strings with identicons and qr codes data, as encountered in
//! test jsons throughout the workspace

/// Empty `30x30` transparent PNG image,
/// used in cases when identicon generation failed or public key does not exist
pub const fn empty_png() -> &'static [u8] {
    include_bytes!("empty_png")
}

/// Identicon for Alice root key, `Sr25519` encryption
pub const fn alice_sr_root() -> &'static [u8] {
    include_bytes!("alice_sr_root.png")
}

/// Identicon for Alice key with derivation `//0`, `Sr25519` encryption
pub const fn alice_sr_0() -> &'static [u8] {
    include_bytes!("alice_sr_0.png")
}

/// Identicon for Alice key with derivation `//1`, `Sr25519` encryption
pub const fn alice_sr_1() -> &'static [u8] {
    include_bytes!("alice_sr_1.png")
}

/// Identicon for Alice key with derivation `//Alice`, `Sr25519` encryption
pub const fn alice_sr_alice() -> &'static [u8] {
    include_bytes!("alice_sr_alice.png")
}

/// Identicon for Alice key with derivation `//kusama`, `Sr25519` encryption
pub const fn alice_sr_kusama() -> &'static [u8] {
    include_bytes!("alice_sr_kusama.png")
}

/// Identicon for Alice key with derivation `//polkadot`, `Sr25519` encryption
pub const fn alice_sr_polkadot() -> &'static [u8] {
    include_bytes!("alice_sr_polkadot.png")
}

/// Identicon for Alice key with derivation `//westend`, `Sr25519` encryption
pub const fn alice_sr_westend() -> &'static [u8] {
    include_bytes!("alice_sr_westend.png")
}

/// Identicon for Alice key with derivation `//westend//0`, `Sr25519` encryption
pub const fn alice_sr_westend_0() -> &'static [u8] {
    include_bytes!("alice_sr_westend_0.png")
}

/// Identicon for Alice key with derivation `//secret///abracadabra`, `Sr25519`
/// encryption
pub const fn alice_sr_secret_abracadabra() -> &'static [u8] {
    include_bytes!("alice_sr_secret_abracadabra.png")
}

/// Identicon for Alice key with derivation `//secret//path///multipass`, `Sr25519`
/// encryption
pub const fn alice_sr_secret_path_multipass() -> &'static [u8] {
    include_bytes!("alice_sr_secret_path_multipass.png")
}

/// Identicon for Alice key with derivation `//Alice/secret//secret`, `Sr25519`
/// encryption
pub const fn alice_sr_alice_secret_secret() -> &'static [u8] {
    include_bytes!("alice_sr_alice_secret_secret.png")
}

/// Identicon for Alice key with derivation `//Alice/westend`, `Sr25519`
/// encryption
pub const fn alice_sr_alice_westend() -> &'static [u8] {
    include_bytes!("alice_sr_alice_westend.png")
}

/// Identicon for `kusama9130` metadata hash
pub const fn kusama_9130() -> &'static [u8] {
    include_bytes!("kusama_9130.png")
}

/// Identicon for `kusama9151` metadata hash
pub const fn kusama_9151() -> &'static [u8] {
    include_bytes!("kusama_9151.png")
}

/// Identicon for `westend9000` metadata hash
pub const fn westend_9000() -> &'static [u8] {
    include_bytes!("westend_9000.png")
}

/// Identicon for `westend9010` metadata hash
pub const fn westend_9010() -> &'static [u8] {
    include_bytes!("westend_9010.png")
}

/// Identicon for `westend9070` metadata hash
pub const fn westend_9070() -> &'static [u8] {
    include_bytes!("westend_9070.png")
}

/// Identicon for `westend9111` metadata hash
pub const fn westend_9111() -> &'static [u8] {
    include_bytes!("westend_9111.png")
}

/// Identicon for `westend9122` metadata hash
pub const fn westend_9122() -> &'static [u8] {
    include_bytes!("westend_9122.png")
}

/// Identicon for `dock31` metadata hash
pub const fn dock_31() -> &'static [u8] {
    include_bytes!("dock_31.png")
}

/// Identicon for `shell200` metadata hash
pub const fn shell_200() -> &'static [u8] {
    include_bytes!("shell_200.png")
}

/// Identicon for test address `id_01`
pub const fn id_01() -> &'static [u8] {
    include_bytes!("id_01.png")
}
/// Identicon for test address `id_02`
pub const fn id_02() -> &'static [u8] {
    include_bytes!("id_02.png")
}
/// Identicon for test address `id_04`
pub const fn id_04() -> &'static [u8] {
    include_bytes!("id_04.png")
}
/// Identicon for test address `id_05`
pub const fn id_05() -> &'static [u8] {
    include_bytes!("id_05.png")
}

/// Identicon for hash of types information
pub const fn types_known() -> &'static [u8] {
    include_bytes!("types_known.png")
}

/// Identicon for hash of unknown to the database types information
pub const fn types_unknown() -> &'static [u8] {
    include_bytes!("types_unknown.png")
}

/// Identicon for Alice key with derivation `//Bob`, aka Bob, `Sr25519` encryption
pub const fn bob() -> &'static [u8] {
    include_bytes!("bob.png")
}

/// Identicon for Alice key with derivation `//Alice`, `Ed25519` encryption
pub const fn ed() -> &'static [u8] {
    include_bytes!("ed.png")
}

/// Export qr code for root Alice secret in westend network
pub const fn alice_westend_secret_qr() -> &'static [u8] {
    include_bytes!("alice_westend_derivation_secret.png")
}
