use crate::crypto::Encryption;

use sp_core::H256;

use parity_scale_codec::{Decode, Encode};

use sp_runtime::MultiSigner;

#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq)]
pub enum DynamicDerivationsRequest {
    V1(DynamicDerivationsAddressRequestV1),
}

#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq)]
pub struct DynamicDerivationsAddressRequestV1 {
    pub addrs: Vec<DynamicDerivationsRequestInfo>,
}

#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq)]
pub struct DynamicDerivationsRequestInfo {
    /// Public key of the root key
    pub multisigner: MultiSigner,
    pub dynamic_derivations: Vec<DynamicDerivationRequestInfo>,
}

#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq)]
pub struct DynamicDerivationRequestInfo {
    /// Dynamic derivation derivation path
    pub derivation_path: String,
    /// The type of encryption in the network
    pub encryption: Encryption,
    /// Genesis hash of the network for the dynamic derivation
    pub genesis_hash: H256,
}

#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq)]
pub enum DynamicDerivationsAddressResponse {
    V1(DynamicDerivationsAddressResponseV1),
}

#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq)]
pub struct DynamicDerivationsAddressResponseV1 {
    pub addrs: Vec<DynamicDerivationsResponseInfo>,
}

#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq)]
pub struct DynamicDerivationsResponseInfo {
    /// Public key of the root key
    pub multisigner: MultiSigner,
    pub dynamic_derivations: Vec<DynamicDerivationResponseInfo>,
}

#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq)]
pub struct DynamicDerivationResponseInfo {
    /// Public key of the root key
    pub derivation_path: String,
    /// The type of encryption in the network
    pub encryption: Encryption,
    /// Public key of the derivation path. The address may be derived from it.
    pub public_key: MultiSigner,
}
