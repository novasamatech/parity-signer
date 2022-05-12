//! Command line parser for the client
//!
//!
//! Expected typical run commands:
//!
//! `$ cargo run show database`
//!
//! `$ cargo run show address_book`
//!
//! `$ cargo run load_metadata -n westend`
//!
//! `$ cargo run add_specs -d -n -ed25519 westend`
//!
//! `$ cargo run add_network -u wss://unknown-network.eu -ecdsa`
//!
//! `$ cargo run derivations -title westend -payload my_derivations_file`

use constants::FOLDER;
use definitions::{
    crypto::{Encryption, SufficientCrypto},
    error_active::{
        Active, CommandBadArgument, CommandDoubleKey, CommandNeedArgument, CommandNeedKey,
        CommandParser, CommandUnexpected, ErrorActive, InputActive, NotHexActive,
    },
    helpers::unhex,
};
use parity_scale_codec::Decode;
use sp_core::{ecdsa, ed25519, sr25519};
use std::convert::TryInto;
use std::{env, path::PathBuf};

/// Commands to execute
pub enum Command {
    /// Execute [`Show`] command.
    ///
    /// # Display content of the metadata [`METATREE`](constants::METATREE) tree of the hot database
    ///
    /// `$ cargo run show -database`
    ///
    /// Function prints for each entry in hot database
    /// [`METATREE`](constants::METATREE) tree:
    ///
    /// - network name
    /// - network version
    /// - hexadecimal metadata hash
    ///
    /// # Display content of the address book [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) tree of the hot database
    ///
    /// `$ cargo run show -address_book`
    ///
    /// Function prints for each entry in hot database
    /// [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) tree:
    ///
    /// - address book title for the network, used only to distinguish between
    /// address book entries
    /// - url address at which rpc calls are made for the network
    /// - network encryption
    /// - additional marker that the network is a default one
    ///
    /// # Check external file with hex-encoded metadata
    ///
    /// `$ cargo run check_file <path>`
    ///
    /// Function asserts that:
    ///
    /// - the file contains valid metadata, with retrievable network name and
    /// version
    /// - if the metadata for same network name and version is in the hot
    /// database, it completely matches the one from the file
    Show(Show),

    /// # Prepare payload for `load_types` update
    ///
    /// `$ cargo run load_types`
    ///
    /// A file is generated in dedicated [`FOLDER`](constants::FOLDER) to
    /// (optionally) be signed and later be transformed into `load_types` update
    /// QR.
    Types,

    /// # Prepare payload for `load_metadata` update according to `Instruction`
    ///
    /// `$ cargo run load_metadata <key(s)> <(argument)>`
    ///
    /// A file is generated in dedicated [`FOLDER`](constants::FOLDER) to
    /// (optionally) be signed and later be transformed into `load_metadata`
    /// update QR.
    ///
    /// Setting keys that could be used in command line (maximum one):
    ///
    /// - `-d`: do **not** update the database, make rpc calls, and produce
    /// output files
    /// - `-f`: do **not** run rpc calls, produce output files from database as
    /// it is
    /// - `-k`: update database through rpc calls, produce output files only for
    /// **updated** database entries
    /// - `-p`: update database through rpc calls, do **not** produce any output
    /// files
    /// - `-t` (no setting key defaults here): update database through rpc
    /// calls, produce output files
    ///
    /// Reference keys (exactly only one has to be used):  
    ///
    /// - `-a`: all networks with entries in the
    /// [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) tree of the hot database
    /// - `-n` followed by single network name: for a network with existing
    /// record in the [`ADDRESS_BOOK`](constants::ADDRESS_BOOK)
    /// - `-u` followed by single url address: reserved for networks with no
    /// record yet in the [`ADDRESS_BOOK`](constants::ADDRESS_BOOK)
    ///
    /// `-s` key could be used to stop processing after first error.
    Load(Instruction),

    /// # Prepare payload for `add_specs` update according to `Instruction`
    ///
    /// `$ cargo run add_specs <keys> <argument(s)>`
    ///
    /// A file is generated in dedicated [`FOLDER`](constants::FOLDER) to
    /// (optionally) be signed and later be transformed into `add_specs` update
    /// QR.
    ///
    /// Setting keys that could be used in command line (maximum one):
    ///
    /// - `-d`: do **not** update the database, make rpc calls, and produce
    /// output files
    /// - `-f`: do **not** run rpc calls, produce output files
    /// - `-p`: update database through rpc calls, do **not** produce any output
    /// files
    /// - `-t` (no setting key defaults here): update database through rpc
    /// calls, produce output files
    ///
    /// Reference keys (exactly only one has to be used):  
    ///
    /// - `-a`: all networks with entries in the
    /// [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) tree of the hot database
    /// - `-n` followed by single network address book title: for a network with
    /// existing record in the [`ADDRESS_BOOK`](constants::ADDRESS_BOOK)
    /// - `-u` followed by single url address: reserved for networks with no
    /// record yet in the [`ADDRESS_BOOK`](constants::ADDRESS_BOOK)
    ///
    /// `-s` key could be used to stop processing after first error.
    ///
    /// Key specifying encryption algorithm supported by the network is optional
    /// for `-n` reference key (since there is already an entry in the database
    /// with specified encryption) and mandatory for `-u` reference key.
    /// Supported variants are:
    ///
    /// - `-ed25519`
    /// - `-sr25519`
    /// - `-ecdsa`
    ///
    /// Sequence invoking token override could be used when processing
    /// individual network that (1) has no database record yet and (2) has
    /// multiple allowed decimals and unit values retrieved as arrays of equal
    /// size. To override token, key `-token` followed by `u8` decimals value
    /// and `String` unit value is used.
    Specs(Instruction),

