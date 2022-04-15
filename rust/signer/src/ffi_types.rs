pub use definitions::crypto::Encryption;
pub use navigator::Action;

use std::convert::TryInto;

#[derive(Clone)]
pub struct IdentityHistory {
    pub seed_name: String,
    pub encryption: Encryption,
    pub public_key: Vec<u8>,
    pub path: String,
    pub network_genesis_hash: Vec<u8>,
}

impl From<IdentityHistory> for definitions::history::IdentityHistory {
    fn from(i: IdentityHistory) -> Self {
        let IdentityHistory {
            seed_name,
            encryption,
            public_key,
            path,
            network_genesis_hash,
        } = i;
        definitions::history::IdentityHistory::new(
            seed_name,
            encryption,
            public_key,
            path,
            network_genesis_hash,
        )
    }
}

#[derive(Clone)]
pub enum MultiSigner {
    Ed25519 { public: Ed25519Public },
    Sr25519 { public: Sr25519Public },
    Ecdsa { public: EcdsaPublic },
}

impl From<MultiSigner> for sp_runtime::MultiSigner {
    fn from(m: MultiSigner) -> Self {
        match m {
            MultiSigner::Ed25519 { public } => sp_runtime::MultiSigner::Ed25519(public.into()),
            MultiSigner::Sr25519 { public } => sp_runtime::MultiSigner::Sr25519(public.into()),
            MultiSigner::Ecdsa { public } => sp_runtime::MultiSigner::Ecdsa(public.into()),
        }
    }
}

#[derive(Clone)]
pub struct SignMessageDisplay {
    pub message: String,
    pub network_name: String,
    pub signed_by: VerifierValue,
    pub user_comment: String,
}

impl From<SignMessageDisplay> for definitions::history::SignMessageDisplay {
    fn from(s: SignMessageDisplay) -> Self {
        let SignMessageDisplay {
            message,
            network_name,
            signed_by,
            user_comment,
        } = s;

        definitions::history::SignMessageDisplay::get(
            &message,
            &network_name,
            &signed_by.into(),
            &user_comment,
        )
    }
}

#[derive(Clone)]
pub struct Verifier {
    pub verifier_value: Option<VerifierValue>,
}

impl From<Verifier> for definitions::network_specs::Verifier {
    fn from(v: Verifier) -> Self {
        definitions::network_specs::Verifier(v.verifier_value.map(Into::into))
    }
}

#[derive(Clone)]
pub enum VerifierValue {
    Standard { multi_signer: MultiSigner },
}

impl From<VerifierValue> for definitions::network_specs::VerifierValue {
    fn from(v: VerifierValue) -> Self {
        match v {
            VerifierValue::Standard { multi_signer } => {
                definitions::network_specs::VerifierValue::Standard(multi_signer.into())
            }
        }
    }
}

#[derive(Clone)]
pub struct MetaValuesExport {
    pub name: String,
    pub version: u32,
    pub meta_hash: Vec<u8>,
    pub signed_by: VerifierValue,
}

impl From<MetaValuesExport> for definitions::history::MetaValuesExport {
    fn from(mv: MetaValuesExport) -> Self {
        let MetaValuesExport {
            name,
            version,
            meta_hash,
            signed_by,
        } = mv;
        definitions::history::MetaValuesExport::new(name, version, meta_hash, signed_by.into())
    }
}

#[derive(Clone)]
pub struct TypesExport {
    pub types_hash: Vec<u8>,
    pub signed_by: VerifierValue,
}

impl From<TypesExport> for definitions::history::TypesExport {
    fn from(t: TypesExport) -> Self {
        let TypesExport {
            types_hash,
            signed_by,
        } = t;
        definitions::history::TypesExport::new(types_hash, signed_by.into())
    }
}

#[derive(Clone)]
pub struct TypesDisplay {
    pub types_hash: Vec<u8>,
    pub verifier: Verifier,
}

impl From<TypesDisplay> for definitions::history::TypesDisplay {
    fn from(t: TypesDisplay) -> Self {
        let TypesDisplay {
            types_hash,
            verifier,
        } = t;
        definitions::history::TypesDisplay::new(types_hash, verifier.into())
    }
}

#[derive(Clone)]
pub enum ValidCurrentVerifier {
    General,
    Custom { verifier: Verifier },
}

