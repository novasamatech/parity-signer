//! Constants used throughout in [Signer](https://github.com/paritytech/parity-signer)
//! and Signer-supporting ecosystem.  
//!
//! Signer uses **cold** database.
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
//! sign updates for a given network
//! - [`METATREE`], with network metadata
//! - [`ADDRTREE`], with user addresses public information
//! - [`SETTREE`], containing general verifier, types information, and Signer
//! danger status
//! - [`TRANSACTION`], used to store temporarily transaction data while the
//! user accepts or declines it
//! - [`HISTORY`], with history log
//!
//! Hot database contains following trees:
//!
//! - [`SPECSTREEPREP`], with network specs in hot database format
//! - [`METATREE`], with network metadata, at most two versions for each network
//! - [`META_HISTORY`], with block hash at which the metadata was fetched
//! - [`ADDRESS_BOOK`], with network information needed to make rpc calls
//! - [`SETTREE`], containing types information
//!
//! Common constants are:
//!
//! - tree names in cold database alone or shared between cold and hot databases
//! - key names in [`SPECSTREE`] tree of cold database alone or shared between
//! cold and hot databases
//! - recurring throughout the Signer seed phrase for Alice
//! - QR graphic settings, used for both static and animated png QR codes
//!
//! # Features
//! Feature `"signer"` corresponds to everything related exclusively to Signer
//! air-gapped device. It includes:
//!
//! - keys for [`TRANSACTION`] tree, used for temporary storage for various
//! kinds of transactions while they are shown to user for approval
//! - display settings for history log and word guesser
//!
//! Feature `"active"` corresponds to all Signer-related things happening
//! exclusively **without** air-gap. It includes:
//!
//! - default database addresses for hot database and cold release database
//! - hot database specific tree names
//! - default color settings for network specs
//! - default files and folders name fragments, for use with `generate_message`
//! - fountain qr generation parameters (large apng fountain qr codes are
//! generated only on the hot side)
//!
//! Feature `"test"` includes all `"signer"` and `"active"` contents, and some
//! recurring data for integration testing, such as known identicons and known
//! export QR codes.

#[cfg(feature = "test")]
pub mod test_values;

/// Default folder for cold database generated during the Signer build
#[cfg(feature = "active")]
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
/// information, Signer danger status.
///
/// In hot database, the settings tree contains types information.
pub const SETTREE: &[u8] = b"settings";

/// Tree name for the tree temporarily storing transaction entries
pub const TRANSACTION: &[u8] = b"transaction";

/// Tree name for the tree storing Signer history
pub const HISTORY: &[u8] = b"history";

/// Key in settings tree [`SETTREE`] for encoded types information
pub const TYPES: &[u8] = b"types";

/// Key in settings tree [`SETTREE`] for general verifier information
pub const GENERALVERIFIER: &[u8] = b"general_verifier";

/// Key in settings tree [`SETTREE`] for Signer danger status
pub const DANGER: &[u8] = b"dangerous_encounter";

/// Key in transactions tree [`TRANSACTION`] for updates data
#[cfg(feature = "signer")]
pub const STUB: &[u8] = b"stub";

/// Key in transactions tree [`TRANSACTION`] for signable transactions
#[cfg(feature = "signer")]
pub const SIGN: &[u8] = b"sign";

/// Key in transactions tree [`TRANSACTION`] for derivations import data
#[cfg(feature = "signer")]
pub const DRV: &[u8] = b"derivations";

/// Maximum number of words displayed to user based on user input in seed
/// recovery process
#[cfg(feature = "signer")]
pub const MAX_WORDS_DISPLAY: usize = 8;

/// Number of entries on log history page
#[cfg(feature = "signer")]
pub const HISTORY_PAGE_SIZE: usize = 20;

/// Default folder for hot database
#[cfg(feature = "active")]
pub const HOT_DB_NAME: &str = "../database/database_hot";

/// Tree name for the tree storing the network specs in hot database
#[cfg(feature = "active")]
pub const SPECSTREEPREP: &[u8] = b"chainspecs_prep";

/// Tree name for the tree storing the network block hash at the time of
/// metadata fetch, in hot database
#[cfg(feature = "active")]
pub const META_HISTORY: &[u8] = b"metadata_history";

/// Tree name for the tree storing the address book in hot database, with data
/// necessary for rpc calls
#[cfg(feature = "active")]
pub const ADDRESS_BOOK: &[u8] = b"address_book";

/// Default `color` to be used in generating network specs with no color
/// information provided
#[cfg(feature = "active")]
pub const COLOR: &str = "#660D35";

/// Default `secondary_color` to be used in generating network specs with no
/// color information provided
#[cfg(feature = "active")]
pub const SECONDARY_COLOR: &str = "#262626";

/// Common part of the file names for `load_metadata` payloads ready for signing,
/// for `generate_message` crate
#[cfg(feature = "active")]
pub fn load_metadata() -> String {
    format!("{}/sign_me_load_metadata", FOLDER)
}

/// File name for `load_types` payload ready for signing, for `generate_message`
/// crate
#[cfg(feature = "active")]
pub fn load_types() -> String {
    format!("{}/sign_me_load_types", FOLDER)
}

/// Common part of the file names for `add_specs` payloads ready for signing,
/// for `generate_message` crate
#[cfg(feature = "active")]
pub fn add_specs() -> String {
    format!("{}/sign_me_add_specs", FOLDER)
}

/// Folder to save payloads ready for signing, for `generate_message` crate
#[cfg(feature = "active")]
pub const FOLDER: &str = "../files/in_progress";

/// Folder to save completed update messages, for `generate_message` crate
#[cfg(feature = "active")]
pub const EXPORT_FOLDER: &str = "../files/completed";

/// Alice seed phrase
pub const ALICE_SEED_PHRASE: &str =
    "bottom drive obey lake curtain smoke basket hold race lonely fit walk";

/// Data chunk size for fountain QR code generation
#[cfg(feature = "active")]
pub const CHUNK_SIZE: u16 = 1072;

/// Main color for QR codes (both static and animated ones)
pub const MAIN_COLOR: [u8; 3] = [0x00, 0x00, 0x00];

/// Background color for QR codes (both static and animated ones)
pub const BACK_COLOR: [u8; 3] = [0xff, 0xff, 0xff];

/// Color palette for QR codes (both static and animated ones)
pub fn qr_palette() -> Vec<u8> {
    [MAIN_COLOR.to_vec(), BACK_COLOR.to_vec()].concat()
}

/// Scaling factor for QR codes (size of QR code dot, in pixels)
pub const SCALING: i32 = 4;

/// Numerator of the fraction of time (in seconds) for which the frame in the
/// animated QR code is displayed
pub const FPS_NOM: u16 = 1;

/// Denominator of the fraction of time (in seconds) for which the frame in the
/// animated QR code is displayed
pub const FPS_DEN: u16 = 30;

/// Width of the QR code border, in QR code dots
pub const BORDER: i32 = 4;