    /// Complete update QR generation, either signed or unsigned
    ///
    /// If update is signed and accepted in Signer, the signature author will
    /// become a verifier in Signer, and some data afterwards could be accepted
    /// by Signer only if signed by the same verifier.
    ///
    /// Verifier keys must be kept safe.
    ///
    /// # Assemble QR update with external signature
    ///
    /// `$ cargo run make <key(s)> <argument(s)>`
    ///
    /// Keys to be used in command line:
    ///
    /// - Optional content key: `-qr` will generate only apng qr code, `-text`
    /// will generate only text file with hex-encoded update. By default, i.e.
    /// if content key is not provided, both qr code and text message are
    /// generated. Optional content key is expected immediately after `make`
    /// command, if at all; keys to follow could go in any order, but with
    /// argument immediately following the key.  
    ///
    /// - Key `-crypto` followed by encryption used to make update signature:
    ///    - `ed25519`  
    ///    - `sr25519`  
    ///    - `ecdsa`  
    ///    - `none` if the message is not verified  
    ///
    /// - Key `-msgtype` followed by message type:  
    ///    - `load_types`  
    ///    - `load_metadata`  
    ///    - `add_specs`
    ///
    /// - Key `-verifier` (can be entered if only the `-crypto` argument was
    /// `ed25519`, `sr25519`, or `ecdsa`), followed by:  
    ///    - `Alice` to generate messages "verified" by Alice (used for tests)  
    ///    - `-hex` followed by hex public key  
    ///    - `-file` followed by file path in dedicated
    /// [`FOLDER`](constants::FOLDER) with public key as raw bytes
    ///
    /// - Key `-payload` followed by file path in dedicated
    /// [`FOLDER`](constants::FOLDER) containing already generated payload as
    /// raw bytes
    ///
    /// - Key `-signature` (can be entered if only the `-crypto` argument was
    /// `ed25519`, `sr25519`, or `ecdsa` **and** `-verifier` is not `Alice`),
    /// followed by:  
    ///    - `-hex` followed by hex signature  
    ///    - `-file` followed by file path in dedicated
    /// [`FOLDER`](constants::FOLDER) with signature as raw bytes
    ///
    /// - Optional key `-name` followed by path override for export file in
    /// dedicated [`EXPORT_FOLDER`](constants::EXPORT_FOLDER)
    ///
    /// ### Example: generate `load_metadata` QR code for westend metadata version 9200.
    ///
    /// At this point the payload is already prepared with `load_metadata`
    /// command. File `sign_me_load_metadata_westendV9200` is in dedicated
    /// [`FOLDER`](constants::FOLDER).
    ///
    /// After `make` command is executed, QR code will appear in dedicated
    /// [`EXPORT_FOLDER`](constants::EXPORT_FOLDER).
    ///
    /// ### `make` for external signature
    ///
    /// Content of the payload file `sign_me_load_metadata_westendV9200` is
    /// signed using some external tool, for example, `subkey`. Hexadecimal
    /// `public key`, hexadecimal `signature`, and `encryption` will be needed
    /// to run command:
    ///
    /// `$ cargo run make -qr -crypto <encryption> -msgtype load_metadata
    /// -verifier -hex <public key> -payload sign_me_load_metadata_westendV9200
    /// -signature -hex <signature>`
    ///
    /// Output file name would be `load_metadata_westendV9200`.
    ///
    /// ### `make` for test verifier Alice
    ///
    /// Alice has a well-known [seed phrase](constants::ALICE_SEED_PHRASE).
    /// Payloads signed by Alice are used for testing in Signer. The signature
    /// in this case is generated automatically and is not supplied in command
    /// line.
    ///
    /// `$ cargo run make -qr -crypto <encryption> -msgtype load_metadata
    /// -verifier Alice -payload sign_me_load_metadata_westendV9200`.
    ///
    /// Output file name would be `load_metadata_westendV9200_Alice-<encryption>`.
    ///
    /// ### `make` with no signature
    ///
    /// `$ cargo run make -qr -crypto none -msgtype load_metadata -payload
    /// sign_me_load_metadata_westendV9200`
    ///
    /// Output file name would be `load_metadata_westendV9200_unverified`.
    ///
    /// Note that the validity of the signature, if applicable, and the payload
    /// content are checked before assembling QR update.
    ///
    /// # Assemble QR update using `SufficientCrypto` produced by Signer
    ///
    /// Updates could be signed in Signer itself, by generating
    /// [`SufficientCrypto`] for data that is already in the Signer, with one of
    /// the Signer keys. Signer exports `SufficientCrypto` as a static QR code,
    /// its content is fed into command line.
    ///
    /// `$ cargo run sign <key(s)> <argument(s)>`
    ///
    /// Keys to be used in command line:
    ///
    /// - Optional content key: `-qr` will generate only apng qr code, `-text`
    /// will generate only text file with hex-encoded update. By default, i.e.
    /// if content key is not provided, both qr code and text message are
    /// generated. Optional content key is expected immediately after `make`
    /// command, if at all; keys to follow could go in any order, but with
    /// argument immediately following the key.
    ///
    /// - Key `-sufficient` followed by:  
    ///    - `-hex` followed by hexadecimal string with SCALE-encoded
    /// [`SufficientCrypto`], i.e. Signer QR code output content
    ///    - `-file` followed by file path in dedicated
    /// [`FOLDER`](constants::FOLDER) with SCALE-encoded [`SufficientCrypto`] as
    /// raw bytes
    ///
    /// - Key `-msgtype` followed by message type:  
    ///    - `load_types`  
    ///    - `load_metadata`  
    ///    - `add_specs`
    ///
    /// - Key `-payload` followed by file path in dedicated
    /// [`FOLDER`](constants::FOLDER) containing already generated payload as
    /// raw bytes
    ///
    /// - Optional key `-name` followed by path override for export file in
    /// dedicated [`EXPORT_FOLDER`](constants::EXPORT_FOLDER)
    ///
    /// Note that the validity of the signature, if applicable, and the payload
    /// content are checked before assembling QR update.
    ///
    /// Generating `SufficientCrypto` in Signer is suggested mainly for update
    /// distribution purposes. A dedicated (i.e. used only for updates signing),
    /// kept physically safe Signer is strongly suggested, with a dedicated key
    /// for updates signing. As the Signer can accept only payloads with
    /// verifier not weaker than the one used before, and the whole purpose of
    /// the process is to generate a signature for payload, it is expected that
    /// this isolated Signer will receive unsigned or weakly signed updates,
    /// thoroughly check them and export `SufficientCrypto`, so that a signed
    /// update could be made for other, routinely used Signer devices.
    ///
    /// ### Example: generate `load_metadata` QR code for westend metadata version 9200.
    ///
    /// At this point the payload is already prepared with `load_metadata`
    /// command. File `sign_me_load_metadata_westendV9200` is in dedicated
    /// [`FOLDER`](constants::FOLDER). Hexadecimal `hex_sufficient` string is
    /// from [`SufficientCrypto`] QR code produced the Signer.
    ///
    /// After `make` command is executed, QR code will appear in dedicated
    /// [`EXPORT_FOLDER`](constants::EXPORT_FOLDER).
    ///
    /// `$ cargo run sign -qr -sufficient -hex <hex_sufficient> -msgtype
    /// load_metadata -payload sign_me_load_metadata_westendV9200`
    ///
    /// Output file name would be `load_metadata_westendV9200`.
    Make(Make),

