//! Information that could be send and received through air-gap as QR codes content  
//!
//! All information that Signer could get without a complete wipe must arrive
//! through the QR codes maintaining the air-gap. In addition to transactions
//! that could be parsed and signed, this includes:
//! - adding new networks,  
//! - updating the metadata of existing networks,  
//! - updating the types information,  
//! - bulk-importing the derivations  
//!
//! QR codes for adding networks, loading metadata and updating types information
//! all have similar structure:
//! - prelude `53xxyy` where `xx` is the encryption type, and `yy` is the message type  
//! - verifier public key (if the QR code is signed by verifier)  
//! - content  
//! - verifier signature (if the QR code is signed by verifier)  
//!
//! QR codes for importing derivations are never signed and have structure:
//! - prelude `53ffde`
//! - content  
//!
//! This module deals with content part of QR codes.  

#[cfg(feature = "signer")]
use blake2_rfc::blake2b::blake2b;
use parity_scale_codec::{Decode, Encode};

use crate::crypto::Encryption;
use crate::error::{ErrorSource, TransferContent};
#[cfg(feature = "active")]
use crate::error_active::ErrorActive;
#[cfg(feature = "signer")]
use crate::error_signer::{ErrorSigner, InputSigner};
#[cfg(feature = "signer")]
use crate::helpers::pic_types;
use crate::network_specs::NetworkSpecsToSend;
use crate::types::TypeEntry;
use sp_core::H256;

/// `load_metadata` QR code content  
///
/// Messages `load_metadata` are used to update through air-gap the network
/// matadata for networks already known to the Signer.  
pub struct ContentLoadMeta(Vec<u8>);

#[derive(Decode, Encode)]
struct DecodedContentLoadMeta {
    meta: Vec<u8>,
    genesis_hash: H256,
}

impl ContentLoadMeta {
    /// Generate [`ContentLoadMeta`] from metadata `&[u8]` slice and network genesis hash.
    pub fn generate(meta: &[u8], genesis_hash: &H256) -> Self {
        Self(
            DecodedContentLoadMeta {
                meta: meta.to_vec(),
                genesis_hash: genesis_hash.to_owned(),
            }
            .encode(),
        )
    }

    /// Transform `&[u8]` slice into [`ContentLoadMeta`].
    pub fn from_slice(slice: &[u8]) -> Self {
        Self(slice.to_vec())
    }

