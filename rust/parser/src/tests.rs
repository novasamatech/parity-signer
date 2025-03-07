use std::convert::TryInto;
use std::str::FromStr;
use std::fs;

use crate::method::OlderMeta;
use crate::Error;
use crate::{
    parse_set, 
    MetadataBundle,
    types:: MetadataProof,
    decoding_with_proof::{decode_call, decode_extensions},
};
use defaults::default_types_vec;
use definitions::metadata::info_from_metadata;
use definitions::network_specs::ShortSpecs;
use frame_metadata::RuntimeMetadata;
use merkleized_metadata::types::Hash;
use parity_scale_codec::Decode;
use parity_scale_codec::Encode;
use pretty_assertions::assert_eq;
use sp_core::H256;

use subxt::{
    config::{
        extrinsic_params::{BaseExtrinsicParams, BaseExtrinsicParamsBuilder, Era, ExtrinsicParams},
        polkadot::PlainTip,
        PolkadotConfig,
    },
    utils::AccountId32,
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

pub(crate) fn parse_and_display_set(
    data: &[u8],
    metadata: &RuntimeMetadata,
    short_specs: &ShortSpecs,
) -> Result<String, Error> {
    let meta_info = info_from_metadata(metadata)?;
    if meta_info.name != short_specs.name {
        return Err(Error::NetworkNameMismatch {
            name_metadata: meta_info.name,
            name_network_specs: short_specs.name.to_string(),
        });
    }
    let metadata_bundle = match metadata {
        RuntimeMetadata::V12(_) | RuntimeMetadata::V13(_) => {
            let older_meta = match metadata {
                RuntimeMetadata::V12(meta_v12) => OlderMeta::V12(meta_v12),
                RuntimeMetadata::V13(meta_v13) => OlderMeta::V13(meta_v13),
                _ => unreachable!(),
            };
            let types = match default_types_vec() {
                Ok(a) => {
                    if a.is_empty() {
                        return Err(Error::NoTypes);
                    }
                    a
                }
                Err(_) => return Err(Error::DefaultTypes),
            };
            MetadataBundle::Older {
                older_meta,
                types,
                network_version: meta_info.version,
            }
        }
        RuntimeMetadata::V14(meta_v14) => MetadataBundle::Sci {
            meta_v14,
            network_version: meta_info.version,
        },
        _ => unreachable!(), // just checked in the info_from_metadata function if the metadata is acceptable one
    };
    let (method_cards_result, extensions_cards, _, _) =
        parse_set(data, &metadata_bundle, short_specs, None)?;
    let mut method = String::new();
    let mut extensions = String::new();
    match method_cards_result {
        Ok(method_cards) => {
            for (i, x) in method_cards.iter().enumerate() {
                if i > 0 {
                    method.push_str(",\n");
                }
                method.push_str(&x.card.show_no_docs(x.indent));
            }
        }
        Err(e) => method = format!("{e}"),
    }
    for (i, x) in extensions_cards.iter().enumerate() {
        if i > 0 {
            extensions.push_str(",\n");
        }
        extensions.push_str(&x.card.show_no_docs(x.indent));
    }
    Ok(format!(
        "\nMethod:\n\n{method}\n\n\nExtensions:\n\n{extensions}"
    ))
}

#[subxt::subxt(runtime_metadata_path = "for_tests/westend9111.scale")]
mod westend9111 {}

#[subxt::subxt(runtime_metadata_path = "for_tests/westend9122.scale")]
mod westend9122 {}

#[subxt::subxt(runtime_metadata_path = "for_tests/acala2012.scale")]
mod acala2012 {}

#[subxt::subxt(runtime_metadata_path = "for_tests/moonbase1802.scale")]
mod moonbase1802 {}

// This struct is needed as a way to add a `Compact`
// length in front of the encoded method payload.
#[derive(parity_scale_codec::Encode)]
struct Method {
    method: Vec<u8>,
}

fn westend_genesis() -> H256 {
    H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap()
}

fn moonbase_genesis() -> H256 {
    H256::from_str("91bc6e169807aaa54802737e1c504b2577d4fafedd5a02c10293b1cd60e39527").unwrap()
}

fn acala_genesis() -> H256 {
    H256::from_str("fc41b9bd8ef8fe53d58c7ea67c794c7ec9a73daf05e6d54b14ff6342c99ba64c").unwrap()
}

// Encode `Call` and `Extensions` into a payload that UOS compatible.
fn encode_call_and_params<I, H, C: Encode, P: ExtrinsicParams<I, H>>(
    call: &C,
    params: &P,
) -> Vec<u8> {
    let call = call.encode();

    let call = Method { method: call };

    let mut extensions = vec![];
    params.encode_extra_to(&mut extensions);
    params.encode_additional_to(&mut extensions);
    let mut tx = call.encode();

    tx.extend_from_slice(extensions.as_slice());

    tx
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
        controller: AccountId32::from_str("5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV")
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
        .map(|addr| AccountId32::from_str(addr).unwrap().into())
        .collect(),
    };

    let genesis_hash = westend_genesis();
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

    let tx = encode_call_and_params(&tx, &params);
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
        controller: AccountId32::from_str("5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV")
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
        .map(|addr| AccountId32::from_str(addr).unwrap().into())
        .collect(),
    };

    let genesis_hash = westend_genesis();
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

    let tx = encode_call_and_params(&tx, &params);
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
    assert_eq!(reply, reply_known);
}

