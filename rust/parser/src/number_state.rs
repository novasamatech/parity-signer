use crate::{
    cards::ParserCard,
    decoding_commons::OutputCard,
    state::DefaultState,
    state::{State, StateError, StateOutput},
};

use merkleized_metadata::ExtraInfo;

use printing_balance::convert_balance_pretty;

use num_bigint::{BigInt, BigUint};

pub trait NumberCardProducing: Sync + Send + Clone {
    fn produce_card(&self, number: String, indent: u32) -> OutputCard;
}

#[derive(Debug, Clone)]
pub struct NumberState<T: NumberCardProducing> {
    pub card_producer: T,
}

impl<T: NumberCardProducing> NumberState<T> {
    fn process_number(&self, number: String, indent: u32) -> StateOutput {
        let card = self.card_producer.produce_card(number, indent);
        StateOutput {
            next_state: Box::new(DefaultState),
            cards: vec![card],
            indent,
        }
    }
}

impl<T: NumberCardProducing + 'static> State for NumberState<T> {
    fn clone_box(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }

    fn process_u8(&self, input: u8, indent: u32) -> Result<StateOutput, StateError> {
        let output = self.process_number(input.to_string(), indent);

        Ok(output)
    }

    fn process_u16(&self, input: u16, indent: u32) -> Result<StateOutput, StateError> {
        let output = self.process_number(input.to_string(), indent);

        Ok(output)
    }

    fn process_u32(&self, input: u32, indent: u32) -> Result<StateOutput, StateError> {
        let output = self.process_number(input.to_string(), indent);

        Ok(output)
    }

    fn process_u64(&self, input: u64, indent: u32) -> Result<StateOutput, StateError> {
        let output = self.process_number(input.to_string(), indent);

        Ok(output)
    }

    fn process_u128(&self, input: u128, indent: u32) -> Result<StateOutput, StateError> {
        let output = self.process_number(input.to_string(), indent);

        Ok(output)
    }

    fn process_u256(
        &self,
        input: &[u8; 32],
        indent: u32,
    ) -> Result<StateOutput, StateError> {
        let target_value = BigUint::from_bytes_le(input);
        let output = self.process_number(target_value.to_string(), indent);

        Ok(output)
    }

    fn process_i8(&self, input: i8, indent: u32) -> Result<StateOutput, StateError> {
        let output = self.process_number(input.to_string(), indent);

        Ok(output)
    }

    fn process_i16<'scale, 'resolver>(
        &self,
        input: i16,
        indent: u32,
    ) -> Result<StateOutput, StateError> {
        let output = self.process_number(input.to_string(), indent);

        Ok(output)
    }

    fn process_i32(&self, input: i32, indent: u32) -> Result<StateOutput, StateError> {
        let output = self.process_number(input.to_string(), indent);

        Ok(output)
    }

    fn process_i64(&self, input: i64, indent: u32) -> Result<StateOutput, StateError> {
        let output = self.process_number(input.to_string(), indent);

        Ok(output)
    }

    fn process_i128(&self, input: i128, indent: u32) -> Result<StateOutput, StateError> {
        let output = self.process_number(input.to_string(), indent);

        Ok(output)
    }

    fn process_i256(
        &self,
        input: &[u8; 32],
        indent: u32,
    ) -> Result<StateOutput, StateError> {
        let target_value = BigInt::from_signed_bytes_le(input);
        let output = self.process_number(target_value.to_string(), indent);

        Ok(output)
    }
}

#[derive(Debug, Clone)]
pub struct BalanceCardProducer(pub ExtraInfo);

impl NumberCardProducing for BalanceCardProducer {
    fn produce_card(&self, number: String, indent: u32) -> OutputCard {
        let balance_output = convert_balance_pretty(&number, self.0.decimals, &self.0.token_symbol);

        OutputCard {
            card: ParserCard::Balance {
                number: balance_output.number,
                units: balance_output.units,
            },
            indent,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NonceCardProducer;

impl NumberCardProducing for NonceCardProducer {
    fn produce_card(&self, number: String, indent: u32) -> OutputCard {
        OutputCard {
            card: ParserCard::Nonce(number),
            indent,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TxVersionCardProducer;

impl NumberCardProducing for TxVersionCardProducer {
    fn produce_card(&self, number: String, indent: u32) -> OutputCard {
        OutputCard {
            card: ParserCard::TxVersion(number),
            indent,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpecVersionCardProducer(pub ExtraInfo);

impl NumberCardProducing for SpecVersionCardProducer {
    fn produce_card(&self, number: String, indent: u32) -> OutputCard {
        OutputCard {
            card: ParserCard::NetworkNameVersion {
                name: self.0.spec_name.clone(),
                version: number,
            },
            indent,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TipCardProducer(pub ExtraInfo);

impl NumberCardProducing for TipCardProducer {
    fn produce_card(&self, number: String, indent: u32) -> OutputCard {
        let balance_output = convert_balance_pretty(&number, self.0.decimals, &self.0.token_symbol);

        OutputCard {
            card: ParserCard::Tip {
                number: balance_output.number,
                units: balance_output.units,
            },
            indent,
        }
    }
}

impl<T: NumberCardProducing> NumberState<T> {
    pub fn balance_state(extra_info: ExtraInfo) -> NumberState<BalanceCardProducer> {
        NumberState {
            card_producer: BalanceCardProducer(extra_info),
        }
    }

    pub fn nonce_state() -> NumberState<NonceCardProducer> {
        NumberState {
            card_producer: NonceCardProducer,
        }
    }

    pub fn tx_version_state() -> NumberState<TxVersionCardProducer> {
        NumberState {
            card_producer: TxVersionCardProducer,
        }
    }

    pub fn spec_version_state(extra_info: ExtraInfo) -> NumberState<SpecVersionCardProducer> {
        NumberState {
            card_producer: SpecVersionCardProducer(extra_info),
        }
    }

    pub fn tip_state(extra_info: ExtraInfo) -> NumberState<TipCardProducer> {
        NumberState {
            card_producer: TipCardProducer(extra_info),
        }
    }
}
