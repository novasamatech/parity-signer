use crate::UniffiCustomTypeConverter;
use definitions::helpers::{get_multisigner, multisigner_to_encryption, multisigner_to_public};
pub use definitions::{
    crypto::Encryption,
    derivations::{
        AddrInfo, DerivedKeyError, DerivedKeyInfo, DerivedKeyPreview, ExportAddrsV1, SeedInfo,
        SeedKeysPreview, SeedKeysPreviewSummary,
    },
    history::{
        Event, IdentityHistory, MetaValuesDisplay, MetaValuesExport, NetworkSpecsDisplay,
        NetworkSpecsExport, NetworkVerifierDisplay, SignDisplay, SignMessageDisplay, TypesDisplay,
        TypesExport,
    },
    metadata::MetaValues,
    navigation::*,
    network_specs::{
        NetworkSpecs, OrderedNetworkSpecs, ValidCurrentVerifier, Verifier, VerifierValue,
    },
};
pub use navigator::Action;

use std::convert::{TryFrom, TryInto};

pub type NetworkSpecsKey = definitions::keyring::NetworkSpecsKey;

impl UniffiCustomTypeConverter for NetworkSpecsKey {
    type Builtin = String;

    fn into_custom(hex_line: Self::Builtin) -> uniffi::Result<Self> {
        Self::from_hex(&hex_line).map_err(|_| anyhow::Error::msg("network specks key"))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        hex::encode(obj.key())
    }
}

pub type Ed25519Public = sp_core::ed25519::Public;
pub type Sr25519Public = sp_core::sr25519::Public;
pub type EcdsaPublic = sp_core::ecdsa::Public;
pub type MultiSigner = sp_runtime::MultiSigner;
pub type H256 = sp_core::H256;

impl UniffiCustomTypeConverter for sp_runtime::MultiSigner {
    type Builtin = Vec<String>;

    fn into_custom(tuple: Self::Builtin) -> uniffi::Result<Self> {
        match tuple.as_slice() {
            [public, encryption] => {
                let encryption = Encryption::try_from(encryption.to_string())
                    .map_err(|_| anyhow::Error::msg("unable to parse encryption".to_string()))?;
                let public = hex::decode(public)
                    .map_err(|_| anyhow::Error::msg("pubkey error".to_string()))?;
                Ok(get_multisigner(&public, &encryption)
                    .map_err(|_| anyhow::Error::msg("invalid multisigner".to_string()))?)
            }
            _ => Err(anyhow::Error::msg("invalid multisigner")),
        }
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        vec![
            hex::encode(multisigner_to_public(&obj)),
            multisigner_to_encryption(&obj).show(),
        ]
    }
}

impl UniffiCustomTypeConverter for Ed25519Public {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        let public = Ed25519Public::try_from(val.as_ref())
            .map_err(|_| anyhow::Error::msg("ed25519".to_string()))?;

        Ok(public)
    }

    // Convert our custom type to Builtin
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0.to_vec()
    }
}

impl UniffiCustomTypeConverter for Sr25519Public {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        let public = Sr25519Public::try_from(val.as_ref())
            .map_err(|_| anyhow::Error::msg("sr25519".to_string()))?;

        Ok(public)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0.to_vec()
    }
}

impl UniffiCustomTypeConverter for EcdsaPublic {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        let public = EcdsaPublic::try_from(val.as_ref())
            .map_err(|_| anyhow::Error::msg("ecdsa".to_string()))?;

        Ok(public)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0.to_vec()
    }
}

impl UniffiCustomTypeConverter for H256 {
    type Builtin = Vec<u8>;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        let hash: [u8; 32] = val
            .try_into()
            .map_err(|_| anyhow::Error::msg("H265".to_string()))?;
        Ok(hash.into())
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0.to_vec()
    }
}
