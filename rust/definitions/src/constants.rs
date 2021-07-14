/// COLD database on Signer device:  
///
/// Contains:
/// network metadata (whatever user wants to keep),
/// network specs as ChainSpecs (with order on device and with verifier for each network)
/// settings with types information and general verifier information
/// TEMPORARILY user identities and addresses - TO BE REMOVED SOON
/// transaction information

pub const COLD_DB_NAME: &str = "../database/database_cold";

/// Tree names:  
pub const SPECSTREE: &[u8] = b"chainspecs";
pub const METATREE: &[u8] = b"metadata";
pub const ADDRTREE: &[u8] = b"addresses";
pub const SETTREE: &[u8] = b"settings";
pub const TRANSACTION: &[u8] = b"transaction";

/// Key names used for settings tree:  
pub const TYPES: &[u8] = b"types";
pub const GENERALVERIFIER: &[u8] = b"general_verifier";

/// Key names used for transaction tree:  
pub const SIGNTRANS: &[u8] = b"sign_transaction";
pub const LOADMETA: &[u8] = b"load_metadata";
pub const ADDMETAVERIFIER: &[u8] = b"add_metadata_verifier";
pub const LOADTYPES: &[u8] = b"load_types";
pub const ADDGENERALVERIFIER: &[u8] = b"add_general_verifier";
pub const ADDNETWORK: &[u8] = b"add_network";


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
pub const ADDDEF: &str = "../files/for_signing/sign_me_add_network_with_defaults";
pub const LOAD: &str = "../files/for_signing/sign_me_load_metadata";
pub const TYLO: &str = "../files/for_signing/sign_me_load_types";

/// Folder name used for imports in generate_message crate
pub const FOLDER: &str = "../files/for_signing";
pub const EXPORT_FOLDER: &str = "../files/signed";
