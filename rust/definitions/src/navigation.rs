use crate::{
    history::Event,
    keyring::NetworkSpecsKey,
    metadata::MetaValues,
    network_specs::{NetworkSpecs, ValidCurrentVerifier, Verifier},
};
use plot_icon::EMPTY_PNG;

#[derive(PartialEq, Clone)]
pub struct SeedNameWithIdenticon {
    pub seed_name: String,
    pub identicon: String,
}

/// Enum containing card sets for three different outcomes:
/// signing (Sign), accepting (Stub) and reading, for example, in case of an error (Read)
#[derive(PartialEq, Debug, Clone)]
pub enum TransactionAction {
    Derivations {
        content: String,
        network_info: String,
        checksum: u32,
        network_specs_key: NetworkSpecsKey,
    },
    Sign {
        content: String,
        checksum: u32,
        has_pwd: bool,
        author_info: String,
        network_info: String,
    },
    Stub {
        s: String,
        u: u32,
        stub: StubNav,
    },
    Read {
        r: String,
    },
}

/// Enum describing Stub content.
/// Is used for proper navigation. Variants:
/// AddSpecs (with associated NetworkSpecsKey), LoadMeta (with associated
/// NetworkSpecsKey for the first by order network using those metadata),
/// and LoadTypes
#[derive(PartialEq, Debug, Clone)]
pub enum StubNav {
    AddSpecs { n: NetworkSpecsKey },
    LoadMeta { l: NetworkSpecsKey },
    LoadTypes,
}

#[derive(Clone, PartialEq)]
pub struct ActionResult {
    pub screen: Option<String>,
    pub screen_label: String,
    pub back: bool,
    pub footer: bool,
    pub footer_button: String,
    pub right_button: String,
    pub screen_name_type: String,
    pub modal: String,
    pub alert: String,
    pub screen_data: ScreenData,
    pub modal_data: String,
    pub alert_data: String,
}

#[derive(Clone, PartialEq)]
pub struct LogScreenEntry {
    pub order: u32,
    pub timestamp: String,
    pub events: Vec<Event>,
}

#[derive(Clone, PartialEq)]
pub enum ScreenData {
    Empty,
    LogData {
        log: Vec<LogScreenEntry>,
    },
    LogDetailsData {
        details_data: String,
    },
    SettingsData {
        verifier: Verifier,
    },
    SelectSeedForBackupData,
    VerifierData {
        verifier: Verifier,
    },
    DocumentsData,
    ManageNetworksData {
        networks: Vec<NetworkSpecs>,
    },
    NetworkDetailsData {
        details: NetworkDetails,
    },
    SignSufficientCryptoData,
    ScanData,
    TransactionData {
        t: TransactionAction,
    },
    SeedNamesWithIdenticons {
        n: Vec<SeedNameWithIdenticon>,
    },
    IdentitiesForSeedNameAndNetworkData {
        i: IdentitiesForSeedNameAndNetwork,
        multiselect_mode: bool,
        multiselect_count: String,
    },
    ExportedKeyData {
        k: ExportedKey,
    },
    ExportedKeyDataMulti {
        k: ExportedKey,
        current_number: u32,
        out_of: u32,
    },
    KeyboardData {
        keyboard: bool,
    },
    RecoverSeedNameData {
        seed_name: String,
        keyboard: bool,
    },
    RecoverSeedPhraseData {
        seed_name: String,
        keyboard: bool,
        user_input: String,
        guess_set: Vec<String>,
        draft: String,
    },
    DerivePrepData {
        d: DerivePrep,
        keyboard: bool,
    },
    AllIdentitiesData {
        identities: Vec<Identity>,
    },
}

#[derive(Clone, PartialEq)]
pub struct Identity {
    pub seed_name: String,
    pub address_key: String,
    pub public_key: String,
    pub identicon: String,
    pub has_pwd: bool,
    pub path: String,
    pub is_multiselect: bool,
    pub base58: String,
}

#[derive(Clone, PartialEq)]
pub struct NetworkDetails {
    pub specs: NetworkSpecs,
    pub current_verifier: ValidCurrentVerifier,
    pub general_verifier: Verifier,
    pub meta_values: Vec<MetaValues>,
}

#[derive(Clone, PartialEq)]
pub struct DerivePrep {
    pub seed_name: String,
    pub network_title: String,
    pub network_logo: String,
    pub network_specs_key: String,
    pub suggested_derivation: String,
    pub collision: Option<CollisionDisplay>,
}

#[derive(Clone, PartialEq)]
pub struct CollisionDisplay {
    pub base58: String,
    pub path: String,
    pub has_pwd: bool,
    pub identicon: String,
    pub seed_name: String,
}

#[derive(Clone, PartialEq)]
pub struct RootNetworkId {
    pub seed_name: String,
    pub identicon: String,
    pub address_key: String,
    pub base58: String,
    pub swiped: bool,
    pub is_multiselect: bool,
}

impl Default for RootNetworkId {
    fn default() -> Self {
        RootNetworkId {
            identicon: hex::encode(EMPTY_PNG),
            seed_name: Default::default(),
            address_key: Default::default(),
            base58: Default::default(),
            swiped: Default::default(),
            is_multiselect: Default::default(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct OtherNetworkId {
    pub address_key: String,
    pub base58: String,
    pub identicon: String,
    pub has_pwd: bool,
    pub path: String,
    pub swiped: bool,
    pub is_multiselect: bool,
}

#[derive(Clone, PartialEq)]
pub struct IdentitiesForSeedNameAndNetwork {
    pub root: RootNetworkId,
    pub set: Vec<OtherNetworkId>,
    pub title: String,
    pub logo: String,
}

#[derive(Clone, PartialEq)]
pub struct ExportedKey {
    pub qr: String,
    pub pubkey: String,
    pub address_base58: String,
    pub identicon: String,
    pub seed_name: String,
    pub path: String,
    pub network_title: String,
    pub network_logo: String,
}
