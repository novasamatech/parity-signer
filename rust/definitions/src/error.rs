use hex::FromHexError;
use sp_core::H256;
#[cfg(feature = "test")]
use variant_count::VariantCount;

use crate::{
    crypto::Encryption,
    keyring::{AddressKey, NetworkSpecsKey},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    HexDecodingError(#[from] FromHexError),

    #[error(transparent)]
    CodecError(#[from] parity_scale_codec::Error),

    #[error(transparent)]
    Wasm(#[from] crate::error_active::Wasm),

    #[error(transparent)]
    Ss58(#[from] sp_core::crypto::PublicError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    LibSecp(#[from] libsecp256k1::Error),

    /// Received public key length is different from the one expected for
    /// given encryption algorithm.
    #[error("Public key length does not match the encryption.")]
    WrongPublicKeyLength,

    /// `Error` when there is encryption mismatch between one of
    /// [`NetworkSpecsKey`] in `network_id` field of [`AddressDetails`] and the
    /// `encryption` field of [`AddressDetails`]
    ///
    /// [`AddressDetails`]: crate::users::AddressDetails
    #[error(
        "Address details entry with address key {} has not matching encryption {}.",
        hex::encode(address_key.key()),
        encryption.show(),
    )]
    EncryptionMismatch {
        address_key: AddressKey,
        encryption: Encryption,
    },

    /// [`NetworkSpecsKey`] is built using network genesis hash and [`Encryption`].
    /// [`OrderedNetworkSpecs`](crate::network_specs::OrderedNetworkSpecs) entry stored under
    /// this `NetworkSpecsKey` in `SPECSTREE` tree of the cold database
    /// contains `genesis_hash` field with a different genesis hash.
    #[error(
        "Network specs (NetworkSpecs) entry with network specs key {} \
        has not matching genesis hash {}.",
        hex::encode(network_specs_key.key()),
        hex::encode(genesis_hash),
    )]
    SpecsGenesisHashMismatch {
        network_specs_key: NetworkSpecsKey,
        genesis_hash: H256,
    },

    /// [`NetworkSpecsKey`] is built using network genesis hash and [`Encryption`].
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) entry
    /// stored under this `NetworkSpecsKey` in `SPECSTREEPREP` tree of the hot
    /// database contains `encryption` field with a different [`Encryption`].
    #[error(
        "specs encryption mismatch {} {}`",
        hex::encode(network_specs_key.key()),
        encryption.show()
    )]
    SpecsToSendEncryptionMismatch {
        network_specs_key: NetworkSpecsKey,
        encryption: Encryption,
    },

    /// [`NetworkSpecsKey`] is built using network genesis hash and [`Encryption`].
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) entry
    /// stored under this `NetworkSpecsKey` in `SPECSTREEPREP` tree of the hot
    /// database contains `genesis_hash` field with a different genesis hash.
    #[error(
        "Network specs (NetworkSpecs) entry with network specs key {} \
        has wrong genesis hash {}.",
        hex::encode(network_specs_key.key()),
        hex::encode(genesis_hash),
    )]
    SpecsToSendGenesisHash {
        network_specs_key: NetworkSpecsKey,
        genesis_hash: H256,
    },

    #[error(transparent)]
    MetadataError(#[from] MetadataError),

    /// Network name and/or network version in [`MetaKey`] do not match the
    /// network name and network version from `Version` constant, `System`
    /// pallet of the metadata stored under this [`MetaKey`].
    ///
    /// Error could be encountered only in the hot database.
    ///
    /// [`MetaKey`]: crate::keyring::MetaKey
    #[error(
        "Meta key corresponds to {this_name}{this_version}. \
        Stored metadata is {that_name}{that_version}."
    )]
    MetadataMismatch {
        this_name: String,
        this_version: u32,
        that_name: String,
        that_version: u32,
    },

    #[error("Cannot convert {0} to valid encryption.")]
    UnknownEncryption(String),

    #[error("Ethereum addresses are not supported here.")]
    EthereumAddressNotSupported,
}

/// Error decoding transfer content
///
/// All variants could be encountered both on the active side
/// (when checking the message content while signing it)
/// and on the Signer side (when processing the received messages)
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum TransferContent {
    /// `add_specs` message content
    AddSpecs,

    /// `load_metadata` message content
    LoadMeta,

    /// `load_types` message content
    LoadTypes,
}

impl TransferContent {
    /// Display [`TransferContent`] in readable form
    pub fn show(&self) -> String {
        let insert = match &self {
            TransferContent::AddSpecs => "`add_specs`",
            TransferContent::LoadMeta => "`load_meta`",
            TransferContent::LoadTypes => "`load_types`",
        };
        format!("Payload could not be decoded as {}.", insert)
    }
}

/// Intrinsic problems of the metadata making it unsuitable for Signer use
#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum MetadataError {
    /// Supported are `V12`, `V13`, and `V14` versions of
    /// [`RuntimeMetadata`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/enum.RuntimeMetadata.html).
    ///
    /// Any other version results in error.
    #[error("version incompatible")]
    VersionIncompatible,

    /// Metadata does not have `System` pallet, i.e. there is no place to look
    /// for network
    /// [`RuntimeVersion`](https://docs.rs/sp-version/latest/sp_version/struct.RuntimeVersion.html)
    #[error("no system pallet")]
    NoSystemPallet,

    /// Metadata does not have `Version` constant in `System` pallet, i.e.
    /// there is no place to look for network
    /// [`RuntimeVersion`](https://docs.rs/sp-version/latest/sp_version/struct.RuntimeVersion.html)
    #[error("no version in constants")]
    NoVersionInConstants,

    /// `Vec<u8>` retrieved from `Version` constant in `System` pallet could
    /// not be decoded as
    /// [`RuntimeVersion`](https://docs.rs/sp-version/latest/sp_version/struct.RuntimeVersion.html)
    #[error("runtime version not decodeable")]
    RuntimeVersionNotDecodeable,

    /// Metadata has `SS58Prefix` constant in `System` pallet, but its content
    /// could not be decoded as valid base58 prefix, i.e. as `u16` or `u8`
    /// number
    #[error("base58 prefix not decodable")]
    Base58PrefixNotDecodeable,

    /// Base58 prefix from metadata (`meta`) does not match base58 prefix in specs (`specs`)
    #[error("base58 prefix from metadata {} mismatches prefix from specs {}", .specs, .meta)]
    Base58PrefixSpecsMismatch { specs: u16, meta: u16 },

    /// Metadata first 4 bytes are not expected `b"meta"` prelude
    #[error("metadata first bytes are not \"meta\" magic")]
    NotMeta,

    /// Metadata body (after `b"meta"` prelude) could not be decoded as
    /// [`RuntimeMetadata`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/enum.RuntimeMetadata.html)
    #[error("metadata body is not decodable")]
    UnableToDecode,
}