    /// Remove data from the hot database
    ///
    /// # Remove a single metadata entry
    ///
    /// `$ cargo run remove -name <network name> -version <network version>`
    ///
    /// # Remove all data associated with a network
    ///
    /// `$ cargo run remove -title <network address book title>`
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
    Remove(Remove),

    /// # Restore hot database to default state
    ///
    /// `$ cargo run restore_defaults`
    ///
    /// By default, hot database contains
    ///
    /// - [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) and
    /// [`SPECSTREEPREP`](constants::SPECSTREEPREP) entries for default networks
    /// Polkadot, Kusama, and Westend
    /// - types information in [`SETTREE`](constants::SETTREE)
    /// - **no** metadata entries in [`METATREE`](constants::METATREE)
    RestoreDefaults,

    /// # Generate release cold database
    ///
    /// `$ cargo run make_cold_release`
    ///
    // TODO remove option from inside or add additional argument for the path,
    // now this became plain silly
    ///
    /// By default, cold release database contains:
    ///
    ///
    MakeColdRelease(Option<PathBuf>),
    TransferMetaRelease,
    Derivations(Derivations),
    Unwasm {
        filename: String,
        update_db: bool,
    },
    MetaDefaultFile {
        name: String,
        version: u32,
    },
}

pub enum Show {
    Database,
    AddressBook,
    CheckFile(String),
}

pub struct Instruction {
    pub set: Set,
    pub content: Content,
    pub pass_errors: bool,
    pub over: Override,
}

pub enum Content {
    All,
    Name(String),
    Address(String),
}

pub enum Set {
    D, // key `-d`: do NOT update the database, make rpc calls, and produce ALL possible output files
    F, // key `-f`: do NOT run rps calls, produce ALL possible output files from existing database
    K, // key `-k`: update database through rpc calls, produce output files only for UPDATED database entries
    P, // key `-p`: update database through rpc calls, do NOT produce any output files
    T, // key `-t`: default setting, update database through rpc calls, produce ALL possible output files
}

pub struct Make {
    pub goal: Goal,
    pub crypto: Crypto,
    pub msg: Msg,
    pub name: Option<String>,
}

pub enum Goal {
    Qr,
    Text,
    Both,
}

pub enum Crypto {
    Alice(Encryption),
    None,
    Sufficient(SufficientCrypto),
}

pub enum Msg {
    LoadTypes(Vec<u8>),
    LoadMetadata(Vec<u8>),
    AddSpecs(Vec<u8>),
}

enum CryptoType {
    Ed25519,
    Sr25519,
    Ecdsa,
    None,
}

enum MsgType {
    LoadTypes,
    LoadMetadata,
    AddSpecs,
}

enum VerKey {
    Hex(String),
    File(String),
    Alice,
}

enum Entry {
    Hex(String),
    File(String),
}

pub enum Remove {
    Title(String),
    SpecNameVersion { name: String, version: u32 },
}

pub struct Derivations {
    pub goal: Goal,
    pub title: String,
    pub derivations: String,
}

pub struct Override {
    pub encryption: Option<Encryption>,
    pub token: Option<TokenOverride>,
}

pub struct TokenOverride {
    pub decimals: u8,
    pub unit: String,
}

