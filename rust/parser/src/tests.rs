use crate::parse_and_display_set;
use definitions::network_specs::ShortSpecs;
use frame_metadata::RuntimeMetadata;
use parity_scale_codec::Decode;

fn metadata(filename: &str) -> RuntimeMetadata {
    let metadata_hex = std::fs::read_to_string(&filename).unwrap();
    let metadata_vec = hex::decode(&metadata_hex.trim()).unwrap()[4..].to_vec();
    RuntimeMetadata::decode(&mut &metadata_vec[..]).unwrap()
}

fn specs() -> ShortSpecs {
    ShortSpecs {
        base58prefix: 42,
        decimals: 12,
        genesis_hash: [
            225, 67, 242, 56, 3, 172, 80, 232, 246, 248, 230, 38, 149, 209, 206, 158, 78, 29, 104,
            170, 54, 193, 205, 44, 253, 21, 52, 2, 19, 243, 66, 62,
        ]
        .into(),
        name: "westend".to_string(),
        unit: "WND".to_string(),
    }
}

#[test]
fn tr_1() {
    let data = hex::decode("4d0210020806000046ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a07001b2c3ef70006050c0008264834504a64ace1373f0c8ed5d57381ddf54a2f67a318fa42b1352681606d00aebb0211dbb07b4d335a657257b8ac5e53794c901e4f616d4a254f2490c43934009ae581fef1fc06828723715731adcf810e42ce4dadad629b1b7fa5c3c144a81d550008009723000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ff").unwrap();
    let reply = parse_and_display_set(&data, &metadata("for_tests/westend9111"), &specs()).unwrap();
    let reply_known = r#"
Method:

pallet: Utility,
  method: batch_all,
    field_name: calls,
      pallet: Staking,
        method: bond,
          field_name: controller,
            enum_variant_name: Id,
              Id: 5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV,
          field_name: value,
            balance: 1.061900000000 WND,
          field_name: payee,
            enum_variant_name: Staked,
      pallet: Staking,
        method: nominate,
          field_name: targets,
            enum_variant_name: Id,
              Id: 5CFPcUJgYgWryPaV1aYjSbTpbTLu42V32Ytw1L9rfoMAsfGh,
            enum_variant_name: Id,
              Id: 5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ,
            enum_variant_name: Id,
              Id: 5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f


Extensions:

era: Mortal, phase: 5, period: 64,
nonce: 2,
tip: 0 pWND,
network: westend9111,
tx_version: 7,
block_hash: 5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ff"#;
    assert!(
        reply == reply_known,
        "Expected: {}\nReceived: {}",
        reply_known,
        reply
    );
}

#[test]
fn tr_2() {
    let data = hex::decode("4d0210020806000046ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a07001b2c3ef70006050c0008264834504a64ace1373f0c8ed5d57381ddf54a2f67a318fa42b1352681606d00aebb0211dbb07b4d335a657257b8ac5e53794c901e4f616d4a254f2490c43934009ae581fef1fc06828723715731adcf810e42ce4dadad629b1b7fa5c3c144a81d550008009723000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ff").unwrap();
    let reply =
        parse_and_display_set(&data, &metadata("for_tests/westend9120"), &specs()).unwrap_err();
    let reply_known = "Network spec version decoded from extensions (9111) differs from the version in metadata (9120).";
    assert!(
        reply == reply_known,
        "Expected: {}\nReceived: {}",
        reply_known,
        reply
    );
}

#[test]
fn tr_3() {
    let data = hex::decode("a40403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480700e8764817b501b8003223000005000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33").unwrap();
    let reply = parse_and_display_set(&data, &metadata("for_tests/westend9010"), &specs()).unwrap();
    let reply_known = "
Method:

pallet: Balances,
  method: transfer_keep_alive,
    varname: dest,
      enum_variant_name: Id,
        Id: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty,
    varname: value,
      balance: 100.000000000 mWND


Extensions:

era: Mortal, phase: 27, period: 64,
nonce: 46,
tip: 0 pWND,
network: westend9010,
tx_version: 5,
block_hash: 538a7d7a0ac17eb6dd004578cb8e238c384a10f57c999a3fa1200409cd9b3f33";
    assert!(
        reply == reply_known,
        "Expected: {}\nReceived: {}",
        reply_known,
        reply
    );
}

