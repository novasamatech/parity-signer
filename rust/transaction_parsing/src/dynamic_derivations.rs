use definitions::dynamic_derivations::DynamicDerivationsRequest;
use std::collections::HashMap;

use db_handling::identities::process_dynamic_derivations_v1;
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