impl Command {
    /// FUnction to interpret command line input
    pub fn new(mut args: env::Args) -> Result<Command, ErrorActive> {
        args.next();

        match args.next() {
            Some(arg) => {
                let arg = arg.to_lowercase();
                match arg.as_str() {
                    "show" => match args.next() {
                        Some(show) => match show.to_lowercase().as_str() {
                            "-database" => {
                                if args.next().is_some() {
                                    Err(ErrorActive::CommandParser(
                                        CommandParser::UnexpectedKeyArgumentSequence,
                                    ))
                                } else {
                                    Ok(Command::Show(Show::Database))
                                }
                            }
                            "-address_book" => {
                                if args.next().is_some() {
                                    Err(ErrorActive::CommandParser(
                                        CommandParser::UnexpectedKeyArgumentSequence,
                                    ))
                                } else {
                                    Ok(Command::Show(Show::AddressBook))
                                }
                            }
                            _ => Err(ErrorActive::CommandParser(
                                CommandParser::UnexpectedKeyArgumentSequence,
                            )),
                        },
                        None => Err(ErrorActive::CommandParser(CommandParser::NeedKey(
                            CommandNeedKey::Show,
                        ))),
                    },
                    "check_file" => match args.next() {
                        Some(path) => {
                            if args.next().is_some() {
                                Err(ErrorActive::CommandParser(
                                    CommandParser::UnexpectedKeyArgumentSequence,
                                ))
                            } else {
                                Ok(Command::Show(Show::CheckFile(path)))
                            }
                        }
                        None => Err(ErrorActive::CommandParser(CommandParser::NeedArgument(
                            CommandNeedArgument::CheckFile,
                        ))),
                    },
                    "load_types" => {
                        if args.next().is_some() {
                            Err(ErrorActive::CommandParser(
                                CommandParser::UnexpectedKeyArgumentSequence,
                            ))
                        } else {
                            Ok(Command::Types)
                        }
                    }
                    "load_metadata" | "add_specs" => {
                        let mut set_key = None;
                        let mut content_key = None;
                        let mut pass_errors = true;
                        let mut name = None;
                        let mut encryption_override_key = None;
                        let mut token = None;
                        while let Some(x) = args.next() {
                            let x = x.to_lowercase();
                            if x.starts_with('-') {
                                match x.as_str() {
                                    "-a" | "-n" | "-u" => match content_key {
                                        Some(_) => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::DoubleKey(CommandDoubleKey::Content),
                                            ))
                                        }
                                        None => content_key = Some(x),
                                    },
                                    "-d" | "-f" | "-k" | "-p" | "-t" => match set_key {
                                        Some(_) => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::DoubleKey(CommandDoubleKey::Set),
                                            ))
                                        }
                                        None => set_key = Some(x),
                                    },
                                    "-s" => pass_errors = false,
                                    "-ed25519" | "-sr25519" | "-ecdsa" => {
                                        if arg == "load_metadata" {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::UnexpectedKeyArgumentSequence,
                                            ));
                                        }
                                        match encryption_override_key {
                                            Some(_) => {
                                                return Err(ErrorActive::CommandParser(
                                                    CommandParser::DoubleKey(
                                                        CommandDoubleKey::CryptoOverride,
                                                    ),
                                                ))
                                            }
                                            None => encryption_override_key = Some(x),
                                        }
                                    }
                                    "-token" => {
                                        if arg == "load_metadata" {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::UnexpectedKeyArgumentSequence,
                                            ));
                                        }
                                        match token {
                                            Some(_) => {
                                                return Err(ErrorActive::CommandParser(
                                                    CommandParser::DoubleKey(
                                                        CommandDoubleKey::TokenOverride,
                                                    ),
                                                ))
                                            }
                                            None => token = match args.next() {
                                                Some(b) => match b.parse::<u8>() {
                                                    Ok(decimals) => match args.next() {
                                                        Some(c) => Some(TokenOverride {
                                                            decimals,
                                                            unit: c.to_string(),
                                                        }),
                                                        None => {
                                                            return Err(ErrorActive::CommandParser(
                                                                CommandParser::NeedArgument(
                                                                    CommandNeedArgument::TokenUnit,
                                                                ),
                                                            ))
                                                        }
                                                    },
                                                    Err(_) => {
                                                        return Err(ErrorActive::CommandParser(
                                                            CommandParser::BadArgument(
                                                                CommandBadArgument::DecimalsFormat,
                                                            ),
                                                        ))
                                                    }
                                                },
                                                None => {
                                                    return Err(ErrorActive::CommandParser(
                                                        CommandParser::NeedArgument(
                                                            CommandNeedArgument::TokenDecimals,
                                                        ),
                                                    ))
                                                }
                                            },
                                        }
                                    }
                                    _ => {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::UnexpectedKeyArgumentSequence,
                                        ))
                                    }
                                }
                            } else {
                                match name {
                                    Some(_) => {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::OnlyOneNetworkId,
                                        ))
                                    }
                                    None => name = Some(x),
                                }
                            }
                        }

                        let set = match set_key {
                            Some(x) => match x.as_str() {
                                "-d" => Set::D,
                                "-f" => Set::F,
                                "-k" => Set::K,
                                "-p" => Set::P,
                                "-t" => Set::T,
                                _ => unreachable!(),
                            },
                            None => Set::T,
                        };

                        let encryption = match encryption_override_key {
                            Some(x) => match x.as_str() {
                                "-ed25519" => Some(Encryption::Ed25519),
                                "-sr25519" => Some(Encryption::Sr25519),
                                "-ecdsa" => Some(Encryption::Ecdsa),
                                _ => unreachable!(),
                            },
                            None => None,
                        };
                        let over = Override { encryption, token };

                        let content = match content_key {
                            Some(x) => match x.as_str() {
                                "-a" => {
                                    if name.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::Unexpected(
                                                CommandUnexpected::KeyAContent,
                                            ),
                                        ));
                                    }
                                    Content::All
                                }
                                "-n" => match name {
                                    Some(n) => Content::Name(n),
                                    None => {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::NeedArgument(
                                                CommandNeedArgument::NetworkName,
                                            ),
                                        ))
                                    }
                                },
                                "-u" => match name {
                                    Some(a) => Content::Address(a),
                                    None => {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::NeedArgument(
                                                CommandNeedArgument::NetworkUrl,
                                            ),
                                        ))
                                    }
                                },
                                _ => unreachable!(),
                            },
                            None => {
                                return Err(ErrorActive::CommandParser(CommandParser::NeedKey(
                                    CommandNeedKey::Content,
                                )))
                            }
                        };

                        let instruction = Instruction {
                            set,
                            content,
                            pass_errors,
                            over,
                        };

                        match arg.as_str() {
                            "load_metadata" => Ok(Command::Load(instruction)),
                            "add_specs" => Ok(Command::Specs(instruction)),
                            _ => unreachable!(),
                        }
                    }
                    "make" => {
                        let mut goal = Goal::Both; // default option for `make`
                        let mut args = args.peekable();
                        match args.peek() {
                            Some(x) => match x.to_lowercase().as_str() {
                                "-qr" => {
                                    goal = Goal::Qr;
                                    args.next();
                                }
                                "-text" => {
                                    goal = Goal::Text;
                                    args.next();
                                }
                                _ => (),
                            },
                            None => {
                                return Err(ErrorActive::CommandParser(
                                    CommandParser::NeedArgument(CommandNeedArgument::Make),
                                ))
                            }
                        }
                        let mut crypto_type_found = None;
                        let mut msg_type_found = None;
                        let mut verifier_found = None;
                        let mut payload_found = None;
                        let mut signature_found = None;
                        let mut name = None; // default option for `make`
                        while let Some(x) = args.next() {
                            let x = x.to_lowercase();
                            match x.as_str() {
                                "-crypto" => {
                                    if crypto_type_found.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(CommandDoubleKey::CryptoKey),
                                        ));
                                    }
                                    crypto_type_found = match args.next() {
                                        Some(x) => match x.to_lowercase().as_str() {
                                            "ed25519" => Some(CryptoType::Ed25519),
                                            "sr25519" => Some(CryptoType::Sr25519),
                                            "ecdsa" => Some(CryptoType::Ecdsa),
                                            "none" => Some(CryptoType::None),
                                            _ => {
                                                return Err(ErrorActive::CommandParser(
                                                    CommandParser::BadArgument(
                                                        CommandBadArgument::CryptoKey,
                                                    ),
                                                ))
                                            }
                                        },
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::CryptoKey,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                "-msgtype" => {
                                    if msg_type_found.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(CommandDoubleKey::MsgType),
                                        ));
                                    }
                                    msg_type_found = match args.next() {
                                        Some(x) => match x.to_lowercase().as_str() {
                                            "load_types" => Some(MsgType::LoadTypes),
                                            "load_metadata" => Some(MsgType::LoadMetadata),
                                            "add_specs" => Some(MsgType::AddSpecs),
                                            _ => {
                                                return Err(ErrorActive::CommandParser(
                                                    CommandParser::BadArgument(
                                                        CommandBadArgument::MsgType,
                                                    ),
                                                ))
                                            }
                                        },
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::MsgType,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                "-verifier" => {
                                    if verifier_found.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(CommandDoubleKey::Verifier),
                                        ));
                                    }
                                    verifier_found = match args.next() {
                                        Some(x) => match x.to_lowercase().as_str() {
                                            "-hex" => match args.next() {
                                                Some(h) => Some(VerKey::Hex(h.to_string())),
                                                None => {
                                                    return Err(ErrorActive::CommandParser(
                                                        CommandParser::NeedArgument(
                                                            CommandNeedArgument::VerifierHex,
                                                        ),
                                                    ))
                                                }
                                            },
                                            "-file" => match args.next() {
                                                Some(f) => Some(VerKey::File(f.to_string())),
                                                None => {
                                                    return Err(ErrorActive::CommandParser(
                                                        CommandParser::NeedArgument(
                                                            CommandNeedArgument::VerifierFile,
                                                        ),
                                                    ))
                                                }
                                            },
                                            "alice" => Some(VerKey::Alice),
                                            _ => {
                                                return Err(ErrorActive::CommandParser(
                                                    CommandParser::BadArgument(
                                                        CommandBadArgument::Verifier,
                                                    ),
                                                ))
                                            }
                                        },
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::Verifier,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                "-payload" => {
                                    if payload_found.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(CommandDoubleKey::Payload),
                                        ));
                                    }
                                    payload_found = match args.next() {
                                        Some(x) => Some(x.to_string()),
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::Payload,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                "-signature" => {
                                    if signature_found.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(CommandDoubleKey::Signature),
                                        ));
                                    }
                                    signature_found = match args.next() {
                                        Some(x) => match x.as_str() {
                                            "-hex" => match args.next() {
                                                Some(h) => Some(Entry::Hex(h.to_string())),
                                                None => {
                                                    return Err(ErrorActive::CommandParser(
                                                        CommandParser::NeedArgument(
                                                            CommandNeedArgument::SignatureHex,
                                                        ),
                                                    ))
                                                }
                                            },
                                            "-file" => match args.next() {
                                                Some(f) => Some(Entry::File(f.to_string())),
                                                None => {
                                                    return Err(ErrorActive::CommandParser(
                                                        CommandParser::NeedArgument(
                                                            CommandNeedArgument::SignatureFile,
                                                        ),
                                                    ))
                                                }
                                            },
                                            _ => {
                                                return Err(ErrorActive::CommandParser(
                                                    CommandParser::BadArgument(
                                                        CommandBadArgument::Signature,
                                                    ),
                                                ))
                                            }
                                        },
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::Signature,
                                                ),
                                            ))
                                        }
                                    }
                                }
                                "-name" => {
                                    if name.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(CommandDoubleKey::Name),
                                        ));
                                    }
                                    name = match args.next() {
                                        Some(x) => Some(x.to_string()),
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::Name,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                _ => {
                                    return Err(ErrorActive::CommandParser(
                                        CommandParser::UnexpectedKeyArgumentSequence,
                                    ))
                                }
                            }
                        }
                        // finalize what was parsed for `make`
                        let crypto = match crypto_type_found {
                            Some(x) => match x {
                                CryptoType::Ed25519 => process_verifier_and_signature(
                                    verifier_found,
                                    signature_found,
                                    Encryption::Ed25519,
                                )?,
                                CryptoType::Sr25519 => process_verifier_and_signature(
                                    verifier_found,
                                    signature_found,
                                    Encryption::Sr25519,
                                )?,
                                CryptoType::Ecdsa => process_verifier_and_signature(
                                    verifier_found,
                                    signature_found,
                                    Encryption::Ecdsa,
                                )?,
                                CryptoType::None => {
                                    if verifier_found.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::Unexpected(
                                                CommandUnexpected::VerifierNoCrypto,
                                            ),
                                        ));
                                    }
                                    if signature_found.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::Unexpected(
                                                CommandUnexpected::SignatureNoCrypto,
                                            ),
                                        ));
                                    }
                                    Crypto::None
                                }
                            },
                            None => {
                                return Err(ErrorActive::CommandParser(CommandParser::NeedKey(
                                    CommandNeedKey::Crypto,
                                )))
                            }
                        };
                        let payload = match payload_found {
                            Some(x) => {
                                let filename = format!("{}/{}", FOLDER, x);
                                match std::fs::read(&filename) {
                                    Ok(a) => a,
                                    Err(e) => return Err(ErrorActive::Input(InputActive::File(e))),
                                }
                            }
                            None => {
                                return Err(ErrorActive::CommandParser(CommandParser::NeedKey(
                                    CommandNeedKey::Payload,
                                )))
                            }
                        };
                        let msg = match msg_type_found {
                            Some(x) => match x {
                                MsgType::LoadTypes => Msg::LoadTypes(payload),
                                MsgType::LoadMetadata => Msg::LoadMetadata(payload),
                                MsgType::AddSpecs => Msg::AddSpecs(payload),
                            },
                            None => {
                                return Err(ErrorActive::CommandParser(CommandParser::NeedKey(
                                    CommandNeedKey::MsgType,
                                )))
                            }
                        };
                        let make = Make {
                            goal,
                            crypto,
                            msg,
                            name,
                        };
                        Ok(Command::Make(make))
                    }
                    "sign" => {
                        let mut goal = Goal::Both; // default option for `sign`
                        let mut args = args.peekable();
                        match args.peek() {
                            Some(x) => match x.to_lowercase().as_str() {
                                "-qr" => {
                                    goal = Goal::Qr;
                                    args.next();
                                }
                                "-text" => {
                                    goal = Goal::Text;
                                    args.next();
                                }
                                _ => (),
                            },
                            None => {
                                return Err(ErrorActive::CommandParser(
                                    CommandParser::NeedArgument(CommandNeedArgument::Sign),
                                ))
                            }
                        }
                        let mut sufficient_crypto_found = None;
                        let mut msg_type_found = None;
                        let mut payload_found = None;
                        let mut name = None; // default option for `sign`
                        while let Some(x) = args.next() {
                            let x = x.to_lowercase();
                            match x.as_str() {
                                "-sufficient" => {
                                    if sufficient_crypto_found.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(
                                                CommandDoubleKey::SufficientCrypto,
                                            ),
                                        ));
                                    }
                                    sufficient_crypto_found = match args.next() {
                                        Some(x) => match x.to_lowercase().as_str() {
                                            "-hex" => match args.next() {
                                                Some(h) => Some(Entry::Hex(h.to_string())),
                                                None => return Err(ErrorActive::CommandParser(
                                                    CommandParser::NeedArgument(
                                                        CommandNeedArgument::SufficientCryptoHex,
                                                    ),
                                                )),
                                            },
                                            "-file" => match args.next() {
                                                Some(f) => Some(Entry::File(f.to_string())),
                                                None => return Err(ErrorActive::CommandParser(
                                                    CommandParser::NeedArgument(
                                                        CommandNeedArgument::SufficientCryptoFile,
                                                    ),
                                                )),
                                            },
                                            _ => {
                                                return Err(ErrorActive::CommandParser(
                                                    CommandParser::BadArgument(
                                                        CommandBadArgument::SufficientCrypto,
                                                    ),
                                                ))
                                            }
                                        },
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::SufficientCrypto,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                "-msgtype" => {
                                    if msg_type_found.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(CommandDoubleKey::MsgType),
                                        ));
                                    }
                                    msg_type_found = match args.next() {
                                        Some(x) => match x.to_lowercase().as_str() {
                                            "load_types" => Some(MsgType::LoadTypes),
                                            "load_metadata" => Some(MsgType::LoadMetadata),
                                            "add_specs" => Some(MsgType::AddSpecs),
                                            _ => {
                                                return Err(ErrorActive::CommandParser(
                                                    CommandParser::BadArgument(
                                                        CommandBadArgument::MsgType,
                                                    ),
                                                ))
                                            }
                                        },
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::MsgType,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                "-payload" => {
                                    if payload_found.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(CommandDoubleKey::Payload),
                                        ));
                                    }
                                    payload_found = match args.next() {
                                        Some(x) => Some(x.to_string()),
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::Payload,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                "-name" => {
                                    if name.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(CommandDoubleKey::Name),
                                        ));
                                    }
                                    name = match args.next() {
                                        Some(x) => Some(x.to_string()),
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::Name,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                _ => {
                                    return Err(ErrorActive::CommandParser(
                                        CommandParser::UnexpectedKeyArgumentSequence,
                                    ))
                                }
                            }
                        }
                        // finalize command parsed with "sign"
                        let crypto = match sufficient_crypto_found {
                            Some(x) => {
                                let sufficient_crypto_vector = match x {
                                    Entry::Hex(h) => {
                                        unhex::<Active>(&h, NotHexActive::InputSufficientCrypto)?
                                    }
                                    Entry::File(f) => {
                                        let filename = format!("{}/{}", FOLDER, f);
                                        match std::fs::read(&filename) {
                                            Ok(a) => a,
                                            Err(e) => {
                                                return Err(ErrorActive::Input(InputActive::File(
                                                    e,
                                                )))
                                            }
                                        }
                                    }
                                };
                                println!(
                                    "sufficient crypto vector: {:?}",
                                    sufficient_crypto_vector
                                );
                                let sufficient_crypto = match <SufficientCrypto>::decode(
                                    &mut &sufficient_crypto_vector[..],
                                ) {
                                    Ok(a) => a,
                                    Err(_) => {
                                        return Err(ErrorActive::Input(
                                            InputActive::DecodingSufficientCrypto,
                                        ))
                                    }
                                };
                                Crypto::Sufficient(sufficient_crypto)
                            }
                            None => {
                                return Err(ErrorActive::CommandParser(CommandParser::NeedKey(
                                    CommandNeedKey::SufficientCrypto,
                                )))
                            }
                        };
                        let payload = match payload_found {
                            Some(x) => {
                                let filename = format!("{}/{}", FOLDER, x);
                                match std::fs::read(&filename) {
                                    Ok(a) => a,
                                    Err(e) => return Err(ErrorActive::Input(InputActive::File(e))),
                                }
                            }
                            None => {
                                return Err(ErrorActive::CommandParser(CommandParser::NeedKey(
                                    CommandNeedKey::Payload,
                                )))
                            }
                        };
                        let msg = match msg_type_found {
                            Some(x) => match x {
                                MsgType::LoadTypes => Msg::LoadTypes(payload),
                                MsgType::LoadMetadata => Msg::LoadMetadata(payload),
                                MsgType::AddSpecs => Msg::AddSpecs(payload),
                            },
                            None => {
                                return Err(ErrorActive::CommandParser(CommandParser::NeedKey(
                                    CommandNeedKey::MsgType,
                                )))
                            }
                        };
                        let make = Make {
                            goal,
                            crypto,
                            msg,
                            name,
                        };
                        Ok(Command::Make(make))
                    }
                    "remove" => {
                        let mut info_found = None;
                        while let Some(a) = args.next() {
                            match a.as_str() {
                                "-title" => {
                                    if info_found.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(CommandDoubleKey::Remove),
                                        ));
                                    }
                                    info_found = match args.next() {
                                        Some(b) => Some(Remove::Title(b.to_string())),
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::RemoveTitle,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                "-name" => {
                                    if info_found.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(CommandDoubleKey::Remove),
                                        ));
                                    }
                                    info_found = match args.next() {
                                        Some(b) => {
                                            let name = b.to_string();
                                            match args.next() {
                                                Some(c) => {
                                                    match c.as_str() {
                                                        "-version" => {
                                                            match args.next() {
                                                                Some(d) => {
                                                                    match d.parse::<u32> () {
                                                                        Ok(version) => Some(Remove::SpecNameVersion{name, version}),
                                                                        Err(_) => return Err(ErrorActive::CommandParser(CommandParser::BadArgument(CommandBadArgument::VersionFormat))),
                                                                    }
                                                                },
                                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::RemoveVersion))),
                                                            }
                                                        }
                                                        _ => return Err(ErrorActive::CommandParser(CommandParser::UnexpectedKeyArgumentSequence)),
                                                    }
                                                },
                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedKey(CommandNeedKey::RemoveVersion))),
                                            }
                                        }
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::RemoveName,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                _ => {
                                    return Err(ErrorActive::CommandParser(
                                        CommandParser::UnexpectedKeyArgumentSequence,
                                    ))
                                }
                            }
                        }
                        match info_found {
                            Some(x) => Ok(Command::Remove(x)),
                            None => Err(ErrorActive::CommandParser(CommandParser::NeedKey(
                                CommandNeedKey::Remove,
                            ))),
                        }
                    }
                    "restore_defaults" => {
                        if args.next().is_some() {
                            Err(ErrorActive::CommandParser(
                                CommandParser::UnexpectedKeyArgumentSequence,
                            ))
                        } else {
                            Ok(Command::RestoreDefaults)
                        }
                    }
                    "make_cold_release" => {
                        if args.next().is_some() {
                            Err(ErrorActive::CommandParser(
                                CommandParser::UnexpectedKeyArgumentSequence,
                            ))
                        } else {
                            Ok(Command::MakeColdRelease(None))
                        }
                    }
                    "transfer_meta_to_cold_release" => {
                        if args.next().is_some() {
                            Err(ErrorActive::CommandParser(
                                CommandParser::UnexpectedKeyArgumentSequence,
                            ))
                        } else {
                            Ok(Command::TransferMetaRelease)
                        }
                    }
                    "derivations" => {
                        let mut goal = Goal::Both; // default option for `derivations`
                        let mut args = args.peekable();
                        match args.peek() {
                            Some(x) => match x.to_lowercase().as_str() {
                                "-qr" => {
                                    goal = Goal::Qr;
                                    args.next();
                                }
                                "-text" => {
                                    goal = Goal::Text;
                                    args.next();
                                }
                                _ => (),
                            },
                            None => {
                                return Err(ErrorActive::CommandParser(
                                    CommandParser::NeedArgument(CommandNeedArgument::Derivations),
                                ))
                            }
                        }
                        let mut found_title = None;
                        let mut found_payload = None;
                        while let Some(a) = args.next() {
                            match a.as_str() {
                                "-title" => {
                                    if found_title.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(
                                                CommandDoubleKey::DerivationsTitle,
                                            ),
                                        ));
                                    }
                                    found_title = match args.next() {
                                        Some(b) => Some(b.to_string()),
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::DerivationsTitle,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                "-payload" => {
                                    if found_payload.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(CommandDoubleKey::Payload),
                                        ));
                                    }
                                    found_payload = match args.next() {
                                        Some(b) => match std::fs::read_to_string(&b) {
                                            Ok(c) => Some(c),
                                            Err(e) => {
                                                return Err(ErrorActive::Input(InputActive::File(
                                                    e,
                                                )))
                                            }
                                        },
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::Payload,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                _ => {
                                    return Err(ErrorActive::CommandParser(
                                        CommandParser::UnexpectedKeyArgumentSequence,
                                    ))
                                }
                            }
                        }
                        let title = match found_title {
                            Some(a) => a,
                            None => {
                                return Err(ErrorActive::CommandParser(CommandParser::NeedKey(
                                    CommandNeedKey::DerivationsTitle,
                                )))
                            }
                        };
                        let derivations = match found_payload {
                            Some(a) => a,
                            None => {
                                return Err(ErrorActive::CommandParser(CommandParser::NeedKey(
                                    CommandNeedKey::Payload,
                                )))
                            }
                        };
                        Ok(Command::Derivations(Derivations {
                            goal,
                            title,
                            derivations,
                        }))
                    }
                    "unwasm" => {
                        let mut found_payload = None;
                        let mut update_db = true;
                        while let Some(a) = args.next() {
                            match a.as_str() {
                                "-payload" => {
                                    if found_payload.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(CommandDoubleKey::Payload),
                                        ));
                                    }
                                    found_payload = match args.next() {
                                        Some(b) => Some(b.to_string()),
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::Payload,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                "-d" => update_db = false,
                                _ => {
                                    return Err(ErrorActive::CommandParser(
                                        CommandParser::UnexpectedKeyArgumentSequence,
                                    ))
                                }
                            }
                        }
                        match found_payload {
                            Some(x) => Ok(Command::Unwasm {
                                filename: x,
                                update_db,
                            }),
                            None => Err(ErrorActive::CommandParser(CommandParser::NeedKey(
                                CommandNeedKey::Payload,
                            ))),
                        }
                    }
                    "meta_default_file" => {
                        let mut name_found = None;
                        let mut version_found = None;
                        while let Some(a) = args.next() {
                            match a.as_str() {
                                "-name" => {
                                    if name_found.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(
                                                CommandDoubleKey::MetaDefaultFileName,
                                            ),
                                        ));
                                    }
                                    name_found = match args.next() {
                                        Some(b) => Some(b.to_string()),
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::MetaDefaultFileName,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                "-version" => {
                                    if version_found.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(
                                                CommandDoubleKey::MetaDefaultFileVersion,
                                            ),
                                        ));
                                    }
                                    version_found = match args.next() {
                                        Some(b) => match b.parse::<u32>() {
                                            Ok(c) => Some(c),
                                            Err(_) => {
                                                return Err(ErrorActive::CommandParser(
                                                    CommandParser::BadArgument(
                                                        CommandBadArgument::VersionFormat,
                                                    ),
                                                ))
                                            }
                                        },
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::MetaDefaultFileVersion,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                _ => {
                                    return Err(ErrorActive::CommandParser(
                                        CommandParser::UnexpectedKeyArgumentSequence,
                                    ))
                                }
                            }
                        }
                        match name_found {
                            Some(name) => match version_found {
                                Some(version) => Ok(Command::MetaDefaultFile { name, version }),
                                None => Err(ErrorActive::CommandParser(CommandParser::NeedKey(
                                    CommandNeedKey::MetaDefaultFileVersion,
                                ))),
                            },
                            None => Err(ErrorActive::CommandParser(CommandParser::NeedKey(
                                CommandNeedKey::MetaDefaultFileName,
                            ))),
                        }
                    }
                    _ => Err(ErrorActive::CommandParser(CommandParser::UnknownCommand)),
                }
            }
            None => Err(ErrorActive::CommandParser(CommandParser::NoCommand)),
        }
    }
}

