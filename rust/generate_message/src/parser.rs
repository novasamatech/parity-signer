//! Command line parser for the client
use constants::{COLD_DB_NAME_RELEASE, EXPORT_FOLDER, FOLDER, HOT_DB_NAME};
use definitions::{
    crypto::{Encryption, SufficientCrypto},
    helpers::unhex,
};
use sp_core::{ecdsa, ed25519, sr25519};
use std::{convert::TryInto, path::PathBuf};

use crate::{error::Result, Error};
use parity_scale_codec::Decode;

use clap::{Args, Parser, Subcommand};

/// Parity Signer data manipulation tool.
#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Display content of the a given tree of the hot database
    Show {
        #[clap(subcommand)]
        s: Show,

        /// Path to the hot database
        #[clap(long= "hot-db-path", global = true, value_name = "HOT_DB_PATH", default_value = HOT_DB_NAME)]
        db_path: PathBuf,
    },

    /// Prepare payload for add-specs update
    ///
    /// This will remove:
    /// - address book entry
    /// [`AddressBookEntry`](definitions::metadata::AddressBookEntry) from
    /// [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) tree
    /// - network specs
    /// [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend)
    /// from [`SPECSTREEPREP`](constants::SPECSTREEPREP) tree
    /// - all associated metadata entries from [`METATREE`](constants::METATREE)
    /// if there are no other address book entries this metadata is associated
    /// with
    /// - all associated meta block history entries from
    /// [`META_HISTORY`](constants::META_HISTORY) if there are no other address book
    /// entries this block history entries are associated with
    #[clap(name = "add-specs")]
    Specs {
        #[clap(flatten)]
        s: InstructionSpecs,
    },

    /// Prepare payload for load-metadata update
    #[clap(name = "load-metadata")]
    Load(InstructionMeta),

    /// Prepare payload for load-types update
    #[clap(name = "load-types")]
    Types {
        /// Path to hot db
        #[clap(long= "hot-db-path", value_name = "HOT_DB_PATH", default_value = HOT_DB_NAME)]
        db_path: PathBuf,

        /// Folder to save payloads ready for signing
        #[clap(long, value_name = "FOLDER_PATH", default_value = FOLDER)]
        files_dir: PathBuf,
    },

    /// Complete update generation according
    Make(Make),

    /// Sign
    Sign(Make),

    /// Remove all data associated with a network
    ///
    /// This will remove:
    /// - address book entry
    /// [`AddressBookEntry`](definitions::metadata::AddressBookEntry) from
    /// [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) tree
    /// - network specs
    /// [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend)
    /// from [`SPECSTREEPREP`](constants::SPECSTREEPREP) tree
    /// - all associated metadata entries from [`METATREE`](constants::METATREE)
    /// if there are no other address book entries this metadata is associated
    /// with
    /// - all associated meta block history entries from
    /// [`META_HISTORY`](constants::META_HISTORY) if there are no other address book
    /// entries this block history entries are associated with
    Remove {
        #[clap(subcommand)]
        r: Remove,

        /// Path to the hot database
        #[clap(long="hot-db-path", global=true, value_name="HOT_DB_PATH", default_value = HOT_DB_NAME)]
        db_path: PathBuf,
    },

    /// Restore hot database to default state
    ///
    /// Removes old hot database and generates new one with default values at
    /// default path [`HOT_DB_NAME`](constants::HOT_DB_NAME).
    ///
    /// By default, hot database contains:
    ///
    /// - [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) entries for default networks
    /// - [`SPECSTREEPREP`](constants::SPECSTREEPREP) entries for default networks
    /// - types information in [`SETTREE`](constants::SETTREE)
    /// - **no** metadata entries in [`METATREE`](constants::METATREE)
    /// - **no** meta block history entries in
    /// [`META_HISTORY`](constants::META_HISTORY)
    ///
    /// Default networks are Polkadot, Kusama, and Westend.
    RestoreDefaults {
        /// Path to hot db
        #[clap(long = "hot-db-path", value_name = "HOT_DB_PATH", default_value = HOT_DB_NAME)]
        db_path: PathBuf,
    },

    /// Generate release cold database at optionally provided path
    ///
    /// Removes old cold release database and generates new one with default values
    /// (uninitiated) at user-provided path or, if no valid path is given, at
    /// default path [`COLD_DB_NAME_RELEASE`](constants::COLD_DB_NAME_RELEASE).
    ///
    /// By default, the uninitiated cold release database contains:
    ///
    /// - [`SPECSTREE`](constants::SPECSTREE) entries for default networks
    /// - [`VERIFIERS`](constants::VERIFIERS) entries for default networks, with
    /// verifiers set to the general one
    /// - two latest metadata versions for default networks in
    /// [`METATREE`](constants::METATREE)
    /// - default types information and clean danger status in
    /// [`SETTREE`](constants::SETTREE)
    ///
    /// Note that the general verifier is not specified and history is not
    /// started. This will be done only in Signer itself. Before initialization,
    /// the cold release database could not be used by Signer.
    MakeColdRelease {
        /// Path to release db
        path: Option<PathBuf>,
    },

    /// Transfer metadata from hot database to release cold database
    ///
    /// Metadata from hot database is transferred to cold release database at
    /// user-provided path or, if no valid path is given, at default path
    /// [`COLD_DB_NAME_RELEASE`](constants::COLD_DB_NAME_RELEASE).
    ///
    /// Metadata is transferred only for the networks that are known to the cold
    /// database, i.e. the ones having
    /// [`NetworkSpecs`](definitions::network_specs::NetworkSpecs) entry in
    /// [`SPECSTREE`](constants::SPECSTREE).
    #[clap(name = "transfer-meta")]
    TransferMetaToColdRelease {
        /// Path to release db
        #[clap(long, value_name = "COLD_DB_PATH", default_value = COLD_DB_NAME_RELEASE)]
        cold_db: PathBuf,

        /// Path to hot db
        #[clap(long, value_name = "HOT_DB_PATH", default_value = HOT_DB_NAME)]
        hot_db: PathBuf,
    },

    /// Make derivations import QR and/or hexadecimal string file
    ///
    /// Output file is in `/generate_message/` folder, file name would be
    /// `derivations-<address_book_title>`.
    Derivations(Derivations),

    /// Prepare payload for `load_metadata` update from `.wasm` file
    ///
    /// This command extracts metadata from `.wasm` file and uses this metadata to
    /// produce `load_metadata` update payload. Only networks with network specs
    /// entries in the hot database could be processed with `unwasm` command, since
    /// the `load_metadata` update payload in addition to metadata requires also
    /// network genesis hash. `unwasm` command could be used to generate update QR
    /// codes before the metadata becomes accessible from the node.
    ///
    /// Network name found in the metadata is used to find
    /// [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend) for
    /// the network. `NetworkSpecsToSend` are used to get genesis hash and to check
    /// base58 prefix, it the network metadata has base58 prefix inside.
    ///
    /// A raw bytes update payload file is generated in dedicated
    /// [`FOLDER`](constants::FOLDER) to (optionally) be signed and later be
    /// transformed into `load_metadata` update QR. Update payload file name is
    /// `sign_me_load_metadata_<network_name>V<version>`.
    ///
    /// By default, metadata extracted from `.wasm` file is added to the database.
    /// Optional `-d` key could be used is database should **not** be updated.
    /// If the metadata gets entered in the database (i.e. no `-d` key used),
    /// [`META_HISTORY`](constants::META_HISTORY) gets no entry. Block hash will be
    /// added if the same metadata is later fetched from a node.
    Unwasm {
        /// WASM file
        #[clap(long, short)]
        filename: String,

        /// update the DB.
        #[clap(long, short)]
        update_db: bool,

        /// Hot database path
        #[clap(long= "hot-db-path", value_name = "HOT_DB_PATH", default_value = HOT_DB_NAME)]
        db_path: PathBuf,

        /// Folder to save payloads ready for signing
        #[clap(long, default_value = FOLDER)]
        files_dir: PathBuf,
    },

    /// Make file with hexadecimal metadata for defaults release metadata set
    ///
    /// Produces file with hex-encoded network metadata from the hot database
    /// [`METATREE`](constants::METATREE) entry.
    ///
    /// Output file named `<network_name><metadata_version>` is generated in
    /// dedicated [`EXPORT_FOLDER`](constants::EXPORT_FOLDER). It contains
    /// hexadecimal network metadata.
    MetaDefaultFile {
        /// File name
        #[clap(long, value_name = "NETWORK NAME")]
        name: String,

        /// Version
        #[clap(long, value_name = "NETWORK VERSION")]
        version: u32,

        /// Hot database path
        #[clap(long= "hot-db-path", value_name = "HOT_DB_PATH", default_value = HOT_DB_NAME)]
        db_path: PathBuf,

        /// Folder to save completed update messages
        #[clap(long, default_value = EXPORT_FOLDER)]
        export_dir: PathBuf,
    },

    /// Create file with network metadata at block hash
    ///
    /// Output file named `<network_name><metadata_version>_<block_hash>` is
    /// generated in dedicated folder.
    /// It contains hexadecimal network metadata.
    /// This command does not address or update the hot database.
    MetaAtBlock {
        /// URL of the chain RPC point
        #[clap(long, value_name = "RPC URL")]
        url: String,

        /// Hash of the block at which meta is asked
        #[clap(long, value_name = "BLOCK HASH")]
        block_hash: String,

        /// Folder to save completed update messages
        #[clap(long, default_value = EXPORT_FOLDER)]
        export_dir: PathBuf,
    },
}