impl From<ValidCurrentVerifier> for definitions::network_specs::ValidCurrentVerifier {
    fn from(v: ValidCurrentVerifier) -> Self {
        match v {
            ValidCurrentVerifier::General => {
                definitions::network_specs::ValidCurrentVerifier::General
            }
            ValidCurrentVerifier::Custom { verifier } => {
                definitions::network_specs::ValidCurrentVerifier::Custom(verifier.into())
            }
        }
    }
}

#[derive(Clone)]
pub struct NetworkVerifierDisplay {
    pub genesis_hash: Vec<u8>,
    pub valid_current_verifier: ValidCurrentVerifier,
    pub general_verifier: Verifier,
}

impl From<NetworkVerifierDisplay> for definitions::history::NetworkVerifierDisplay {
    fn from(n: NetworkVerifierDisplay) -> Self {
        let NetworkVerifierDisplay {
            genesis_hash,
            valid_current_verifier,
            general_verifier,
        } = n;
        definitions::history::NetworkVerifierDisplay::get(
            &definitions::keyring::VerifierKey::from_parts(&genesis_hash),
            &valid_current_verifier.into(),
            &general_verifier.into(),
        )
    }
}

#[derive(Clone)]
pub struct NetworkSpecsToSend {
    pub base58prefix: u16,
    pub color: String,
    pub decimals: u8,
    pub encryption: Encryption,
    pub genesis_hash: Vec<u8>,
    pub logo: String,
    pub name: String,
    pub path_id: String,
    pub secondary_color: String,
    pub title: String,
    pub unit: String,
}

impl From<NetworkSpecsToSend> for definitions::network_specs::NetworkSpecsToSend {
    fn from(n: NetworkSpecsToSend) -> Self {
        let NetworkSpecsToSend {
            base58prefix,
            color,
            decimals,
            encryption,
            genesis_hash,
            logo,
            name,
            path_id,
            secondary_color,
            title,
            unit,
        } = n;

        definitions::network_specs::NetworkSpecsToSend {
            base58prefix,
            color,
            decimals,
            encryption,
            genesis_hash: genesis_hash.try_into().unwrap(),
            logo,
            name,
            path_id,
            secondary_color,
            title,
            unit,
        }
    }
}

#[derive(Clone)]
pub struct NetworkSpecsExport {
    pub specs_to_send: NetworkSpecsToSend,
    pub signed_by: VerifierValue,
}

impl From<NetworkSpecsExport> for definitions::history::NetworkSpecsExport {
    fn from(a: NetworkSpecsExport) -> Self {
        let NetworkSpecsExport {
            specs_to_send,
            signed_by,
        } = a;
        definitions::history::NetworkSpecsExport::get(&specs_to_send.into(), &signed_by.into())
    }
}

#[derive(Clone)]
pub struct SignDisplay {
    pub transaction: Vec<u8>,
    pub network_name: String,
    pub signed_by: VerifierValue,
    pub user_comment: String,
}

impl From<SignDisplay> for definitions::history::SignDisplay {
    fn from(s: SignDisplay) -> Self {
        let SignDisplay {
            transaction,
            network_name,
            signed_by,
            user_comment,
        } = s;
        definitions::history::SignDisplay::new(
            transaction,
            network_name,
            signed_by.into(),
            user_comment,
        )
    }
}

#[derive(Clone)]
pub struct MetaValuesDisplay {
    pub name: String,
    pub version: u32,
    pub meta_hash: Vec<u8>,
}

impl From<MetaValuesDisplay> for definitions::history::MetaValuesDisplay {
    fn from(m: MetaValuesDisplay) -> Self {
        let MetaValuesDisplay {
            name,
            version,
            meta_hash,
        } = m;
        definitions::history::MetaValuesDisplay::new(name, version, meta_hash)
    }
}

#[derive(Clone)]
pub struct NetworkSpecs {
    pub base58prefix: u16,
    pub color: String,
    pub decimals: u8,
    pub encryption: Encryption,
    pub genesis_hash: Vec<u8>,
    pub logo: String,
    pub name: String,
    pub order: u8,
    pub path_id: String,
    pub secondary_color: String,
    pub title: String,
    pub unit: String,
}

