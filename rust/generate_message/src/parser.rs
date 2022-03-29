use std::{env, path::PathBuf};
use constants::{EXPORT_FOLDER, FOLDER};
use definitions::{crypto::{Encryption, SufficientCrypto}, error::{Active, CommandBadArgument, CommandDoubleKey, CommandNeedArgument, CommandNeedKey, CommandParser, CommandUnexpected, ErrorActive, InputActive, NotHexActive}, helpers::unhex};
use parity_scale_codec::Decode;
use sp_core::{ed25519, sr25519, ecdsa};
use std::convert::TryInto;

/// Expected typical run commands:
/// `$ cargo run show database`
/// `$ cargo run show address_book`
/// `$ cargo run load_metadata -n westend`
/// `$ cargo run add_specs -d -n -ed25519 westend`
/// `$ cargo run add_network -u wss://unknown-network.eu -ecdsa`
/// `$ cargo run derivations -title westend -payload my_derivations_file`


/// Enum to describe the incoming command contents
pub enum Command {
    Show(Show),
    Types,
    Load(Instruction),
    Specs(Instruction),
    Make(Make),
    Remove(Remove),
    RestoreDefaults,
    MakeColdWithIdentities,
    TransferMeta,
    MakeColdRelease(Option<PathBuf>),
    TransferMetaRelease,
    Derivations(Derivations),
    Unwasm{filename: String, update_db: bool},
}

