use crate::{
    cards::ParserCard,
    decoding_commons::OutputCard,
    state::DefaultState,
    state::{State, StateError, StateInputCompound, StateInputCompoundItem, StateOutput},
};

use merkleized_metadata::ExtraInfo;

use sp_core::crypto::AccountId32;

use std::convert::TryInto;

use hex::encode;

// Account is expected as [u8, 32] or [u8; 20] sequence
#[derive(Debug, Clone)]
pub struct AccountState {
    len: Option<usize>,
    partial: Vec<u8>,
    extra_info: ExtraInfo,
}

impl AccountState {
    pub fn new(extra_info: ExtraInfo) -> Self {
        Self {
            len: None,
            partial: vec![],
            extra_info,
        }
    }
}

impl AccountState {
    fn get_default_output(&self, indent: u32) -> OutputCard {
        let hex_string = encode(&self.partial);

        OutputCard {
            card: ParserCard::Default(hex_string),
            indent,
        }
    }
}

impl State for AccountState {
    fn clone_box(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }

    fn process_field(
        &self,
        _input: &StateInputCompoundItem,
        indent: u32,
    ) -> Result<StateOutput, StateError> {
        Ok(StateOutput::with(
            Box::new(AccountState::new(self.extra_info.clone())),
            indent,
        ))
    }

    fn process_array(
        &self,
        input: &StateInputCompound,
        indent: u32,
    ) -> Result<StateOutput, StateError> {
        Ok(StateOutput::with(
            Box::new(AccountState {
                len: Some(input.items_count),
                partial: vec![],
                extra_info: self.extra_info.clone(),
            }),
            indent,
        ))
    }

    fn process_array_item(
        &self,
        input: &StateInputCompoundItem,
        indent: u32,
    ) -> Result<StateOutput, StateError> {
        Ok(StateOutput::with(
            Box::new(AccountState {
                len: Some(input.items_count),
                partial: self.partial.clone(),
                extra_info: self.extra_info.clone(),
            }),
            indent,
        ))
    }

    fn process_u8(&self, input: u8, indent: u32) -> Result<StateOutput, StateError> {
        let mut new_partial = self.partial.clone();
        new_partial.push(input);

        let current_len = match self.len {
            Some(l) => l,
            _ => {
                let card = self.get_default_output(indent);
                return Ok(StateOutput {
                    next_state: Box::new(DefaultState),
                    cards: vec![card],
                    indent,
                });
            }
        };

        if new_partial.len() < current_len {
            return Ok(StateOutput::with(
                Box::new(AccountState {
                    len: self.len,
                    partial: new_partial,
                    extra_info: self.extra_info.clone(),
                }),
                indent,
            ));
        }

        let card = match current_len {
            32 => {
                let account_id: [u8; 32] = new_partial
                    .try_into()
                    .map_err(|_err| StateError::BadInput("32 bytes expected".to_string()))?;

                OutputCard {
                    card: ParserCard::Id {
                        id: AccountId32::new(account_id),
                        base58prefix: self.extra_info.base58_prefix,
                    },
                    indent,
                }
            }
            20 => {
                let account_id: [u8; 20] = new_partial
                    .try_into()
                    .map_err(|_err| StateError::BadInput("20 bytes expected".to_string()))?;

                OutputCard {
                    card: ParserCard::Id20 {
                        id: account_id,
                        base58prefix: self.extra_info.base58_prefix,
                    },
                    indent,
                }
            }
            _ => self.get_default_output(indent),
        };

        Ok(StateOutput {
            next_state: Box::new(DefaultState),
            cards: vec![card],
            indent,
        })
    }
}
