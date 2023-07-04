use definitions::dynamic_derivations::DynamicDerivationsRequest;
use std::collections::HashMap;

use db_handling::identities::import_dynamic_addrs;
use definitions::helpers::unhex;
use definitions::navigation::{DDPreview, DecodeSequenceResult};
use parity_scale_codec::Decode;

use crate::error::Result;

pub fn decode_dynamic_derivations(data_hex: &str) -> Result<DecodeSequenceResult> {
    let data = unhex(data_hex)?;
    let _ = <DynamicDerivationsRequest>::decode(&mut &data[3..])?;
    Ok(DecodeSequenceResult::DynamicDerivations {
        s: data_hex.to_string(),
    })
}

pub fn import_dderivations(
    database: &sled::Db,
    seeds: HashMap<String, String>,
    data_hex: &str,
) -> Result<DDPreview> {
    let data = unhex(data_hex)?;
    match <DynamicDerivationsRequest>::decode(&mut &data[3..])? {
        DynamicDerivationsRequest::V1(r) => Ok(import_dynamic_addrs(database, seeds, r)?),
    }
}