pub enum Show {
    Database,
    AddressBook,
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
    AddSpecs
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
    SpecNameVersion{name: String, version: u32},
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
                    "show" => {
                        match args.next() {
                            Some(show) => match show.to_lowercase().as_str() {
                                "-database" => Ok(Command::Show(Show::Database)),
                                "-address_book" => Ok(Command::Show(Show::AddressBook)),
                                _ => {return Err(ErrorActive::CommandParser(CommandParser::UnexpectedKeyArgumentSequence))},
                            },
                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedKey(CommandNeedKey::Show)))
                        }
                    },
                    "load_types" => Ok(Command::Types),
                    "load_metadata"|"add_specs" => {
                        let mut set_key = None;
                        let mut content_key = None;
                        let mut pass_errors = true;
                        let mut name = None;
                        let mut encryption_override_key = None;
                        let mut token = None;

                        loop {
                            match args.next() {
                                Some(x) => {
                                    let x = x.to_lowercase();
                                    if x.starts_with("-") {
                                        match x.as_str() {
                                            "-a"|"-n"|"-u" => {
                                                match content_key {
                                                    Some(_) => {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::Content)))},
                                                    None => {content_key = Some(x)}
                                                }
                                            },
                                            "-d"|"-f"|"-k"|"-p"|"-t" => {
                                                match set_key {
                                                    Some(_) => {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::Set)))},
                                                    None => {set_key = Some(x)}
                                                }
                                            },
                                            "-s" => {pass_errors = false},
                                            "-ed25519"|"-sr25519"|"-ecdsa" => {
                                                if arg == "load_metadata" {return Err(ErrorActive::CommandParser(CommandParser::UnexpectedKeyArgumentSequence))}
                                                match encryption_override_key {
                                                    Some(_) =>  {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::CryptoOverride)))},
                                                    None => {encryption_override_key = Some(x)}
                                                }
                                            },
                                            "-token" => {
                                                if arg == "load_metadata" {return Err(ErrorActive::CommandParser(CommandParser::UnexpectedKeyArgumentSequence))}
                                                match token {
                                                    Some(_) => return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::TokenOverride))),
                                                    None => {
                                                        token = match args.next() {
                                                            Some(b) => {
                                                                match b.parse::<u8> () {
                                                                    Ok(decimals) => {
                                                                       match args.next() {
                                                                           Some(c) => Some(TokenOverride{decimals, unit: c.to_string()}),
                                                                           None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::TokenUnit)))
                                                                       }
                                                                    },
                                                                    Err(_) => return Err(ErrorActive::CommandParser(CommandParser::Unexpected(CommandUnexpected::DecimalsFormat))),
                                                                }
                                                            },
                                                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::TokenDecimals)))
                                                        }
                                                    },
                                                }
                                            },
                                            _ => {return Err(ErrorActive::CommandParser(CommandParser::UnexpectedKeyArgumentSequence))},
                                        }
                                    }
                                    else {
                                        match name {
                                            Some(_) => return Err(ErrorActive::CommandParser(CommandParser::OnlyOneNetworkId)),
                                            None => {name = Some(x)}
                                        }
                                    }
                                },
                                None => break,
                            }
                        }

                        let set = match set_key {
                            Some(x) => {
                                match x.as_str() {
                                    "-d" => Set::D,
                                    "-f" => Set::F,
                                    "-k" => Set::K,
                                    "-p" => Set::P,
                                    "-t" => Set::T,
                                    _ => unreachable!(),
                                }
                            },
                            None => Set::T,
                        };

                        let encryption = match encryption_override_key {
                            Some(x) => {
                                match x.as_str() {
                                    "-ed25519" => Some(Encryption::Ed25519),
                                    "-sr25519" => Some(Encryption::Sr25519),
                                    "-ecdsa" => Some(Encryption::Ecdsa),
                                    _ => unreachable!(),
                                }
                            },
                            None => None,
                        };
                        let over = Override{encryption, token};

                        let content = match content_key {
                            Some(x) => {
                                match x.as_str() {
                                    "-a" => {
                                        if let Some(_) = name {return Err(ErrorActive::CommandParser(CommandParser::Unexpected(CommandUnexpected::KeyAContent)))}
                                        Content::All
                                    },
                                    "-n" => {
                                        match name {
                                            Some(n) => Content::Name(n),
                                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::NetworkName)))
                                        }
                                    },
                                    "-u" => {
                                        match name {
                                            Some(a) => Content::Address(a),
                                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::NetworkUrl)))
                                        }
                                    },
                                    _ => unreachable!(),
                                }
                            },
                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedKey(CommandNeedKey::Content))),
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
                    },
                    "make" => {
                        let mut goal = Goal::Both; // default option for `make`
                        let mut args = args.peekable();
                        match args.peek() {
                            Some(x) => {
                                match x.to_lowercase().as_str() {
                                    "-qr" => {
                                        goal = Goal::Qr;
                                        args.next();
                                    },
                                    "-text" => {
                                        goal = Goal::Text;
                                        args.next();
                                    },
                                    _ => (),
                                }
                            },
                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::Make))),
                        }
                        let mut crypto_type_found = None;
                        let mut msg_type_found = None;
                        let mut verifier_found = None;
                        let mut payload_found = None;
                        let mut signature_found = None;
                        let mut name = None; // default option for `make`
                        loop {
                            match args.next() {
                                Some(x) => {
                                    let x = x.to_lowercase();
                                    match x.as_str() {
                                        "-crypto" => {
                                            if let Some(_) = crypto_type_found {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::CryptoKey)))}
                                            crypto_type_found = match args.next() {
                                                Some(x) => {
                                                    match x.to_lowercase().as_str() {
                                                        "ed25519" => Some(CryptoType::Ed25519),
                                                        "sr25519" => Some(CryptoType::Sr25519),
                                                        "ecdsa" => Some(CryptoType::Ecdsa),
                                                        "none" => Some(CryptoType::None),
                                                        _ => return Err(ErrorActive::CommandParser(CommandParser::BadArgument(CommandBadArgument::CryptoKey))),
                                                    }
                                                },
                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::CryptoKey))),
                                            };
                                        },
                                        "-msgtype" => {
                                            if let Some(_) = msg_type_found {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::MsgType)))}
                                            msg_type_found = match args.next() {
                                                Some(x) => {
                                                    match x.to_lowercase().as_str() {
                                                        "load_types" => Some(MsgType::LoadTypes),
                                                        "load_metadata" => Some(MsgType::LoadMetadata),
                                                        "add_specs" => Some(MsgType::AddSpecs),
                                                        _ => return Err(ErrorActive::CommandParser(CommandParser::BadArgument(CommandBadArgument::MsgType))),
                                                    }
                                                },
                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::MsgType))),
                                            };
                                        },
                                        "-verifier" => {
                                            if let Some(_) = verifier_found {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::Verifier)))}
                                            verifier_found = match args.next() {
                                                Some(x) => {
                                                    match x.to_lowercase().as_str() {
                                                        "-hex" => {
                                                            match args.next() {
                                                                Some(h) => Some(VerKey::Hex(h.to_string())),
                                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::VerifierHex))),
                                                            }
                                                        },
                                                        "-file" => {
                                                            match args.next() {
                                                                Some(f) => Some(VerKey::File(f.to_string())),
                                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::VerifierFile))),
                                                            }
                                                        },
                                                        "alice" => Some(VerKey::Alice),
                                                        _ => return Err(ErrorActive::CommandParser(CommandParser::BadArgument(CommandBadArgument::Verifier))),
                                                    }
                                                },
                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::Verifier))),
                                            };
                                        },
                                        "-payload" => {
                                            if let Some(_) = payload_found {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::Payload)))}
                                            payload_found = match args.next() {
                                                Some(x) => Some(x.to_string()),
                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::Payload))),
                                            };
                                        },
                                        "-signature" => {
                                            if let Some(_) = signature_found {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::Signature)))}
                                            signature_found = match args.next() {
                                                Some(x) => {
                                                    match x.as_str() {
                                                        "-hex" => {
                                                            match args.next() {
                                                                Some(h) => Some(Entry::Hex(h.to_string())),
                                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::SignatureHex))),
                                                            }
                                                        },
                                                        "-file" => {
                                                            match args.next() {
                                                                Some(f) => Some(Entry::File(f.to_string())),
                                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::SignatureFile))),
                                                            }
                                                        },
                                                        _ => return Err(ErrorActive::CommandParser(CommandParser::BadArgument(CommandBadArgument::Signature))),
                                                    }
                                                },
                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::Signature))),
                                            }
                                        },
                                        "-name" => {
                                            if let Some(_) = name {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::Name)))}
                                            name = match args.next() {
                                                Some(x) => Some(format!("{}/{}", EXPORT_FOLDER, x)),
                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::Name))),
                                            };
                                        },
                                        _ => return Err(ErrorActive::CommandParser(CommandParser::UnexpectedKeyArgumentSequence)),
                                    }
                                },
                                None => break,
                            }
                        }
                    // finalize what was parsed for `make`
                        let crypto = match crypto_type_found {
                            Some(x) => {
                                match x {
                                    CryptoType::Ed25519 => process_verifier_and_signature (verifier_found, signature_found, Encryption::Ed25519)?,
                                    CryptoType::Sr25519 => process_verifier_and_signature (verifier_found, signature_found, Encryption::Sr25519)?,
                                    CryptoType::Ecdsa => process_verifier_and_signature (verifier_found, signature_found, Encryption::Ecdsa)?,
                                    CryptoType::None => {
                                        if let Some(_) = verifier_found {return Err(ErrorActive::CommandParser(CommandParser::Unexpected(CommandUnexpected::VerifierNoCrypto)))}
                                        if let Some(_) = signature_found {return Err(ErrorActive::CommandParser(CommandParser::Unexpected(CommandUnexpected::SignatureNoCrypto)))}
                                        Crypto::None
                                    },
                                }
                            },
                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedKey(CommandNeedKey::Crypto))),
                        };
                        let payload = match payload_found {
                            Some(x) => {
                                let filename = format!("{}/{}", FOLDER, x);
                                match std::fs::read(&filename) {
                                    Ok(a) => a,
                                    Err(e) => return Err(ErrorActive::Input(InputActive::File(e))),
                                }
                            },
                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedKey(CommandNeedKey::Payload))),
                        };
                        let msg = match msg_type_found {
                            Some(x) => {
                                match x {
                                    MsgType::LoadTypes => Msg::LoadTypes(payload),
                                    MsgType::LoadMetadata => Msg::LoadMetadata(payload),
                                    MsgType::AddSpecs => Msg::AddSpecs(payload),
                                }
                            },
                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedKey(CommandNeedKey::MsgType))),
                        };
                        let make = Make {
                            goal,
                            crypto,
                            msg,
                            name,
                        };
                        Ok(Command::Make(make))
                    },
                    "sign" => {
                        let mut goal = Goal::Both; // default option for `sign`
                        let mut args = args.peekable();
                        match args.peek() {
                            Some(x) => {
                                match x.to_lowercase().as_str() {
                                    "-qr" => {
                                        goal = Goal::Qr;
                                        args.next();
                                    },
                                    "-text" => {
                                        goal = Goal::Text;
                                        args.next();
                                    },
                                    _ => (),
                                }
                            },
                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::Sign))),
                        }
                        let mut sufficient_crypto_found = None;
                        let mut msg_type_found = None;
                        let mut payload_found = None;
                        let mut name = None; // default option for `sign`
                        loop {
                            match args.next() {
                                Some(x) => {
                                    let x = x.to_lowercase();
                                    match x.as_str() {
                                        "-sufficient" => {
                                            if let Some(_) = sufficient_crypto_found {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::SufficientCrypto)))}
                                            sufficient_crypto_found = match args.next() {
                                                Some(x) => {
                                                    match x.to_lowercase().as_str() {
                                                        "-hex" => {
                                                            match args.next() {
                                                                Some(h) => Some(Entry::Hex(h.to_string())),
                                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::SufficientCryptoHex))),
                                                            }
                                                        },
                                                        "-file" => {
                                                            match args.next() {
                                                                Some(f) => Some(Entry::File(f.to_string())),
                                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::SufficientCryptoFile))),
                                                            }
                                                        },
                                                        _ => return Err(ErrorActive::CommandParser(CommandParser::BadArgument(CommandBadArgument::SufficientCrypto))),
                                                    }
                                                },
                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::SufficientCrypto))),
                                            };
                                        },
                                        "-msgtype" => {
                                            if let Some(_) = msg_type_found {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::MsgType)))}
                                            msg_type_found = match args.next() {
                                                Some(x) => {
                                                    match x.to_lowercase().as_str() {
                                                        "load_types" => Some(MsgType::LoadTypes),
                                                        "load_metadata" => Some(MsgType::LoadMetadata),
                                                        "add_specs" => Some(MsgType::AddSpecs),
                                                        _ => return Err(ErrorActive::CommandParser(CommandParser::BadArgument(CommandBadArgument::MsgType))),
                                                    }
                                                },
                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::MsgType))),
                                            };
                                        },
                                        "-payload" => {
                                            if let Some(_) = payload_found {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::Payload)))}
                                            payload_found = match args.next() {
                                                Some(x) => Some(x.to_string()),
                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::Payload))),
                                            };
                                        },
                                        "-name" => {
                                            if let Some(_) = name {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::Name)))}
                                            name = match args.next() {
                                                Some(x) => Some(format!("{}/{}", EXPORT_FOLDER, x)),
                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::Name))),
                                            };
                                        },
                                        _ => return Err(ErrorActive::CommandParser(CommandParser::UnexpectedKeyArgumentSequence)),
                                    }
                                },
                                None => break,
                            }
                        }
                    // finalize command parsed with "sign"
                        let crypto = match sufficient_crypto_found {
                            Some(x) => {
                                let sufficient_crypto_vector = match x {
                                    Entry::Hex(h) => unhex::<Active>(&h, NotHexActive::InputSufficientCrypto)?,
                                    Entry::File(f) => {
                                        let filename = format!("{}/{}", FOLDER, f);
                                        match std::fs::read(&filename) {
                                            Ok(a) => a,
                                            Err(e) => return Err(ErrorActive::Input(InputActive::File(e))),
                                        }
                                    },
                                };
                                println!("sufficient crypto vector: {:?}", sufficient_crypto_vector);
                                let sufficient_crypto = match <SufficientCrypto>::decode(&mut &sufficient_crypto_vector[..]) {
                                    Ok(a) => a,
                                    Err(_) => return Err(ErrorActive::Input(InputActive::DecodingSufficientCrypto)),
                                };
                                Crypto::Sufficient(sufficient_crypto)
                            },
                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedKey(CommandNeedKey::SufficientCrypto))),
                        };
                        let payload = match payload_found {
                            Some(x) => {
                                let filename = format!("{}/{}", FOLDER, x);
                                match std::fs::read(&filename) {
                                    Ok(a) => a,
                                    Err(e) => return Err(ErrorActive::Input(InputActive::File(e))),
                                }
                            },
                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedKey(CommandNeedKey::Payload))),
                        };
                        let msg = match msg_type_found {
                            Some(x) => {
                                match x {
                                    MsgType::LoadTypes => Msg::LoadTypes(payload),
                                    MsgType::LoadMetadata => Msg::LoadMetadata(payload),
                                    MsgType::AddSpecs => Msg::AddSpecs(payload),
                                }
                            },
                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedKey(CommandNeedKey::MsgType))),
                        };
                        let make = Make {
                            goal,
                            crypto,
                            msg,
                            name,
                        };
                        Ok(Command::Make(make))
                    },
                    "remove" => {
                        let mut info_found = None;
                        loop {
                            match args.next() {
                                Some(a) => {
                                    match a.as_str() {
                                        "-title" => {
                                            if let Some(_) = info_found {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::Remove)))}
                                            info_found = match args.next() {
                                                Some(b) => Some(Remove::Title(b.to_string())),
                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::RemoveTitle))),
                                            };
                                        },
                                        "-name" => {
                                            if let Some(_) = info_found {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::Remove)))}
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
                                                                                Err(_) => return Err(ErrorActive::CommandParser(CommandParser::Unexpected(CommandUnexpected::VersionFormat))),
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
                                                },
                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::RemoveName))),
                                            };
                                        },
                                        _ => return Err(ErrorActive::CommandParser(CommandParser::UnexpectedKeyArgumentSequence)),
                                    }
                                },
                                None => break,
                            }
                        }
                        match info_found {
                            Some(x) => Ok(Command::Remove(x)),
                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedKey(CommandNeedKey::Remove))),
                        }
                    },
                    "restore_defaults" => Ok(Command::RestoreDefaults),
                    "make_cold_with_identities" => Ok(Command::MakeColdWithIdentities),
                    "transfer_meta_to_cold" => Ok(Command::TransferMeta),
                    "make_cold_release" => Ok(Command::MakeColdRelease(None)),
                    "transfer_meta_to_cold_release" => Ok(Command::TransferMetaRelease),
                    "derivations" => {
                        let mut goal = Goal::Both; // default option for `derivations`
                        let mut args = args.peekable();
                        match args.peek() {
                            Some(x) => {
                                match x.to_lowercase().as_str() {
                                    "-qr" => {
                                        goal = Goal::Qr;
                                        args.next();
                                    },
                                    "-text" => {
                                        goal = Goal::Text;
                                        args.next();
                                    },
                                    _ => (),
                                }
                            },
                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::Derivations))),
                        }
                        let mut found_title = None;
                        let mut found_payload = None;
                        loop {
                            match args.next() {
                                Some(a) => {
                                    match a.as_str() {
                                        "-title" => {
                                            if let Some(_) = found_title {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::DerivationsTitle)))}
                                            found_title = match args.next() {
                                                Some(b) => Some(b.to_string()),
                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::DerivationsTitle))),
                                            };
                                        },
                                        "-payload" => {
                                            if let Some(_) = found_payload {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::Payload)))}
                                            found_payload = match args.next() {
                                                Some(b) => {
                                                    match std::fs::read_to_string(&b) {
                                                        Ok(c) => Some(c),
                                                        Err(e) => return Err(ErrorActive::Input(InputActive::File(e))),
                                                    }
                                                },
                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::Payload))),
                                            };
                                        },
                                        _ => return Err(ErrorActive::CommandParser(CommandParser::UnexpectedKeyArgumentSequence)),
                                    }
                                },
                                None => break,
                            }
                        }
                        let title = match found_title {
                            Some(a) => a,
                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedKey(CommandNeedKey::DerivationsTitle))),
                        };
                        let derivations = match found_payload {
                            Some(a) => a,
                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedKey(CommandNeedKey::Payload))),
                        };
                        Ok(Command::Derivations(Derivations{goal, title, derivations}))
                    },
                    "unwasm" => {
                        let mut found_payload = None;
                        let mut update_db = true;
                        loop {
                            match args.next() {
                                Some(a) => {
                                    match a.as_str() {
                                        "-payload" => {
                                            if let Some(_) = found_payload {return Err(ErrorActive::CommandParser(CommandParser::DoubleKey(CommandDoubleKey::Payload)))}
                                            found_payload = match args.next() {
                                                Some(b) => Some(b.to_string()),
                                                None => return Err(ErrorActive::CommandParser(CommandParser::NeedArgument(CommandNeedArgument::Payload))),
                                            };
                                        },
                                        "-d" => {update_db = false},
                                        _ => return Err(ErrorActive::CommandParser(CommandParser::UnexpectedKeyArgumentSequence)),
                                    }
                                },
                                None => break,
                            }
                        }
                        match found_payload {
                            Some(x) => Ok(Command::Unwasm{filename: x, update_db}),
                            None => return Err(ErrorActive::CommandParser(CommandParser::NeedKey(CommandNeedKey::Payload))),
                        }
                    },
                    _ => return Err(ErrorActive::CommandParser(CommandParser::UnknownCommand)),
                }
            },
            None => return Err(ErrorActive::CommandParser(CommandParser::NoCommand)),
        }
    }
}


