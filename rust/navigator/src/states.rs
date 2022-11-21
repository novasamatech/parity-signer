use std::{collections::HashMap, path::Path};

use definitions::navigation::MAddressCard;
use transaction_parsing::TransactionAction;

use crate::{Error, Result};

const MAX_COUNT_SET: u8 = 3;

/// The result of a step within the signing protocol
/// between the user and the Signer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignResult {
    /// A password for one of the passworded keys is requested.
    RequestPassword { idx: usize, counter: u8 },

    /// All signatures are ready.
    Ready { signatures: Vec<Vec<u8>> },
}

/// State of transaction screen.
///
/// In general case Signer may sign a bulk of transactions
/// (more than one) and any subset of the transactions within
/// a bulk may be signed by the passworded keys. This structure
/// implements an interactive protocol between Signer and the user
/// where user repeatedly enters all necessary passwords until all
/// transactions are successfully signed or a password entry limit
/// occurs.
///
/// If the same passworded key is used to sign more than one
/// transaction in the bulk the password for this key is only
/// requested once.
///
/// In case when no transactions in the bulk are signed by passworded
/// key the result becomes available to the user right away minimizing
/// the amount of user actions necessary.
#[derive(Clone, Debug)]
pub struct TransactionState {
    /// The vector of seeds per-transcation being signed.
    /// Since it is possible that signer is signing more than
    /// one transaction (a bulk) this is a `Vec`.
    seeds: Vec<String>,

    /// Passwords for the accounts.
    passwords: HashMap<(String, String), String>,

    /// The `TransactionAction` being processed.
    action: TransactionAction,

    /// User-provided comments for each transaction.
    comments: Vec<String>,

    /// Failed password entries counter for the tx
    /// currently being signed if any.
    counter: u8,

    /// The index of the transaction being currently signed.
    currently_signing: usize,

    /// Accumulates already-produced signatures.
    signatures: Vec<Vec<u8>>,
}

impl TransactionState {
    pub fn current_password_author_info(&self) -> Option<MAddressCard> {
        match &self.action {
            TransactionAction::Sign { actions, .. } => {
                Some(actions[self.currently_signing].author_info.clone())
            }
            _ => None,
        }
    }

    pub fn new<P: AsRef<Path>>(details_str: &str, dbname: P) -> Self {
        Self {
            seeds: vec![],
            passwords: HashMap::new(),
            action: transaction_parsing::produce_output(details_str, dbname),
            comments: vec![],
            counter: 1,
            currently_signing: 0,
            signatures: vec![],
        }
    }

    pub fn update_seeds(&mut self, seeds: &str) {
        if self.seeds.is_empty() {
            self.seeds = seeds.lines().map(|seed| seed.to_string()).collect();
        }
    }

    pub fn update_comments(&mut self, comments: &str) {
        if self.comments.is_empty() {
            self.comments = comments
                .lines()
                .map(|comment| comment.to_string())
                .collect();
        }
    }

    pub fn update_checksum_sign(&mut self, new_checksum: u32) {
        if let TransactionAction::Sign {
            actions: _,
            checksum,
        } = &mut self.action
        {
            *checksum = new_checksum;
        }
    }

    pub fn action(&self) -> &TransactionAction {
        &self.action
    }

    pub fn handle_sign<P: AsRef<Path>>(&mut self, db_path: P) -> Result<SignResult> {
        if let TransactionAction::Sign { actions, checksum } = &mut self.action {
            if self.seeds.len() != actions.len() {
                return Err(Error::SeedsNumMismatch(self.seeds.len(), actions.len()));
            }

            loop {
                let action = &actions[self.currently_signing];
                if self.signatures.len() == actions.len() {
                    break;
                }
                let password = if action.has_pwd {
                    let seed = &self.seeds[self.currently_signing];
                    match self
                        .passwords
                        .get(&(seed.to_string(), action.author_info.base58.clone()))
                    {
                        Some(pwd) => pwd,
                        None => {
                            return Ok(SignResult::RequestPassword {
                                idx: self.currently_signing,
                                counter: self.counter,
                            });
                        }
                    }
                } else {
                    ""
                };

                match transaction_signing::create_signature(
                    &self.seeds[self.currently_signing],
                    password,
                    self.comments
                        .get(self.currently_signing)
                        .map(|s| s.as_str())
                        .unwrap_or_else(|| ""),
                    &db_path,
                    *checksum,
                    self.currently_signing,
                    action.network_info.specs.encryption,
                ) {
                    Ok((signature, new_checksum)) => {
                        self.currently_signing += 1;
                        self.counter = 1;
                        *checksum = new_checksum;
                        self.signatures.push(hex::decode(signature)?);
                        if self.currently_signing == self.seeds.len() {
                            break;
                        }
                    }
                    Err(e) => {
                        if let transaction_signing::Error::WrongPasswordNewChecksum(new_checksum) =
                            e
                        {
                            self.update_checksum_sign(new_checksum);
                            self.counter += 1;

                            return Ok(SignResult::RequestPassword {
                                idx: self.currently_signing,
                                counter: self.counter,
                            });
                        } else {
                            return Err(Error::TransactionSigning(e));
                        }
                    }
                }
            }
        } else {
            return Err(Error::TxActionNotSign);
        }

        Ok(SignResult::Ready {
            signatures: self.signatures.clone(),
        })
    }

    pub fn password_entered(&mut self, pwd: &str) {
        if let TransactionAction::Sign { actions, .. } = &self.action {
            let current = &self.seeds[self.currently_signing];
            let base58 = actions[self.currently_signing].author_info.base58.clone();
            self.passwords
                .insert((current.to_string(), base58), pwd.to_string());
        }
    }

    pub fn get_comment(&self) -> String {
        String::new()
    }

    pub fn ok(&self) -> bool {
        self.counter < MAX_COUNT_SET
    }

    pub fn counter(&self) -> u8 {
        self.counter
    }
}