/// Display data commands.
#[derive(Clone, Debug, Subcommand)]
pub enum Show {
    /// Show all hot database entries
    Metadata,

    /// Show all hot database entries
    Networks,

    /// Show network specs from entry.
    Specs {
        #[clap(value_name = "ADDRESS BOOK TITLE")]
        /// Address book title
        s: String,
    },

    /// Check that external file is valid network metadata and search for
    /// similar entry in hot database
    CheckFile {
        #[clap(value_name = "METADATA FILE")]
        /// Path to metadata file
        s: String,
    },

    /// Show all entries from `META_HISTORY` tree
    BlockHistory,
}

/// Command details for `load_metadata`.
#[derive(clap::Args, Debug)]
pub struct InstructionMeta {
    /// Setting key, as read from command line
    #[clap(flatten)]
    pub set: SetFlags,

    /// Reference key, as read from command line
    #[clap(flatten)]
    pub content: ContentArgs,

    /// Path to the hot database
    #[clap(long= "hot-db-path", value_name = "HOT_DB_PATH", default_value = HOT_DB_NAME)]
    pub db: PathBuf,

    /// Folder to save payloads ready for signing
    #[clap(long, default_value = FOLDER)]
    pub files_dir: PathBuf,
}

impl From<SetFlags> for Set {
    fn from(set: SetFlags) -> Self {
        match (set.d, set.f, set.k, set.p, set.t) {
            (true, false, false, false, false) => Set::D,
            (false, true, false, false, false) => Set::F,
            (false, false, true, false, false) => Set::K,
            (false, false, false, true, false) => Set::P,
            (false, false, false, false, true) => Set::T,
            _ => panic!("mutually exclusive args"),
        }
    }
}

