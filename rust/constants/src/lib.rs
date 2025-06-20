//! Constants used throughout in [Vault](https://github.com/paritytech/parity-signer)
//! and Vault-supporting ecosystem.
//!
//! Vault uses **cold** database.
//!
//! The database used on a non air-gapper device for updates generation is
//! called **hot** database.
//!
//! Cold database is generated during the build, so it gets addressed both on
//! active and on signer side.
//!
//! Cold database contains following trees:
//!
//! - [`SPECSTREE`], with network specs in cold database format
//! - [`VERIFIERS`], with network verifiers, i.e. data on who user trusts to
//!   sign updates for a given network
//! - [`METATREE`], with network metadata
//! - [`ADDRTREE`], with user addresses public information
//! - [`SETTREE`], containing general verifier, types information, and Vault
//!   danger status
//! - [`TRANSACTION`], used to store temporarily transaction data while the
//!   user accepts or declines it
//! - [`HISTORY`], with history log
//!
//! Hot database contains following trees:
//!
//! - [`SPECSTREEPREP`], with network specs in hot database format
//! - [`METATREE`], with network metadata, at most two versions for each network
//! - [`META_HISTORY`], with block hash at which the metadata was fetched
//! - [`ADDRESS_BOOK`], with network information needed to make RPC calls
//! - [`SETTREE`], containing types information
//!
//! Common constants are:
//!
//! - tree names in cold database alone or shared between cold and hot databases
//! - key names in [`SPECSTREE`] tree of cold database alone or shared between
//!   cold and hot databases
//! - recurring throughout the Vault seed phrase for Alice
//! - QR graphic settings, used for both static and animated PNG QR codes
//!
//! # Features
//! Feature `"signer"` corresponds to everything related exclusively to Vault
//! air-gapped device. It includes:
//!
//! - keys for [`TRANSACTION`] tree, used for temporary storage for various
//!   kinds of transactions while they are shown to user for approval
//! - display settings for history log and word guesser
//!
//! Feature `"active"` corresponds to all Vault-related things happening
//! exclusively **without** air-gap. It includes:
//!
//! - default database addresses for hot database and cold release database
//! - hot database specific tree names
//! - default color settings for network specs
//! - default files and folders name fragments, for use with `generate_message`
//! - fountain qr generation parameters (large apng fountain qr codes are
//!   generated only on the hot side)
//!
//! Feature `"test"` includes all `"signer"` and `"active"` contents, and some
//! recurring data for integration testing, such as known identicons and known
//! export QR codes.

#![deny(rustdoc::broken_intra_doc_links)]

pub mod test_values;

/// Default folder for cold database generated during the Vault build
pub const COLD_DB_NAME_RELEASE: &str = "../database/database_cold_release";

/// Tree name for the tree storing the network specs in cold database
pub const SPECSTREE: &[u8] = b"chainspecs";

/// Tree name for the tree storing the network verifiers information in cold
/// database
pub const VERIFIERS: &[u8] = b"verifiers";

/// Tree name for the tree storing the network metadata in cold and in hot
/// databases
pub const METATREE: &[u8] = b"metadata";

/// Tree name for the tree storing user addresses associated public
/// information in cold database
pub const ADDRTREE: &[u8] = b"addresses";

/// Tree name for the tree storing database settings
///
/// In cold database, the settings tree contains general verifier, types
/// information, Vault danger status.
///
/// In hot database, the settings tree contains types information.
pub const SETTREE: &[u8] = b"settings";

/// Tree name for the tree temporarily storing transaction entries
pub const TRANSACTION: &[u8] = b"transaction";

/// Tree name for the tree storing Vault history
pub const HISTORY: &[u8] = b"history";

/// Key in settings tree [`SETTREE`] for encoded types information
pub const TYPES: &[u8] = b"types";

/// Key in settings tree [`SETTREE`] for general verifier information
pub const GENERALVERIFIER: &[u8] = b"general_verifier";

