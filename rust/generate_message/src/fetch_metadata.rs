//! Fetch network information from a node using rpc calls
//!
//! Preparing `add_specs` and `load_metadata` update payload may require
//! gathering network information from a node.
//!
//! For `add_specs` update payload:
//!
//! <table>
//!     <tr>
//!         <th>call</th>
//!         <th>fetched information</th>
//!     </tr>
//!     <tr>
//!         <td><code>state_getMetadata</code>, for current block</td>
//!         <td>current block network metadata, that will be used to get:<br>
//!             - network name<br>
//!             - base58 prefix from metadata
//!         </td>
//!     </tr>
//!     <tr>
//!         <td><code>chain_getBlockHash</code>, for 0th block</td>
//!         <td>network genesis hash</td>
//!     </tr>
//!     <tr>
//!         <td><code>system_properties</code></td>
//!         <td>- base58 prefix<br>
//!             - decimals<br>
//!             - unit<br>
//!         </td>
//!     </tr>
//! </table>
//!
//! Network name expected to remain the same for the network over time. The only
//! way to get network name is from the network metadata `Version` constant.
//!
//! For `load_metadata` update:
//!
//! <table>
//!     <tr>
//!         <th>call</th>
//!         <th>fetched information</th>
//!     </tr>
//!     <tr>
//!         <td><code>chain_getBlockHash</code>, for current block</td>
//!         <td>current block hash</td>
//!     </tr>
//!     <tr>
//!         <td><code>state_getMetadata</code>, for just fetched block hash</td>
//!         <td>latest network metadata</td>
//!     </tr>
//!     <tr>
//!         <td><code>chain_getBlockHash</code>, for 0th block</td>
//!         <td>network genesis hash</td>
//!     </tr>
//! </table>
//!
//! Block hash is fetched first to always have network metadata matching the
//! block hash, even if the two rpc calls were done during block switching.
//!
//! Addresses for rpc calls in different networks could be found
//! [here](https://github.com/polkadot-js/apps/tree/master/packages/apps-config/src/endpoints)
//!
//! This module deals only with the rpc calls part and does **no processing**
//! of the fetched data.
use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::ws_client::WsClientBuilder;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::{
    map::Map,
    value::{Number, Value},
};
use sp_core::H256;

/// Data from rpc calls for `load_metadata` update payload.
///
/// This data is **sufficient** for `load_metadata` update payload generation,
/// i.e. nothing else has to be known about the network beforehand to produce an
/// update payload.
pub struct FetchedInfo {
    /// Fetched metadata, as a hexadecimal string
    pub meta: String,

    /// Block hash, at which the metadata was fetched, as a hexadecimal string
    pub block_hash: String,

    /// Fetched genesis hash, as a hexadecimal string
    pub genesis_hash: String,
}

/// Data from rpc calls for `add_specs` update payload.
///
/// Note that this data is **not sufficient** for `add_specs` update payload
/// generation. At least network encryption is needed additionally.
pub struct FetchedInfoWithNetworkSpecs {
    /// Fetched metadata, as a hexadecimal string
    pub meta: String,

    /// Fetched genesis hash, as a hexadecimal string
    pub genesis_hash: String,

    /// Fetched network properties, as a `Map`
    ///
    /// Properties are expected to contain base58 prefix, decimals, and units,
    /// but in some cases some data may be missing.
    pub properties: Map<String, Value>,
}

lazy_static! {
    /// Regex to add port to addresses that have no port specified.
    ///
    /// See tests for behavior examples.
    static ref PORT: Regex = Regex::new(r"^(?P<body>wss://[^/]*?)(?P<port>:[0-9]+)?(?P<tail>/.*)?$").expect("known value");
}

/// Supply address with port if needed.
///
/// Transform address as it is displayed to user in <https://polkadot.js.org/>
/// to address with port added if necessary that could be fed to `jsonrpsee`
/// client.
///
/// The port is set here to default 443 if there is no port specified in
/// address itself, since default port in `jsonrpsee` is unavailable for now.
///
/// See for details <https://github.com/paritytech/jsonrpsee/issues/554`>
///
/// Some addresses have port specified, and should be left as is.
fn address_with_port(str_address: &str) -> String {
    match PORT.captures(str_address) {
        Some(caps) => {
            if caps.name("port").is_some() {
                str_address.to_string()
            } else {
                match caps.name("tail") {
                    Some(tail) => format!("{}:443{}", &caps["body"], tail.as_str()),
                    None => format!("{}:443", &caps["body"]),
                }
            }
        }
        None => str_address.to_string(),
    }
}