impl From<NetworkSpecs> for definitions::network_specs::NetworkSpecs {
    fn from(n: NetworkSpecs) -> Self {
        let NetworkSpecs {
            base58prefix,
            color,
            decimals,
            encryption,
            genesis_hash,
            logo,
            name,
            order,
            path_id,
            secondary_color,
            title,
            unit,
        } = n;

        definitions::network_specs::NetworkSpecs {
            base58prefix,
            color,
            decimals,
            encryption,
            genesis_hash: genesis_hash.try_into().unwrap(),
            logo,
            name,
            order,
            path_id,
            secondary_color,
            title,
            unit,
        }
    }
}

#[derive(Clone)]
pub struct NetworkSpecsDisplay {
    pub specs: NetworkSpecs,
    pub valid_current_verifier: ValidCurrentVerifier,
    pub general_verifier: Verifier,
}

impl From<NetworkSpecsDisplay> for definitions::history::NetworkSpecsDisplay {
    fn from(n: NetworkSpecsDisplay) -> Self {
        let NetworkSpecsDisplay {
            specs,
            valid_current_verifier,
            general_verifier,
        } = n;

        definitions::history::NetworkSpecsDisplay::get(
            &specs.into(),
            &valid_current_verifier.into(),
            &general_verifier.into(),
        )
    }
}

#[derive(Clone)]
pub struct Ed25519Public {
    pub public: Vec<u8>,
}

impl From<Ed25519Public> for sp_core::ed25519::Public {
    fn from(p: Ed25519Public) -> Self {
        sp_core::ed25519::Public(
            p.public
                .try_into()
                .expect("keys always have fixed length; qed"),
        )
    }
}

#[derive(Clone)]
pub struct Sr25519Public {
    pub public: Vec<u8>,
}

impl From<Sr25519Public> for sp_core::sr25519::Public {
    fn from(p: Sr25519Public) -> Self {
        sp_core::sr25519::Public(
            p.public
                .try_into()
                .expect("keys always have fixed length; qed"),
        )
    }
}

#[derive(Clone)]
pub struct EcdsaPublic {
    pub public: Vec<u8>,
}

impl From<EcdsaPublic> for sp_core::ecdsa::Public {
    fn from(p: EcdsaPublic) -> Self {
        sp_core::ecdsa::Public(
            p.public
                .try_into()
                .expect("keys always have fixed length; qed"),
        )
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
            } => definitions::history::Event::MetadataAdded(meta_values_display.into()),
            Event::MetadataRemoved {
                meta_values_display,
            } => definitions::history::Event::MetadataRemoved(meta_values_display.into()),
            Event::MetadataSigned { meta_values_export } => {
                definitions::history::Event::MetadataSigned(meta_values_export.into())
            }
            Event::NetworkSpecsAdded {
                network_specs_display,
            } => definitions::history::Event::NetworkSpecsAdded(network_specs_display.into()),
            Event::NetworkSpecsRemoved {
                network_specs_display,
            } => definitions::history::Event::NetworkSpecsRemoved(network_specs_display.into()),
            Event::NetworkSpecsSigned {
                network_specs_export,
            } => definitions::history::Event::NetworkSpecsSigned(network_specs_export.into()),
            Event::NetworkVerifierSet {
                network_verifier_display,
            } => definitions::history::Event::NetworkVerifierSet(network_verifier_display.into()),
            Event::GeneralVerifierSet { verifier } => {
                definitions::history::Event::GeneralVerifierSet(verifier.into())
            }
            Event::TypesAdded { types_display } => {
                definitions::history::Event::TypesAdded(types_display.into())
            }
            Event::TypesRemoved { types_display } => {
                definitions::history::Event::TypesRemoved(types_display.into())
            }
            Event::TypesSigned { types_export } => {
                definitions::history::Event::TypesSigned(types_export.into())
            }
            Event::TransactionSigned { sign_display } => {
                definitions::history::Event::TransactionSigned(sign_display.into())
            }
            Event::TransactionSignError { sign_display } => {
                definitions::history::Event::TransactionSigned(sign_display.into())
            }
            Event::MessageSigned {
                sign_message_display,
            } => definitions::history::Event::MessageSigned(sign_message_display.into()),
            Event::MessageSignError {
                sign_message_display,
            } => definitions::history::Event::MessageSignError(sign_message_display.into()),
            Event::IdentityAdded { identity_history } => {
                definitions::history::Event::IdentityAdded(identity_history.into())
            }
            Event::IdentityRemoved { identity_history } => {
                definitions::history::Event::IdentityRemoved(identity_history.into())
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
