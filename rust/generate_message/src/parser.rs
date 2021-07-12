use std::env;
use definitions::constants::FOLDER;

/// Expected typical run commands:
/// `$ cargo run show -database`
/// `$ cargo run show -address_book`
/// `$ cargo run load -n -s polkadot westend`
/// `$ cargo run load -d -n polkadot`
/// `$ cargo run -p -a`
/// `$ cargo run add -t -u wss://westend-rpc.polkadot.io`


/// Enum to describe the incoming command contents
pub enum Command {
    Show(Show),
    Types,
    Load(Instruction),
    Add(Instruction),
    Make(Make),
}

pub enum Show {
    Database,
    AddressBook,
}

pub struct Instruction {
    pub set: Set,
    pub content: Content,
    pub pass_errors: bool,
}

pub enum Content {
    All,
    Name(Vec<String>),
    Address(Vec<String>),
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
    AddNetwork(Vec<u8>),
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
    AddNetwork,
}

enum VerKey {
    Hex(String),
    File(String),
    Alice,
}

enum Sign {
    Hex(String),
    File(String),
}

impl Command {
    /// FUnction to interpret command line input
    pub fn new(mut args: env::Args) -> Result<Command, &'static str> {
        args.next();

        match args.next() {
            Some(arg) => {
                match arg.as_str() {
                    "show" => {
                        match args.next() {
                            Some(show) => match show.as_str() {
                                "-database" => Ok(Command::Show(Show::Database)),
                                "-address_book" => Ok(Command::Show(Show::AddressBook)),
                                _ => return Err("Requested show command is not supported."),
                            },
                            None => return Err("Show command requires argument.")
                        }
                    },
                    "types" => Ok(Command::Types),
                    "load"|"add" => {
                        let mut set_key = None;
                        let mut content_key = None;
                        let mut pass_errors = true;
                        let mut names: Vec<String> = Vec::new();
                        
                        loop {
                            match args.next() {
                                Some(x) => {
                                    if x.starts_with("-") {
                                        match x.as_str() {
                                            "-a"|"-n"|"-u" => {
                                                match content_key {
                                                    Some(_) => {return Err("Only one content key allowed: -a, -n or -u.")},
                                                    None => {content_key = Some(x)}
                                                }
                                            },
                                            "-d"|"-f"|"-k"|"-p"|"-t" => {
                                                match set_key {
                                                    Some(_) => {return Err("Maximum one setting key allowed: -d, -f, -k, -p, -t.")},
                                                    None => {set_key = Some(x)}
                                                }
                                            },
                                            "-s" => {pass_errors = false},
                                            _ => return Err("Unknown key.")
                                        }
                                    }
                                    else {
                                    // name or address are recorded only once
                                        if !names.contains(&x) {names.push(x)}
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
                                    _ => return Err("Unexpected set key.")
                                }
                            },
                            None => Set::T,
                        };
                        
                        let content = match content_key {
                            Some(x) => {
                                match x.as_str() {
                                    "-a" => {
                                        if names.len() != 0 {return Err("Key -a is used to process all, name was not expected.")}
                                        else {Content::All}
                                    },
                                    "-n" => {
                                        if names.len() == 0 {return Err("Expected to get some network names with key -n.")}
                                        else {Content::Name(names)}
                                    },
                                    "-u" => {
                                        if names.len() == 0 {return Err("Expected to get some url addresses with key -u.")}
                                        else {
                                            if let Set::F = set {return Err("Could not process url addresses without rpc queries.")}
                                            else {Content::Address(names)}
                                        }
                                    },
                                    _ => return Err("Unexpected content key.")
                                }
                            },
                            None => {return Err("Expected some content key.")}
                        };
                        
                        let instruction = Instruction {
                            set,
                            content,
                            pass_errors,
                        };
                        
                        if arg == "load" {Ok(Command::Load(instruction))}
                        else {Ok(Command::Add(instruction))}
                    },
                    "make" => {
                        let mut goal = Goal::Both; // default option for `make`
                        let mut args = args.peekable();
                        match args.peek() {
                            Some(x) => {
                                match x.as_str() {
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
                            None => {return Err("Not enough arguments.")}
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
                                    match x.as_str() {
                                        "-crypto" => {
                                            match args.next() {
                                                Some(x) => {
                                                    match x.as_str() {
                                                        "ed25519" => {
                                                            crypto_type_found = match crypto_type_found {
                                                                Some(_) => {return Err("`-crypto` key could be used only once.")},
                                                                None => Some(CryptoType::Ed25519),
                                                            };
                                                        },
                                                        "sr25519" => {
                                                            crypto_type_found = match crypto_type_found {
                                                                Some(_) => {return Err("`-crypto` key could be used only once.")},
                                                                None => Some(CryptoType::Sr25519),
                                                            };
                                                        },
                                                        "ecdsa" => {
                                                            crypto_type_found = match crypto_type_found {
                                                                Some(_) => {return Err("`-crypto` key could be used only once.")},
                                                                None => Some(CryptoType::Ecdsa),
                                                            };
                                                        },
                                                        "none" => {
                                                            crypto_type_found = match crypto_type_found {
                                                                Some(_) => {return Err("`-crypto` key could be used only once.")},
                                                                None => Some(CryptoType::None),
                                                            };
                                                        },
                                                        _ => {return Err("Invalid `-crypto` key argument.")}
                                                    }
                                                },
                                                None => {return Err("`-crypto` key must be followed by an argument.")},
                                            }
                                        },
                                        "-msgtype" => {
                                            match args.next() {
                                                Some(x) => {
                                                    match x.as_str() {
                                                        "load_types" => {
                                                            msg_type_found = match msg_type_found {
                                                                Some(_) => {return Err("`-msgtype` key could be used only once.")},
                                                                None => Some(MsgType::LoadTypes),
                                                            };
                                                        },
                                                        "load_metadata" => {
                                                            msg_type_found = match msg_type_found {
                                                                Some(_) => {return Err("`-msgtype` key could be used only once.")},
                                                                None => Some(MsgType::LoadMetadata),
                                                            };
                                                        },
                                                        "add_network" => {
                                                            msg_type_found = match msg_type_found {
                                                                Some(_) => {return Err("`-msgtype` key could be used only once.")},
                                                                None => Some(MsgType::AddNetwork),
                                                            };
                                                        },
                                                        _ => {return Err("Invalid `-msgtype` key argument.")}
                                                    }
                                                },
                                                None => {return Err("`-msgtype` key must be followed by an argument.")},
                                            }
                                        },
                                        "-verifier" => {
                                            match args.next() {
                                                Some(x) => {
                                                    match x.as_str() {
                                                        "-hex" => {
                                                            match args.next() {
                                                                Some(h) => {
                                                                    verifier_found = match verifier_found {
                                                                        Some(_) => {return Err("`-verifier` key could be used maximum once.")},
                                                                        None => Some(VerKey::Hex(h.to_string())),
                                                                    };
                                                                },
                                                            None => {return Err("Expected hex line for verifier after -verifier -hex sequence.")},
                                                            }
                                                        },
                                                        "-file" => {
                                                            match args.next() {
                                                                Some(f) => {
                                                                    verifier_found = match verifier_found {
                                                                        Some(_) => {return Err("`-verifier` key could be used maximum once.")},
                                                                        None => Some(VerKey::File(f.to_string())),
                                                                    };
                                                                },
                                                                None => {return Err("Expected file name for verifier after -verifier -file sequence.")},
                                                            }
                                                        },
                                                        "Alice" => {
                                                        // test verifier Alice
                                                            verifier_found = match verifier_found {
                                                                Some(_) => {return Err("`-verifier` key could be used maximum once.")},
                                                                None => Some(VerKey::Alice),
                                                            };
                                                        },
                                                        _ => {return Err("Invalid `-verifier` key argument.")},
                                                    }
                                                },
                                                None => {return Err("`-verifier` key must be followed by an argument.")},
                                            }
                                        },
                                        "-payload" => {
                                            match args.next() {
                                                Some(x) => {
                                                // interpret as filename with payload
                                                    payload_found = match payload_found {
                                                        Some(_) => {return Err("`-payload` key could be used only once.")},
                                                        None => Some(x.to_string()),
                                                    };
                                                },
                                                None => {return Err("`-payload` key must be followed by an argument.")},
                                            }
                                        },
                                        "-signature" => {
                                            match args.next() {
                                                Some(x) => {
                                                    match x.as_str() {
                                                        "-hex" => {
                                                            match args.next() {
                                                                Some(h) => {
                                                                    signature_found = match signature_found {
                                                                        Some(_) => {return Err("`-signature` key could be used maximum once.")},
                                                                        None => Some(Sign::Hex(h.to_string())),
                                                                    };
                                                                },
                                                                None => {return Err("Expected hex line for signature after -signature -hex sequence.")},
                                                            }
                                                        },
                                                        "-file" => {
                                                            match args.next() {
                                                                Some(f) => {
                                                                    signature_found = match signature_found {
                                                                        Some(_) => {return Err("`-signature` key could be used maximum once.")},
                                                                        None => Some(Sign::File(f.to_string())),
                                                                    };
                                                                },
                                                                None => {return Err("Expected file namefor signature after -signature -file sequence.")},
                                                            }
                                                        },
                                                        _ => {return Err("Invalid `-signature` key argument.")},
                                                    }
                                                },
                                                None => {return Err("`-signature` key must be followed by an argument.")},
                                            }
                                        },
                                        "-name" => {
                                            match args.next() {
                                                Some(x) => {
                                                // interpret as custom filename for export
                                                    name = match name {
                                                        Some(_) => {return Err("`-name` key could be used maximum once.")},
                                                        None => Some(x.to_string()),
                                                    };
                                                },
                                                None => {return Err("`-name` key must be followed by an argument.")},
                                            }
                                        },
                                        _ => {return Err("Unexpected key and argument sequence.")},
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
                                        if let Some(_) = verifier_found {return Err("No verifier was expected unverified message.")}
                                        if let Some(_) = signature_found {return Err("No signature was expected unverified message.")}
                                        Crypto::None
                                    },
                                }
                            },
                            None => {return Err("`-crypto` key must have been used.")},
                        };
                        let payload = match payload_found {
                            Some(x) => {
                                let filename = format!("{}/{}", FOLDER, x);
                                match std::fs::read(&filename) {
                                    Ok(a) => a,
                                    Err(_) => {return Err("Error reading payload file.")} 
                                }
                            },
                            None => {return Err("`-payload` key must have been used.")},
                        };
                        let msg = match msg_type_found {
                            Some(x) => {
                                match x {
                                    MsgType::LoadTypes => Msg::LoadTypes(payload),
                                    MsgType::LoadMetadata => Msg::LoadMetadata(payload),
                                    MsgType::AddNetwork => Msg::AddNetwork(payload),
                                }
                            },
                            None => {return Err("`-msgtype` key must have been used.")},
                        };
                        let make = Make {
                            goal,
                            crypto,
                            msg,
                            name,
                        };
                        Ok(Command::Make(make))
                    },
                    _ => return Err("Command type is not supported."),
                }
            },
            None => return Err("Didn't get any command."),
        }
    }
}


