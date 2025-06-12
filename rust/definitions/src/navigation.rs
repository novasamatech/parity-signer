use sp_core::H256;

use crate::derivations::SeedKeysPreview;
use crate::{
    crypto::Encryption, history::Event, keyring::NetworkSpecsKey,
    network_specs::OrderedNetworkSpecs,
};

pub use crate::network_specs::NetworkSpecs;

#[derive(PartialEq, Eq, Clone)]
pub struct SeedNameWithIdenticon {
    pub seed_name: String,
    pub identicon: Identicon,
}

/// A single transaction signing action.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TransactionSignAction {
    /// Parsed contents of the transaction.
    pub content: TransactionCardSet,

    /// If this transaction should be signed with a passworded key.
    pub has_pwd: bool,

    /// Information about the signing key of this transaction.
    pub author_info: MAddressCard,

    /// Info about the network this tx happens on.
    pub network_info: OrderedNetworkSpecs,
}

/// Enum containing card sets for four different outcomes:
/// importing derivations (Derivations), signing (Sign),
/// accepting (Stub) and reading, for example, in case of an error (Read)
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TransactionAction {
    Derivations {
        content: Box<TransactionCardSet>,
    },
    Sign {
        actions: Vec<TransactionSignAction>,
        checksum: u32,
    },
    Stub {
        s: Box<TransactionCardSet>,
        u: u32,
        stub: StubNav,
    },
    Read {
        r: Box<TransactionCardSet>,
    },
}

