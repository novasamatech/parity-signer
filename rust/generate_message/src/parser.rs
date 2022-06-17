//! Command line parser for the client
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

/// Commands to execute.
pub enum Command {
    /// Execute [`Show`] command.
    Show(Show),

    /// Prepare payload for `add_specs` update according to
    /// [`InstructionSpecs`].
    Specs(InstructionSpecs),

    /// Prepare payload for `load_metadata` update according to
    /// [`InstructionMeta`].
    Load(InstructionMeta),

    /// Prepare payload for `load_types` update.
    Types,

    /// Complete update generation according to [`Make`] settings.
    Make(Make),

    /// Remove data from the hot database.
    Remove(Remove),

    /// Restore hot database to default state
    RestoreDefaults,

    /// Generate release cold database at optionally provided path
    MakeColdRelease(Option<PathBuf>),

    /// Transfer metadata from hot database to release cold database at
    /// optionally provided path
    TransferMetaRelease(Option<PathBuf>),

    /// Make derivations import
    Derivations(Derivations),

    /// Prepare payload for `load_metadata` update from `.wasm` file
    Unwasm { filename: String, update_db: bool },

    /// Make file with hexadecimal metadata for `defaults` set
    MetaDefaultFile { name: String, version: u32 },

    /// Make file with hexadecimal metadata from specific block
    MetaAtBlock { url: String, block_hash: String },
}

/// Display data commands.
pub enum Show {
    /// Show all hot database [`METATREE`](constants::METATREE) entries
    Metadata,

    /// Show all hot database [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) entries
    Networks,

    /// Show network specs from [`SPECSTREEPREP`](constants::SPECSTREEPREP)
    /// entry. Associated data is user-entered network address book title.
    Specs(String),

    /// Check that external file is valid network metadata and search for
    /// similar entry in hot database [`METATREE`](constants::METATREE).
    /// Associated data is user-provided path to the metadata file.
    CheckFile(String),

    /// Show all entries from [`META_HISTORY`](constants::META_HISTORY) tree
    BlockHistory,
}

/// Command details for `load_metadata`.
pub struct InstructionMeta {
    /// Setting key, as read from command line
    pub set: Set,

    /// Reference key, as read from command line
    pub content: Content,
}

/// Command details for `add_specs`.
pub struct InstructionSpecs {
    /// Setting key, as read from command line
    pub set: Set,

    /// Reference key, as read from command line
    pub content: Content,

    /// Overrides, relevant only for `add_specs` command
    pub over: Override,
}

/// Reference key for `load_metadata` and `add_specs` commands.
pub enum Content {
    /// Key `-a`: deal with all relevant database entries
    ///
    /// Associated data is a flag to indicate skipping errors when processing
    /// `-a`.
    /// Passing optional `-s` key sets this to false, i.e. makes the run stop
    /// after the first error encountered.
    All { pass_errors: bool },

    /// Key `-n`: process only the network referred to by:
    ///
    /// - network name (in `load_metadata` command)
    /// - network address book title (in `add_specs` command)
    Name(String),

    /// Key `-u`: process only the network referred to by url address
    Address(String),
}

/// Setting key for `load_metadata` and `add_specs` commands.
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
pub struct Make {
    /// Target output format
    pub goal: Goal,

    /// Who is signing the payload
    pub crypto: Crypto,

    /// Payload
    pub msg: Msg,

    /// Output name override
    pub name: Option<String>,
}

/// Target output format for `derivations`, `make` and `sign` commands.
pub enum Goal {
    /// Only QR code
    Qr,

    /// Only text file with hexadecimal string (used for tests)
    Text,

    /// Both QR code and text file, default
    Both,
}

/// Verifier-to-be, for `make` and `sign` commands.
pub enum Crypto {
    /// Alice key, with well-known [seed phrase](constants::ALICE_SEED_PHRASE)
    /// and derivation `//Alice`, to generate test updating payloads.
    ///
    /// Associated data is [`Encryption`] algorithm used.
    Alice(Encryption),

    /// No verifier, to make unsigned updates.
    None,