/// Command details for `add_specs`.
#[derive(clap::Args, Debug)]
#[clap(group(clap::ArgGroup::new("referencekey")
                .required(true)
                .args(&["all", "name", "address"])
))]
pub struct InstructionSpecs {
    #[clap(flatten)]
    pub set: SetFlags,

    /// Overrides, relevant only for `add_specs` command
    #[clap(flatten)]
    pub over: Override,

    #[clap(flatten)]
    pub content: ContentArgs,

    /// Path to the hot database
    #[clap(long = "hot-db-path", value_name = "HOT_DB_PATH", default_value = HOT_DB_NAME)]
    pub db: PathBuf,

    /// Folder to save payloads ready for signing
    #[clap(long, default_value = FOLDER)]
    pub files_dir: PathBuf,
}

#[derive(clap::Args, Debug, Default, Clone)]
pub struct ContentArgs {
    /// Deal with all relevant database entries
    #[clap(long, short)]
    pub all: bool,

    /// Process only a specified network
    #[clap(long, short)]
    pub name: Option<String>,

    /// Process only the network referred to by URL address
    #[clap(short = 'u', long = "url")]
    pub address: Option<String>,

    /// Skip errors
    #[clap(long)]
    pub pass_errors: bool,
}

impl From<ContentArgs> for Content {
    fn from(args: ContentArgs) -> Self {
        match (args.all, &args.name, &args.address) {
            (true, None, None) => Content::All {
                pass_errors: args.pass_errors,
            },
            (false, Some(name), None) => Content::Name { s: name.clone() },
            (false, None, Some(address)) => Content::Address { s: address.clone() },
            _ => panic!("mutually exclusive flags"),
        }
    }
}

