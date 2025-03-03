use std::convert::TryInto;
use std::str::FromStr;

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
    let data = hex::decode("60000341000000031102082873705f72756e74696d65384d756c74695369676e6174757265011c4564323535313904001685010148656432353531393a3a5369676e617475726500f504082873705f72756e74696d65384d756c74695369676e6174757265011c5372323535313904001685010148737232353531393a3a5369676e617475726504f504082873705f72756e74696d65384d756c74695369676e6174757265011445636473610400161102014065636473613a3a5369676e617475726508f50404184f7074696f6e0110536f6d65040016040004990610306672616d655f73797374656d28657874656e73696f6e733c636865636b5f6d6f7274616c69747938436865636b4d6f7274616c69747900040016a106010c4572619d06102873705f72756e74696d651c67656e657269630c6572610c4572610120496d6d6f7274616c0000a10610306672616d655f73797374656d28657874656e73696f6e732c636865636b5f6e6f6e636528436865636b4e6f6e6365000400110120543a3a4e6f6e6365a506086870616c6c65745f7472616e73616374696f6e5f7061796d656e74604368617267655472616e73616374696f6e5061796d656e7400040013013042616c616e63654f663c543ea90608746672616d655f6d657461646174615f686173685f657874656e73696f6e44436865636b4d6574616461746148617368000401106d6f646516b10601104d6f6465ad0608746672616d655f6d657461646174615f686173685f657874656e73696f6e104d6f6465011c456e61626c65640004b1060c1c73705f636f72651863727970746f2c4163636f756e7449643332000400160401205b75383b2033325d000003200000000304083c7072696d69746976655f74797065731048323536000400160401205b75383b2033325d0c000203100003140000000350085873746167696e675f6b7573616d615f72756e74696d652c52756e74696d6543616c6c012042616c616e636573040016090101b50173656c663a3a73705f6170695f68696464656e5f696e636c756465735f636f6e7374727563745f72756e74696d653a3a68696464656e5f696e636c7564653a3a64697370617463680a3a3a43616c6c61626c6543616c6c466f723c42616c616e6365732c2052756e74696d653e10c40c2873705f72756e74696d65306d756c746961646472657373304d756c746941646472657373010849640400160001244163636f756e7449640005010c2873705f72756e74696d65306d756c746961646472657373304d756c7469416464726573730114496e64657804001501304163636f756e74496e6465780405010c2873705f72756e74696d65306d756c746961646472657373304d756c746941646472657373010c52617704001610011c5665633c75383e0805010c2873705f72756e74696d65306d756c746961646472657373304d756c74694164647265737301244164647265737333320400160401205b75383b2033325d0c05010c2873705f72756e74696d65306d756c746961646472657373304d756c74694164647265737301244164647265737332300400165001205b75383b2032305d1005010c3c70616c6c65745f62616c616e6365731870616c6c65741043616c6c01507472616e736665725f616c6c6f775f64656174680801106465737416050101504163636f756e7449644c6f6f6b75704f663c543e011476616c7565130128543a3a42616c616e63650009010003400000000385016069080000280a0000290a00002a0a0000260c0000270c0000280c0000280d0000290d00002a0d00002c0d00009606000097060000990600009a060000cf06000040070000900700009107000092070000930700009407000095070000e9070000e4703e8894f16ebdddd78bcbfb4515610caf771595904de66167f2e185bc20891b9ba7258b77cb883b4c1ad21b03819c4b5647a208d42701a72926e1f5c593f165b1b1519755cd94d82bf864dbdcb98d5d35a7b7efe61221fb5a9b2b477313a7f148123bc34b73c6412247df27dea44beb0c68d2e0842847a43fe256d2a4274421f6d13467e99bc29a73393436d4a1d5fb64d2386b785f36b5265195e27e4ae87d01d0606446a7c97d4f66cb2a9bd43bb9318bb189d15bcaa30678b84488d95c21c7009de256b20e8803393c9d8799abb62239d9d97fec8897a90ae7797e9ac8d27237d41c085f8838e3b938e2b24ea0bf5993b843e76331cfbd12381c936493f0454f3c8c24969855dd9e680d2c27952b7d64e2d369b0ad5750cea0ad4e237fccbd52da3c79c836bae74e0587c71c8c48d29f04cfe6afa00cbb3bdb1580351ab313b39b0f57aca514fec890f6304c6b5233e3a815802439cf19fe9ecec2c88334a693b63d584dc3581db368dc3be143bb376166d5d828bce0b2b03e158d9bbbec1a87211b6097dca8537d4f3ecc5fa0b20ddcd4a1d32e8028182b7e770809a5f565333c9be85eecbaefb0bffc8cedcffd54dbfb28640e0f0367ef26bdaac1148386f2efcd3bcd1ccd95e40d02a84c438e5791385f1448de9b93a1947b91c1ea5c4cb5a6c60f946bb39ddfa419cc5bd97478067c8a22f3f79729125d9362684b10053e17d18a5ff8171dbd25dbe0eb3e16eb66f22bf7bbb4f4d3bec8d8cc77414c957764ce39bd3babf1d4bda04d64d1df4b1fc2bd8c197181c961ac43585cfb8f0db0d0c98594d52e43b83beaaebe89857a7a23b5c9dbaec324bd53639ac8e1a6db3e4c6e73b7704b51fde8a42b4c78f49dceeb1a57286e0f7d25b4b55b1dbbc9e8d1036ac7f65d110f790f142ee609b1d6ddace5ed19f17833ac91458858b3d66fd5597864036218b09bbd058b1d56c6dd4c16d02c4127b3ffb2446a6f23fc76dc3eafe2f3a41a0a9e2b2486aa10dcd14a1ba0eb231799948264f33998119d039de987660ae1afb7fdb3c643d5cc67ac5105f74c2f8c3f61440562b2de579dd86ca136a91ed479244ca6a0cc3e7879e4c7b3844104d3ab9c7fdb6c8c30d20fe7c1507546f97650eedcfb67f99afc7086be9fdb4823d012a6872e992e22e7e3a12d2c40720ea9ed350daa484f4ecd0fc77cada99bd7731e280a5ad01e89875788a59612ffe1cfff0ac75e4b70b3116b535ab276d8cd561fd101e7b5600ffd41f0246ff387cb531313e556e93ff9901e0ee287c1bc610c5685c58a36653ba49926dcd0889f624dd6d1bf2a102b07ab1703f8646885efba024ba335422cceb78fbcd3814e151905ab6499c76ccb290d3cc9fa2f41bd00139140b7206e31ff9d98e46bcb4a390fb36de6ae704ea64833eaceb9a16e4ccdc6694265ab5b5bf9d3c3cc5902b4e9428cdf83c31e32e354e62ece41a061cf458f67e0f7b0f50de0d6e945f348206210d990d1edac69e5f4662956151b2627ba1d1780085e943297ec28acc959b5b61034ddbe0748d5b74e251bed7d6fc93fb5ea49d3a4d20a8377fbe21a0a6f9a77af07a6e327c294a37e08cb6ee843efc6d8d37e0bac411a39ddb82dde562e99e5de9a2ac842938216124e57f4bbb18942cb3168f9356c52a5c7c66ce0ecc709ea59127487c3062d51189d09fbdfbd1ac5bd003db0c5619eb211b58941a7fd45b12cbecf31add5b6dbb749115183dcc059860ffaceb2514015bc0536b96de7122697c842ea9016de8f549729f4b1b0b4807150e5204b459cf5a48b24745538e012aec0584d0c622ab61705b3d00dfaeaba7731c2a2f684f130a3fc8c60733b72288a9dd65cac4a6aa965d291acdee7f300153ef6b4466d936f5f90006939f6fdc2094d8b3e3ea6e9d27c35781422c6a0d91a5dd92f8551307151074260313a5328d3c0248944b03603f022a16ef789790494093e21bda021136dfa7fb64d504354eeb528654b065d7713311c94bf2f9a3519222cf260eb8c7f6ed8d64f96e69975bdf28f17003f59fd4cd0d2581d5826489d17a1f05e9e76979e2850d35bb0bdb9ba49e3d399dd0a6f9294a59731c0085017e74a1b6690e4ad0e2406a80665fb75f49ad043a3ccdf1e2d06656f498c736a9f8ca46535b191b54d8c5985925d75cbff17e16d959a0f9c03dd8f15204a2eb33c5bcfa9fc1b71dbc2cc99e24bb3cd887b4e0bd1df7da5a8ca5b908a24f18a982453e9ba50954886f68cfe97c15bd19b56f6f955a9c726c60a7dfd38a140befa8a6cdd05686ca2916b8d6e731fd783dbe50b4479b9e102fb7906d1512cda1fd495bcff19fc36788d2ad2dd6fd55d99d99c4e4a82990b3a738a6b1c9bd0efc790993fec6a4605deb295277e013ab6d381b57453f5a62fae2c1df4a5a4d71d7480f301e1effec910fb778c6ee40ea6bf3b3e5c325a6126523a0e42c8158d371951cb9587196450c0681696fc9503b3ba991b967965009e3d0dc5effd655616e2aeb0d0328799b161337164ae4a1e34f865e3346b96d22e21ecf9d4e26fae7237deb69234a031e5874285f320060416050116c416f5042448436865636b4e6f6e5a65726f53656e646572151540436865636b5370656356657273696f6e150538436865636b547856657273696f6e150530436865636b47656e6573697315160c38436865636b4d6f7274616c697479169d06160c28436865636b4e6f6e636516a506152c436865636b5765696768741515604368617267655472616e73616374696f6e5061796d656e7416a9061544436865636b4d657461646174614861736816ad06169906e0510f00186b7573616d6102000c0c4b534d").unwrap();
    let metadata = MetadataProof::decode(&mut &data[..]).ok().unwrap();

    let call_data = hex::decode("040000e2e058da1316f8425be6c6f7104bb44c96fa29ea8b890b4c2866ba8bb6bc67b807007c118d35").unwrap();

    let call_result = decode_call(&mut &call_data[..], &metadata);

    assert!(call_result.is_ok());

    let mut extension_data = hex::decode("00000001").unwrap();
    let mut included_in_signature_data = hex::decode("e0510f001a000000b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafeb0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe0170d7bc8f2b306a20914d9dac518ccfd7b08e1e0404e0379534327560d5391c38").unwrap();

    extension_data.append(&mut included_in_signature_data);

    let extension_result = decode_extensions(&mut &extension_data[..], &metadata);

    assert!(extension_result.is_ok());
}