#[test]
fn tr_4() {
    use westend9111::runtime_types::{
        pallet_balances::pallet::Call as BalancesCall, westend_runtime::Call,
    };

    let dest = AccountId32::from_str("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty")
        .unwrap()
        .into();
    let genesis_hash = westend_genesis();
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

    let tx = encode_call_and_params(&call, &params);

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
    use westend9122::runtime_types::frame_system::pallet::Call as SystemCall;
    use westend9122::runtime_types::westend_runtime::Call;

    let remark =
"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Congue eu consequat ac felis donec. Turpis egestas integer eget aliquet nibh praesent. Neque convallis a cras semper auctor neque. Netus et malesuada fames ac turpis egestas sed tempus. Pellentesque habitant morbi tristique senectus et netus et. Pretium vulputate sapien nec sagittis aliquam. Convallis aenean et tortor at risus viverra. Vivamus arcu felis bibendum ut tristique et egestas quis ipsum. Malesuada proin libero nunc consequat interdum varius. ".as_bytes().to_vec();

    let call = SystemCall::remark { remark };
    let call = Call::System(call);
    let genesis_hash = westend_genesis();
    let block_hash =
        H256::from_str("1b2b0a177ad4f3f93f9a56dae700e938a40201a5beabbda160a74c70e612c66a").unwrap();

    let params = BaseExtrinsicParams::<PolkadotConfig, PlainTip>::new(
        9122,
        7,
        11,
        genesis_hash,
        BaseExtrinsicParamsBuilder::new()
            .tip(PlainTip::new(0))
            .era(Era::Mortal(64, 36), block_hash),
    );

    let tx = encode_call_and_params(&call, &params);
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

    assert_eq!(reply, reply_known);
}

#[test]
fn tr_6() {
    use acala2012::runtime_types::{
        acala_runtime::Call, pallet_balances::pallet::Call as BalancesCall,
    };

    let dest = AccountId32::from_str("25rZGFcFEWz1d81xB98PJN8LQu5cCwjyazAerGkng5NDuk9C")
        .unwrap()
        .into();

    let call = BalancesCall::transfer {
        dest,
        value: 100_000_000_000_000,
    };

    let call = Call::Balances(call);

    let genesis_hash = acala_genesis();
    let block_hash =
        H256::from_str("5cfeb3e46c080274613bdb80809a3e84fe782ac31ea91e2c778de996f738e620").unwrap();
    let params = BaseExtrinsicParams::<PolkadotConfig, PlainTip>::new(
        2012,
        1,
        0,
        genesis_hash,
        BaseExtrinsicParamsBuilder::new()
            .tip(PlainTip::new(0))
            .era(Era::Mortal(32, 18), block_hash),
    );

    let tx = encode_call_and_params(&call, &params);

    let specs_acala = ShortSpecs {
        base58prefix: 10,
        decimals: 12,
        genesis_hash,
        name: "acala".to_string(),
        unit: "ACA".to_string(),
    };
    let reply = parse_and_display_set(&tx, &metadata("for_tests/acala2012"), &specs_acala).unwrap();
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
    assert_eq!(reply, reply_known);
}

#[test]
fn tr_7() {
    use moonbase1802::runtime_types::{
        account::AccountId20, moonbase_runtime::Call, pallet_balances::pallet::Call as BalancesCall,
    };

    let dest = AccountId20(
        TryInto::<[u8; 20]>::try_into(
            hex::decode("e855f5e79ca68ecae0fe99a3fa46806461740e1a").unwrap(),
        )
        .unwrap(),
    );

    let call = BalancesCall::transfer {
        dest,
        value: 10_000_000_000_000_000,
    };
    let call = Call::Balances(call);

    let genesis_hash = moonbase_genesis();
    let block_hash =
        H256::from_str("2470dff6295dd9bb3e5a89c9eb7647d7c5ae525618d77757171718dc034be8f5").unwrap();
    let params = BaseExtrinsicParams::<PolkadotConfig, PlainTip>::new(
        1802,
        2,
        0,
        genesis_hash,
        BaseExtrinsicParamsBuilder::new()
            .tip(PlainTip::new(0))
            .era(Era::Mortal(32, 14), block_hash),
    );

    let tx = encode_call_and_params(&call, &params);

    let specs_moonbase = ShortSpecs {
        base58prefix: 1287,
        decimals: 18,
        genesis_hash,
        name: "moonbase".to_string(),
        unit: "DEV".to_string(),
    };

    let reply =
        parse_and_display_set(&tx, &metadata("for_tests/moonbase1802"), &specs_moonbase).unwrap();

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

#[test]
fn parse_raw_extrinsic_with_proof() {
    let proof_line = fs::read_to_string("for_tests/kusama_transfer_metadata_proof.txt").unwrap();
    let data = hex::decode(proof_line).unwrap();
    let metadata = MetadataProof::decode(&mut &data[..]).ok().unwrap();

    let call_data = hex::decode("040000e2e058da1316f8425be6c6f7104bb44c96fa29ea8b890b4c2866ba8bb6bc67b807007c118d35").unwrap();

    let call_result = decode_call(&mut &call_data[..], &metadata);

    assert!(call_result.is_ok());

    let mut extension_data = hex::decode("00000001").unwrap();
    let mut included_in_signature_data = hex::decode("e0510f001a000000b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafeb0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe0170d7bc8f2b306a20914d9dac518ccfd7b08e1e0404e0379534327560d5391c38").unwrap();

    extension_data.append(&mut included_in_signature_data);

    let genesis_hash: Hash = hex::decode("b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe")
        .unwrap()
        .try_into()
        .expect("Slice must be exactly 32 bytes");

    let extension_result = decode_extensions(&mut &extension_data[..], &metadata, &genesis_hash);

    assert!(extension_result.is_ok());
}