use std::env;
use constants::FOLDER;
use definitions::crypto::{Encryption, SufficientCrypto};
use parity_scale_codec::Decode;
use anyhow;
use db_handling::{helpers::unhex, error::NotHex};

use crate::error::{Error, NotDecodeable, NeedArgument, DoubleKey, NeedKey, BadArgument, Unexpected};

/// Expected typical run commands:
/// `$ cargo run show database`
/// `$ cargo run show address_book`
/// `$ cargo run load_metadata -n westend`
/// `$ cargo run add_specs -d -n -ed25519 westend`
/// `$ cargo run add_network -u wss://unknown-network.eu -ecdsa`


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
    MakeColdRelease,
    TransferMetaRelease,
}

pub enum Show {
    Database,
    AddressBook,
}

pub struct Instruction {
    pub set: Set,
    pub content: Content,
    pub pass_errors: bool,
    pub encryption_override: Option<Encryption>,
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
    Ed25519 (VerifierKind),
    Sr25519 (VerifierKind),
    Ecdsa (VerifierKind),
    None,
}

pub enum VerifierKind {
    Alice,
    Normal {verifier_public_key: Vec<u8>, signature: Vec<u8>},
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

impl Command {
    /// FUnction to interpret command line input
    pub fn new(mut args: env::Args) -> anyhow::Result<Command> {
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
                                _ => {return Err(Error::UnexpectedKeyArgumentSequence.show())},
                            },
                            None => return Err(Error::NeedKey(NeedKey::Show).show())
                        }
                    },
                    "load_types" => Ok(Command::Types),
                    "load_metadata"|"add_specs" => {
                        let mut set_key = None;
                        let mut content_key = None;
                        let mut pass_errors = true;
                        let mut name = None;
                        let mut encryption_override_key = None;
                        
                        loop {
                            match args.next() {
                                Some(x) => {
                                    let x = x.to_lowercase();
                                    if x.starts_with("-") {
                                        match x.as_str() {
                                            "-a"|"-n"|"-u" => {
                                                match content_key {
                                                    Some(_) => {return Err(Error::DoubleKey(DoubleKey::Content).show())},
                                                    None => {content_key = Some(x)}
                                                }
                                            },
                                            "-d"|"-f"|"-k"|"-p"|"-t" => {
                                                match set_key {
                                                    Some(_) => {return Err(Error::DoubleKey(DoubleKey::Set).show())},
                                                    None => {set_key = Some(x)}
                                                }
                                            },
                                            "-s" => {pass_errors = false},
                                            "-ed25519"|"-sr25519"|"-ecdsa" => {
                                                match encryption_override_key {
                                                    Some(_) =>  {return Err(Error::DoubleKey(DoubleKey::CryptoOverride).show())},
                                                    None => {encryption_override_key = Some(x)}
                                                }
                                            },
                                            _ => {return Err(Error::UnexpectedKeyArgumentSequence.show())},
                                        }
                                    }
                                    else {
                                        match name {
                                            Some(_) => return Err(Error::OnlyOneNetworkId.show()),
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
                        
                        let encryption_override = match encryption_override_key {
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
                        
                        let content = match content_key {
                            Some(x) => {
                                match x.as_str() {
                                    "-a" => {
                                        if let Some(_) = name {return Err(Error::Unexpected(Unexpected::KeyAContent).show())}
                                        Content::All
                                    },
                                    "-n" => {
                                        match name {
                                            Some(n) => Content::Name(n),
                                            None => return Err(Error::NeedArgument(NeedArgument::NetworkName).show())
                                        }
                                    },
                                    "-u" => {
                                        match name {
                                            Some(a) => Content::Address(a),
                                            None => return Err(Error::NeedArgument(NeedArgument::NetworkUrl).show())
                                        }
                                    },
                                    _ => unreachable!(),
                                }
                            },
                            None => {return Err(Error::NeedKey(NeedKey::Content).show())}
                        };
                        
                        let instruction = Instruction {
                            set,
                            content,
                            pass_errors,
                            encryption_override,
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
                            None => {return Err(Error::NeedArgument(NeedArgument::Make).show())}
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
                                            if let Some(_) = crypto_type_found {return Err(Error::DoubleKey(DoubleKey::CryptoKey).show())}
                                            crypto_type_found = match args.next() {
                                                Some(x) => {
                                                    match x.to_lowercase().as_str() {
                                                        "ed25519" => Some(CryptoType::Ed25519),
                                                        "sr25519" => Some(CryptoType::Sr25519),
                                                        "ecdsa" => Some(CryptoType::Ecdsa),
                                                        "none" => Some(CryptoType::None),
                                                        _ => {return Err(Error::BadArgument(BadArgument::CryptoKey).show())}
                                                    }
                                                },
                                                None => {return Err(Error::NeedArgument(NeedArgument::CryptoKey).show())},
                                            };
                                        },
                                        "-msgtype" => {
                                            if let Some(_) = msg_type_found {return Err(Error::DoubleKey(DoubleKey::MsgType).show())}
                                            msg_type_found = match args.next() {
                                                Some(x) => {
                                                    match x.to_lowercase().as_str() {
                                                        "load_types" => Some(MsgType::LoadTypes),
                                                        "load_metadata" => Some(MsgType::LoadMetadata),
                                                        "add_specs" => Some(MsgType::AddSpecs),
                                                        _ => {return Err(Error::BadArgument(BadArgument::MsgType).show())}
                                                    }
                                                },
                                                None => {return Err(Error::NeedArgument(NeedArgument::MsgType).show())},
                                            };
                                        },
                                        "-verifier" => {
                                            if let Some(_) = verifier_found {return Err(Error::DoubleKey(DoubleKey::Verifier).show())}
                                            verifier_found = match args.next() {
                                                Some(x) => {
                                                    match x.to_lowercase().as_str() {
                                                        "-hex" => {
                                                            match args.next() {
                                                                Some(h) => Some(VerKey::Hex(h.to_string())),
                                                                None => {return Err(Error::NeedArgument(NeedArgument::VerifierHex).show())},
                                                            }
                                                        },
                                                        "-file" => {
                                                            match args.next() {
                                                                Some(f) => Some(VerKey::File(f.to_string())),
                                                                None => {return Err(Error::NeedArgument(NeedArgument::VerifierFile).show())},
                                                            }
                                                        },
                                                        "alice" => Some(VerKey::Alice),
                                                        _ => {return Err(Error::BadArgument(BadArgument::Verifier).show())}
                                                    }
                                                },
                                                None => {return Err(Error::NeedArgument(NeedArgument::Verifier).show())},
                                            };
                                        },
                                        "-payload" => {
                                            if let Some(_) = payload_found {return Err(Error::DoubleKey(DoubleKey::Payload).show())}
                                            payload_found = match args.next() {
                                                Some(x) => Some(x.to_string()),
                                                None => {return Err(Error::NeedArgument(NeedArgument::Payload).show())},
                                            };
                                        },
                                        "-signature" => {
                                            if let Some(_) = signature_found {return Err(Error::DoubleKey(DoubleKey::Signature).show())}
                                            signature_found = match args.next() {
                                                Some(x) => {
                                                    match x.as_str() {
                                                        "-hex" => {
                                                            match args.next() {
                                                                Some(h) => Some(Entry::Hex(h.to_string())),
                                                                None => {return Err(Error::NeedArgument(NeedArgument::SignatureHex).show())},
                                                            }
                                                        },
                                                        "-file" => {
                                                            match args.next() {
                                                                Some(f) => Some(Entry::File(f.to_string())),
                                                                None => {return Err(Error::NeedArgument(NeedArgument::SignatureFile).show())},
                                                            }
                                                        },
                                                        _ => {return Err(Error::BadArgument(BadArgument::Signature).show())}
                                                    }
                                                },
                                                None => {return Err(Error::NeedArgument(NeedArgument::Signature).show())},
                                            }
                                        },
                                        "-name" => {
                                            if let Some(_) = name {return Err(Error::DoubleKey(DoubleKey::Name).show())}
                                            name = match args.next() {
                                                Some(x) => Some(x.to_string()),
                                                None => {return Err(Error::NeedArgument(NeedArgument::Name).show())},
                                            };
                                        },
                                        _ => {return Err(Error::UnexpectedKeyArgumentSequence.show())},
                                    }
                                },
                                None => break,
                            }
                        }
                    // finalize what was parsed for `make`
                        let crypto = match crypto_type_found {
                            Some(x) => {
                                match x {
                                    CryptoType::Ed25519 => Crypto::Ed25519(process_verifier_and_signature (verifier_found, signature_found)?),
                                    CryptoType::Sr25519 => Crypto::Sr25519(process_verifier_and_signature (verifier_found, signature_found)?),
                                    CryptoType::Ecdsa => Crypto::Ecdsa(process_verifier_and_signature (verifier_found, signature_found)?),
                                    CryptoType::None => {
                                        if let Some(_) = verifier_found {return Err(Error::Unexpected(Unexpected::VerifierNoCrypto).show())}
                                        if let Some(_) = signature_found {return Err(Error::Unexpected(Unexpected::SignatureNoCrypto).show())}
                                        Crypto::None
                                    },
                                }
                            },
                            None => {return Err(Error::NeedKey(NeedKey::Crypto).show())},
                        };
                        let payload = match payload_found {
                            Some(x) => {
                                let filename = format!("{}/{}", FOLDER, x);
                                match std::fs::read(&filename) {
                                    Ok(a) => a,
                                    Err(e) => {return Err(Error::InputOutputError(e.to_string()).show())} 
                                }
                            },
                            None => {return Err(Error::NeedKey(NeedKey::Payload).show())},
                        };
                        let msg = match msg_type_found {
                            Some(x) => {
                                match x {
                                    MsgType::LoadTypes => Msg::LoadTypes(payload),
                                    MsgType::LoadMetadata => Msg::LoadMetadata(payload),
                                    MsgType::AddSpecs => Msg::AddSpecs(payload),
                                }
                            },
                            None => {return Err(Error::NeedKey(NeedKey::MsgType).show())},
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
                            None => {return Err(Error::NeedArgument(NeedArgument::Sign).show())}
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
                                            if let Some(_) = sufficient_crypto_found {return Err(Error::DoubleKey(DoubleKey::SufficientCrypto).show())}
                                            sufficient_crypto_found = match args.next() {
                                                Some(x) => {
                                                    match x.to_lowercase().as_str() {
                                                        "-hex" => {
                                                            match args.next() {
                                                                Some(h) => Some(Entry::Hex(h.to_string())),
                                                                None => {return Err(Error::NeedArgument(NeedArgument::SufficientCryptoHex).show())},
                                                            }
                                                        },
                                                        "-file" => {
                                                            match args.next() {
                                                                Some(f) => Some(Entry::File(f.to_string())),
                                                                None => {return Err(Error::NeedArgument(NeedArgument::SufficientCryptoFile).show())},
                                                            }
                                                        },
                                                        _ => {return Err(Error::BadArgument(BadArgument::SufficientCrypto).show())}
                                                    }
                                                },
                                                None => {return Err(Error::NeedArgument(NeedArgument::SufficientCrypto).show())},
                                            };
                                        },
                                        "-msgtype" => {
                                            if let Some(_) = msg_type_found {return Err(Error::DoubleKey(DoubleKey::MsgType).show())}
                                            msg_type_found = match args.next() {
                                                Some(x) => {
                                                    match x.to_lowercase().as_str() {
                                                        "load_types" => Some(MsgType::LoadTypes),
                                                        "load_metadata" => Some(MsgType::LoadMetadata),
                                                        "add_specs" => Some(MsgType::AddSpecs),
                                                        _ => {return Err(Error::BadArgument(BadArgument::MsgType).show())}
                                                    }
                                                },
                                                None => {return Err(Error::NeedArgument(NeedArgument::MsgType).show())},
                                            };
                                        },
                                        "-payload" => {
                                            if let Some(_) = payload_found {return Err(Error::DoubleKey(DoubleKey::Payload).show())}
                                            payload_found = match args.next() {
                                                Some(x) => Some(x.to_string()),
                                                None => {return Err(Error::NeedArgument(NeedArgument::Payload).show())},
                                            };
                                        },
                                        "-name" => {
                                            if let Some(_) = name {return Err(Error::DoubleKey(DoubleKey::Name).show())}
                                            name = match args.next() {
                                                Some(x) => Some(x.to_string()),
                                                None => {return Err(Error::NeedArgument(NeedArgument::Name).show())},
                                            };
                                        },
                                        _ => {return Err(Error::UnexpectedKeyArgumentSequence.show())},
                                    }
                                },
                                None => break,
                            }
                        }
                    // finalize command parsed with "sign"
                        let crypto = match sufficient_crypto_found {
                            Some(x) => {
                                let sufficient_crypto_vector = match x {
                                    Entry::Hex(h) => unhex(&h, NotHex::SufficientCrypto)?,
                                    Entry::File(f) => {
                                        let filename = format!("{}/{}", FOLDER, f);
                                        match std::fs::read(&filename) {
                                            Ok(a) => a,
                                            Err(e) => {return Err(Error::InputOutputError(e.to_string()).show())},
                                        }
                                    },
                                };
                                let sufficient_crypto = match <SufficientCrypto>::decode(&mut &sufficient_crypto_vector[..]) {
                                    Ok(a) => a,
                                    Err(_) => {return Err(Error::NotDecodeable(NotDecodeable::SufficientCrypto).show())},
                                };
                                match sufficient_crypto {
                                    SufficientCrypto::Ed25519 {public_key, signature} => {
                                        Crypto::Ed25519(VerifierKind::Normal {verifier_public_key: public_key.to_vec(), signature: signature.0.to_vec()})
                                    },
                                    SufficientCrypto::Sr25519 {public_key, signature} => {
                                        Crypto::Sr25519(VerifierKind::Normal {verifier_public_key: public_key.to_vec(), signature: signature.0.to_vec()})
                                    },
                                    SufficientCrypto::Ecdsa {public_key, signature} => {
                                        Crypto::Ecdsa(VerifierKind::Normal {verifier_public_key: public_key.0.to_vec(), signature: signature.0.to_vec()})
                                    },
                                }
                            },
                            None => {return Err(Error::NeedKey(NeedKey::SufficientCrypto).show())},
                        };
                        let payload = match payload_found {
                            Some(x) => {
                                let filename = format!("{}/{}", FOLDER, x);
                                match std::fs::read(&filename) {
                                    Ok(a) => a,
                                    Err(e) => {return Err(Error::InputOutputError(e.to_string()).show())},
                                }
                            },
                            None => {return Err(Error::NeedKey(NeedKey::Payload).show())},
                        };
                        let msg = match msg_type_found {
                            Some(x) => {
                                match x {
                                    MsgType::LoadTypes => Msg::LoadTypes(payload),
                                    MsgType::LoadMetadata => Msg::LoadMetadata(payload),
                                    MsgType::AddSpecs => Msg::AddSpecs(payload),
                                }
                            },
                            None => {return Err(Error::NeedKey(NeedKey::MsgType).show())},
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
                                            if let Some(_) = info_found {return Err(Error::DoubleKey(DoubleKey::Remove).show())}
                                            info_found = match args.next() {
                                                Some(b) => Some(Remove::Title(b.to_string())),
                                                None => {return Err(Error::NeedArgument(NeedArgument::RemoveTitle).show())},
                                            };
                                        },
                                        "-name" => {
                                            if let Some(_) = info_found {return Err(Error::DoubleKey(DoubleKey::Remove).show())}
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
                                                                                Err(_) => {return Err(Error::Unexpected(Unexpected::VersionFormat).show())},
                                                                            }
                                                                        },
                                                                        None => {return Err(Error::NeedArgument(NeedArgument::RemoveVersion).show())}
                                                                    }
                                                                }
                                                                _ => {return Err(Error::UnexpectedKeyArgumentSequence.show())},
                                                            }
                                                        },
                                                        None => {return Err(Error::NeedKey(NeedKey::RemoveVersion).show())},
                                                    }
                                                },
                                                None => {return Err(Error::NeedArgument(NeedArgument::RemoveName).show())},
                                            };
                                        },
                                        _ => {return Err(Error::UnexpectedKeyArgumentSequence.show())},
                                    }
                                },
                                None => break,
                            }
                        }
                        match info_found {
                            Some(x) => Ok(Command::Remove(x)),
                            None => {return Err(Error::NeedKey(NeedKey::Remove).show())}
                        }                        
                    },
                    "restore_defaults" => Ok(Command::RestoreDefaults),
                    "make_cold_with_identities" => Ok(Command::MakeColdWithIdentities),
                    "transfer_meta_to_cold" => Ok(Command::TransferMeta),
                    "make_cold_release" => Ok(Command::MakeColdRelease),
                    "transfer_meta_to_cold_release" => Ok(Command::TransferMetaRelease),
                    _ => return Err(Error::UnknownCommand.show()),
                }
            },
            None => return Err(Error::NoCommand.show()),
        }
    }
}