    /// Real verifier, [`SufficientCrypto`] is either assembled from `make`
    /// command input parts or from `sign` command input directly.
    Sufficient(SufficientCrypto),
}

/// Payload for `make` and `sign` commands.
///
/// Associated data is `Vec<u8>` blob that becomes part of the update.
///
/// Payload content details are described in [definitions::qr_transfers].
pub enum Msg {
    /// `load_types` payload
    LoadTypes(Vec<u8>),

    /// `load_metadata` payload
    LoadMetadata(Vec<u8>),

    /// `add_specs` payload
    AddSpecs(Vec<u8>),
}

/// Argument for `-crypto` key in `make` command.
enum CryptoType {
    /// `ed25519` argument
    Ed25519,

    /// `sr25519` argument
    Sr25519,

    /// `ecdsa` argument
    Ecdsa,

    /// `none` argument
    None,
}

/// Argument for `-msgtype` key in `make` and `sign` commands.
enum MsgType {
    /// `load_types` argument
    LoadTypes,

    /// `load_metadata` argument
    LoadMetadata,

    /// `add_specs` argument
    AddSpecs,
}

/// Argument for `-verifier` key in `make` command.
enum VerKey {
    /// Hexadecimal string input, entered with `-hex` key.
    ///
    /// Associated data is the string itself.
    Hex(String),

    /// Input from file, entered with `-file` key.
    ///
    /// Associated data is the file path.
    File(String),

    /// Verifier is Alice
    Alice,
}

/// Argument for `-signature` key in `make` command.
enum Entry {
    /// Hexadecimal string input, entered with `-hex` key.
    ///
    /// Associated data is the string itself.
    Hex(String),

    /// Input from file, entered with `-file` key.
    ///
    /// Associated data is the file path.
    File(String),
}

/// Data to process `remove` command.
pub enum Remove {
    /// Removing all network data by network address book title.
    ///
    /// Associated data is user-entered network address book title.
    Title(String),

    /// Remove specified network metadata entry.
    ///
    /// Associated data is network name and version.
    SpecNameVersion { name: String, version: u32 },
}

/// Data to process `derivations` command.
pub struct Derivations {
    /// Target output format
    pub goal: Goal,

    /// Address book title for network in which addresses with imported
    /// derivations will be made in Signer
    pub title: String,

    /// Contents of the payload file
    pub derivations: String,
}

/// Overrides for `add_specs` command.
pub struct Override {
    /// [`Encryption`] override to specify encryption algorithm used by a new
    /// network or to add another encryption algorithm in known network.
    pub encryption: Option<Encryption>,

    /// Network title override, so that user can specify the network title in
    /// [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend)
    /// that determines under what title the network is displayed in the Signer
    pub title: Option<String>,

    /// Token override to specify decimals and units used to display balance in
    /// network transactions.
    ///
    /// Token override could be invoked only if:
    ///
    /// - network has no database record yet
    /// - network has multiple decimals and unit values, those were retrieved as
    /// arrays of equal size.
    pub token: Option<Token>,
}

impl Override {
    /// Flag to indicate that no overrides were invoked.
    pub fn all_empty(&self) -> bool {
        self.encryption.is_none() && self.title.is_none() && self.token.is_none()
    }
}

/// Data from command line for token override.
pub struct Token {
    pub decimals: u8,
    pub unit: String,
}