fn process_verifier_and_signature (verifier_found: Option<VerKey>, signature_found: Option<Sign>) -> Result<VerifierKind, &'static str> {
    
    match verifier_found {
        Some(VerKey::Hex(x)) => {
            let hex_key = {
                if x.starts_with("0x") {x[2..].to_string()}
                else {x}
            };
            let verifier_public_key = match hex::decode(&hex_key) {
                Ok(a) => a,
                Err(_) => {return Err("Error decoding hex public key line.")} 
            };
            let signature = match signature_found {
                Some(Sign::Hex(t)) => {
                    let hex_signature = {
                        if t.starts_with("0x") {t[2..].to_string()}
                        else {t}
                    };
                    match hex::decode(&hex_signature) {
                        Ok(a) => a,
                        Err(_) => {return Err("Error decoding hex signature line.")} 
                    }
                },
                Some(Sign::File(t)) => {
                    let signature_filename = format!("{}/{}", FOLDER, t);
                    match std::fs::read(&signature_filename) {
                        Ok(a) => a,
                        Err(_) => {return Err("Error reading verifier public key file.")},
                    }
                },
                None => {return Err("`-signature` key must have been used.")},
            };
            Ok(VerifierKind::Normal{verifier_public_key, signature})
        },
        Some(VerKey::File(x)) => {
            let verifier_filename = format!("{}/{}", FOLDER, x);
            let verifier_public_key = match std::fs::read(&verifier_filename) {
                Ok(a) => a,
                Err(_) => {return Err("Error reading verifier public key file.")},
            };
            let signature = match signature_found {
                Some(Sign::Hex(t)) => {
                    let hex_signature = {
                        if t.starts_with("0x") {t[2..].to_string()}
                        else {t}
                    };
                    match hex::decode(&hex_signature) {
                        Ok(a) => a,
                        Err(_) => {return Err("Error decoding hex signature line.")} 
                    }
                },
                Some(Sign::File(t)) => {
                    let signature_filename = format!("{}/{}", FOLDER, t);
                    match std::fs::read(&signature_filename) {
                        Ok(a) => a,
                        Err(_) => {return Err("Error reading verifier public key file.")},
                    }
                },
                None => {return Err("`-signature` key must have been used.")},
            };
            Ok(VerifierKind::Normal{verifier_public_key, signature})
        },
        Some(VerKey::Alice) => {
            if let Some(_) = signature_found {return Err("No signature was expected for verifier Alice.")}
            Ok(VerifierKind::Alice)
        },
        None => {return Err("`-verifier` key must have been used.")},
    }
    
}