/// Enum describing Stub content.
/// Is used for proper navigation. Variants:
/// `AddSpecs` (with associated `NetworkSpecsKey`), `LoadMeta` (with associated
/// `NetworkSpecsKey` for the first by order network using those metadata),
/// and `LoadTypes`
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum StubNav {
    AddSpecs { n: NetworkSpecsKey },
    LoadMeta { l: NetworkSpecsKey },
    LoadTypes,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum FooterButton {
    Log,
    Scan,
    Keys,
    Settings,
    Back,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum RightButton {
    LogRight,
    NewSeed,
    Backup,
    MultiSelect,
    NDMenu,
    TypesInfo,
    KeyMenu,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ScreenNameType {
    H1,
    H4,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ShieldAlert {
    Past,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AlertData {
    Shield { f: Option<ShieldAlert> },
    ErrorData { f: String },
    Confirm,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionResult {
    pub screen_label: String,
    pub back: bool,
    pub footer: bool,
    pub footer_button: Option<FooterButton>,
    pub right_button: Option<RightButton>,
    pub screen_name_type: ScreenNameType,
    pub screen_data: ScreenData,
    pub modal_data: Option<ModalData>,
    pub alert_data: Option<AlertData>,
}

#[derive(Clone, PartialEq, Eq)]
pub struct LogScreenEntry {
    pub timestamp: String,
    pub events: Vec<Event>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScreenData {
    Scan,
    Keys { f: String },
    Settings { f: MSettings },
    Log { f: MLog },
    LogDetails { f: MLogDetails },
    Transaction { f: Vec<MTransaction> },
    SeedSelector { f: MSeeds },
    KeyDetails { f: Option<MKeyDetails> },
    NewSeed { f: MNewSeed },
    RecoverSeedName { f: MRecoverSeedName },
    RecoverSeedPhrase { f: MRecoverSeedPhrase },
    DeriveKey { f: MDeriveKey },
    VVerifier { f: MVerifierDetails },
    ManageNetworks { f: MManageNetworks },
    NNetworkDetails { f: MNetworkDetails },
    SignSufficientCrypto { f: MSignSufficientCrypto },
    SelectSeedForBackup { f: MSeeds },
    Documents,
    KeyDetailsMulti { f: MKeyDetailsMulti },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MKeysCard {
    pub address: Address,
    pub address_key: String,
    pub base58: String,
    pub swiped: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MKeysNew {
    pub root: Option<MAddressCard>,
    pub set: Vec<MKeyAndNetworkCard>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MKeyAndNetworkCard {
    pub key: MKeysCard,
    pub network: MSCNetworkInfo,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MNetworkCard {
    pub title: String,
    pub logo: String,
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct MSettings {
    pub public_key: Option<String>,
    pub identicon: Option<Identicon>,
    pub encryption: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct History {
    pub order: u32,
    pub timestamp: String,
    pub events: Vec<Event>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MLog {
    pub log: Vec<History>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MEventMaybeDecoded {
    pub event: Event,
    pub decoded: Option<TransactionCardSet>,
    pub signed_by: Option<MAddressCard>,
    pub verifier_details: Option<MVerifierDetails>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MLogDetails {
    pub timestamp: String,
    pub events: Vec<MEventMaybeDecoded>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionType {
    Sign,
    Stub,
    Read,
    ImportDerivations,
    Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionCard {
    pub index: u32,
    pub indent: u32,
    pub card: Card,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BananaSplitRecoveryResult {
    RequestPassword,
    RecoveredSeed { s: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DynamicDerivationTransactionPayload {
    WithoutProof { s: String },
    WithProof { s: String }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeSequenceResult {
    BBananaSplitRecoveryResult { b: BananaSplitRecoveryResult },
    DynamicDerivations { s: String },
    DynamicDerivationTransaction { s: Vec<DynamicDerivationTransactionPayload> },
    Other { s: String },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TransactionCardSet {
    pub author: Option<Vec<TransactionCard>>,
    pub error: Option<Vec<TransactionCard>>,
    pub extensions: Option<Vec<TransactionCard>>,
    pub importing_derivations: Option<Vec<TransactionCard>>,
    pub message: Option<Vec<TransactionCard>>,
    pub meta: Option<Vec<TransactionCard>>,
    pub method: Option<Vec<TransactionCard>>,
    pub new_specs: Option<Vec<TransactionCard>>,
    pub verifier: Option<Vec<TransactionCard>>,
    pub warning: Option<Vec<TransactionCard>>,
    pub types_info: Option<Vec<TransactionCard>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MTransaction {
    pub content: TransactionCardSet,
    pub ttype: TransactionType,
    pub author_info: Option<MAddressCard>,
    pub network_info: Option<MSCNetworkInfo>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSignedTransaction {
    pub transaction: Vec<MTransaction>,
    pub signature: MSignatureReady,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedNameCard {
    pub seed_name: String,
    pub identicon: Identicon,
    pub used_in_networks: Vec<String>,
    pub derived_keys_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSeeds {
    pub seed_name_cards: Vec<SeedNameCard>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MKeyDetails {
    pub qr: QrData,
    pub pubkey: String,
    pub network_info: MSCNetworkInfo,
    pub base58: String,
    pub address: Address,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MNewSeed {
    pub keyboard: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MRecoverSeedName {
    pub keyboard: bool,
    pub seed_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MRecoverSeedPhrase {
    pub keyboard: bool,
    pub seed_name: String,
    pub user_input: String,
    pub guess_set: Vec<String>,
    pub draft: Vec<String>,
    pub ready_seed: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct DerivationCheck {
    pub button_good: bool,
    pub where_to: Option<DerivationDestination>,
    pub collision: Option<MAddressCard>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Address {
    pub path: String,
    pub has_pwd: bool,
    pub identicon: Identicon,
    pub seed_name: String,
    pub secret_exposed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MAddressCard {
    pub base58: String,
    pub address_key: String,
    pub address: Address,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DerivationDestination {
    Pwd,
    Pin,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MDeriveKey {
    pub seed_name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MVerifierDetails {
    pub public_key: String,
    pub identicon: Identicon,
    pub encryption: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MVerifier {
    pub ttype: String,
    pub details: MVerifierDetails,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MMetadataRecord {
    pub specname: String,
    pub specs_version: String,
    pub meta_hash: String,
    pub meta_id_pic: Identicon,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MNetworkDetails {
    pub base58prefix: u16,
    pub color: String,
    pub decimals: u8,
    pub encryption: Encryption,
    pub genesis_hash: H256,
    pub logo: String,
    pub name: String,
    pub order: String,
    pub path_id: String,
    pub secondary_color: String,
    pub title: String,
    pub unit: String,
    pub current_verifier: MVerifier,
    pub meta: Vec<MMetadataRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MRawKey {
    pub address: Address,
    pub address_key: String,
    pub public_key: String,
    pub network_logo: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSignSufficientCrypto {
    pub identities: Vec<MRawKey>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MKeyDetailsMulti {
    pub key_details: MKeyDetails,
    pub current_number: String,
    pub out_of: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MMNetwork {
    pub key: String,
    pub title: String,
    pub logo: String,
    pub order: u8,
    pub path_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MManageNetworks {
    pub networks: Vec<MMNetwork>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExportedSet {
    All,
    Selected { s: Vec<PathAndNetwork> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathAndNetwork {
    pub derivation: String,
    pub network_specs_key: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MSCContent {
    LoadTypes { types: String, pic: Identicon },
    LoadMetadata { name: String, version: u32 },
    AddSpecs { f: MSCNetworkInfo },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QrData {
    Regular { data: Vec<u8> },
    Sensitive { data: Vec<u8> },
}

impl QrData {
    /// Get the length of the underlying data
    pub fn len(&self) -> usize {
        match self {
            QrData::Regular { data } | QrData::Sensitive { data } => data.len(),
        }
    }

    /// Get a reference to the underlying data.
    pub fn data(&self) -> &[u8] {
        match self {
            QrData::Regular { data } | QrData::Sensitive { data } => data,
        }
    }

    /// If the underlying data is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            QrData::Regular { data } | QrData::Sensitive { data } => data.is_empty(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSufficientCryptoReady {
    pub author_info: MAddressCard,
    pub sufficient: Vec<u8>,
    pub content: MSCContent,
    pub network_logo: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DerivationEntry {
    pub path: String,
    pub has_pwd: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DerivationPack {
    pub network_title: String,
    pub network_logo: String,
    pub network_order: String,
    pub id_set: Vec<DerivationEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MBackup {
    pub seed_name: String,
    pub derivations: Vec<DerivationPack>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSeedMenu {
    pub seed: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MNewSeedBackup {
    pub seed: String,
    pub seed_phrase: String,
    pub identicon: Identicon,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Network {
    pub key: String,
    pub logo: String,
    pub order: u32,
    pub selected: bool,
    pub title: String,
}

impl From<OrderedNetworkSpecs> for Network {
    fn from(n: OrderedNetworkSpecs) -> Self {
        let key = hex::encode(
            NetworkSpecsKey::from_parts(&n.specs.genesis_hash, &n.specs.encryption).key(),
        );
        Network {
            key,
            logo: n.specs.logo,
            order: n.order as u32,
            selected: false,
            title: n.specs.title,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MNetworkMenu {
    pub networks: Vec<Network>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MPasswordConfirm {
    pub pwd: String,
    pub seed_name: String,
    pub cropped_path: String,
}

/// Data about signatures that are ready.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSignatureReady {
    /// Frames of the animated QR code that should be displayed by the UI.
    pub signatures: Vec<QrData>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MEnterPassword {
    pub author_info: MAddressCard,
    pub network_info: Option<MSCNetworkInfo>,
    pub counter: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MLogRight {
    pub checksum: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MMMNetwork {
    pub title: String,
    pub logo: String,
    pub order: u32,
    pub current_on_screen: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MManageMetadata {
    pub name: String,
    pub version: String,
    pub meta_hash: String,
    pub meta_id_pic: Identicon,
    pub networks: Vec<MMMNetwork>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MTypesInfo {
    pub types_on_file: bool,
    pub types_hash: Option<String>,
    pub types_id_pic: Option<Identicon>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModalData {
    SufficientCryptoReady { f: MSufficientCryptoReady },
    Backup { f: MBackup },
    SeedMenu { f: MSeedMenu },
    NewSeedBackup { f: MNewSeedBackup },
    NetworkSelector { f: MNetworkMenu },
    PasswordConfirm { f: MPasswordConfirm },
    SignatureReady { f: MSignatureReady },
    EnterPassword { f: MEnterPassword },
    LogRight { f: MLogRight },
    TypesInfo { f: MTypesInfo },
    NewSeedMenu,
    NetworkDetailsMenu,
    ManageMetadata { f: MManageMetadata },
    KeyDetailsAction,
    LogComment,
    SelectSeed { f: MSeeds },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSCCall {
    pub method_name: String,
    pub docs: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSCCurrency {
    pub amount: String,
    pub units: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSCEnumVariantName {
    pub name: String,
    pub docs_enum_variant: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSCEraMortal {
    pub era: String,
    pub phase: String,
    pub period: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSCFieldName {
    pub name: String,
    pub docs_field_name: String,
    pub path_type: String,
    pub docs_type: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSCFieldNumber {
    pub number: String,
    pub docs_field_number: String,
    pub path_type: String,
    pub docs_type: String,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Identicon {
    Dots { identity: Vec<u8> },
    Blockies { identity: String },
    Jdenticon { identity: String },
}

impl std::fmt::Debug for Identicon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Identicon::Dots { identity } => write!(f, "Dots {}", hex::encode(&identity[..32])),
            Identicon::Blockies { identity } => {
                write!(f, "Blockies {identity}")
            }
            Identicon::Jdenticon { identity } => {
                write!(f, "Jdenticon {identity}")
            }
        }
    }
}

impl Default for Identicon {
    fn default() -> Self {
        Self::Dots { identity: vec![] }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSCId {
    pub base58: String,
    pub identicon: Identicon,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSCNameVersion {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSCNetworkInfo {
    pub network_title: String,
    pub network_logo: String,
    pub network_specs_key: String,
}

/// Dynamic deprivations model
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DDPreview {
    pub qr: Vec<QrData>,
    pub key_set: DDKeySet,
    pub is_some_already_imported: bool,
    pub is_some_network_missing: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DDKeySet {
    pub seed_name: String,
    pub derivations: Vec<DDDetail>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DDDetail {
    pub base58: String,
    pub path: String,
    pub network_logo: String,
    pub network_specs_key: String,
    pub identicon: Identicon,
}

impl From<OrderedNetworkSpecs> for MSCNetworkInfo {
    fn from(o: OrderedNetworkSpecs) -> Self {
        MSCNetworkInfo {
            network_title: o.specs.name,
            network_logo: o.specs.logo,
            network_specs_key: hex::encode(
                NetworkSpecsKey::from_parts(&o.specs.genesis_hash, &o.specs.encryption).key(),
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSCTip {
    pub amount: String,
    pub units: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MSCTxSpecPlain {
    pub network_genesis_hash: H256,
    pub version: String,
    pub tx_version: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MKeysInfoExport {
    pub frames: Vec<QrData>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Card {
    AuthorCard { f: MAddressCard },
    AuthorPlainCard { f: MSCId },
    AuthorPublicKeyCard { f: MVerifierDetails },
    BalanceCard { f: MSCCurrency },
    BitVecCard { f: String },
    BlockHashCard { f: String },
    CallCard { f: MSCCall },
    DefaultCard { f: String },
    DerivationsCard { f: Vec<SeedKeysPreview> },
    EnumVariantNameCard { f: MSCEnumVariantName },
    EraImmortalCard,
    EraMortalCard { f: MSCEraMortal },
    ErrorCard { f: String },
    FieldNameCard { f: MSCFieldName },
    FieldNumberCard { f: MSCFieldNumber },
    IdCard { f: MSCId },
    IdentityFieldCard { f: String },
    MetaCard { f: MMetadataRecord },
    NameVersionCard { f: MSCNameVersion },
    NetworkGenesisHashCard { f: String },
    NetworkNameCard { f: String },
    NetworkInfoCard { f: MSCNetworkInfo },
    NewSpecsCard { f: NetworkSpecs },
    NonceCard { f: String },
    NoneCard,
    PalletCard { f: String },
    TextCard { f: String },
    TipCard { f: MSCCurrency },
    TipPlainCard { f: String },
    TxSpecCard { f: String },
    TxSpecPlainCard { f: MSCTxSpecPlain },
    TypesInfoCard { f: MTypesInfo },
    VarNameCard { f: String },
    VerifierCard { f: MVerifierDetails },
    WarningCard { f: String },
}