fn process_verifier_and_signature(
    verifier_found: Option<VerKey>,
    signature_found: Option<Entry>,
    encryption: Encryption,
) -> Result<Crypto, ErrorActive> {
    match verifier_found {
        Some(VerKey::Hex(x)) => {
            let verifier_public_key = unhex::<Active>(&x, NotHexActive::InputPublicKey)?;
            let signature = get_needed_signature(signature_found)?;
            Ok(Crypto::Sufficient(into_sufficient(
                verifier_public_key,
                signature,
                encryption,
            )?))
        }
        Some(VerKey::File(x)) => {
            let verifier_filename = format!("{}/{}", FOLDER, x);
            let verifier_public_key = match std::fs::read(&verifier_filename) {
                Ok(a) => a,
                Err(e) => return Err(ErrorActive::Input(InputActive::File(e))),
            };
            let signature = get_needed_signature(signature_found)?;
            Ok(Crypto::Sufficient(into_sufficient(
                verifier_public_key,
                signature,
                encryption,
            )?))
        }
        Some(VerKey::Alice) => {
            if signature_found.is_some() {
                return Err(ErrorActive::CommandParser(CommandParser::Unexpected(
                    CommandUnexpected::AliceSignature,
                )));
            }
            Ok(Crypto::Alice(encryption))
        }
        None => Err(ErrorActive::CommandParser(CommandParser::NeedKey(
            CommandNeedKey::Verifier,
        ))),
    }
}

