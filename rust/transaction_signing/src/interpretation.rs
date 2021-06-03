use hex;
use regex::Regex;
use lazy_static::lazy_static;

// struct to store action line information
pub struct Action <'a> {
    pub crypto: &'a str,
    pub path: &'a str,
    pub seed: &'a str,
    pub transaction: Vec<u8>,
    pub has_pwd: bool,
}

// Making lazy statics for regex interpreting input action string

lazy_static! {
    static ref REG_CRYPTO: Regex = Regex::new(r#"(?i)"crypto":( )*"(?P<crypto>[a-z0-9]*)""#).unwrap();
    static ref REG_PATH: Regex = Regex::new(r#"(?i)"derivation_path":( )*"(?P<path>.*?)""#).unwrap();
    static ref REG_SEED: Regex = Regex::new(r#"(?i)"encrypted_seed":( )*(?P<seed>.*?"\{.*?\}")"#).unwrap();
    static ref REG_TRANSACTION: Regex = Regex::new(r#"(?i)"transaction":( )*"(?P<transaction>([a-z0-9][a-z0-9])*)""#).unwrap();
    static ref REG_HASPWD: Regex = Regex::new(r#"(?i)"has_password":( )*"(?P<has_pwd>(true|false))""#).unwrap();
}

pub fn get_info <'a> (action_line: &'a str) -> Result<Action, &'static str> {
    let path = match REG_PATH.captures(&action_line) {
        Some(caps) => {
            match caps.name("path") {
                Some(c) => c.as_str(),
                None => {return Err("No derivation path found. Wrong action line formatting.")}
            }
        },
        None => {return Err("No derivation path found. Wrong action line formatting.")},
    };
    let seed = match REG_SEED.captures(&action_line) {
        Some(caps) => {
            match caps.name("seed") {
                Some(c) => c.as_str(),
                None => {return Err("No encrypted seed found. Wrong action line formatting.")}
            }
        },
        None => {return Err("No encrypted seed found. Wrong action line formatting.")},
    };
    let has_pwd: bool = match REG_HASPWD.captures(&action_line) {
        Some(caps) => {
            match caps.name("has_pwd") {
                Some(c) => c.as_str().parse().expect("Should have found only bool values by regex."),
                None => {return Err("No has_password field found. Wrong action line formatting.")},
            }
        },
        None => {return Err("No has_password field found. Wrong action line formatting.")},
    };
    let transaction = match REG_TRANSACTION.captures(&action_line) {
        Some(caps) => {
            match caps.name("transaction") {
                Some(c) => {
                    let tr_hex = c.as_str();
                    hex::decode(&tr_hex).expect("Only hex decodeable line should be found by regex.")
                },
                None => {return Err("No transaction found. Wrong action line formatting.")},
            }
        },
        None => {return Err("No transaction found. Wrong action line formatting.")},
    };
    let crypto = match REG_CRYPTO.captures(&action_line) {
        Some(caps) => {
            match caps.name("crypto") {
                Some(c) => c.as_str(),
                None => {return Err("No encryption method found. Wrong action line formatting.")},
            }
        },
        None => {return Err("No encryption method found. Wrong action line formatting.")},
    };
    Ok(Action{
        crypto,
        path,
        seed,
        transaction,
        has_pwd,
    })
}
