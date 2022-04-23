use crate::{
    crypto::Encryption, history::Event, keyring::NetworkSpecsKey, network_specs::NetworkSpecs,
};

pub use crate::network_specs::NetworkSpecsToSend;

#[derive(PartialEq, Clone)]
pub struct SeedNameWithIdenticon {
    pub seed_name: String,
    pub identicon: String,
}

/// Enum containing card sets for three different outcomes:
/// signing (Sign), accepting (Stub) and reading, for example, in case of an error (Read)
#[derive(PartialEq, Clone)]
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
        author_info: TransactionAuthor,
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
    pub modal_data: ModalData,
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
    Scan,
    Keys { f: MKeys },
    Settings { f: MSettings },
    Log { f: MLog },
    LogDetails { f: MLogDetails },
    Transaction { f: MTransaction },
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
    pub identicon: String,
    pub has_pwd: bool,
    pub path: String,
    pub is_multiselect: bool,
    pub base58: String,
}

#[derive(Clone, PartialEq)]
pub struct MKeysCard {
    pub address_key: String,
    pub base58: String,
    pub identicon: String,
    pub has_pwd: bool,
    pub path: String,
    pub swiped: bool,
    pub multiselect: bool,
}

#[derive(Clone, Default, PartialEq)]
pub struct MNetworkCard {
    pub title: String,
    pub logo: String,
}

// TODO: This has to have a custom default.
#[derive(Clone, Default, PartialEq)]
pub struct MSeedKeyCard {
    pub seed_name: String,
    pub identicon: String,
    pub address_key: String,
    pub base58: String,
    pub swiped: bool,
    pub multiselect: bool,
}

#[derive(Clone, PartialEq)]
pub struct MKeys {
    pub set: Vec<MKeysCard>,
    pub root: MSeedKeyCard,
    pub network: MNetworkCard,
    pub multiselect_mode: bool,
    pub multiselect_count: String,
}