/// Reference key for `load_metadata` and `add_specs` commands.
#[derive(Subcommand, Debug)]
pub enum Content {
    /// Deal with all relevant database entries
    All {
        #[clap(short)]
        /// Skip errors
        pass_errors: bool,
    },

    /// Process only a specified network
    Name {
        /// network name or network address book title
        #[clap(long, short)]
        s: String,
    },

    /// Process only the network referred to by URL address
    Address {
        /// Network address
        #[clap(long, short)]
        s: String,
    },
}

/// Setting key for `load_metadata` and `add_specs` commands.
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Set {
    /// Key `-d`: do **not** update the database, make RPC calls, and produce
    /// output files
    #[clap(name = "-d")]
    D,

    /// Key `-f`: do **not** run RPC calls, produce output files from database
    /// as it is
    #[clap(name = "-f")]
    F,

    /// Key `-k`: update database through RPC calls, produce output files only
    /// for **updated** database entries
    #[clap(name = "-k")]
    K,

    /// Key `-p`: update database through RPC calls, do **not** produce any
    /// output files
    #[clap(name = "-p")]
    P,

    /// Key `-t` (no setting key defaults here): update database through RPC
    /// calls, produce output files
    #[clap(name = "-t")]
    T,
}

#[derive(clap::Args, Default, Clone, Debug)]
#[clap(group(clap::ArgGroup::new("setflags")
                .required(true)
                .args(&["d", "f", "k", "p", "t"])
))]
pub struct SetFlags {
    /// do not update the database, make RPC calls, and produce output files
    #[clap(short = 'd')]
    pub d: bool,

    /// do not run RPC calls, produce output files from database as it is
    #[clap(short = 'f')]
    pub f: bool,

    /// update database through RPC calls, produce output files only
    /// for updated database entries
    #[clap(short = 'k')]
    pub k: bool,

    /// update database through RPC calls, do **not** produce any output files
    #[clap(short = 'p')]
    pub p: bool,

    /// (no setting key defaults here): update database through RPC
    /// calls, produce output files
    #[clap(short = 't')]
    pub t: bool,
}

impl std::fmt::Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Data to process `make` and `sign` commands.
#[derive(clap::Args, Debug)]
pub struct Make {
    /// payload
    #[clap(long, name = "msg", value_parser)]
    pub msg: Msg,

    #[clap(long, name = "payload")]
    pub payload: PathBuf,

    /// target output format
    #[clap(long, name = "goal", value_parser, default_value_t = Goal::Both)]
    pub goal: Goal,

    #[clap(flatten)]
    pub verifier: Verifier,

    #[clap(flatten)]
    pub signature: Signature,

    #[clap(flatten)]
    pub sufficient: Sufficient,

    /// who is signing the payload
    #[clap(long, name = "crypto", value_parser = encryption_from_args)]
    pub crypto: Option<Encryption>,

    /// output name override
    #[clap(long, name = "name")]
    pub name: Option<PathBuf>,

    /// Folder to save payloads ready for signing
    #[clap(long, default_value = FOLDER)]
    pub files_dir: PathBuf,

    /// Folder to save completed update messages
    #[clap(long, default_value = EXPORT_FOLDER)]
    pub export_dir: PathBuf,
}

impl Make {
    pub fn payload(&self) -> Result<Vec<u8>> {
        Ok(std::fs::read(&self.files_dir.join(&self.payload))?)
    }

    pub fn crypto(&self) -> Result<Crypto> {
        if let Some(s) = match (
            &self.sufficient.sufficient_hex,
            &self.sufficient.sufficient_file,
        ) {
            (Some(hex), None) => Some(unhex(hex)?),
            (None, Some(path)) => {
                let sufficient_filename = &self.files_dir.join(path);
                Some(std::fs::read(sufficient_filename)?)
            }
            _ => None,
        } {
            let s = <SufficientCrypto>::decode(&mut &s[..])?;
            return Ok(Crypto::Sufficient { s });
        }
        let verifier_public_key = match (
            &self.verifier.verifier_alice,
            &self.verifier.verifier_hex,
            &self.verifier.verifier_file,
        ) {
            (Some(e), None, None) => return Ok(Crypto::Alice { e: e.clone() }),
            (None, Some(hex), None) => unhex(hex)?,
            (None, None, Some(path)) => {
                let verifier_filename = &self.files_dir.join(path);
                std::fs::read(verifier_filename)?
            }
            f => {
                if self.signature.signature_file.is_none() && self.signature.signature_hex.is_none()
                {
                    return Ok(Crypto::None);
                } else {
                    panic!("mutually exclusive flags: {:?}", f);
                }
            }
        };

        let signature = match (
            &self.signature.signature_hex,
            &self.signature.signature_file,
        ) {
            (Some(hex), None) => unhex(hex)?,
            (None, Some(path)) => {
                let signature_filename = &self.files_dir.join(path);
                std::fs::read(signature_filename)?
            }
            f => panic!("mutually exclusive flags: {:?}", f),
        };

        Ok(Crypto::Sufficient {
            s: into_sufficient(verifier_public_key, signature, self.crypto.clone().unwrap())?,
        })
    }
}

