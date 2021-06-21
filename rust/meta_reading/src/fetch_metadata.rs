use jsonrpsee_types::{
    JsonValue, 
    v2::params::JsonRpcParams,
    traits::Client,
};
use jsonrpsee_ws_client::WsClientBuilder;
use serde_json::value::Number;

pub struct FetchedInfo {
    pub meta: String,
    pub genesis_hash: String,
}

/// Function to fetch the metadata as String and genesis hash as String from given address,
/// actually fetches stuff, is slow

#[tokio::main]
pub async fn fetch_info(str_address: &str) -> Result<FetchedInfo, Box<dyn std::error::Error>> {
    let client = WsClientBuilder::default().build(str_address).await?;
    let response: JsonValue = client.request("state_getMetadata", JsonRpcParams::NoParams).await?;
    let meta = match response {
        JsonValue::String(x) => x,
        _ => return Err(Box::from("Unexpected state metadata format")),
    };
    let response: JsonValue = client.request("chain_getBlockHash", JsonRpcParams::ArrayRef(&[JsonValue::Number(Number::from(0 as u8))])).await?;
    let genesis_hash = match response {
        JsonValue::String(x) => x,
        _ => return Err(Box::from("Unexpected genesis hash format")),
    };
    Ok(FetchedInfo{
        meta,
        genesis_hash,
    })
}

