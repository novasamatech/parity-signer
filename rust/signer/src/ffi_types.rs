use crate::UniffiCustomTypeConverter;
pub use definitions::{
    crypto::Encryption,
    history::{
        IdentityHistory, MetaValuesDisplay, MetaValuesExport, NetworkSpecsDisplay,
        NetworkSpecsExport, NetworkVerifierDisplay, SignDisplay, SignMessageDisplay, TypesDisplay,
        TypesExport,
    },
    network_specs::{
        NetworkSpecs, NetworkSpecsToSend, ValidCurrentVerifier, Verifier, VerifierValue,
    },
};
pub use navigator::Action;

use std::convert::TryFrom;

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

    // Convert our custom type to Builtin
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

    fn into_custom(_val: Self::Builtin) -> uniffi::Result<Self> {
        unimplemented!()
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0.to_vec()
    }
}

#[derive(Clone)]
pub enum Event {
    /// Network metadata was added
    MetadataAdded {
        meta_values_display: MetaValuesDisplay,
    },

    /// Network metadata was removed
    MetadataRemoved {
        meta_values_display: MetaValuesDisplay,
    },

    /// User has generated [`SufficientCrypto`](crate::crypto::SufficientCrypto)
    /// with one of Signer addresses for `load_metadata` update
    MetadataSigned {
        meta_values_export: MetaValuesExport,
    },

    /// Network specs were added
    NetworkSpecsAdded {
        network_specs_display: NetworkSpecsDisplay,
    },

    /// Network specs were removed
    NetworkSpecsRemoved {
        network_specs_display: NetworkSpecsDisplay,
    },

    /// User has generated [`SufficientCrypto`](crate::crypto::SufficientCrypto)
    /// with one of Signer addresses for `add_specs` update
    NetworkSpecsSigned {
        network_specs_export: NetworkSpecsExport,
    },

    /// Network verifier with [`ValidCurrentVerifier`] was set for network
    NetworkVerifierSet {
        network_verifier_display: NetworkVerifierDisplay,
    },

    /// General verifier was set up
    GeneralVerifierSet { verifier: Verifier },

    /// Types information was added
    TypesAdded { types_display: TypesDisplay },

    /// Types information was removed
    TypesRemoved { types_display: TypesDisplay },

    /// User has generated [`SufficientCrypto`](crate::crypto::SufficientCrypto)
    /// with one of Signer addresses for `load_types` update
    TypesSigned { types_export: TypesExport },

    /// User has generated signature for a transaction
    TransactionSigned { sign_display: SignDisplay },

    /// User tried to generate signature for a transaction, but failed to enter
    /// a valid password
    TransactionSignError { sign_display: SignDisplay },

    /// User has generated signature for a message
    MessageSigned {
        sign_message_display: SignMessageDisplay,
    },

    /// User tried to generate signature for a message, but failed to enter
    /// a valid password
    MessageSignError {
        sign_message_display: SignMessageDisplay,
    },

    /// User generated a new address
    IdentityAdded { identity_history: IdentityHistory },

    /// User removed an address
    IdentityRemoved { identity_history: IdentityHistory },

    /// All identities were wiped
    IdentitiesWiped,

    /// Signer was online, i.e. the air-gap was broken
    DeviceWasOnline,

    /// User has acknowledged the dangers detected and has reset the Signer
    /// danger status
    ResetDangerRecord,

    /// New seed was created (stored value here is the seed name)
    SeedCreated { seed_created: String },

    /// User opened seed backup, and seed phrase for shortly shown as a plain
    /// text on screen (stored value here is the seed name)
    SeedNameWasShown { seed_name_was_shown: String }, // for individual seed_name

    /// A warning was produces and displayed to user
    Warning { warning: String },

    /// User has entered wrong password
    WrongPassword,

    /// User has manually added entry to history log
    UserEntry { user_entry: String },

    /// System-generated entry into history log
    SystemEntry { system_entry: String },

    /// History was cleared
    HistoryCleared,

    /// Database was initiated
    DatabaseInitiated,
}

impl From<Event> for definitions::history::Event {
    fn from(e: Event) -> Self {
        match e {
            Event::MetadataAdded {
                meta_values_display,
            } => definitions::history::Event::MetadataAdded(meta_values_display),
            Event::MetadataRemoved {
                meta_values_display,
            } => definitions::history::Event::MetadataRemoved(meta_values_display),
            Event::MetadataSigned { meta_values_export } => {
                definitions::history::Event::MetadataSigned(meta_values_export)
            }
            Event::NetworkSpecsAdded {
                network_specs_display,
            } => definitions::history::Event::NetworkSpecsAdded(network_specs_display),
            Event::NetworkSpecsRemoved {
                network_specs_display,
            } => definitions::history::Event::NetworkSpecsRemoved(network_specs_display),
            Event::NetworkSpecsSigned {
                network_specs_export,
            } => definitions::history::Event::NetworkSpecsSigned(network_specs_export),
            Event::NetworkVerifierSet {
                network_verifier_display,
            } => definitions::history::Event::NetworkVerifierSet(network_verifier_display),
            Event::GeneralVerifierSet { verifier } => {
                definitions::history::Event::GeneralVerifierSet(verifier)
            }
            Event::TypesAdded { types_display } => {
                definitions::history::Event::TypesAdded(types_display)
            }
            Event::TypesRemoved { types_display } => {
                definitions::history::Event::TypesRemoved(types_display)
            }
            Event::TypesSigned { types_export } => {
                definitions::history::Event::TypesSigned(types_export)
            }
            Event::TransactionSigned { sign_display } => {
                definitions::history::Event::TransactionSigned(sign_display)
            }
            Event::TransactionSignError { sign_display } => {
                definitions::history::Event::TransactionSigned(sign_display)
            }
            Event::MessageSigned {
                sign_message_display,
            } => definitions::history::Event::MessageSigned(sign_message_display),
            Event::MessageSignError {
                sign_message_display,
            } => definitions::history::Event::MessageSignError(sign_message_display),
            Event::IdentityAdded { identity_history } => {
                definitions::history::Event::IdentityAdded(identity_history)
            }
            Event::IdentityRemoved { identity_history } => {
                definitions::history::Event::IdentityRemoved(identity_history)
            }
            Event::IdentitiesWiped => definitions::history::Event::IdentitiesWiped,
            Event::DeviceWasOnline => definitions::history::Event::DeviceWasOnline,
            Event::ResetDangerRecord => definitions::history::Event::ResetDangerRecord,
            Event::SeedCreated { seed_created } => {
                definitions::history::Event::SeedCreated(seed_created)
            }
            Event::SeedNameWasShown {
                seed_name_was_shown,
            } => definitions::history::Event::SeedNameWasShown(seed_name_was_shown),
            Event::Warning { warning } => definitions::history::Event::Warning(warning),
            Event::WrongPassword => definitions::history::Event::WrongPassword,
            Event::UserEntry { user_entry } => definitions::history::Event::UserEntry(user_entry),
            Event::SystemEntry { system_entry } => {
                definitions::history::Event::SystemEntry(system_entry)
            }
            Event::HistoryCleared => definitions::history::Event::HistoryCleared,
            Event::DatabaseInitiated => definitions::history::Event::DatabaseInitiated,
        }
    }
}
