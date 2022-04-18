use crate::UniffiCustomTypeConverter;
pub use definitions::{
    crypto::Encryption,
    history::{
        Event, IdentityHistory, MetaValuesDisplay, MetaValuesExport, NetworkSpecsDisplay,
        NetworkSpecsExport, NetworkVerifierDisplay, SignDisplay, SignMessageDisplay, TypesDisplay,
        TypesExport,
    },
    network_specs::{
        NetworkSpecs, NetworkSpecsToSend, ValidCurrentVerifier, Verifier, VerifierValue,
    },
};
pub use navigator::Action;

use std::convert::{TryFrom, TryInto};

pub type Ed25519Public = sp_core::ed25519::Public;
pub type Sr25519Public = sp_core::sr25519::Public;
pub type EcdsaPublic = sp_core::ecdsa::Public;
pub type MultiSigner = sp_runtime::MultiSigner;
pub type H256 = sp_core::H256;

impl UniffiCustomTypeConverter for sp_runtime::MultiSigner {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(serde_json::from_str(&val)?)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_string()
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
