use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::ws_client::WsClientBuilder;
use serde_json::{value::{Number, Value}, map::Map};

pub struct FetchedInfo {
    pub meta: String,
    pub genesis_hash: String,
}

pub struct FetchedInfoWithNetworkSpecs {
    pub meta: String,
    pub genesis_hash: String,
    pub properties: Map<String, Value>,
}

/// Function to fetch the metadata as String and genesis hash as String from given address,
/// actually fetches stuff, is slow

#[tokio::main]
pub async fn fetch_info(str_address: &str) -> Result<FetchedInfo, Box<dyn std::error::Error>> {
    // note: here the port is set to 443, since default port is unavailable for now;
    // see for details `https://github.com/paritytech/jsonrpsee/issues/554`
    let client = WsClientBuilder::default().build(str_address.to_owned() + ":443").await?;
    let response: Value = client.request("state_getMetadata", rpc_params![]).await?;
    let meta = match response {
        Value::String(x) => x,
        _ => return Err(Box::from("Unexpected metadata format")),
    };
    let response: Value = client.request("chain_getBlockHash", rpc_params![Value::Number(Number::from(0u8))]).await?;
    let genesis_hash = match response {
        Value::String(x) => x,
        _ => return Err(Box::from("Unexpected genesis hash format")),
    };
    Ok(FetchedInfo{
        meta,
        genesis_hash,
    })
}

/// Function to fetch the metadata as String, genesis hash as String, and network specs from given address,
/// actually fetches stuff, is slow

#[tokio::main]
pub async fn fetch_info_with_network_specs(str_address: &str) -> Result<FetchedInfoWithNetworkSpecs, Box<dyn std::error::Error>> {
    // note: here the port is set to 443, since default port is unavailable for now;
    // see for details `https://github.com/paritytech/jsonrpsee/issues/554`
    let client = WsClientBuilder::default().build(str_address.to_owned() + ":443").await?;
    let response: Value = client.request("state_getMetadata", rpc_params![]).await?;
    let meta = match response {
        Value::String(x) => x,
        _ => return Err(Box::from("Unexpected metadata format")),
    };
    let response: Value = client.request("chain_getBlockHash", rpc_params![Value::Number(Number::from(0u8))]).await?;
    let genesis_hash = match response {
        Value::String(x) => x,
        _ => return Err(Box::from("Unexpected genesis hash format")),
    };
    let response: Value = client.request("system_properties", rpc_params![]).await?;
    let properties = match response {
        Value::Object(x) => x,
        _ => return Err(Box::from("Unexpected system properties format")),
    };
    Ok(FetchedInfoWithNetworkSpecs{
        meta,
        genesis_hash,
        properties,
    })
}

