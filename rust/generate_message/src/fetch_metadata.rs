use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::ws_client::WsClientBuilder;
use lazy_static::lazy_static;
use regex::Regex;
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

lazy_static! {
// stolen from sp_core
// removed seed phrase part
// last '+' used to be '*', but empty password is an error
    static ref PORT: Regex = Regex::new(r"^(?P<body>wss://[^/]*?)(?P<port>:[0-9]+)?(?P<tail>/.*)?$").expect("known value");
}

fn address_with_port (str_address: &str) -> String {
    // note: here the port is set to 443 if there is no default, since default port is unavailable for now;
    // see for details `https://github.com/paritytech/jsonrpsee/issues/554`
    // some addresses already have port specified, and should be left as is
    match PORT.captures(str_address) {
        Some(caps) => {
            if caps.name("port").is_some() {str_address.to_string()}
            else {
                match caps.name("tail") {
                    Some(tail) => format!("{}:443{}", &caps["body"], tail.as_str()),
                    None => format!("{}:443", &caps["body"]),
                }
            }
        },
        None => str_address.to_string(),
    }
}

/// Function to fetch the metadata as String and genesis hash as String from given address,
/// actually fetches stuff, is slow

#[tokio::main]
pub async fn fetch_info(str_address: &str) -> Result<FetchedInfo, Box<dyn std::error::Error>> {
    
    let client = WsClientBuilder::default().build(address_with_port(str_address)).await?;
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
    let client = WsClientBuilder::default().build(address_with_port(str_address)).await?;
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn address_1() {
        let address = "wss://rpc.polkadot.io";
        let address_expected = "wss://rpc.polkadot.io:443";
        let address_calc = address_with_port(address);
        assert!(address_expected == address_calc, "Fetch address calc: \n{}", address_calc);
    }

    #[test]
    fn address_2() {
        let address = "wss://polkadot.api.onfinality.io/public-ws";
        let address_expected = "wss://polkadot.api.onfinality.io:443/public-ws";
        let address_calc = address_with_port(address);
        assert!(address_expected == address_calc, "Fetch address calc: \n{}", address_calc);
    }

    #[test]
    fn address_3() {
        let address = "wss://node-6907995778982338560.sz.onfinality.io/ws?apikey=b5324589-1447-4699-92a6-025bc2cc2ac1";
        let address_expected = "wss://node-6907995778982338560.sz.onfinality.io:443/ws?apikey=b5324589-1447-4699-92a6-025bc2cc2ac1";
        let address_calc = address_with_port(address);
        assert!(address_expected == address_calc, "Fetch address calc: \n{}", address_calc);
    }

    #[test]
    fn address_4() {
        let address = "wss://westend.kilt.io:9977";
        let address_expected = "wss://westend.kilt.io:9977";
        let address_calc = address_with_port(address);
        assert!(address_expected == address_calc, "Fetch address calc: \n{}", address_calc);
    }

    #[test]
    fn address_5() {
        let address = "wss://full-nodes.kilt.io:9944/";
        let address_expected = "wss://full-nodes.kilt.io:9944/";
        let address_calc = address_with_port(address);
        assert!(address_expected == address_calc, "Fetch address calc: \n{}", address_calc);
    }

    #[test]
    fn address_6() {
        let address = "wss://peregrine.kilt.io/parachain-public-ws/";
        let address_expected = "wss://peregrine.kilt.io:443/parachain-public-ws/";
        let address_calc = address_with_port(address);
        assert!(address_expected == address_calc, "Fetch address calc: \n{}", address_calc);
    }
}
