//! Command line parser for the client
use constants::FOLDER;
use definitions::{
    crypto::{Encryption, SufficientCrypto},
    helpers::unhex,
};
use std::path::PathBuf;

use crate::error::Result;

use clap::{Args, Parser, Subcommand};

/// Parity Signer data manipulation tool.
#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(subcommand)]
    Show(Show),

    /// Prepare payload for add-specs update
    Specs {
        #[clap(flatten)]
        s: InstructionSpecs,
    },

    /// Prepare payload for load-metadata update
    Load(InstructionMeta),

    /// Prepare payload for load-types update
    Types,

    /// Complete update generation according
    Make(Make),

    /// Sign
    Sign(Make),

    /// Remove data from the hot database
    #[clap(subcommand)]
    Remove(Remove),

    /// Restore hot database to default state
    RestoreDefaults,

    /// Generate release cold database at optionally provided path
    MakeColdRelease {
        #[clap(short, long)]
        /// Path to release db
        path: Option<PathBuf>,
    },

    /// Transfer metadata from hot database to release cold database
    TransferMetaRelease {
        #[clap(short, long)]
        /// Path to release db
        path: Option<PathBuf>,
    },

    /// Make derivations import
    Derivations(Derivations),

    /// Prepare payload for `load_metadata` update from `.wasm` file
    Unwasm {
        /// WASM file
        #[clap(long, short)]
        filename: String,

        /// update the DB.
        #[clap(long, short)]
        update_db: bool,
    },

    /// Make file with hexadecimal metadata for `defaults` set
    MetaDefaultFile {
        /// File name
        name: String,
        /// Version
        version: u32,
    },

    /// Make file with hexadecimal metadata from specific block
    MetaAtBlock {
        /// Url of the chain RPC point
        url: String,
        /// Hash of the block at which meta is asked
        block_hash: String,
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
        #[clap(short, long, value_name = "specs")]
        /// Address book title
        s: String,
    },

    /// Check that external file is valid network metadata and search for
    /// similar entry in hot database
    CheckFile {
        #[clap(short, long, value_name = "metadata file")]
        /// Path to metadata file
        s: String,
    },

    /// Show all entries from META_HISTORY tree
    BlockHistory,
}

/// Command details for `load_metadata`.
#[derive(clap::Args, Debug)]
pub struct InstructionMeta {
    /// Setting key, as read from command line
    #[clap(value_parser)]
    pub set: Set,

    /// Reference key, as read from command line
    #[clap(subcommand)]
    pub content: Content,
}

/// Command details for `add_specs`.
#[derive(clap::Args, Debug)]
pub struct InstructionSpecs {
    /// Reference key, as read from command line
    #[clap(subcommand)]
    pub content: Content,

    /// Setting key, as read from command line
    #[clap(value_parser)]
    pub set: Set,

    /// Overrides, relevant only for `add_specs` command
    #[clap(flatten)]
    pub over: Override,
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

    /// Process only the network referred to by url address
    Address {
        /// Network address
        #[clap(long, short)]
        s: String,
    },
}

/// Setting key for `load_metadata` and `add_specs` commands.
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Set {
    /// Key `-d`: do **not** update the database, make rpc calls, and produce
    /// output files
    D,

    /// Key `-f`: do **not** run rpc calls, produce output files from database
    /// as it is
    F,

    /// Key `-k`: update database through rpc calls, produce output files only
    /// for **updated** database entries
    K,

    /// Key `-p`: update database through rpc calls, do **not** produce any
    /// output files
    P,

    /// Key `-t` (no setting key defaults here): update database through rpc
    /// calls, produce output files
    T,
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

    /// who is signing the payload
    #[clap(long, name = "crypto", value_parser = encryption_from_args)]
    pub crypto: Option<Encryption>,

    /// output name override
    #[clap(long, name = "name")]
    pub name: Option<PathBuf>,
}

impl Make {
    pub fn payload(&self) -> Result<Vec<u8>> {
        Ok(std::fs::read(&format!(
            "{}/{}",
            FOLDER,
            self.payload.to_string_lossy()
        ))?)
    }

    pub fn crypto(&self) -> Result<Crypto> {
        use parity_scale_codec::Decode;

        match (
            &self.verifier.verifier_alice,
            &self.verifier.verifier_hex,
            &self.verifier.verifier_file,
        ) {
            (Some(e), None, None) => Ok(Crypto::Alice { e: e.clone() }),
            (None, Some(hex), None) => {
                let verifier_public_key = unhex(hex)?;
                let s = <SufficientCrypto>::decode(&mut &verifier_public_key[..])?;
                Ok(Crypto::Sufficient { s })
            }
            (None, None, Some(path)) => {
                let verifier_filename = format!("{}/{}", FOLDER, path.to_string_lossy());
                println!("verifier_filename {}", verifier_filename);
                let verifier_public_key = std::fs::read(&verifier_filename)?;
                let s = <SufficientCrypto>::decode(&mut &verifier_public_key[..])?;

                Ok(Crypto::Sufficient { s })
            }
            f => panic!("mutually exclusive flags: {:?}", f),
        }
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
                .required(true)
                .args(&["alice", "hex", "file"])
        ))]
pub struct Verifier {
    /// Use Alice key with a specified encryption scheme
    #[clap(long, name = "alice", value_parser = encryption_from_args)]
    verifier_alice: Option<Encryption>,

    /// Specify Verifier as a hex string argument
    #[clap(long, name = "hex")]
    verifier_hex: Option<String>,

    /// Read Verifier from a file
    #[clap(long, name = "file")]
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
                .required(true)
                .args(&["hex", "file"])
        ))]
pub struct Signature {
    /// Supply signature in hex format as command line argument
    #[clap(long, name = "hex")]
    signature_hex: Option<String>,

    /// Read signature from a file
    #[clap(long, name = "file")]
    signature_file: Option<String>,
}

/// Payload for `make` and `sign` commands.
///
/// Associated data is `Vec<u8>` blob that becomes part of the update.
///
/// Payload content details are described in [definitions::qr_transfers].
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