/// Key in settings tree [`SETTREE`] for Vault danger status
pub const DANGER: &[u8] = b"dangerous_encounter";

/// Key in settings tree [`SETTREE`] for Vault database schema version
pub const SCHEMA_VERSION: &[u8] = b"schema_version";

/// Key in transactions tree [`TRANSACTION`] for updates data
pub const STUB: &[u8] = b"stub";

/// Key in transactions tree [`TRANSACTION`] for signable transactions
pub const SIGN: &[u8] = b"sign";

/// Key in transactions tree [`TRANSACTION`] for derivations import data
pub const DRV: &[u8] = b"derivations";

/// Address prefix to display general/chain agnostic addresses
pub const GENERAL_SUBSTRATE_PREFIX: u16 = 42;

/// Maximum number of words displayed to user based on user input in seed
/// recovery process
pub const MAX_WORDS_DISPLAY: usize = 8;

/// Number of entries on log history page
pub const HISTORY_PAGE_SIZE: usize = 20;

/// Default folder for hot database
pub const HOT_DB_NAME: &str = "../database/database_hot";

/// Tree name for the tree storing the network specs in hot database
pub const SPECSTREEPREP: &[u8] = b"chainspecs_prep";

/// Tree name for the tree storing the network block hash at the time of
/// metadata fetch, in hot database
pub const META_HISTORY: &[u8] = b"metadata_history";

/// Tree name for the tree storing the address book in hot database, with data
/// necessary for RPC calls
pub const ADDRESS_BOOK: &[u8] = b"address_book";

/// Default `color` to be used in generating network specs with no color
/// information provided
pub const COLOR: &str = "#660D35";

/// Default `secondary_color` to be used in generating network specs with no
/// color information provided
pub const SECONDARY_COLOR: &str = "#262626";

/// Folder to save payloads ready for signing, for `generate_message` crate
pub const FOLDER: &str = "../files/in_progress";

/// Folder to save completed update messages, for `generate_message` crate
pub const EXPORT_FOLDER: &str = "../files/completed";

/// Alice seed phrase
pub const ALICE_SEED_PHRASE: &str =
    "bottom drive obey lake curtain smoke basket hold race lonely fit walk";

/// Data chunk size for fountain QR code generation
pub const CHUNK_SIZE: u16 = 1072;

/// Main color for QR codes (both static and animated ones)
pub const MAIN_COLOR: [u8; 3] = [0x00, 0x00, 0x00];

/// Main color for **dangerous** QR codes (static only, in Vault)
pub const MAIN_COLOR_DANGER: [u8; 3] = [0xe6, 0x00, 0x7a];

/// Background color for QR codes (both static and animated ones)
pub const BACK_COLOR: [u8; 3] = [0xff, 0xff, 0xff];

/// Color palette for QR codes (both static and animated ones)
pub fn qr_palette() -> Vec<u8> {
    [MAIN_COLOR.to_vec(), BACK_COLOR.to_vec()].concat()
}

/// Color palette for **dangerous** QR codes (static only, in Vault)
pub fn qr_palette_danger() -> Vec<u8> {
    [MAIN_COLOR_DANGER.to_vec(), BACK_COLOR.to_vec()].concat()
}

/// Scaling factor for QR codes (size of QR code dot, in pixels)
pub const SCALING: i32 = 4;

/// Numerator of the fraction of time (in seconds) for which the frame in the
/// animated QR code is displayed
pub const FPS_NOM: u16 = 1;

/// Denominator of the fraction of time (in seconds) for which the frame in the
/// animated QR code is displayed
pub const FPS_DEN: u16 = 15;

/// Width of the QR code border, in QR code dots
pub const BORDER: i32 = 4;

/// Current database schema version
pub const LIVE_SCHEMA_VERSION: u32 = 1;

/// Feature flag to disable dynamic derivations
pub const ENABLE_DYNAMIC_DERIVATIONS: bool = true;