    /// Get metadata `Vec<u8>` from [`ContentLoadMeta`].
    pub fn meta<T: ErrorSource>(&self) -> Result<Vec<u8>, T::Error> {
        match <DecodedContentLoadMeta>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.meta),
            Err(_) => Err(<T>::transfer_content_error(TransferContent::LoadMeta)),
        }
    }

    /// Get genesis hash `[u8; 32]` from [`ContentLoadMeta`].
    pub fn genesis_hash<T: ErrorSource>(&self) -> Result<H256, T::Error> {
        match <DecodedContentLoadMeta>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.genesis_hash),
            Err(_) => Err(<T>::transfer_content_error(TransferContent::LoadMeta)),
        }
    }

    /// Get metadata `Vec<u8>` and genesis hash `[u8; 32]` from [`ContentLoadMeta`] as a tuple.
    pub fn meta_genhash<T: ErrorSource>(&self) -> Result<(Vec<u8>, H256), T::Error> {
        match <DecodedContentLoadMeta>::decode(&mut &self.0[..]) {
            Ok(a) => Ok((a.meta, a.genesis_hash)),
            Err(_) => Err(<T>::transfer_content_error(TransferContent::LoadMeta)),
        }
    }

    /// Write [`ContentLoadMeta`] into file that could be signed by the verifier.
    #[cfg(feature = "active")]
    pub fn write(&self, filename: &str) -> Result<(), ErrorActive> {
        match std::fs::write(&filename, &self.to_sign()) {
            Ok(_) => Ok(()),
            Err(e) => Err(ErrorActive::Output(e)),
        }
    }

    /// Transform [`ContentLoadMeta`] into `Vec<u8>` that could be signed by the verifier.
    pub fn to_sign(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Transform [`ContentLoadMeta`] into `Vec<u8>` that is concatenated with
    /// other parts of the QR code.
    ///
    /// Currently similar to `.to_sign()` function.
    pub fn to_transfer(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

/// `add_specs` QR code content  
///
/// Messages `add_specs` are used to add new networks to the Signer.  
#[derive(Decode, Encode)]
pub struct ContentAddSpecs(Vec<u8>);

#[derive(Decode, Encode)]
struct DecodedContentAddSpecs {
    specs: NetworkSpecsToSend,
}

impl ContentAddSpecs {
    /// Generate [`ContentAddSpecs`] from network specs [`NetworkSpecsToSend`].
    pub fn generate(specs: &NetworkSpecsToSend) -> Self {
        Self(
            DecodedContentAddSpecs {
                specs: specs.to_owned(),
            }
            .encode(),
        )
    }

    /// Transform `&[u8]` slice into [`ContentAddSpecs`].
    pub fn from_slice(slice: &[u8]) -> Self {
        Self(slice.to_vec())
    }

    /// Get network specs [`NetworkSpecsToSend`] from [`ContentAddSpecs`].
    pub fn specs<T: ErrorSource>(&self) -> Result<NetworkSpecsToSend, T::Error> {
        match <DecodedContentAddSpecs>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.specs),
            Err(_) => Err(<T>::transfer_content_error(TransferContent::AddSpecs)),
        }
    }

    /// Write [`ContentAddSpecs`] into file that could be signed by the verifier.
    #[cfg(feature = "active")]
    pub fn write(&self, filename: &str) -> Result<(), ErrorActive> {
        match std::fs::write(&filename, &self.to_sign()) {
            Ok(_) => Ok(()),
            Err(e) => Err(ErrorActive::Output(e)),
        }
    }

    /// Transform [`ContentAddSpecs`] into `Vec<u8>` that could be signed by the verifier.
    pub fn to_sign(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Transform [`ContentAddSpecs`] into `Vec<u8>` that is concatenated with
    /// other parts of the QR code.
    ///
    /// Note that it is different from `.to_sign()` function. Effectively, already
    /// SCALE-encoded [`NetworkSpecsToSend`] are encoded second time as an opaque
    /// Vec<u8>. This is done to have encoded piece length announced at the
    /// beginning of the `u8` set, to simplify cutting the received message in Signer.
    pub fn to_transfer(&self) -> Vec<u8> {
        self.encode()
    }
}

/// `load_types` QR code content  
///
/// Messages `load_types` are used to add or update Signer types information.  
///
/// Externally acquired types information is needed only for
/// [`RuntimeMetadata`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/enum.RuntimeMetadata.html)
/// V13 and below. After V14 all types information is contained within the metadata.  
///
/// This kind of messages is expected to be used seldom, if ever.  
#[derive(Decode, Encode)]
pub struct ContentLoadTypes(Vec<u8>);

#[derive(Decode, Encode)]
struct DecodedContentLoadTypes {
    types: Vec<TypeEntry>,
}

impl ContentLoadTypes {
    /// Generate [`ContentLoadTypes`] from types information `&[TypeEntry]`.  
    pub fn generate(types: &[TypeEntry]) -> Self {
        Self(
            DecodedContentLoadTypes {
                types: types.to_owned(),
            }
            .encode(),
        )
    }

    /// Transform `&[u8]` slice into [`ContentLoadTypes`].  
    pub fn from_slice(slice: &[u8]) -> Self {
        Self(slice.to_vec())
    }

    /// Get types information `Vec<TypeEntry>` from [`ContentLoadTypes`].  
    pub fn types<T: ErrorSource>(&self) -> Result<Vec<TypeEntry>, T::Error> {
        match <DecodedContentLoadTypes>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.types),
            Err(_) => Err(<T>::transfer_content_error(TransferContent::LoadTypes)),
        }
    }

    /// Write [`ContentLoadTypes`] into file that could be signed by the verifier.  
    #[cfg(feature = "active")]
    pub fn write(&self, filename: &str) -> Result<(), ErrorActive> {
        match std::fs::write(&filename, &self.to_sign()) {
            Ok(_) => Ok(()),
            Err(e) => Err(ErrorActive::Output(e)),
        }
    }

    /// Transform [`ContentLoadTypes`] into `Vec<u8>` to be put in the database.  
    pub fn store(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Transform [`ContentLoadTypes`] into `Vec<u8>` that could be signed by the verifier.  
    pub fn to_sign(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Transform [`ContentLoadTypes`] into `Vec<u8>` that is concatenated with
    /// other parts of the QR code.  
    ///
    /// Note that it is different from `.to_sign()` function. Effectively, already
    /// SCALE-encoded `Vec<TypeEntry>` is encoded second time as an opaque
    /// Vec<u8>. This is done to have encoded piece length announced at the
    /// beginning of the `u8` set, to simplify cutting the received message in Signer.  
    pub fn to_transfer(&self) -> Vec<u8> {
        self.encode()
    }

    /// Generate types information hash and corresponding id pic  
    ///
    /// Types information hash is calculated for `Vec<u8>` of encoded types information,
    /// as it would be stored in the database  
    #[cfg(feature = "signer")]
    pub fn show(&self) -> (String, Vec<u8>) {
        let types_hash = blake2b(32, &[], &self.store()).as_bytes().to_vec();
        let types_id_pic = pic_types(&types_hash);
        (hex::encode(types_hash), types_id_pic)
    }
}

/// Derivations import QR code content  
///
/// Derivations import could be used to generate or to restore a set of
/// **password-free** derivations in Signer avoiding manual adding of multiple
/// derivations.  
///
/// Popular request.  
///
/// In addition to derivations themselves, contains information about encryption
/// and genesis hash of the network in which the corresponding keys would be
/// generated or updated.  
#[derive(Decode, Encode)]
pub struct ContentDerivations(Vec<u8>);

#[derive(Decode, Encode)]
struct DecodedContentDerivations {
    encryption: Encryption,
    genesis_hash: H256,
    derivations: Vec<String>,
}

impl ContentDerivations {
    /// Generate [`ContentDerivations`] from network encryption [`Encryption`],
    /// genesis hash `[u8; 32]`, and set of derivations `&[String]`.  
    pub fn generate(encryption: &Encryption, genesis_hash: &H256, derivations: &[String]) -> Self {
        Self(
            DecodedContentDerivations {
                encryption: encryption.to_owned(),
                genesis_hash: genesis_hash.to_owned(),
                derivations: derivations.to_vec(),
            }
            .encode(),
        )
    }

    /// Transform `&[u8]` slice into [`ContentDerivations`].  
    pub fn from_slice(slice: &[u8]) -> Self {
        Self(slice.to_vec())
    }

    /// Get encryption [`Encryption`], genesis hash `[u8; 32]` and derivations `Vec<String>`
    /// from [`ContentDerivations`] as a tuple.
    #[cfg(feature = "signer")]
    pub fn encryption_genhash_derivations(
        &self,
    ) -> Result<(Encryption, H256, Vec<String>), ErrorSigner> {
        match <DecodedContentDerivations>::decode(&mut &self.0[..]) {
            Ok(a) => Ok((a.encryption, a.genesis_hash, a.derivations)),
            Err(_) => Err(ErrorSigner::Input(InputSigner::TransferDerivations)),
        }
    }

    /// Transform [`ContentDerivations`] into `Vec<u8>` that is concatenated with
    /// prelude to get the QR code data.  
    pub fn to_transfer(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}