/// Fetch data for `load_metadata` update payload through rpc calls.
///
/// Function inputs address at which rpc calls are made.
///
/// Data fetched:
///
/// 1. current block hash
/// 2. metadata at this block hash
/// 3. network genesis hash
#[tokio::main]
pub async fn fetch_info(str_address: &str) -> Result<FetchedInfo, Box<dyn std::error::Error>> {
    let client = WsClientBuilder::default()
        .build(address_with_port(str_address)) // port supplied if needed
        .await?;
    let response: Value = client.request("chain_getBlockHash", rpc_params![]).await?;
    let block_hash = match response {
        Value::String(x) => x,
        _ => return Err(Box::from("Unexpected block hash format")),
    };
    let response: Value = client
        .request("state_getMetadata", rpc_params![&block_hash])
        .await?;
    let meta = match response {
        Value::String(x) => x,
        _ => return Err(Box::from("Unexpected metadata format")),
    };
    let response: Value = client
        .request(
            "chain_getBlockHash",
            rpc_params![Value::Number(Number::from(0u8))],
        )
        .await?;
    let genesis_hash = match response {
        Value::String(x) => x,
        _ => return Err(Box::from("Unexpected genesis hash format")),
    };
    Ok(FetchedInfo {
        meta,
        block_hash,
        genesis_hash,
    })
}

/// Fetch network metadata from given url address at given block through rpc
/// call.
///
/// Function inputs address at which rpc call is made and block hash in [`H256`]
/// format. Outputs hexadecimal metadata.
#[tokio::main]
pub async fn fetch_meta_at_block(
    str_address: &str,
    block_hash: H256,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = WsClientBuilder::default()
        .build(address_with_port(str_address)) // port supplied if needed
        .await?;
    let response: Value = client
        .request(
            "state_getMetadata",
            rpc_params![Value::String(format!("0x{}", hex::encode(block_hash)))],
        )
        .await?;
    match response {
        Value::String(x) => Ok(x),
        _ => Err(Box::from("Unexpected metadata format")),
    }
}

/// Fetch data for `add_specs` update payload through rpc calls.
///
/// Function inputs address at which rpc calls are made.
///
/// Data fetched:
///
/// 1. current network metadata
/// 2. network genesis hash
/// 3. network system properties (could contain base58 prefix, decimals, unit)
#[tokio::main]
pub async fn fetch_info_with_network_specs(
    str_address: &str,
) -> Result<FetchedInfoWithNetworkSpecs, Box<dyn std::error::Error>> {
    let client = WsClientBuilder::default()
        .build(address_with_port(str_address)) // port supplied if needed
        .await?;
    let response: Value = client.request("state_getMetadata", rpc_params![]).await?;
    let meta = match response {
        Value::String(x) => x,
        _ => return Err(Box::from("Unexpected metadata format")),
    };
    let response: Value = client
        .request(
            "chain_getBlockHash",
            rpc_params![Value::Number(Number::from(0u8))],
        )
        .await?;
    let genesis_hash = match response {
        Value::String(x) => x,
        _ => return Err(Box::from("Unexpected genesis hash format")),
    };
    let response: Value = client.request("system_properties", rpc_params![]).await?;
    let properties = match response {
        Value::Object(x) => x,
        _ => return Err(Box::from("Unexpected system properties format")),
    };
    Ok(FetchedInfoWithNetworkSpecs {
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
        assert!(
            address_expected == address_calc,
            "Fetch address calc: \n{}",
            address_calc
        );
    }

    #[test]
    fn address_2() {
        let address = "wss://polkadot.api.onfinality.io/public-ws";
        let address_expected = "wss://polkadot.api.onfinality.io:443/public-ws";
        let address_calc = address_with_port(address);
        assert!(
            address_expected == address_calc,
            "Fetch address calc: \n{}",
            address_calc
        );
    }

    #[test]
    fn address_3() {
        let address = "wss://node-6907995778982338560.sz.onfinality.io/ws?apikey=b5324589-1447-4699-92a6-025bc2cc2ac1";
        let address_expected = "wss://node-6907995778982338560.sz.onfinality.io:443/ws?apikey=b5324589-1447-4699-92a6-025bc2cc2ac1";
        let address_calc = address_with_port(address);
        assert!(
            address_expected == address_calc,
            "Fetch address calc: \n{}",
            address_calc
        );
    }

    #[test]
    fn address_4() {
        let address = "wss://westend.kilt.io:9977";
        let address_expected = "wss://westend.kilt.io:9977";
        let address_calc = address_with_port(address);
        assert!(
            address_expected == address_calc,
            "Fetch address calc: \n{}",
            address_calc
        );
    }

    #[test]
    fn address_5() {
        let address = "wss://full-nodes.kilt.io:9944/";
        let address_expected = "wss://full-nodes.kilt.io:9944/";
        let address_calc = address_with_port(address);
        assert!(
            address_expected == address_calc,
            "Fetch address calc: \n{}",
            address_calc
        );
    }

    #[test]
    fn address_6() {
        let address = "wss://peregrine.kilt.io/parachain-public-ws/";
        let address_expected = "wss://peregrine.kilt.io:443/parachain-public-ws/";
        let address_calc = address_with_port(address);
        assert!(
            address_expected == address_calc,
            "Fetch address calc: \n{}",
            address_calc
        );
    }
}