#[derive(Clone, PartialEq)]
pub struct MSettings {
    pub public_key: Option<String>,
    pub identicon: Option<String>,
    pub encryption: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct History {
    pub order: u32,
    pub timestamp: String,
    pub events: Vec<Event>,
}

#[derive(Clone, PartialEq)]
pub struct MLog {
    pub log: Vec<History>,
    pub total_entries: u32,
}

#[derive(Clone, PartialEq)]
pub struct MLogDetails {
    pub timestamp: String,
    pub events: Vec<Event>,
}

#[derive(Clone, PartialEq)]
pub enum TransactionType {
    Sign,
    Stub,
    Read,
    ImportDerivations,
    Done,
}

#[derive(Clone, PartialEq)]
pub struct TransactionNetworkInfo {
    pub network_title: String,
    pub network_logo: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransactionAuthor {
    pub base58: String,
    pub identicon: String,
    pub seed: String,
    pub derivation_path: String,
}

#[derive(Clone, PartialEq)]
pub struct TransactionCard {
    pub index: u32,
    pub indent: u32,
    pub card: Card,
}

#[derive(Clone, Default, PartialEq)]
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

#[derive(Clone, PartialEq)]
pub struct MTransaction {
    pub content: TransactionCardSet,
    pub ttype: TransactionType,
    pub author_info: Option<TransactionAuthor>,
    pub network_info: Option<TransactionNetworkInfo>,
}

#[derive(Clone, PartialEq)]
pub struct SeedNameCard {
    pub seed_name: String,
    pub identicon: String,
}

#[derive(Clone, PartialEq)]
pub struct MSeeds {
    pub seed_name_cards: Vec<SeedNameCard>,
}

#[derive(Clone, PartialEq)]
pub struct MKeyDetails {
    pub qr: String,
    pub pubkey: String,
    pub base58: String,
    pub identicon: String,
    pub seed_name: String,
    pub path: String,
    pub network_title: String,
    pub network_logo: String,
}

#[derive(Clone, PartialEq)]
pub struct MNewSeed {
    pub keyboard: bool,
}

#[derive(Clone, PartialEq)]
pub struct MRecoverSeedName {
    pub keyboard: bool,
    pub seed_name: String,
}

#[derive(Clone, PartialEq)]
pub struct MRecoverSeedPhrase {
    pub keyboard: bool,
    pub seed_name: String,
    pub user_input: String,
    pub guess_set: Vec<String>,
    pub draft: Vec<SeedWord>,
    pub ready_seed: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct SeedWord {
    pub order: u32,
    pub content: String,
}

#[derive(Clone, PartialEq)]
pub struct DerivationCheck {
    pub button_good: Option<bool>,
    pub where_to: Option<DerivationDestination>,
    pub collision: Option<Address>,
    pub error: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct Address {
    pub base58: String,
    pub path: String,
    pub has_pwd: bool,
    pub identicon: String,
    pub seed_name: String,
    pub multiselect: Option<bool>,
}

#[derive(Clone, PartialEq)]
pub enum DerivationDestination {
    Pwd,
    Pin,
}

#[derive(Clone, PartialEq)]
pub struct MDeriveKey {
    pub seed_name: String,
    pub network_title: String,
    pub network_logo: String,
    pub network_specs_key: String,
    pub suggested_derivation: String,
    pub keyboard: bool,
    pub derivation_check: Option<DerivationCheck>,
}

#[derive(Clone, Default, PartialEq)]
pub struct MVerifierDetails {
    pub public_key: String,
    pub identicon: String,
    pub encryption: String,
}

#[derive(Clone, Default, PartialEq)]
pub struct MVerifier {
    pub ttype: String,
    pub details: MVerifierDetails,
}

#[derive(Clone, PartialEq)]
pub struct MMetadataRecord {
    pub specs_version: String,
    pub meta_hash: String,
    pub meta_id_pic: String,
}

#[derive(Clone, PartialEq)]
pub struct MNetworkDetails {
    pub base58prefix: u16,
    pub color: String,
    pub decimals: u8,
    pub encryption: Encryption,
    pub genesis_hash: String,
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

#[derive(Clone, PartialEq)]
pub struct MRawKey {
    pub seed_name: String,
    pub address_key: String,
    pub public_key: String,
    pub identicon: String,
    pub has_pwd: bool,
    pub path: String,
}

#[derive(Clone, PartialEq)]
pub struct MSignSufficientCrypto {
    pub identities: Vec<MRawKey>,
}

#[derive(Clone, PartialEq)]
pub struct MKeyDetailsMulti {
    pub key_details: MKeyDetails,
    pub current_number: String,
    pub out_of: String,
}

#[derive(Clone, PartialEq)]
pub struct MMNetwork {
    pub key: String,
    pub title: String,
    pub logo: String,
    pub order: u8,
}

#[derive(Clone, PartialEq)]
pub struct MManageNetworks {
    pub networks: Vec<MMNetwork>,
}

#[derive(Clone, PartialEq)]
pub struct MSCContent {
    pub ttype: String,
}

#[derive(Clone, PartialEq)]
pub struct MSCAuthor {
    pub base58: String,
    pub identicon: String,
    pub seed: String,
    pub derivation_path: String,
    pub has_password: Option<bool>,
}

#[derive(Clone, PartialEq)]
pub struct MSufficientCryptoReady {
    pub author_info: MSCAuthor,
    pub sufficient: String,
    pub content: MSCContent,
}

#[derive(Clone, PartialEq)]
pub struct DerivationEntry {
    pub path: String,
    pub has_pwd: bool,
}

#[derive(Clone, PartialEq)]
pub struct DerivationPack {
    pub network_title: String,
    pub network_logo: String,
    pub network_order: String,
    pub id_set: Vec<DerivationEntry>,
}

#[derive(Clone, PartialEq)]
pub struct MBackup {
    pub seed_name: String,
    pub derivations: Vec<DerivationPack>,
}

#[derive(Clone, PartialEq)]
pub struct MSeedMenu {
    pub seed: String,
}

#[derive(Clone, PartialEq)]
pub struct MNewSeedBackup {
    pub seed: String,
    pub seed_phrase: String,
    pub identicon: String,
}

#[derive(Clone, PartialEq)]
pub struct Network {
    pub key: String,
    pub logo: String,
    pub order: u32,
    pub selected: bool,
    pub title: String,
}

impl From<NetworkSpecs> for Network {
    fn from(n: NetworkSpecs) -> Self {
        let key = hex::encode(
            NetworkSpecsKey::from_parts(n.genesis_hash.as_bytes(), &n.encryption).key(),
        );
        Network {
            key,
            logo: n.logo,
            order: n.order as u32,
            selected: false,
            title: n.title,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct MNetworkMenu {
    pub networks: Vec<Network>,
}

#[derive(Clone, PartialEq)]
pub struct MPasswordConfirm {
    pub pwd: String,
    pub seed_name: String,
    pub cropped_path: String,
}

#[derive(Clone, PartialEq)]
pub struct MSignatureReady {
    pub signature: String,
}

#[derive(Clone, PartialEq)]
pub struct MEnterPassword {
    pub author_info: TransactionAuthor,
    pub counter: u32,
}

#[derive(Clone, PartialEq)]
pub struct MLogRight {
    pub checksum: String,
}

#[derive(Clone, PartialEq)]
pub struct MMMNetwork {
    pub title: String,
    pub logo: String,
    pub order: u32,
    pub current_on_screen: bool,
}

#[derive(Clone, PartialEq)]
pub struct MMManageNetworks {
    pub name: String,
    pub version: String,
    pub meta_hash: String,
    pub meta_id_pic: String,
    pub networks: Vec<MMMNetwork>,
}

#[derive(Clone, PartialEq)]
pub struct MTypesInfo {
    pub types_on_file: bool,
    pub types_hash: Option<String>,
    pub types_id_pic: Option<String>,
}

#[derive(Clone, PartialEq)]
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
    ManageNetworks { f: MMManageNetworks },
    TypesInfo { f: MTypesInfo },
    Text { f: String },
    NewSeedMenu,
    NetworkDetailsMenu,
    ManageMetadata,
    KeyDetailsAction,
    LogComment,
    SelectSeed,
}

#[derive(Clone, PartialEq)]
pub struct MSCAuthorPlain {
    pub base58: String,
    pub identicon: String,
}

#[derive(Clone, PartialEq)]
pub struct MSCCall {
    pub method_name: String,
    pub docs: String,
}

#[derive(Clone, PartialEq)]
pub struct MSCCurrency {
    pub amount: String,
    pub units: String,
}

#[derive(Clone, PartialEq)]
pub struct MSCEnumVariantName {
    pub name: String,
    pub docs_enum_variant: String,
}

#[derive(Clone, PartialEq)]
pub struct MSCEraMortal {
    pub era: String,
    pub phase: String,
    pub period: String,
}

#[derive(Clone, PartialEq)]
pub struct MSCFieldName {
    pub name: String,
    pub docs_field_name: String,
    pub path_type: String,
    pub docs_type: String,
}

#[derive(Clone, PartialEq)]
pub struct MSCFieldNumber {
    pub number: String,
    pub docs_field_number: String,
    pub path_type: String,
    pub docs_type: String,
}

#[derive(Clone, PartialEq)]
pub struct MSCId {
    pub base58: String,
    pub identicon: String,
}

#[derive(Clone, PartialEq)]
pub struct MSCMetaSpecs {
    pub specname: String,
    pub spec_version: String,
    pub meta_hash: String,
    pub meta_id_pic: String,
}

#[derive(Clone, PartialEq)]
pub struct MSCNameVersion {
    pub name: String,
    pub version: String,
}

#[derive(Clone, PartialEq)]
pub struct MSCNetworkInfo {
    pub network_title: String,
    pub network_logo: String,
}

#[derive(Clone, PartialEq)]
pub struct MSCTip {
    pub amount: String,
    pub units: String,
}

#[derive(Clone, PartialEq)]
pub struct MSCTxSpecPlain {
    pub network_genesis_hash: String,
    pub version: String,
    pub tx_version: String,
}

#[derive(Clone, PartialEq)]
pub enum Card {
    AuthorCard { f: TransactionAuthor },
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
    MetaCard { f: MSCMetaSpecs },
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