fn process_verifier_and_signature (verifier_found: Option<VerKey>, signature_found: Option<Entry>, encryption: Encryption) -> Result<Crypto, ErrorActive> {
    match verifier_found {
        Some(VerKey::Hex(x)) => {
            let verifier_public_key = unhex::<Active>(&x, NotHexActive::InputPublicKey)?;
            let signature = get_needed_signature(signature_found)?;
            Ok(Crypto::Sufficient(into_sufficient(verifier_public_key, signature, encryption)?))
        },
        Some(VerKey::File(x)) => {
            let verifier_filename = format!("{}/{}", FOLDER, x);
            let verifier_public_key = match std::fs::read(&verifier_filename) {
                Ok(a) => a,
                Err(e) => return Err(ErrorActive::Input(InputActive::File(e))),
            };
            let signature = get_needed_signature(signature_found)?;
            Ok(Crypto::Sufficient(into_sufficient(verifier_public_key, signature, encryption)?))
        },
        Some(VerKey::Alice) => {
            if let Some(_) = signature_found {return Err(ErrorActive::CommandParser(CommandParser::Unexpected(CommandUnexpected::AliceSignature)))}
            Ok(Crypto::Alice(encryption))
        },
        None => return Err(ErrorActive::CommandParser(CommandParser::NeedKey(CommandNeedKey::Verifier))),
    }
}

