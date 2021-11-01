#[cfg(test)]
mod tests {
    use crate::{parse_and_display_set};
    use definitions::network_specs::ShortSpecs;
    use frame_metadata::RuntimeMetadata;
    use hex;
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
            genesis_hash: [225, 67, 242, 56, 3, 172, 80, 232, 246, 248, 230, 38, 149, 209, 206, 158, 78, 29, 104, 170, 54, 193, 205, 44, 253, 21, 52, 2, 19, 243, 66, 62],
            name: "westend".to_string(),
            unit: "WND".to_string(),
        }
    }
    
    #[test]
    fn tr_1() {
        let data = hex::decode("4d0210020806000046ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a07001b2c3ef70006050c0008264834504a64ace1373f0c8ed5d57381ddf54a2f67a318fa42b1352681606d00aebb0211dbb07b4d335a657257b8ac5e53794c901e4f616d4a254f2490c43934009ae581fef1fc06828723715731adcf810e42ce4dadad629b1b7fa5c3c144a81d550008009723000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ff").unwrap();
        let reply = parse_and_display_set(data, &metadata("for_tests/westend9111"), &specs());
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

era: Mortal, phase: 64, period: 5,
nonce: 2,
tip: 0 pWND,
version: 9111,
tx_version: 7,
network: westend,
block_hash: 5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ff"#;
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
    }
    
    #[test]
    fn tr_2() {
        let data = hex::decode("4d0210020806000046ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a07001b2c3ef70006050c0008264834504a64ace1373f0c8ed5d57381ddf54a2f67a318fa42b1352681606d00aebb0211dbb07b4d335a657257b8ac5e53794c901e4f616d4a254f2490c43934009ae581fef1fc06828723715731adcf810e42ce4dadad629b1b7fa5c3c144a81d550008009723000007000000e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e5b1d91c89d3de85a4d6eee76ecf3a303cf38b59e7d81522eb7cd24b02eb161ff").unwrap();
        let reply = parse_and_display_set(data, &metadata("for_tests/westend9120"), &specs());
        let reply_known = "Metadata network spec version (9111) differs from the version in extensions (9120).";
        assert!(reply == reply_known, "Expected: {}\nReceived: {}", reply_known, reply);
    }
}
