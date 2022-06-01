//! Encryption-related types with public information  
//!
//! Signer supports all three encryption algorithms currently used by the
//! Substrate: `Ed25519`, `Sr25519`, and `Ecdsa`.
//!
//! In addition to [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)
//! and [`MultiSignature`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSignature.html),
//! Signer uses similarly structured enums [`Encryption`] with only encryption
//! algorithm information and [`SufficientCrypto`] with encryption, public key
//! and data signature combined.  
//!
//! Networks are expected to use certain encryption algorithm [`Encryption`].
//! This is reflected in [`NetworkSpecs`](crate::network_specs::NetworkSpecs)
//! and in [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend)
//! field `encryption`, and also in corresponding database keys
//! [`NetworkSpecsKey`](crate::keyring::NetworkSpecsKey).
//! In principle, network could support more than one encryption algorithm,
//! in this case there would be more than one database entry for the network
//! specs with different keys.  
//! Default networks (Polkadot, Kusama, and Westend) all operate with `Sr25519`
//! encryption.  
//!
//! Each address in Signer has an associated encryption algorithm
//! [`Encryption`]. This is reflected in [`AddressDetails`](crate::users::AddressDetails)
//! field `encryption` and in corresponding database key
//! [`AddressKey`](crate::keyring::AddressKey). Address is associated with a
//! network, and only matching [`Encryption`] is allowed.
//! Address is generated from seed phrase and derivation, public keys produced
//! for different encryption algorithms with same seed phrase and derivation
//! are totally different.  
//!
//! Both general verifier and each individual network
//! [`CurrentVerifier`](crate::network_specs::CurrentVerifier) also have
//! associated [`Encryption`]. Encryption algorithm used by verifiers could be
//! different from the one used by the network they are verifying, it is used
//! only to check the validity of the signed update.  
//!
//! Signer receives Substrate transactions starting in hexadecimal recording
//! with `53xxyy` where `xx` corresponds to the encryption type.
//!
//! For signable transactions, possible encryptions are:
//!
//! - `00` for `Ed25519`
//! - `01` for `Sr25519`
//! - `02` for `Ecdsa`
//!
//! To be able to sign transaction in Signer, address producing transaction
//! must be associated with declared encryption algorithm and with transaction
//! network with. Also, transaction network must have entry in the database
//! having declared encryption algorithm.
//!
//! Updating transaction, in addition to three variants above, also may be
//! unsigned, with `ff` as an encryption piece. `ff` means only that no
//! signature is provided, it has no corresponding value in [`Encryption`].
//!
//! [`SufficientCrypto`] is a construction that could be created in Signer
//! and exported through static QR code. It contains encryption algorithm,
//! public key and corresponding signature for some data. Data could be network
//! specs, network metadata, or types information, that is loaded into Signer.
//! The information in [`SufficientCrypto`] is sufficient to verify the updating
//! payload validity elsewhere.
//!
//! This way Signer user can produce verified updating transactions with
//! own signatures.  
//!
//! [`SufficientCrypto`] could be used by the hot-side client to generate QR
//! codes with updating payloads signed by an address from Signer.  
//!
//! Signer keeps track of the generated [`SufficientCrypto`] QR codes in
//! history log.  
use parity_scale_codec::{Decode, Encode};
use sp_core;
use sp_runtime::{MultiSignature, MultiSigner};

use crate::network_specs::VerifierValue;

/// Encryption algorithm
///
/// Lists all encryption algorithms supported by Substrate
#[derive(Clone, PartialEq, Debug, Decode, Encode)]
pub enum Encryption {
    Ed25519,
    Sr25519,
    Ecdsa,
}

impl Encryption {
    /// Display the encryption  
    ///
    /// This is used both in error printing and in json data exports  
    pub fn show(&self) -> String {
        match &self {
            Encryption::Ed25519 => String::from("ed25519"),
            Encryption::Sr25519 => String::from("sr25519"),
            Encryption::Ecdsa => String::from("ecdsa"),
        }
    }
}

/// Data sufficient to generate signed update  
///
/// Contains public key and signature for data within certain encryption
/// algorithm.  
///
/// Stores no information on what data exactly is signed, supposedly user
/// keeps track of what they are signing.  
#[derive(Decode, Encode, PartialEq, Debug)]
pub enum SufficientCrypto {
    /// `Ed25519` encryption algorithm
    Ed25519 {
        /// public key of the signature author
        public: sp_core::ed25519::Public,
        /// signature for the data
        signature: sp_core::ed25519::Signature,
    },

    /// `Sr25519` encryption algorithm
    Sr25519 {
        /// public key of the signature author
        public: sp_core::sr25519::Public,
        /// signature for the data
        signature: sp_core::sr25519::Signature,
    },

    /// `Ecdsa` encryption algorithm
    Ecdsa {
        /// public key of the signature author
        public: sp_core::ecdsa::Public,
        /// signature for the data
        signature: sp_core::ecdsa::Signature,
    },
}

impl SufficientCrypto {
    /// Get [`VerifierValue`] from public key part of [`SufficientCrypto`]
    pub fn verifier_value(&self) -> VerifierValue {
        match &self {
            SufficientCrypto::Ed25519 {
                public,
                signature: _,
            } => VerifierValue::Standard {
                m: MultiSigner::Ed25519(public.to_owned()),
            },
            SufficientCrypto::Sr25519 {
                public,
                signature: _,
            } => VerifierValue::Standard {
                m: MultiSigner::Sr25519(public.to_owned()),
            },
            SufficientCrypto::Ecdsa {
                public,
                signature: _,
            } => VerifierValue::Standard {
                m: MultiSigner::Ecdsa(public.to_owned()),
            },
        }
    }

    /// Get [`MultiSignature`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSignature.html)
    /// from signature part of [`SufficientCrypto`]
    pub fn multi_signature(&self) -> MultiSignature {
        match &self {
            SufficientCrypto::Ed25519 {
                public: _,
                signature,
            } => MultiSignature::Ed25519(signature.to_owned()),
            SufficientCrypto::Sr25519 {
                public: _,
                signature,
            } => MultiSignature::Sr25519(signature.to_owned()),
            SufficientCrypto::Ecdsa {
                public: _,
                signature,
            } => MultiSignature::Ecdsa(signature.to_owned()),
        }
    }
}