/// Target output format for `derivations`, `make` and `sign` commands.
#[derive(clap::ValueEnum, Debug, Clone)]
pub enum Goal {
    /// Only QR code
    Qr,

    /// Only text file with hexadecimal string (used for tests)
    Text,

    /// Both QR code and text file, default
    Both,
}

impl std::fmt::Display for Goal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Goal::Qr => "qr",
            Goal::Text => "text",
            Goal::Both => "both",
        };
        write!(f, "{}", s)
    }
}

/// Verifier-to-be, for `make` and `sign` commands.
#[derive(clap::Args, Debug, Clone)]
#[clap(group(clap::ArgGroup::new("verifier")
                .args(&["alice", "HEX", "FILE"])
        ))]
pub struct Verifier {
    /// Use Alice key with a specified encryption scheme
    #[clap(long, name = "alice", value_parser = encryption_from_args)]
    verifier_alice: Option<Encryption>,

    /// Specify Verifier as a hex string argument
    #[clap(long, name = "HEX")]
    verifier_hex: Option<String>,

    /// Read Verifier from a file
    #[clap(long, name = "FILE")]
    verifier_file: Option<PathBuf>,
}

/// Verifier-to-be, for `make` and `sign` commands.
pub enum Crypto {
    /// Alice key
    Alice {
        /// Encryption scheme to use
        e: Encryption,
    },

    /// No verifier, to make unsigned updates.
    None,

    /// Real verifier, [`SufficientCrypto`] is either assembled from `make`
    /// command input parts or from `sign` command input directly.
    Sufficient { s: SufficientCrypto },
}

#[derive(clap::Args, Debug, Clone)]
#[clap(group(clap::ArgGroup::new("signature")
                .args(&["signature-hex", "signature-file"])
        ))]
pub struct Signature {
    /// Supply signature in hex format as command line argument
    #[clap(long, value_name = "HEX")]
    signature_hex: Option<String>,

    /// Read signature from a file
    #[clap(long, value_name = "FILE")]
    signature_file: Option<String>,
}

#[derive(clap::Args, Debug, Clone)]
#[clap(group(clap::ArgGroup::new("sufficient")
                .args(&["sufficient-hex", "sufficient-file"])
        ))]
pub struct Sufficient {
    /// Supply signature in hex format as command line argument
    #[clap(long, value_name = "HEX")]
    sufficient_hex: Option<String>,

    /// Read signature from a file
    #[clap(long, value_name = "FILE")]
    sufficient_file: Option<String>,
}
/// Payload for `make` and `sign` commands.
///
/// Associated data is `Vec<u8>` blob that becomes part of the update.
///
/// Payload content details are described in [`definitions::qr_transfers`].
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Msg {
    /// `load_types` payload
    LoadTypes,

    /// `load_metadata` payload
    LoadMetadata,

    /// `add_specs` payload
    AddSpecs,
}

/// Data to process `remove` command.
#[derive(clap::Subcommand, Clone, Debug)]
pub enum Remove {
    /// Removing all network data by network address book title.
    ///
    /// Associated data is user-entered network address book title.
    Title { t: String },

    /// Remove specified network metadata entry.
    ///
    /// Associated data is network name and version.
    SpecNameVersion { name: String, version: u32 },
}

/// Data to process `derivations` command.
#[derive(clap::Args, Clone, Debug)]
pub struct Derivations {
    /// Target output format
    #[clap(long, value_parser)]
    pub goal: Goal,

    /// Address book title for network in which addresses with imported
    /// derivations will be made in Signer
    #[clap(long)]
    pub title: String,

