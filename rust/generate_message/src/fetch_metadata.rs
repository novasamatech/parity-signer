use jsonrpsee_types::{traits::Client, v2::params::JsonRpcParams, JsonValue};
use jsonrpsee_ws_client::WsClientBuilder;
use serde_json::{map::Map, value::Number};

pub struct FetchedInfo {
    pub meta: String,
    pub genesis_hash: String,
}

pub struct FetchedInfoWithNetworkSpecs {
    pub meta: String,
    pub genesis_hash: String,
    pub properties: Map<String, JsonValue>,
}

/// Function to fetch the metadata as String and genesis hash as String from given address,
/// actually fetches stuff, is slow

#[tokio::main]
pub async fn fetch_info(str_address: &str) -> Result<FetchedInfo, Box<dyn std::error::Error>> {
    let client = WsClientBuilder::default().build(str_address).await?;
    let response: JsonValue = client
        .request("state_getMetadata", JsonRpcParams::NoParams)
        .await?;
    let meta = match response {
        JsonValue::String(x) => x,
        _ => return Err(Box::from("Unexpected metadata format")),
    };
    let response: JsonValue = client
        .request(
            "chain_getBlockHash",
            JsonRpcParams::ArrayRef(&[JsonValue::Number(Number::from(0_u8))]),
        )
        .await?;
    let genesis_hash = match response {
        JsonValue::String(x) => x,
        _ => return Err(Box::from("Unexpected genesis hash format")),
    };
    Ok(FetchedInfo { meta, genesis_hash })
}

/// Function to fetch the metadata as String, genesis hash as String, and network specs from given address,
/// actually fetches stuff, is slow

#[tokio::main]
pub async fn fetch_info_with_network_specs(
    str_address: &str,
) -> Result<FetchedInfoWithNetworkSpecs, Box<dyn std::error::Error>> {
    let client = WsClientBuilder::default().build(str_address).await?;
    let response: JsonValue = client
        .request("state_getMetadata", JsonRpcParams::NoParams)
        .await?;
    let meta = match response {
        JsonValue::String(x) => x,
        _ => return Err(Box::from("Unexpected metadata format")),
    };
    let response: JsonValue = client
        .request(
            "chain_getBlockHash",
            JsonRpcParams::ArrayRef(&[JsonValue::Number(Number::from(0_u8))]),
        )
        .await?;
    let genesis_hash = match response {
        JsonValue::String(x) => x,
        _ => return Err(Box::from("Unexpected genesis hash format")),
    };
    let response: JsonValue = client
        .request("system_properties", JsonRpcParams::NoParams)
        .await?;
    let properties = match response {
        JsonValue::Object(x) => x,
        _ => return Err(Box::from("Unexpected system properties format")),
    };
    Ok(FetchedInfoWithNetworkSpecs {
        meta,
        genesis_hash,
        properties,
    })
}
