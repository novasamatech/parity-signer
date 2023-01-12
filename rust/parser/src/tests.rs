use std::str::FromStr;

use crate::{parse_and_display_set, Error};
use definitions::network_specs::ShortSpecs;
use frame_metadata::RuntimeMetadata;
use parity_scale_codec::Decode;
use parity_scale_codec::Encode;
use pretty_assertions::assert_eq;
use sp_core::H256;

use subxt::{
    config::PolkadotConfig,
    ext::{sp_core::crypto::Ss58Codec, sp_runtime::AccountId32},
    tx::{BaseExtrinsicParams, BaseExtrinsicParamsBuilder, Era, ExtrinsicParams, PlainTip},
};

fn metadata(filename: &str) -> RuntimeMetadata {
    let metadata_hex = std::fs::read_to_string(filename).unwrap();
    let metadata_vec = hex::decode(metadata_hex.trim()).unwrap()[4..].to_vec();
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

#[allow(clippy::all)]
#[subxt::subxt(runtime_metadata_path = "for_tests/westend9111.scale")]
mod westend9111 {}

#[subxt::subxt(runtime_metadata_path = "for_tests/westend9122.scale")]
mod westend9122 {}

#[derive(parity_scale_codec::Encode)]
struct Method {
    method: Vec<u8>,
}

#[test]
fn tr_1() {
    use westend9111::runtime_types::{
        pallet_staking::{pallet::pallet::Call as StakingCall, RewardDestination},
        pallet_utility::pallet::Call as UtilityCall,
        westend_runtime::Call,
    };
    let mut calls = Vec::new();

    let staking_call_bond = StakingCall::bond {
        controller: AccountId32::from_ss58check("5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV")
            .unwrap()
            .into(),
        value: 1061900000000,
        payee: RewardDestination::Staked,
    };
    calls.push(Call::Staking(staking_call_bond));

    let staking_call_nominate = StakingCall::nominate {
        targets: [
            "5CFPcUJgYgWryPaV1aYjSbTpbTLu42V32Ytw1L9rfoMAsfGh",
            "5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ",
            "5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f",
        ]
        .iter()
        .map(|addr| AccountId32::from_ss58check(addr).unwrap().into())
        .collect(),
    };

    let genesis_hash =
        H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap();
    let block_hash =
        H256::from_str("5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ff").unwrap();
    let params = BaseExtrinsicParams::<PolkadotConfig, PlainTip>::new(
        9111,
        7,
        2,
        genesis_hash,
        BaseExtrinsicParamsBuilder::new()
            .tip(PlainTip::new(0))
            .era(Era::Mortal(64, 5), block_hash),
    );

    calls.push(Call::Staking(staking_call_nominate));

    let batch = UtilityCall::batch_all { calls };

    let tx = Call::Utility(batch);
    let tx = tx.encode();

    let call = Method { method: tx };

    let mut extensions = vec![];
    params.encode_extra_to(&mut extensions);
    params.encode_additional_to(&mut extensions);
    let mut tx = call.encode();

    tx.extend_from_slice(extensions.as_slice());

    let reply = parse_and_display_set(&tx, &metadata("for_tests/westend9111"), &specs()).unwrap();
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
    assert_eq!(reply, reply_known,);
}

#[test]
fn tr_2() {
    use westend9111::runtime_types::{
        pallet_staking::{pallet::pallet::Call as StakingCall, RewardDestination},
        pallet_utility::pallet::Call as UtilityCall,
        westend_runtime::Call,
    };
    let mut calls = Vec::new();

    let staking_call_bond = StakingCall::bond {
        controller: AccountId32::from_ss58check("5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV")
            .unwrap()
            .into(),
        value: 1061900000000,
        payee: RewardDestination::Staked,
    };
    calls.push(Call::Staking(staking_call_bond));

    let staking_call_nominate = StakingCall::nominate {
        targets: [
            "5CFPcUJgYgWryPaV1aYjSbTpbTLu42V32Ytw1L9rfoMAsfGh",
            "5G1ojzh47Yt8KoYhuAjXpHcazvsoCXe3G8LZchKDvumozJJJ",
            "5FZoQhgUCmqBxnkHX7jCqThScS2xQWiwiF61msg63CFL3Y8f",
        ]
        .iter()
        .map(|addr| AccountId32::from_ss58check(addr).unwrap().into())
        .collect(),
    };

    let genesis_hash =
        H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap();
    let block_hash =
        H256::from_str("5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ff").unwrap();
    let params = BaseExtrinsicParams::<PolkadotConfig, PlainTip>::new(
        9111,
        7,
        2,
        genesis_hash,
        BaseExtrinsicParamsBuilder::new()
            .tip(PlainTip::new(0))
            .era(Era::Mortal(64, 5), block_hash),
    );

    calls.push(Call::Staking(staking_call_nominate));

    let batch = UtilityCall::batch_all { calls };

    let tx = Call::Utility(batch);
    let tx = tx.encode();

    let call = Method { method: tx };

    let mut extensions = vec![];
    params.encode_extra_to(&mut extensions);
    params.encode_additional_to(&mut extensions);
    let mut tx = call.encode();

    tx.extend_from_slice(extensions.as_slice());

    let reply =
        parse_and_display_set(&tx, &metadata("for_tests/westend9120"), &specs()).unwrap_err();

    if let Error::WrongNetworkVersion {
        as_decoded,
        in_metadata,
    } = reply
    {
        assert_eq!(as_decoded, "9111".to_string());
        assert_eq!(in_metadata, 9120)
    } else {
        panic!("Expected Error::WrongNetworkVersion, got {:?}", reply);
    }
}

// Unable to use subxt, metadata v12.
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
    use westend9111::runtime_types::{
        pallet_balances::pallet::Call as BalancesCall, westend_runtime::Call,
    };

    let dest = AccountId32::from_ss58check("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty")
        .unwrap()
        .into();
    let genesis_hash =
        H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap();
    let block_hash =
        H256::from_str("98a8ee9e389043cd8a9954b254d822d34138b9ae97d3b7f50dc6781b13df8d84").unwrap();
    let params = BaseExtrinsicParams::<PolkadotConfig, PlainTip>::new(
        9111,
        7,
        261,
        genesis_hash,
        BaseExtrinsicParamsBuilder::new()
            .tip(PlainTip::new(10_000_000))
            .era(Era::Mortal(64, 61), block_hash),
    );
    let call = BalancesCall::transfer_keep_alive {
        dest,
        value: 100_000_000,
    };

    let call = Call::Balances(call);
    let call = call.encode();
    let call = Method { method: call };

    let mut extensions = vec![];
    params.encode_extra_to(&mut extensions);
    params.encode_additional_to(&mut extensions);
    let mut tx = call.encode();

    tx.extend_from_slice(extensions.as_slice());

    let reply = parse_and_display_set(&tx, &metadata("for_tests/westend9111"), &specs()).unwrap();
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
    assert_eq!(reply, reply_known);
}