fn process_verifier_and_signature (verifier_found: Option<VerKey>, signature_found: Option<Entry>) -> anyhow::Result<VerifierKind> {
    
    match verifier_found {
        Some(VerKey::Hex(x)) => {
            let verifier_public_key = unhex(&x, NotHex::PublicKey)?;
            let signature = match signature_found {
                Some(Entry::Hex(t)) => unhex(&t, NotHex::Signature)?,
                Some(Entry::File(t)) => {
                    let signature_filename = format!("{}/{}", FOLDER, t);
                    match std::fs::read(&signature_filename) {
                        Ok(a) => a,
                        Err(e) => {return Err(Error::InputOutputError(e.to_string()).show())},
                    }
                },
                None => {return Err(Error::NeedKey(NeedKey::Signature).show())},
            };
            Ok(VerifierKind::Normal{verifier_public_key, signature})
        },
        Some(VerKey::File(x)) => {
            let verifier_filename = format!("{}/{}", FOLDER, x);
            let verifier_public_key = match std::fs::read(&verifier_filename) {
                Ok(a) => a,
                Err(e) => {return Err(Error::InputOutputError(e.to_string()).show())},
            };
            let signature = match signature_found {
                Some(Entry::Hex(t)) => unhex(&t, NotHex::Signature)?,
                Some(Entry::File(t)) => {
                    let signature_filename = format!("{}/{}", FOLDER, t);
                    match std::fs::read(&signature_filename) {
                        Ok(a) => a,
                        Err(e) => {return Err(Error::InputOutputError(e.to_string()).show())},
                    }
                },
                None => {return Err(Error::NeedKey(NeedKey::Signature).show())},
            };
            Ok(VerifierKind::Normal{verifier_public_key, signature})
        },
        Some(VerKey::Alice) => {
            if let Some(_) = signature_found {return Err(Error::Unexpected(Unexpected::AliceSignature).show())}
            Ok(VerifierKind::Alice)
        },
        None => {return Err(Error::NeedKey(NeedKey::Verifier).show())},
    }
    
}