#[test]
fn tr_4() {
    let data = hex::decode("9c0403008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480284d717d5031504025a62029723000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e98a8ee9e389043cd8a9954b254d822d34138b9ae97d3b7f50dc6781b13df8d84").unwrap();
    let reply = parse_and_display_set(&data, &metadata("for_tests/westend9111"), &specs()).unwrap();
    let reply_known = "
Method:

pallet: Balances,
  method: transfer_keep_alive,
    field_name: dest,
      enum_variant_name: Id,
        Id: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty,
    field_name: value,
      balance: 100.000000 uWND


Extensions:

era: Mortal, phase: 61, period: 64,
nonce: 261,
tip: 10.000000 uWND,
network: westend9111,
tx_version: 7,
block_hash: 98a8ee9e389043cd8a9954b254d822d34138b9ae97d3b7f50dc6781b13df8d84";
    assert!(
        reply == reply_known,
        "Expected: {}\nReceived: {}",
        reply_known,
        reply
    );
}

#[test]
fn tr_5() {
    let data = hex::decode("2509000115094c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e20436f6e67756520657520636f6e7365717561742061632066656c697320646f6e65632e20547572706973206567657374617320696e7465676572206567657420616c6971756574206e696268207072616573656e742e204e6571756520636f6e76616c6c6973206120637261732073656d70657220617563746f72206e657175652e204e65747573206574206d616c6573756164612066616d6573206163207475727069732065676573746173207365642074656d7075732e2050656c6c656e746573717565206861626974616e74206d6f726269207472697374697175652073656e6563747573206574206e657475732065742e205072657469756d2076756c7075746174652073617069656e206e656320736167697474697320616c697175616d2e20436f6e76616c6c69732061656e65616e20657420746f72746f7220617420726973757320766976657272612e20566976616d757320617263752066656c697320626962656e64756d207574207472697374697175652065742065676573746173207175697320697073756d2e204d616c6573756164612070726f696e206c696265726f206e756e6320636f6e73657175617420696e74657264756d207661726975732e2045022c00a223000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e1b2b0a177ad4f3f93f9a56dae700e938a40201a5beabbda160a74c70e612c66a").unwrap();
    let reply = parse_and_display_set(&data, &metadata("for_tests/westend9122"), &specs()).unwrap();
    let reply_known = "
Method:

pallet: System,
  method: remark,
    field_name: remark,
      text: Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Congue eu consequat ac felis donec. Turpis egestas integer eget aliquet nibh praesent. Neque convallis a cras semper auctor neque. Netus et malesuada fames ac turpis egestas sed tempus. Pellentesque habitant morbi tristique senectus et netus et. Pretium vulputate sapien nec sagittis aliquam. Convallis aenean et tortor at risus viverra. Vivamus arcu felis bibendum ut tristique et egestas quis ipsum. Malesuada proin libero nunc consequat interdum varius. 


Extensions:

era: Mortal, phase: 36, period: 64,
nonce: 11,
tip: 0 pWND,
network: westend9122,
tx_version: 7,
block_hash: 1b2b0a177ad4f3f93f9a56dae700e938a40201a5beabbda160a74c70e612c66a";
    assert!(
        reply == reply_known,
        "Expected: {}\nReceived: {}",
        reply_known,
        reply
    );
}

#[test]
fn tr_6() {
    let data = hex::decode("a80a0000dc621b10081b4b51335553ef8df227feb0327649d00beab6e09c10a1dce973590b00407a10f35a24010000dc07000001000000fc41b9bd8ef8fe53d58c7ea67c794c7ec9a73daf05e6d54b14ff6342c99ba64c5cfeb3e46c080274613bdb80809a3e84fe782ac31ea91e2c778de996f738e620").unwrap();
    let specs_acala = ShortSpecs {
        base58prefix: 10,
        decimals: 12,
        genesis_hash: [
            252, 65, 185, 189, 142, 248, 254, 83, 213, 140, 126, 166, 124, 121, 76, 126, 201, 167,
            61, 175, 5, 230, 213, 75, 20, 255, 99, 66, 201, 155, 166, 76,
        ]
        .into(),
        name: "acala".to_string(),
        unit: "ACA".to_string(),
    };
    let reply =
        parse_and_display_set(&data, &metadata("for_tests/acala2012"), &specs_acala).unwrap();
    let reply_known = r#"
Method:

pallet: Balances,
  method: transfer,
    field_name: dest,
      enum_variant_name: Id,
        Id: 25rZGFcFEWz1d81xB98PJN8LQu5cCwjyazAerGkng5NDuk9C,
    field_name: value,
      balance: 100.000000000000 ACA


Extensions:

era: Mortal, phase: 18, period: 32,
nonce: 0,
tip: 0 pACA,
network: acala2012,
tx_version: 1,
block_hash: 5cfeb3e46c080274613bdb80809a3e84fe782ac31ea91e2c778de996f738e620"#;
    assert!(
        reply == reply_known,
        "Expected: {}\nReceived: {}",
        reply_known,
        reply
    );
}
