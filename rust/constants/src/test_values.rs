//! Hexadecimal strings with identicons and qr codes data, as encountered in
//! test jsons throughout the workspace

/// Empty `30x30` transparent PNG image,
/// used in cases when identicon generation failed or public key does not exist
pub fn empty_png() -> Vec<u8> {
    vec![]
}

/// Identicon for Alice root key, `Sr25519` encryption
pub fn alice_sr_root() -> Vec<u8> {
    hex::decode("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a").unwrap()
}

/// Identicon for Alice key with derivation `//0`, `Sr25519` encryption
pub fn alice_sr_0() -> Vec<u8> {
    hex::decode("2afba9278e30ccf6a6ceb3a8b6e336b70068f045c666f2e7f4f9cc5f47db8972").unwrap()
}

/// Identicon for Alice key with derivation `//1`, `Sr25519` encryption
pub fn alice_sr_1() -> Vec<u8> {
    hex::decode("b606fc73f57f03cdb4c932d475ab426043e429cecc2ffff0d2672b0df8398c48").unwrap()
}

/// Identicon for Alice key with derivation `//Alice`, `Sr25519` encryption
pub fn alice_sr_alice() -> Vec<u8> {
    hex::decode("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").unwrap()
}

/// Identicon for Alice key with derivation `//kusama`, `Sr25519` encryption
pub fn alice_sr_kusama() -> Vec<u8> {
    hex::decode("64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05").unwrap()
}

/// Identicon for Alice key with derivation `//polkadot`, `Sr25519` encryption
pub fn alice_sr_polkadot() -> Vec<u8> {
    hex::decode("f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730").unwrap()
}

/// Identicon for Alice ethereum key with derivation `//polkadot`, `Ethereum` encryption
pub fn alice_ethereum_polkadot() -> String {
    "0xe9267b732a8e9c9444e46f3d04d4610a996d682d".to_string()
}

/// Identicon for Alice key with derivation `//westend`, `Sr25519` encryption
pub fn alice_sr_westend() -> Vec<u8> {
    hex::decode("3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34").unwrap()
}

/// Identicon for Alice key with derivation `//westend//0`, `Sr25519` encryption
pub fn alice_sr_westend_0() -> Vec<u8> {
    hex::decode("e655361d12f3ccca5f128187cf3f5eea052be722746e392c8b498d0d18723470").unwrap()
}

/// Identicon for Alice key with derivation `//secret///abracadabra`, `Sr25519`
/// encryption
pub fn alice_sr_secret_abracadabra() -> Vec<u8> {
    hex::decode("76b68c7ad0e084b37d6a0c8c92d792f6041da5e7ff0c6c3d4a2d6b97772ad46e").unwrap()
}

/// Identicon for Alice key with derivation `//secret//path///multipass`, `Sr25519`
/// encryption
pub fn alice_sr_secret_path_multipass() -> Vec<u8> {
    hex::decode("e83f1549880f33524079201c5c7aed839f56c73adb2f61d9b271ae2d692dfe2c").unwrap()
}

/// Identicon for Alice key with derivation `//Alice/secret//secret`, `Sr25519`
/// encryption
pub fn alice_sr_alice_secret_secret() -> Vec<u8> {
    hex::decode("8266a693d6872d2b6437215c198ee25cabf2e4256df9ad00e979e84b00b5235e").unwrap()
}

/// Identicon for Alice key with derivation `//Alice/westend`, `Sr25519`
/// encryption
pub fn alice_sr_alice_westend() -> Vec<u8> {
    hex::decode("9cd20feb68e0535a6c1cdeead4601b652cf6af6d76baf370df26ee25adde0805").unwrap()
}

/// Identicon for `kusama9130` metadata hash
pub fn kusama_9130() -> Vec<u8> {
    hex::decode("88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee").unwrap()
}

/// Identicon for `kusama9151` metadata hash
pub fn kusama_9151() -> Vec<u8> {
    hex::decode("88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee").unwrap()
}

/// Identicon for `westend9000` metadata hash
pub fn westend_9000() -> Vec<u8> {
    hex::decode("e80237ad8b2e92b72fcf6beb8f0e4ba4a21043a7115c844d91d6c4f981e469ce").unwrap()
}

/// Identicon for `westend9010` metadata hash
pub fn westend_9010() -> Vec<u8> {
    hex::decode("70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf").unwrap()
}

/// Identicon for `westend9070` metadata hash
pub fn westend_9070() -> Vec<u8> {
    hex::decode("e281fbc53168a6b87d1ea212923811f4c083e7be7d18df4b8527b9532e5f5fec").unwrap()
}

/// Identicon for `westend9111` metadata hash
pub fn westend_9111() -> Vec<u8> {
    hex::decode("207956815bc7b3234fa8827ef40df5fd2879e93f18a680e22bc6801bca27312d").unwrap()
}

/// Identicon for `westend9122` metadata hash
pub fn westend_9122() -> Vec<u8> {
    hex::decode("d656951f4c58c9fdbe029be33b02a7095abc3007586656be7ff68fd0550d6ced").unwrap()
}

/// Identicon for `dock31` metadata hash
pub fn dock_31() -> Vec<u8> {
    hex::decode("28c25067d5c0c739f64f7779c5f3095ecf57d9075b0c5258f3be2df6d7f323d0").unwrap()
}

/// Identicon for `shell200` metadata hash
pub fn shell_200() -> Vec<u8> {
    hex::decode("65f0d394de10396c6c1800092f9a95c48ec1365d9302dbf5df736c5e0c54fde3").unwrap()
}

/// Identicon for test address `id_01`
pub fn id_01() -> Vec<u8> {
    hex::decode("aebb0211dbb07b4d335a657257b8ac5e53794c901e4f616d4a254f2490c43934").unwrap()
}
/// Identicon for test address `id_02`
pub fn id_02() -> Vec<u8> {
    hex::decode("9ae581fef1fc06828723715731adcf810e42ce4dadad629b1b7fa5c3c144a81d").unwrap()
}
/// Identicon for test address `id_04`
pub fn id_04() -> Vec<u8> {
    hex::decode("08264834504a64ace1373f0c8ed5d57381ddf54a2f67a318fa42b1352681606d").unwrap()
}
/// Identicon for test address `id_05`
pub fn id_05() -> Vec<u8> {
    hex::decode("dc621b10081b4b51335553ef8df227feb0327649d00beab6e09c10a1dce97359").unwrap()
}

/// Identicon for hash of types information
pub fn types_known() -> Vec<u8> {
    hex::decode("d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb").unwrap()
}

/// Identicon for hash of unknown to the database types information
pub fn types_unknown() -> Vec<u8> {
    hex::decode("d2c5b096be10229ce9ea9d219325c4399875b52ceb4264add89b0d7c5e9ad574").unwrap()
}

/// Identicon for Alice key with derivation `//Bob`, aka Bob, `Sr25519` encryption
pub fn bob() -> Vec<u8> {
    hex::decode("8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48").unwrap()
}

/// Identicon for Alice key with derivation `//Alice`, `Ed25519` encryption
pub fn ed() -> Vec<u8> {
    hex::decode("88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee").unwrap()
}

/// Export qr code for root Alice secret in westend network
pub fn alice_westend_secret_qr() -> Vec<u8> {
    hex::decode("88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee").unwrap()
}