impl Command {
    /// Interpret command line input into `Command`.
    pub fn new(mut args: env::Args) -> Result<Command, ErrorActive> {
        args.next();

        match args.next() {
            Some(arg) => {
                let arg = arg.to_lowercase();
                match arg.as_str() {
                    "show" => match args.next() {
                        Some(show) => match show.to_lowercase().as_str() {
                            "-metadata" => {
                                if args.next().is_some() {
                                    Err(ErrorActive::CommandParser(
                                        CommandParser::UnexpectedKeyArgumentSequence,
                                    ))
                                } else {
                                    Ok(Command::Show(Show::Metadata))
                                }
                            }
                            "-networks" => {
                                if args.next().is_some() {
                                    Err(ErrorActive::CommandParser(
                                        CommandParser::UnexpectedKeyArgumentSequence,
                                    ))
                                } else {
                                    Ok(Command::Show(Show::Networks))
                                }
                            }
                            "-specs" => match args.next() {
                                Some(title) => {
                                    if args.next().is_some() {
                                        Err(ErrorActive::CommandParser(
                                            CommandParser::UnexpectedKeyArgumentSequence,
                                        ))
                                    } else {
                                        Ok(Command::Show(Show::Specs(title)))
                                    }
                                }
                                None => {
                                    Err(ErrorActive::CommandParser(CommandParser::NeedArgument(
                                        CommandNeedArgument::ShowSpecsTitle,
                                    )))
                                }
                            },
                            "-block_history" => {
                                if args.next().is_some() {
                                    Err(ErrorActive::CommandParser(
                                        CommandParser::UnexpectedKeyArgumentSequence,
                                    ))
                                } else {
                                    Ok(Command::Show(Show::BlockHistory))
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
                        let mut set = None;
                        let mut content_key = None;
                        let mut s_key_used = false;
                        let mut name = None;
                        let mut encryption = None;
                        let mut token = None;
                        let mut title = None;
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
                                    "-d" | "-f" | "-k" | "-p" | "-t" => match set {
                                        Some(_) => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::DoubleKey(CommandDoubleKey::Set),
                                            ))
                                        }
                                        None => {
                                            set = match x.as_str() {
                                                "-d" => Some(Set::D),
                                                "-f" => Some(Set::F),
                                                "-k" => Some(Set::K),
                                                "-p" => Some(Set::P),
                                                "-t" => Some(Set::T),
                                                _ => unreachable!(),
                                            };
                                        }
                                    },
                                    "-s" => s_key_used = true,
                                    "-ed25519" | "-sr25519" | "-ecdsa" => match encryption {
                                        Some(_) => {
                                            if arg == "load_metadata" {
                                                return Err(ErrorActive::CommandParser(
                                                    CommandParser::UnexpectedKeyArgumentSequence,
                                                ));
                                            } else {
                                                return Err(ErrorActive::CommandParser(
                                                    CommandParser::DoubleKey(
                                                        CommandDoubleKey::CryptoOverride,
                                                    ),
                                                ));
                                            }
                                        }
                                        None => {
                                            encryption = match x.as_str() {
                                                "-ed25519" => Some(Encryption::Ed25519),
                                                "-sr25519" => Some(Encryption::Sr25519),
                                                "-ecdsa" => Some(Encryption::Ecdsa),
                                                _ => unreachable!(),
                                            };
                                        }
                                    },
                                    "-token" => {
                                        match token {
                                            Some(_) => {
                                                if arg == "load_metadata" {
                                                    return Err(ErrorActive::CommandParser(
                                                        CommandParser::UnexpectedKeyArgumentSequence,
                                                    ));
                                                } else {
                                                    return Err(ErrorActive::CommandParser(
                                                        CommandParser::DoubleKey(
                                                            CommandDoubleKey::TokenOverride,
                                                        ),
                                                    ));
                                                }
                                            }
                                            None => token = match args.next() {
                                                Some(b) => match b.parse::<u8>() {
                                                    Ok(decimals) => match args.next() {
                                                        Some(c) => Some(Token {
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
                                    "-title" => match title {
                                        Some(_) => {
                                            if arg == "load_metadata" {
                                                return Err(ErrorActive::CommandParser(
                                                    CommandParser::UnexpectedKeyArgumentSequence,
                                                ));
                                            } else {
                                                return Err(ErrorActive::CommandParser(
                                                    CommandParser::DoubleKey(
                                                        CommandDoubleKey::TitleOverride,
                                                    ),
                                                ));
                                            }
                                        }
                                        None => {
                                            title = match args.next() {
                                                Some(b) => Some(b),
                                                None => {
                                                    return Err(ErrorActive::CommandParser(
                                                        CommandParser::NeedArgument(
                                                            CommandNeedArgument::TitleOverride,
                                                        ),
                                                    ))
                                                }
                                            }
                                        }
                                    },
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

                        let set = set.unwrap_or(Set::T);

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
                                    if s_key_used {
                                        Content::All { pass_errors: false }
                                    } else {
                                        Content::All { pass_errors: true }
                                    }
                                }
                                "-n" => match name {
                                    Some(n) => {
                                        if s_key_used {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::UnexpectedKeyArgumentSequence,
                                            ));
                                        } else {
                                            Content::Name(n)
                                        }
                                    }
                                    None => {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::NeedArgument(
                                                CommandNeedArgument::NetworkName,
                                            ),
                                        ))
                                    }
                                },
                                "-u" => match name {
                                    Some(a) => {
                                        if s_key_used {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::UnexpectedKeyArgumentSequence,
                                            ));
                                        } else {
                                            Content::Address(a)
                                        }
                                    }
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

                        match arg.as_str() {
                            "load_metadata" => {
                                if encryption.is_some() || token.is_some() || title.is_some() {
                                    return Err(ErrorActive::CommandParser(
                                        CommandParser::UnexpectedKeyArgumentSequence,
                                    ));
                                }
                                Ok(Command::Load(InstructionMeta { set, content }))
                            }
                            "add_specs" => {
                                let over = Override {
                                    encryption,
                                    title,
                                    token,
                                };
                                Ok(Command::Specs(InstructionSpecs { set, content, over }))
                            }
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
                    "make_cold_release" => match args.next() {
                        Some(path) => {
                            if args.next().is_some() {
                                Err(ErrorActive::CommandParser(
                                    CommandParser::UnexpectedKeyArgumentSequence,
                                ))
                            } else {
                                Ok(Command::MakeColdRelease(Some(PathBuf::from(path))))
                            }
                        }
                        None => Ok(Command::MakeColdRelease(None)),
                    },
                    "transfer_meta_to_cold_release" => match args.next() {
                        Some(path) => {
                            if args.next().is_some() {
                                Err(ErrorActive::CommandParser(
                                    CommandParser::UnexpectedKeyArgumentSequence,
                                ))
                            } else {
                                Ok(Command::TransferMetaRelease(Some(PathBuf::from(path))))
                            }
                        }
                        None => Ok(Command::TransferMetaRelease(None)),
                    },
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
                    "meta_at_block" => {
                        let mut url_found = None;
                        let mut block_found = None;
                        while let Some(a) = args.next() {
                            match a.as_str() {
                                "-u" => {
                                    if url_found.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(
                                                CommandDoubleKey::MetaAtBlockUrl,
                                            ),
                                        ));
                                    }
                                    url_found = match args.next() {
                                        Some(b) => Some(b.to_string()),
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::MetaAtBlockUrl,
                                                ),
                                            ))
                                        }
                                    };
                                }
                                "-block" => {
                                    if block_found.is_some() {
                                        return Err(ErrorActive::CommandParser(
                                            CommandParser::DoubleKey(
                                                CommandDoubleKey::MetaAtBlockHash,
                                            ),
                                        ));
                                    }
                                    block_found = match args.next() {
                                        Some(b) => Some(b.to_string()),
                                        None => {
                                            return Err(ErrorActive::CommandParser(
                                                CommandParser::NeedArgument(
                                                    CommandNeedArgument::MetaAtBlockHash,
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
                        match url_found {
                            Some(url) => match block_found {
                                Some(block_hash) => Ok(Command::MetaAtBlock { url, block_hash }),
                                None => Err(ErrorActive::CommandParser(CommandParser::NeedKey(
                                    CommandNeedKey::MetaAtBlockHash,
                                ))),
                            },
                            None => Err(ErrorActive::CommandParser(CommandParser::NeedKey(
                                CommandNeedKey::MetaAtBlockUrl,
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

/// Get verifier infortmation from command line arguments.
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

/// Get `Vec<u8>` signature draft from command line entry into.
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

/// Fit public key and signature drafts into [`SufficientCrypto`].
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