    /// Contents of the payload file
    #[clap(long)]
    pub derivations: String,

    /// Path to the hot database
    #[clap(long= "hot-db-path", value_name = "HOT_DB_PATH", default_value = HOT_DB_NAME)]
    pub db: PathBuf,
}

/// Overrides for `add_specs` command.
#[derive(Args, Debug)]
pub struct Override {
    /// [`Encryption`] override to specify encryption algorithm used by a new
    /// network or to add another encryption algorithm in known network.
    #[clap(long, value_parser = encryption_from_args)]
    pub encryption: Option<Encryption>,

    /// Network title override, so that user can specify the network title in
    /// [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend)
    /// that determines under what title the network is displayed in the Signer
    #[clap(long)]
    pub title: Option<String>,

    /// Token override to specify decimals used to display balance in
    /// network transactions.
    ///
    /// Token override could be invoked only if:
    ///
    /// - network has no database record yet
    /// - network has multiple decimals and unit values, those were retrieved as
    /// arrays of equal size.
    #[clap(long)]
    pub token_decimals: Option<u8>,

    /// Token override to specify units used to display balance in
    /// network transactions.
    ///
    /// Token override could be invoked only if:
    ///
    /// - network has no database record yet
    /// - network has multiple decimals and unit values, those were retrieved as
    /// arrays of equal size.
    #[clap(long)]
    pub token_unit: Option<String>,
}

impl Override {
    pub fn token(&self) -> Option<Token> {
        match (self.token_decimals, self.token_unit.as_ref()) {
            (Some(d), Some(u)) => Some(Token {
                decimals: d,
                unit: u.clone(),
            }),
            _ => None,
        }
    }
}

fn encryption_from_args(s: &str) -> std::result::Result<Encryption, &'static str> {
    match s {
        "ed25519" => Ok(Encryption::Ed25519),
        "sr25519" => Ok(Encryption::Sr25519),
        "ecdsa" => Ok(Encryption::Ecdsa),
        _ => Err("unexpected encryption type, expected `ed25519`, `sr25519` or `ecdsa`"),
    }
}

impl Override {
    /// Flag to indicate that no overrides were invoked.
    pub fn all_empty(&self) -> bool {
        self.encryption.is_none() && self.title.is_none() && self.token().is_none()
    }
}

/// Data from command line for token override.
#[derive(Debug)]
pub struct Token {
    /// Decimals of the token
    pub decimals: u8,

    /// Units of the token
    pub unit: String,
}

fn vec_to_pubkey_array<const N: usize>(v: Vec<u8>) -> Result<[u8; N]> {
    v.try_into()
        .map_err(|e: Vec<_>| Error::PublicKeyWrongLength(N, e.len()))
}

fn vec_to_signature_array<const N: usize>(v: Vec<u8>) -> Result<[u8; N]> {
    v.try_into()
        .map_err(|e: Vec<_>| Error::SignatureWrongLength(N, e.len()))
}

/// Fit public key and signature drafts into [`SufficientCrypto`].
fn into_sufficient(
    verifier_public_key: Vec<u8>,
    signature: Vec<u8>,
    encryption: Encryption,
) -> Result<SufficientCrypto> {
    match encryption {
        Encryption::Ed25519 => {
            let into_pubkey = vec_to_pubkey_array(verifier_public_key)?;
            let public = ed25519::Public::from_raw(into_pubkey);
            let into_sign = vec_to_signature_array(signature)?;
            let signature = ed25519::Signature::from_raw(into_sign);
            Ok(SufficientCrypto::Ed25519 { public, signature })
        }
        Encryption::Sr25519 => {
            let into_pubkey = vec_to_pubkey_array(verifier_public_key)?;
            let public = sr25519::Public::from_raw(into_pubkey);
            let into_sign = vec_to_signature_array(signature)?;
            let signature = sr25519::Signature::from_raw(into_sign);
            Ok(SufficientCrypto::Sr25519 { public, signature })
        }
        Encryption::Ecdsa => {
            let into_pubkey = vec_to_pubkey_array(verifier_public_key)?;
            let public = ecdsa::Public::from_raw(into_pubkey);
            let into_sign = vec_to_signature_array(signature)?;
            let signature = ecdsa::Signature::from_raw(into_sign);
            Ok(SufficientCrypto::Ecdsa { public, signature })
        }
    }
}
