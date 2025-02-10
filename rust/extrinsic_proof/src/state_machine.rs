use parser::decoding_commons::OutputCard;
use parser::cards::ParserCard;
use num_bigint::{BigInt, BigUint};

pub struct StateInput<T> {
  value: T
}

pub struct StateInputCompoundValue {
  name: String,
  path: Option<String>
}

pub struct StateOutput {
  cards: Vec<OutputCard>
}

impl StateOutput {
    fn empty() -> Self {
      StateOutput {
        cards: vec![]
      }
    }
}

pub trait State {
  fn get_default_output(
    &self,
		input: String
  ) -> StateOutput {
    let card = OutputCard {
			card: ParserCard::Default(input),
			indent: self.get_indent()
		};

    StateOutput {
      cards: vec![card]
    }
  }

  fn get_indent(&self) -> u32;

  fn get_default_next_state(&self) -> Box<dyn State>;

  fn process_bool(
		&self,
		input: StateInput<bool>
	) -> Result<(StateOutput, Box<dyn State>), String> {
    let output = self.get_default_output(input.value.to_string());
    let next_state = self.get_default_next_state();

    return Ok((output, next_state));
  }

	fn process_char(
		&self,
		input: StateInput<char>
	) -> Result<(StateOutput, Box<dyn State>), String> {
    let output = self.get_default_output(input.value.to_string());
    let next_state = self.get_default_next_state();

    return Ok((output, next_state));
  }

	fn process_u8(
		&self,
		input: StateInput<u8>
	) -> Result<(StateOutput, Box<dyn State>), String> {
    let output = self.get_default_output(input.value.to_string());
    let next_state = self.get_default_next_state();

    return Ok((output, next_state));
  }

	fn process_u16(
		&self,
		input: StateInput<u16>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let output = self.get_default_output(input.value.to_string());
    let next_state = self.get_default_next_state();

    return Ok((output, next_state));
	}

	fn process_u32(
		&self,
		input: StateInput<u32>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let output = self.get_default_output(input.value.to_string());
    let next_state = self.get_default_next_state();

    return Ok((output, next_state));
	}

	fn process_u64(
		&self,
		input: StateInput<u64>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let output = self.get_default_output(input.value.to_string());
    let next_state = self.get_default_next_state();

    return Ok((output, next_state));
	}

	fn process_u128(
		&self,
		input: StateInput<u128>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let output = self.get_default_output(input.value.to_string());
    let next_state = self.get_default_next_state();

    return Ok((output, next_state));
	}

	fn process_u256<'a>(
		&self,
		input: StateInput<&'a [u8; 32]>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let target_value = BigUint::from_bytes_le(input.value);
		let output = self.get_default_output(target_value.to_string());
    let next_state = self.get_default_next_state();

    return Ok((output, next_state));
	}

	fn process_i8(
		&self,
		input: StateInput<i8>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let output = self.get_default_output(input.value.to_string());
    let next_state = self.get_default_next_state();

    return Ok((output, next_state));
	}

	fn process_i16<'scale, 'resolver>(
		&self,
		input: StateInput<i16>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let output = self.get_default_output(input.value.to_string());
    let next_state = self.get_default_next_state();

    return Ok((output, next_state));
	}

	fn process_i32(
		&self,
		input: StateInput<i32>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let output = self.get_default_output(input.value.to_string());
    let next_state = self.get_default_next_state();

    return Ok((output, next_state));
	}

	fn process_i64(
		&self,
		input: StateInput<i64>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let output = self.get_default_output(input.value.to_string());
    let next_state = self.get_default_next_state();

    return Ok((output, next_state));
	}

	fn process_i128(
		&self,
		input: StateInput<i128>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let output = self.get_default_output(input.value.to_string());
    let next_state = self.get_default_next_state();

    return Ok((output, next_state));
	}

	fn process_i256<'a>(
		&self,
		input: StateInput<&'a [u8; 32]>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let target_value = BigInt::from_signed_bytes_le(input.value);
		let output = self.get_default_output(target_value.to_string());
    let next_state = self.get_default_next_state();

    return Ok((output, next_state));
	}

	fn start_sequence(
		&self,
    _input: StateInput<StateInputCompoundValue>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let next_state = self.get_default_next_state();

		Ok((StateOutput::empty(), next_state))
	}

  fn complete_sequence(
    &self,
    _input: StateInput<StateInputCompoundValue>
  ) -> Result<(StateOutput, Box<dyn State>), String> {
    let next_state = self.get_default_next_state();

		Ok((StateOutput::empty(), next_state))
  }

	fn start_composite(
		&self,
		_input: StateInput<StateInputCompoundValue>
	) -> Result<(StateOutput, Box<dyn State>), String> {
    let next_state = self.get_default_next_state();

		Ok((StateOutput::empty(), next_state))
	}

  fn complete_composite(
		&self,
		_input: StateInput<StateInputCompoundValue>
	) -> Result<(StateOutput, Box<dyn State>), String> {
    let next_state = self.get_default_next_state();

		Ok((StateOutput::empty(), next_state))
	}

	fn start_tuple(
		&self,
		_input: StateInput<StateInputCompoundValue>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let next_state = self.get_default_next_state();

    Ok((StateOutput::empty(), next_state))
	}

  fn complete_tuple(
    &self,
    _input: StateInput<StateInputCompoundValue>
  ) -> Result<(StateOutput, Box<dyn State>), String> {
    let next_state = self.get_default_next_state();

    Ok((StateOutput::empty(), next_state))
  }

	fn process_str(
		&self,
		input: StateInput<String>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let output = self.get_default_output(input.value);
    let next_state = self.get_default_next_state();

    return Ok((output, next_state));
	}

  fn start_variant(
		&self,
		_input: StateInput<StateInputCompoundValue>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let next_state = self.get_default_next_state();

    Ok((StateOutput::empty(), next_state))
	}

  fn complete_variant(
    &self,
    _input: StateInput<StateInputCompoundValue>
  ) -> Result<(StateOutput, Box<dyn State>), String> {
    let next_state = self.get_default_next_state();

    Ok((StateOutput::empty(), next_state))
  }

	fn start_array(
		&self,
		_input: StateInput<StateInputCompoundValue>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let next_state = self.get_default_next_state();

    Ok((StateOutput::empty(), next_state))
	}

  fn complete_array(
    &self,
    _input: StateInput<StateInputCompoundValue>
  ) -> Result<(StateOutput, Box<dyn State>), String> {
    let next_state = self.get_default_next_state();

    Ok((StateOutput::empty(), next_state))
  }

	fn process_bitsequence(
		&self,
		input: StateInput<Vec<bool>>
	) -> Result<(StateOutput, Box<dyn State>), String> {
		let string_repr = input.value.into_iter().map(|b| if b  { '1' }  else { '0' }).collect();

		let card = OutputCard {
			card: ParserCard::BitVec(string_repr),
			indent: self.get_indent()
		};

    let output = StateOutput {
      cards: vec![card]
    };

    let next_state = self.get_default_next_state();

		Ok((output, next_state))
	}
}

struct DefaultState {
  indent: u32
}

impl State for DefaultState {
    fn get_default_next_state(&self) -> Box<dyn State> {
        Box::new(DefaultState { indent: self.get_indent() })
    }

    fn get_indent(&self) -> u32 {
        self.indent
    }
}

struct OptionState {
  indent: u32
}