use definitions::dynamic_derivations::DynamicDerivationsRequest;
use std::collections::HashMap;
use std::convert::TryInto;

use crate::Error;
use db_handling::identities::{process_dynamic_derivations_v1, DynamicDerivationTransaction};
use definitions::crypto::Encryption;
use definitions::helpers::{multisigner_to_encryption, unhex};
use definitions::navigation::{DDPreview, DecodeSequenceResult};
use parity_scale_codec::{Decode, Encode};
use sp_core::H256;

use crate::error::Result;

/// Parses the payload to make sure it is a dynamic derivation request
pub(crate) fn decode_dynamic_derivations(data_hex: &str) -> Result<DecodeSequenceResult> {
    let data = unhex(data_hex)?;
    let _ = <DynamicDerivationsRequest>::decode(&mut &data[3..])?;
    Ok(DecodeSequenceResult::DynamicDerivations {
        s: data_hex.to_string(),
    })
}

/// Parses dynamic derivation request and returns preview of the result
pub fn process_dynamic_derivations(
    database: &sled::Db,
    seeds: HashMap<String, String>,
    data_hex: &str,
) -> Result<DDPreview> {
    let data = unhex(data_hex)?;
    match <DynamicDerivationsRequest>::decode(&mut &data[3..])? {
        DynamicDerivationsRequest::V1(r) => Ok(process_dynamic_derivations_v1(database, seeds, r)?),
    }
}

/// Partially parses dynamic derivation transaction to make it compatible with `multisigner_msg_genesis_encryption`
pub fn dd_transaction_msg_genesis_encryption(
    data_hex: &str,
) -> Result<(DynamicDerivationTransaction, Vec<u8>, H256, Encryption)> {
    let mut data = unhex(data_hex)?;
    // Swap `encryption` and `payload code` bytes for convenient decoding
    (data[1], data[2]) = (data[2], data[1]);
    let transaction = <DynamicDerivationTransaction>::decode(&mut &data[2..])?;
    data = data[(transaction.encode().len() + 2)..].to_vec();
    let encryption = transaction.encryption;

    // network genesis hash
    let raw_hash: [u8; 32] = data[data.len() - 32..]
        .try_into()
        .map_err(|_| Error::TooShort)?;
    let genesis_hash_vec = H256::from(raw_hash);
    let msg = data[..data.len() - 32].to_vec();
    Ok((transaction, msg, genesis_hash_vec, encryption))
}
