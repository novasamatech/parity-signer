/// COLD database on Signer device:  
///
/// Contains:
/// network metadata (whatever user wants to keep),
/// network specs as ChainSpecs (with order on device and with verifier for each network)
/// settings with types information and general verifier information
/// TEMPORARILY user identities and addresses - TO BE REMOVED SOON
/// transaction information

pub const COLD_DB_NAME: &str = "../database/database_cold";
pub const COLD_DB_NAME_RELEASE: &str = "../database/database_cold_release";

/// Tree names:  
pub const SPECSTREE: &[u8] = b"chainspecs";
pub const VERIFIERS: &[u8] = b"verifiers";
pub const METATREE: &[u8] = b"metadata";
pub const ADDRTREE: &[u8] = b"addresses";
pub const SETTREE: &[u8] = b"settings";
pub const TRANSACTION: &[u8] = b"transaction";
pub const HISTORY: &[u8] = b"history";

/// Key names used for settings tree:  
pub const TYPES: &[u8] = b"types";
pub const GENERALVERIFIER: &[u8] = b"general_verifier";
pub const DANGER: &[u8] = b"dangerous_encounter";

/// Key names used for transaction tree:  
pub const STUB: &[u8] = b"stub";
pub const SIGN: &[u8] = b"sign";

/// Display constants
pub const MAX_WORDS_DISPLAY: usize = 8;
pub const HISTORY_PAGE_SIZE: usize = 20;

/// HOT database on external device:  
///
/// Contains:
/// network metadata (maximum two latest versions for each of networks),
/// network specs as ChainSpecsToSend
/// types information
/// address book

pub const HOT_DB_NAME: &str = "../database/database_hot";

/// Tree names:
pub const SPECSTREEPREP: &[u8] = b"chainspecs_prep";
pub const ADDRESS_BOOK: &[u8] = b"address_book";
/// Also hot database uses:
/// - tree METATREE with same (key, value) properties as in cold database
/// - tree SETTREE with key TYPES as in cold database


/// Other constants:

/// Default colors to be used when forming ChainSpecsToSend
/// for networks without known network specs
pub const COLOR: &str = "#660D35";
pub const SECONDARY_COLOR: &str = "#262626";

/// File name parts used for exports in generate_message crate
pub const ADD: &str = "../files/for_signing/sign_me_add_network";
pub const LOAD: &str = "../files/for_signing/sign_me_load_metadata";
pub const TYLO: &str = "../files/for_signing/sign_me_load_types";
pub const SPECS: &str = "../files/for_signing/sign_me_add_specs";

/// Folder name used for imports in generate_message crate
pub const FOLDER: &str = "../files/for_signing";
pub const EXPORT_FOLDER: &str = "../files/signed";

/// QR making, raptorq:
pub const CHUNK_SIZE: u16 = 1072;

/// QR making, both apng and png, grayscale:
pub const MAIN_COLOR: u8 = 0x00;
pub const BACK_COLOR: u8 = 0xFF;
pub const SCALING: i32 = 4;
pub const FPS_NOM: u16 = 1;
pub const FPS_DEN: u16 = 30;
pub const BORDER: i32 = 4;
