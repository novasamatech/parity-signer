use sp_core::H256;

use crate::{
    crypto::Encryption, history::Event, keyring::NetworkSpecsKey, network_specs::NetworkSpecs,
};

pub use crate::network_specs::NetworkSpecsToSend;

#[derive(PartialEq, Clone)]
pub struct SeedNameWithIdenticon {
    pub seed_name: String,
    pub identicon: Vec<u8>,
}

/// Enum containing card sets for four different outcomes:
/// importing derivations (Derivations), signing (Sign),
/// accepting (Stub) and reading, for example, in case of an error (Read)
#[derive(PartialEq, Debug, Clone)]
pub enum TransactionAction {
    Derivations {
        content: TransactionCardSet,
        network_info: NetworkSpecs,
        checksum: u32,
        network_specs_key: NetworkSpecsKey,
    },
    Sign {
        content: TransactionCardSet,
        checksum: u32,
        has_pwd: bool,
        author_info: Address,
        network_info: NetworkSpecs,
    },
    Stub {
        s: TransactionCardSet,
        u: u32,
        stub: StubNav,
    },
    Read {
        r: TransactionCardSet,
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

#[derive(PartialEq, Debug, Clone)]
pub enum FooterButton {
    Log,
    Scan,
    Keys,
    Settings,
    Back,
}

#[derive(PartialEq, Debug, Clone)]
pub enum RightButton {
    LogRight,
    NewSeed,
    Backup,
    MultiSelect,
    NDMenu,
    TypesInfo,
    KeyMenu,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ScreenNameType {
    H1,
    H4,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ShieldAlert {
    Past,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AlertData {
    Shield { f: Option<ShieldAlert> },
    ErrorData { f: String },
    Confirm,
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, PartialEq)]
pub struct LogScreenEntry {
    pub timestamp: String,
    pub events: Vec<Event>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScreenData {
    Scan,
    Keys { f: MKeys },
    Settings { f: MSettings },
    Log { f: MLog },
    LogDetails { f: MLogDetails },
    Transaction { f: MTransaction },
    SignatureReady { f: MSignatureReady },
    SeedSelector { f: MSeeds },
    KeyDetails { f: MKeyDetails },
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

#[derive(Clone, PartialEq)]
pub struct Identity {
    pub seed_name: String,
    pub address_key: String,
    pub public_key: String,
    pub identicon: Vec<u8>,
    pub has_pwd: bool,
    pub path: String,
    pub is_multiselect: bool,
    pub base58: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MKeysCard {
    pub address_key: String,
    pub base58: String,
    pub identicon: Vec<u8>,
    pub has_pwd: bool,
    pub path: String,
    pub swiped: bool,
    pub multiselect: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MNetworkCard {
    pub title: String,
    pub logo: String,
}

// TODO: This has to have a custom default.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MSeedKeyCard {
    pub seed_name: String,
    pub identicon: Vec<u8>,
    pub address_key: String,
    pub base58: String,
    pub swiped: bool,
    pub multiselect: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MKeys {
    pub set: Vec<MKeysCard>,
    pub root: MSeedKeyCard,
    pub network: MNetworkCard,
    pub multiselect_mode: bool,
    pub multiselect_count: String,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct MSettings {
    pub public_key: Option<String>,
    pub identicon: Option<Vec<u8>>,
    pub encryption: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct History {
    pub order: u32,
    pub timestamp: String,
    pub events: Vec<Event>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MLog {
    pub log: Vec<History>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MEventMaybeDecoded {
    pub event: Event,
    pub decoded: Option<TransactionCardSet>,
    pub signed_by: Option<Address>,
    pub verifier_details: Option<MVerifierDetails>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MLogDetails {
    pub timestamp: String,
    pub events: Vec<MEventMaybeDecoded>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TransactionType {
    Sign,
    Stub,
    Read,
    ImportDerivations,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransactionCard {
    pub index: u32,
    pub indent: u32,
    pub card: Card,
}

#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct MTransaction {
    pub content: TransactionCardSet,
    pub ttype: TransactionType,
    pub author_info: Option<Address>,
    pub network_info: Option<MSCNetworkInfo>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MSignatureReady {
    pub author_info: Address,
    pub content: TransactionCardSet,
    pub network_info: MSCNetworkInfo,
    pub signature: Vec<u8>,
    pub user_comment: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SeedNameCard {
    pub seed_name: String,
    pub identicon: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MSeeds {
    pub seed_name_cards: Vec<SeedNameCard>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MKeyDetails {
    pub qr: Vec<u8>,
    pub pubkey: String,
    pub network_info: MSCNetworkInfo,
    pub address: Address,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MNewSeed {
    pub keyboard: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MRecoverSeedName {
    pub keyboard: bool,
    pub seed_name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MRecoverSeedPhrase {
    pub keyboard: bool,
    pub seed_name: String,
    pub user_input: String,
    pub guess_set: Vec<String>,
    pub draft: Vec<String>,
    pub ready_seed: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct DerivationCheck {
    pub button_good: bool,
    pub where_to: Option<DerivationDestination>,
    pub collision: Option<Address>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Address {
    pub base58: String,
    pub path: String,
    pub has_pwd: bool,
    pub identicon: Vec<u8>,
    pub seed_name: String,
    pub multiselect: Option<bool>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DerivationDestination {
    Pwd,
    Pin,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MDeriveKey {
    pub seed_name: String,
    pub network_title: String,
    pub network_logo: String,
    pub network_specs_key: String,
    pub suggested_derivation: String,
    pub keyboard: bool,
    pub derivation_check: DerivationCheck,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MVerifierDetails {
    pub public_key: String,
    pub identicon: Vec<u8>,
    pub encryption: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MVerifier {
    pub ttype: String,
    pub details: MVerifierDetails,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MMetadataRecord {
    pub specname: String,
    pub specs_version: String,
    pub meta_hash: String,
    pub meta_id_pic: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct MRawKey {
    pub seed_name: String,
    pub address_key: String,
    pub public_key: String,
    pub identicon: Vec<u8>,
    pub has_pwd: bool,
    pub path: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MSignSufficientCrypto {
    pub identities: Vec<MRawKey>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MKeyDetailsMulti {
    pub key_details: MKeyDetails,
    pub current_number: String,
    pub out_of: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MMNetwork {
    pub key: String,
    pub title: String,
    pub logo: String,
    pub order: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MManageNetworks {
    pub networks: Vec<MMNetwork>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MSCContent {
    LoadTypes { types: String, pic: Vec<u8> },
    LoadMetadata { name: String, version: u32 },
    AddSpecs { f: MSCNetworkInfo },
}

#[derive(Clone, Debug, PartialEq)]
pub struct MSufficientCryptoReady {
    pub author_info: Address,
    pub sufficient: Vec<u8>,
    pub content: MSCContent,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DerivationEntry {
    pub path: String,
    pub has_pwd: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DerivationPack {
    pub network_title: String,
    pub network_logo: String,
    pub network_order: String,
    pub id_set: Vec<DerivationEntry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MBackup {
    pub seed_name: String,
    pub derivations: Vec<DerivationPack>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MSeedMenu {
    pub seed: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MNewSeedBackup {
    pub seed: String,
    pub seed_phrase: String,
    pub identicon: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Network {
    pub key: String,
    pub logo: String,
    pub order: u32,
    pub selected: bool,
    pub title: String,
}

impl From<NetworkSpecs> for Network {
    fn from(n: NetworkSpecs) -> Self {
        let key = hex::encode(NetworkSpecsKey::from_parts(&n.genesis_hash, &n.encryption).key());
        Network {
            key,
            logo: n.logo,
            order: n.order as u32,
            selected: false,
            title: n.title,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MNetworkMenu {
    pub networks: Vec<Network>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MPasswordConfirm {
    pub pwd: String,
    pub seed_name: String,
    pub cropped_path: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MEnterPassword {
    pub author_info: Address,
    pub counter: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MLogRight {
    pub checksum: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MMMNetwork {
    pub title: String,
    pub logo: String,
    pub order: u32,
    pub current_on_screen: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MManageMetadata {
    pub name: String,
    pub version: String,
    pub meta_hash: String,
    pub meta_id_pic: Vec<u8>,
    pub networks: Vec<MMMNetwork>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MTypesInfo {
    pub types_on_file: bool,
    pub types_hash: Option<String>,
    pub types_id_pic: Option<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModalData {
    SufficientCryptoReady { f: MSufficientCryptoReady },
    Backup { f: MBackup },
    SeedMenu { f: MSeedMenu },
    NewSeedBackup { f: MNewSeedBackup },
    NetworkSelector { f: MNetworkMenu },
    PasswordConfirm { f: MPasswordConfirm },
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

#[derive(Clone, Debug, PartialEq)]
pub struct MSCAuthorPlain {
    pub base58: String,
    pub identicon: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MSCCall {
    pub method_name: String,
    pub docs: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MSCCurrency {
    pub amount: String,
    pub units: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MSCEnumVariantName {
    pub name: String,
    pub docs_enum_variant: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MSCEraMortal {
    pub era: String,
    pub phase: String,
    pub period: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MSCFieldName {
    pub name: String,
    pub docs_field_name: String,
    pub path_type: String,
    pub docs_type: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MSCFieldNumber {
    pub number: String,
    pub docs_field_number: String,
    pub path_type: String,
    pub docs_type: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MSCId {
    pub base58: String,
    pub identicon: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MSCNameVersion {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MSCNetworkInfo {
    pub network_title: String,
    pub network_logo: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MSCTip {
    pub amount: String,
    pub units: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MSCTxSpecPlain {
    pub network_genesis_hash: H256,
    pub version: String,
    pub tx_version: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Card {
    AuthorCard { f: Address },
    AuthorPlainCard { f: MSCAuthorPlain },
    AuthorPublicKeyCard { f: MVerifierDetails },
    BalanceCard { f: MSCCurrency },
    BitVecCard { f: String },
    BlockHashCard { f: String },
    CallCard { f: MSCCall },
    DefaultCard { f: String },
    DerivationsCard { f: Vec<String> },
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
    NewSpecsCard { f: NetworkSpecsToSend },
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