#[test]
fn tr_5() {
    use westend9122::runtime_types::westend_runtime::Call;
    use westend9122::runtime_types::frame_system::pallet::Call as SystemCall;

    let remark = 
"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Congue eu consequat ac felis donec. Turpis egestas integer eget aliquet nibh praesent. Neque convallis a cras semper auctor neque. Netus et malesuada fames ac turpis egestas sed tempus. Pellentesque habitant morbi tristique senectus et netus et. Pretium vulputate sapien nec sagittis aliquam. Convallis aenean et tortor at risus viverra. Vivamus arcu felis bibendum ut tristique et egestas quis ipsum. Malesuada proin libero nunc consequat interdum varius. ".as_bytes().to_vec();

    let call = SystemCall::remark { remark };
    let call = Call::System(call);
    let genesis_hash =
        H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap();
    let block_hash =
        H256::from_str(
"1b2b0a177ad4f3f93f9a56dae700e938a40201a5beabbda160a74c70e612c66a"
    ).unwrap();

    let params = BaseExtrinsicParams::<PolkadotConfig, PlainTip>::new(
        9122,
        7,
        11,
        genesis_hash,
        BaseExtrinsicParamsBuilder::new()
            .tip(PlainTip::new(0))
            .era(Era::Mortal(64, 36), block_hash),
    );

    let call = call.encode();
    let call = Method { method: call };

    let mut extensions = vec![];
    params.encode_extra_to(&mut extensions);
    params.encode_additional_to(&mut extensions);
    let mut tx = call.encode();

    tx.extend_from_slice(extensions.as_slice());

    let reply = parse_and_display_set(&tx, &metadata("for_tests/westend9122"), &specs()).unwrap();
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

    assert_eq!(reply,reply_known);
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

#[test]
fn tr_7() {
    let data = hex::decode(concat!(
           "780300e855f5e79ca68ecae0fe99a3fa46806461740e1a0f0000c16ff28623e40000000a0700000200000091bc6e169807aaa54802737e1c504b2577d4fafedd5a02c10293b1cd60e395272470dff6295dd9bb3e5a89c9eb7647d7c5ae525618d77757171718dc034be8f5")
        ).unwrap();
    let specs_moonbase = ShortSpecs {
        base58prefix: 1287,
        decimals: 18,
        genesis_hash: [
            145, 188, 110, 22, 152, 7, 170, 165, 72, 2, 115, 126, 28, 80, 75, 37, 119, 212, 250,
            254, 221, 90, 2, 193, 2, 147, 177, 205, 96, 227, 149, 39,
        ]
        .into(),
        name: "moonbase".to_string(),
        unit: "DEV".to_string(),
    };

    let reply =
        parse_and_display_set(&data, &metadata("for_tests/moonbase1802"), &specs_moonbase).unwrap();

    let reply_known = r#"
Method:

pallet: Balances,
  method: transfer,
    field_name: dest,
      Id: 0xe855f5e79ca68ecae0fe99a3fa46806461740e1a,
    field_name: value,
      balance: 10.000000000000000 mDEV


Extensions:

era: Mortal, phase: 14, period: 32,
nonce: 0,
tip: 0 aDEV,
network: moonbase1802,
tx_version: 2,
block_hash: 2470dff6295dd9bb3e5a89c9eb7647d7c5ae525618d77757171718dc034be8f5"#;

    assert_eq!(reply, reply_known)
}
