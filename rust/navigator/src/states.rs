use std::collections::HashMap;

use definitions::{navigation::MAddressCard, network_specs::OrderedNetworkSpecs};
use sp_runtime::MultiSignature;
use transaction_parsing::{produce_output, TransactionAction};
use transaction_signing::{Error as SignError, SignatureType};

use crate::{Error, Result};

const MAX_COUNT_SET: u8 = 3;

/// The result of a step within the signing protocol
/// between the user and the Vault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignResult {
    /// A password for one of the passworded keys is requested.
    RequestPassword { idx: usize, counter: u8 },

    /// All signatures are ready.
    Ready {
        signatures: Vec<(MultiSignature, SignatureType)>,
    },
}

/// State of transaction screen.
///
/// In general case Vault may sign a bulk of transactions
/// (more than one) and any subset of the transactions within
/// a bulk may be signed by the passworded keys. This structure
/// implements an interactive protocol between Vault and the user
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

    /// Passwords for the accounts. Key is the ss58 addr.
    passwords: HashMap<String, String>,

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
    signatures: Vec<(MultiSignature, SignatureType)>,
}

impl TransactionState {
    pub fn current_password_author_info(
        &self,
    ) -> Option<(MAddressCard, Option<OrderedNetworkSpecs>)> {
        match &self.action {
            TransactionAction::Sign { actions, .. } => Some((
                actions[self.currently_signing].author_info.clone(),
                actions[self.currently_signing].get_network_spec(),
            )),
            _ => None,
        }
    }

    pub fn new(database: &sled::Db, details_str: &str) -> Result<Self> {
        Ok(Self {
            seeds: vec![],
            action: produce_output(database, details_str)?,
            counter: 1,
            passwords: HashMap::new(),
            comments: vec![],
            currently_signing: 0,
            signatures: vec![],
        })
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
        if let TransactionAction::Sign { checksum, .. } = &mut self.action {
            *checksum = new_checksum;
        }
    }

    pub fn action(&self) -> &TransactionAction {
        &self.action
    }

    /// Try to further progress the signing of transactions.
    pub fn handle_sign(&mut self, database: &sled::Db) -> Result<SignResult> {
        if let TransactionAction::Sign { actions, checksum } = &mut self.action {
            if self.seeds.len() != actions.len() {
                return Err(Error::SeedsNumMismatch(self.seeds.len(), actions.len()));
            }

            loop {
                // Get the tx currently being signed.
                let action = &actions[self.currently_signing];

                // Get the password; if there is none, request one.
                let password = if action.has_pwd {
                    match self.passwords.get(&action.author_info.base58) {
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

                // Try to sign it.
                match transaction_signing::create_signature(
                    database,
                    &self.seeds[self.currently_signing],
                    password,
                    self.comments
                        .get(self.currently_signing)
                        .map(|s| s.as_str())
                        .unwrap_or_else(|| ""),
                    *checksum,
                    self.currently_signing,
                    action.get_encryption(),
                ) {
                    Ok(signature_and_checksum) => {
                        // If signed successfully progress to the
                        // next transaction in the bulk.
                        self.currently_signing += 1;
                        self.counter = 1;
                        *checksum = signature_and_checksum.new_checksum();
                        self.signatures.push((
                            signature_and_checksum.signature().clone(),
                            signature_and_checksum.signature_type(),
                        ));

                        // If this is the last tx, return
                        //
                        if self.currently_signing == self.seeds.len() {
                            return Ok(SignResult::Ready {
                                signatures: self.signatures.clone(),
                            });
                        }
                    }
                    Err(e) => {
                        match e {
                            SignError::WrongPasswordNewChecksum(new_checksum) => {
                                // Password was not correct, re-request it.
                                self.update_checksum_sign(new_checksum);
                                self.counter += 1;

                                return Ok(SignResult::RequestPassword {
                                    idx: self.currently_signing,
                                    counter: self.counter,
                                });
                            }
                            _ => {
                                // Other signing error happened.
                                return Err(Error::TransactionSigning(e));
                            }
                        }
                    }
                }
            }
        } else {
            Err(Error::TxActionNotSign)
        }
    }

    pub fn password_entered(&mut self, pwd: &str) {
        if let TransactionAction::Sign { actions, .. } = &self.action {
            let base58 = actions[self.currently_signing].author_info.base58.clone();
            self.passwords.insert(base58, pwd.to_string());
        }
    }

    pub fn ok(&self) -> bool {
        self.counter < MAX_COUNT_SET
    }

    pub fn counter(&self) -> u8 {
        self.counter
    }
}