fn get_needed_signature(signature_found: Option<Entry>) -> Result<Vec<u8>, ErrorActive> {
    match signature_found {
        Some(Entry::Hex(t)) => Ok(unhex::<Active>(&t, NotHexActive::InputSignature)?),
        Some(Entry::File(t)) => {
            let signature_filename = format!("{}/{}", FOLDER, t);
            match std::fs::read(&signature_filename) {
                Ok(a) => Ok(a),
                Err(e) => Err(ErrorActive::Input(InputActive::File(e))),
            }
        }
        None => Err(ErrorActive::CommandParser(CommandParser::NeedKey(
            CommandNeedKey::Signature,
        ))),
    }
}

fn into_sufficient(
    verifier_public_key: Vec<u8>,
    signature: Vec<u8>,
    encryption: Encryption,
) -> Result<SufficientCrypto, ErrorActive> {
    match encryption {
        Encryption::Ed25519 => {
            let into_pubkey: [u8; 32] = match verifier_public_key.try_into() {
                Ok(a) => a,
                Err(_) => return Err(ErrorActive::Input(InputActive::PublicKeyLength)),
            };
            let public = ed25519::Public::from_raw(into_pubkey);
            let into_sign: [u8; 64] = match signature.try_into() {
                Ok(a) => a,
                Err(_) => return Err(ErrorActive::Input(InputActive::SignatureLength)),
            };
            let signature = ed25519::Signature::from_raw(into_sign);
            Ok(SufficientCrypto::Ed25519 { public, signature })
        }
        Encryption::Sr25519 => {
            let into_pubkey: [u8; 32] = match verifier_public_key.try_into() {
                Ok(a) => a,
                Err(_) => return Err(ErrorActive::Input(InputActive::PublicKeyLength)),
            };
            let public = sr25519::Public::from_raw(into_pubkey);
            let into_sign: [u8; 64] = match signature.try_into() {
                Ok(a) => a,
                Err(_) => return Err(ErrorActive::Input(InputActive::SignatureLength)),
            };
            let signature = sr25519::Signature::from_raw(into_sign);
            Ok(SufficientCrypto::Sr25519 { public, signature })
        }
        Encryption::Ecdsa => {
            let into_pubkey: [u8; 33] = match verifier_public_key.try_into() {
                Ok(a) => a,
                Err(_) => return Err(ErrorActive::Input(InputActive::PublicKeyLength)),
            };
            let public = ecdsa::Public::from_raw(into_pubkey);
            let into_sign: [u8; 65] = match signature.try_into() {
                Ok(a) => a,
                Err(_) => return Err(ErrorActive::Input(InputActive::SignatureLength)),
            };
            let signature = ecdsa::Signature::from_raw(into_sign);
            Ok(SufficientCrypto::Ecdsa { public, signature })
        }
    }
}