fn get_needed_signature(signature_found: Option<Entry>) -> Result<Vec<u8>, ErrorActive> {
    match signature_found {
        Some(Entry::Hex(t)) => Ok(unhex::<Active>(&t, NotHexActive::InputSignature)?),
        Some(Entry::File(t)) => {
            let signature_filename = format!("{}/{}", FOLDER, t);
            match std::fs::read(&signature_filename) {
                Ok(a) => Ok(a),
                Err(e) => return Err(ErrorActive::Input(InputActive::File(e))),
            }
        },
        None => return Err(ErrorActive::CommandParser(CommandParser::NeedKey(CommandNeedKey::Signature))),
    }
}

fn into_sufficient (verifier_public_key: Vec<u8>, signature: Vec<u8>, encryption: Encryption) -> Result<SufficientCrypto, ErrorActive> {
    match encryption {
        Encryption::Ed25519 => {
            let into_pubkey: [u8;32] = match verifier_public_key.try_into() {
                Ok(a) => a,
                Err(_) => return Err(ErrorActive::Input(InputActive::PublicKeyLength)),
            };
            let public = ed25519::Public::from_raw(into_pubkey);
            let into_sign: [u8;64] = match signature.try_into() {
                Ok(a) => a,
                Err(_) => return Err(ErrorActive::Input(InputActive::SignatureLength)),
            };
            let signature = ed25519::Signature::from_raw(into_sign);
            Ok(SufficientCrypto::Ed25519{public, signature})
        },
        Encryption::Sr25519 => {
            let into_pubkey: [u8;32] = match verifier_public_key.try_into() {
                Ok(a) => a,
                Err(_) => return Err(ErrorActive::Input(InputActive::PublicKeyLength)),
            };
            let public = sr25519::Public::from_raw(into_pubkey);
            let into_sign: [u8;64] = match signature.try_into() {
                Ok(a) => a,
                Err(_) => return Err(ErrorActive::Input(InputActive::SignatureLength)),
            };
            let signature = sr25519::Signature::from_raw(into_sign);
            Ok(SufficientCrypto::Sr25519{public, signature})
        },
        Encryption::Ecdsa => {
            let into_pubkey: [u8;33] = match verifier_public_key.try_into() {
                Ok(a) => a,
                Err(_) => return Err(ErrorActive::Input(InputActive::PublicKeyLength)),
            };
            let public = ecdsa::Public::from_raw(into_pubkey);
            let into_sign: [u8;65] = match signature.try_into() {
                Ok(a) => a,
                Err(_) => return Err(ErrorActive::Input(InputActive::SignatureLength)),
            };
            let signature = ecdsa::Signature::from_raw(into_sign);
            Ok(SufficientCrypto::Ecdsa{public, signature})
        },
    }
}
