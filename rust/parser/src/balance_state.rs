use crate::{
  state::{State, StateOutput, StateError},
  cards::ParserCard,
  decoding_commons::OutputCard,
  default_state::DefaultState,
  types::ExtraInfo
};

use printing_balance::convert_balance_pretty;
use num_bigint::{BigInt, BigUint};

#[derive(Debug, Clone)]
pub struct BalanceState {
  pub extra_info: ExtraInfo
}

impl BalanceState {
    fn process(&self, balance: String, indent: u32) -> StateOutput {
      let balance_output = convert_balance_pretty(&balance, self.extra_info.decimals, &self.extra_info.token_symbol);

      let card = OutputCard {
        card: ParserCard::Balance { number: balance_output.number, units: balance_output.units }, 
        indent
      };

      StateOutput { next_state: Box::new(DefaultState), cards: vec![card], indent: indent }
    }
}

impl State for BalanceState {
  fn clone_box(&self) -> Box<dyn State> {
      Box::new(self.clone())
  }

  fn process_u8(
		&self,
		input: u8,
    indent: u32
	) -> Result<StateOutput, StateError> {
    let output = self.process(input.to_string(), indent);

    return Ok(output);
  }

	fn process_u16(
		&self,
		input: u16,
    indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_u32(
		&self,
		input: u32,
    indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_u64(
		&self,
		input: u64,
    indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_u128(
		&self,
		input: u128,
    indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_u256<'a>(
		&self,
		input: &'a [u8; 32],
    indent: u32
	) -> Result<StateOutput, StateError> {
		let target_value = BigUint::from_bytes_le(input);
		let output = self.process(target_value.to_string(), indent);

    return Ok(output);
	}

	fn process_i8(
		&self,
		input: i8,
    indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_i16<'scale, 'resolver>(
		&self,
		input: i16,
    indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_i32(
		&self,
		input: i32,
    indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_i64(
		&self,
		input: i64,
    indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_i128(
		&self,
		input: i128,
    indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_i256<'a>(
		&self,
		input: &'a [u8; 32],
    indent: u32
	) -> Result<StateOutput, StateError> {
		let target_value = BigInt::from_signed_bytes_le(input);
		let output = self.process(target_value.to_string(), indent);

    return Ok(output);
	}
}