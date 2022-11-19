use std::{collections::HashMap, path::Path};

use definitions::navigation::MAddressCard;

use crate::Result;

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
    action: transaction_parsing::TransactionAction,

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
            transaction_parsing::TransactionAction::Sign { actions, .. } => {
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

    pub fn update_seeds(&self, seeds: &str) -> Self {
        let mut new = self.clone();
        if new.seeds.is_empty() {
            new.seeds = seeds.lines().map(|seed| seed.to_string()).collect();
        }
        new
    }

    pub fn update_comments(&self, comments: &str) -> Self {
        let mut new = self.clone();
        if new.comments.is_empty() {
            new.comments = comments
                .lines()
                .map(|comment| comment.to_string())
                .collect();
        }
        new
    }

    pub fn update_checksum_sign(&self, new_checksum: u32) -> Self {
        let mut new = self.clone();
        new.counter += 1;

        if let transaction_parsing::TransactionAction::Sign {
            actions: _,
            checksum,
        } = &mut new.action
        {
            *checksum = new_checksum;
        }
        new
    }

    pub fn action(&self) -> &transaction_parsing::TransactionAction {
        &self.action
    }

    pub fn handle_sign<P: AsRef<Path>>(&self, db_path: P) -> Result<(SignResult, Self)> {
        let mut new = self.clone();

        if let transaction_parsing::TransactionAction::Sign { actions, checksum } = &mut new.action
        {
            if self.seeds.len() != actions.len() {
                return Err(crate::Error::SeedsNumMismatch(self.seeds.concat()));
            }

            loop {
                let action = &actions[new.currently_signing];
                if new.signatures.len() == actions.len() {
                    break;
                }
                let password = if action.has_pwd {
                    let seed = &self.seeds[new.currently_signing];
                    match self
                        .passwords
                        .get(&(seed.to_string(), action.author_info.base58.clone()))
                    {
                        Some(pwd) => pwd,
                        None => {
                            return Ok((
                                SignResult::RequestPassword {
                                    idx: new.currently_signing,
                                    counter: 1,
                                },
                                new,
                            ));
                        }
                    }
                } else {
                    ""
                };

                match transaction_signing::create_signature(
                    &new.seeds[new.currently_signing],
                    password,
                    new.comments
                        .get(new.currently_signing)
                        .map(|s| s.as_str())
                        .unwrap_or_else(|| ""),
                    &db_path,
                    *checksum,
                    new.currently_signing,
                    action.network_info.specs.encryption,
                ) {
                    Ok((signature, new_checksum)) => {
                        new.currently_signing += 1;
                        new.counter = 0;
                        *checksum = new_checksum;
                        new.signatures.push(hex::decode(signature)?);
                        if new.currently_signing == self.seeds.len() {
                            break;
                        }
                    }
                    Err(e) => {
                        if let transaction_signing::Error::WrongPasswordNewChecksum(new_checksum) =
                            e
                        {
                            new = new.update_checksum_sign(new_checksum);

                            return Ok((
                                SignResult::RequestPassword {
                                    idx: new.currently_signing,
                                    counter: new.counter,
                                },
                                new,
                            ));
                        } else {
                            return Err(crate::Error::TransactionSigning(e));
                        }
                    }
                }
            }
        } else {
            return Err(crate::Error::TxActionNotSign);
        }

        Ok((
            SignResult::Ready {
                signatures: new.signatures.clone(),
            },
            new,
        ))
    }

    pub fn password_entered(&self, pwd: &str) -> Self {
        let mut new = self.clone();

        if let transaction_parsing::TransactionAction::Sign {
            actions,
            checksum: _,
        } = &self.action
        {
            let current = &self.seeds[self.currently_signing];
            let base58 = actions[self.currently_signing].author_info.base58.clone();
            new.passwords
                .insert((current.to_string(), base58), pwd.to_string());
        }
        new
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
