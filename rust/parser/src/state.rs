use scale_decode::visitor::DecodeError;
use crate::{
  decoding_commons::OutputCard,
  cards::ParserCard,
  default_state::DefaultState
};

use merkleized_metadata::ExtraInfo;

#[derive(Debug)]
pub enum StateError {
  Decoding(DecodeError),
  UnexpectedInput(String),
  UnexpectedAccountFormat(String),
  BadInput(String)
}

impl From<DecodeError> for StateError {
  fn from(value: DecodeError) -> Self {
    StateError::Decoding(value)
  }
}

pub struct StateInputCompound<'a> {
  pub name: Option<String>,
  pub path: &'a Option<Vec<String>>,
  pub extra_info: ExtraInfo,
  pub items_count: usize
}

pub struct StateInputCompoundItem<'a> {
  pub index: usize,
  pub name: Option<String>,
  pub parent_path: &'a Option<Vec<String>>,
  pub type_name: Option<String>,
  pub extra_info: ExtraInfo,
  pub items_count: usize
}

pub struct StateOutput {
  pub next_state: Box<dyn State>,
  pub cards: Vec<OutputCard>,
  pub indent: u32
}

impl StateOutput {
  pub fn with(state: Box<dyn State>, indent: u32) -> Self {
      StateOutput {
        next_state: state,
        cards: vec![],
        indent
      }
    }
}

pub trait State: Send + Sync {
  fn clone_box(&self) -> Box<dyn State>;

  fn get_default_output(
    &self,
		input: String,
    indent: u32
  ) -> StateOutput {
    let card = OutputCard {
			card: ParserCard::Default(input),
			indent
		};

    StateOutput {
      next_state: Box::new(DefaultState),
      cards: vec![card],
      indent
    }
  }

  fn process_bool(
		&self,
		_input: bool,
    _indent: u32
	) -> Result<StateOutput, StateError> {
    Err(StateError::UnexpectedInput(String::from("bool")))
  }

	fn process_char(
		&self,
		_input: char,
    _indent: u32
	) -> Result<StateOutput, StateError> {
    Err(StateError::UnexpectedInput(String::from("char")))
  }

	fn process_u8(
		&self,
		_input: u8,
    _indent: u32
	) -> Result<StateOutput, StateError> {
    Err(StateError::UnexpectedInput(String::from("u8")))
  }

	fn process_u16(
		&self,
		_input: u16,
    _indent: u32
	) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("u16")))
	}

	fn process_u32(
		&self,
		_input: u32,
    _indent: u32
	) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("u32")))
	}

	fn process_u64(
		&self,
		_input: u64,
    _indent: u32
	) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("u64")))
	}

	fn process_u128(
		&self,
		_input: u128,
    _indent: u32
	) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("u128")))
	}

	fn process_u256<'a>(
		&self,
		_input: &'a [u8; 32],
    _indent: u32
	) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("u256")))
	}

	fn process_i8(
		&self,
		_input: i8,
    _indent: u32
	) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("i8")))
	}

	fn process_i16<'scale, 'resolver>(
		&self,
		_input: i16,
    _indent: u32
	) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("i16")))
	}

	fn process_i32(
		&self,
		_input: i32,
    _indent: u32
	) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("i32")))
	}

	fn process_i64(
		&self,
		_input: i64,
    _indent: u32
	) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("i64")))
	}

	fn process_i128(
		&self,
		_input: i128,
    _indent: u32
	) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("i128")))
	}

	fn process_i256<'a>(
		&self,
		_input: &'a [u8; 32],
    _indent: u32
	) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("i256")))
	}

  fn process_void(
		&self,
    _indent: u32
	) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("Void")))
	}

  fn process_str(
		&self,
		_input: String,
    _indent: u32
	) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("str")))
	}

  fn process_bitsequence(
		&self,
		_input: Vec<bool>,
    _indent: u32
	) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("bitsequence")))
	}

	fn process_sequence(
		&self,
    _input: &StateInputCompound,
    _indent: u32
	) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("sequence")))
	}

  fn process_sequence_item(
    &self,
    _input: &StateInputCompoundItem,
    _indent: u32
  ) -> Result<StateOutput, StateError> {
    Err(StateError::UnexpectedInput(String::from("sequenceItem")))
  }

	fn process_composite(
		&self,
		_input: &StateInputCompound,
    _indent: u32
	) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("composite")))
	}

  fn process_field(
    &self,
    _input: &StateInputCompoundItem,
    _indent: u32
  ) -> Result<StateOutput, StateError> {
    Err(StateError::UnexpectedInput(String::from("field")))
  }

	fn process_tuple(
		&self,
		_input: &StateInputCompound,
    _indent: u32
	) -> Result<StateOutput, StateError> {
    Err(StateError::UnexpectedInput(String::from("tuple")))
	}

  fn process_tuple_item(
    &self,
    _input: &StateInputCompoundItem,
    _indent: u32
  ) -> Result<StateOutput, StateError> {
    Err(StateError::UnexpectedInput(String::from("tupleItem")))
  }

  fn process_variant(
		&self,
		_input: &StateInputCompound,
    _indent: u32
	) -> Result<StateOutput, StateError> {
    Err(StateError::UnexpectedInput(String::from("variant")))
	}

	fn process_array(
		&self,
		_input: &StateInputCompound,
    _indent: u32
	) -> Result<StateOutput, StateError> {
    Err(StateError::UnexpectedInput(String::from("array")))
	}

  fn process_array_item(
    &self,
    _input: &StateInputCompoundItem,
    _indent: u32
  ) -> Result<StateOutput, StateError> {
		Err(StateError::UnexpectedInput(String::from("arrayItem")))
  }
}

impl Clone for Box<dyn State> {
  fn clone(&self) -> Self {
    self.clone_box()
  }
}