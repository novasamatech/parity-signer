use scale_decode::visitor::DecodeError;
use crate::{
  account_state::AccountState, 
	call_state::CallPalletState, 
	cards::ParserCard, 
	decoding_commons::OutputCard, 
	number_state::{BalanceCardProducer, NumberState}, 
	utils::{
    field_type_name_is_account,
    field_type_name_is_balance,
    field_type_name_is_call,
    path_from_parent
  }
};

use alloc::fmt;

use merkleized_metadata::ExtraInfo;

use num_bigint::{BigInt, BigUint};



#[derive(Debug)]
pub enum StateError {
  Decoding(DecodeError),
  UnexpectedInput(String),
  UnexpectedAccountFormat(String),
  BadInput(String)
}

impl fmt::Display for StateError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			match self {
					StateError::Decoding(decoding_error) => write!(f, "decoding failed {}", decoding_error),
					StateError::UnexpectedInput(input) => write!(f, "unexpected input {}", input),
					StateError::UnexpectedAccountFormat(account_format) => write!(f, "unexpected account format {}", account_format),
					StateError::BadInput(input) => write!(f, "bad input {}.", input),
			}
	}
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
		input: bool,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.get_default_output(input.to_string(), indent);

		return Ok(output);
	}

	fn process_char(
		&self,
		input: char,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.get_default_output(input.to_string(), indent);

		return Ok(output);
	}

	fn process_u8(
		&self,
		input: u8,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.get_default_output(input.to_string(), indent);

		return Ok(output);
	}

	fn process_u16(
		&self,
		input: u16,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.get_default_output(input.to_string(), indent);

		return Ok(output);
	}

	fn process_u32(
		&self,
		input: u32,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.get_default_output(input.to_string(), indent);

		return Ok(output);
	}

	fn process_u64(
		&self,
		input: u64,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.get_default_output(input.to_string(), indent);

		return Ok(output);
	}

	fn process_u128(
		&self,
		input: u128,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.get_default_output(input.to_string(), indent);

		return Ok(output);
	}

	fn process_u256<'a>(
		&self,
		input: &'a [u8; 32],
		indent: u32
	) -> Result<StateOutput, StateError> {
		let target_value = BigUint::from_bytes_le(input);
		let output = self.get_default_output(target_value.to_string(), indent);

		return Ok(output);
	}

	fn process_i8(
		&self,
		input: i8,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.get_default_output(input.to_string(), indent);

		return Ok(output);
	}

	fn process_i16<'scale, 'resolver>(
		&self,
		input: i16,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.get_default_output(input.to_string(), indent);

		return Ok(output);
	}

	fn process_i32(
		&self,
		input: i32,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.get_default_output(input.to_string(), indent);

		return Ok(output);
	}

	fn process_i64(
		&self,
		input: i64,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.get_default_output(input.to_string(), indent);

		return Ok(output);
	}

	fn process_i128(
		&self,
		input: i128,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.get_default_output(input.to_string(), indent);

		return Ok(output);
	}

	fn process_i256<'a>(
		&self,
		input: &'a [u8; 32],
		indent: u32
	) -> Result<StateOutput, StateError> {
		let target_value = BigInt::from_signed_bytes_le(input);
		let output = self.get_default_output(target_value.to_string(), indent);

		return Ok(output);
	}

	fn process_str(
		&self,
		input: String,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let output = self.get_default_output(input, indent);

		return Ok(output);
	}

	fn process_bitsequence(
		&self,
		input: Vec<bool>,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let string_repr = input.into_iter().map(|b| if b  { '1' }  else { '0' }).collect();

		let card = OutputCard {
			card: ParserCard::BitVec(string_repr),
			indent
		};

		let output = StateOutput {
			next_state: Box::new(DefaultState),
			cards: vec![card],
			indent
		};

		Ok(output)
	}

	fn process_void(
		&self,
		indent: u32
	) -> Result<StateOutput, StateError> {
		Ok(StateOutput::with(Box::new(DefaultState), indent))
	}

	fn process_sequence(
		&self,
		_input: &StateInputCompound,
		indent: u32
	) -> Result<StateOutput, StateError> {
		Ok(StateOutput::with(Box::new(DefaultState), indent))
	}

	fn process_sequence_item(
		&self,
		_input: &StateInputCompoundItem,
		indent: u32
	) -> Result<StateOutput, StateError> {
		Ok(StateOutput::with(Box::new(DefaultState), indent))
	}

	fn process_composite(
		&self,
		_input: &StateInputCompound,
		indent: u32
	) -> Result<StateOutput, StateError> {
		Ok(StateOutput::with(Box::new(DefaultState), indent))
	}

	fn process_field(
		&self,
		input: &StateInputCompoundItem,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let maybe_field_type = &input.type_name;

		// TODO: duplicates logic of process_fields function in the parser crate

		let full_path = path_from_parent(&input.parent_path, maybe_field_type);

		let (cards, next_indent) = match &input.name {
			Some(field_name) => {
				let card = OutputCard {
					card: ParserCard::FieldName {
							name: field_name.to_string(),
							docs_field_name: "".to_string(),
							path_type: full_path,
							docs_type: "".to_string(),
					},
					indent,
				};

				(vec![card], indent + 1)
			}
			None => {
				if input.items_count > 1 {
					let card = OutputCard {
						card: ParserCard::FieldNumber { 
							number: input.index, 
							docs_field_number: "".to_string(),
							path_type: full_path,
							docs_type: "".to_string()
						},
						indent
					};
					
					(vec![card], indent + 1)
				} else {
					(vec![], indent)
				}
			}
		};

		let next_state: Box<dyn State> = match maybe_field_type {
				Some(field_type) if field_type_name_is_call(&field_type) => {
					Box::new(CallPalletState)
				},
				Some(field_type) if field_type_name_is_account(&field_type) => {
					Box::new(AccountState::new(input.extra_info.clone()))
				},
				Some(field_type) if field_type_name_is_balance(&field_type) => {
					Box::new(NumberState::<BalanceCardProducer>::balance_state(input.extra_info.clone()))
				},
				_ => Box::new(DefaultState)
		};

		Ok(StateOutput { next_state, cards, indent: next_indent })
	}

	fn process_tuple(
		&self,
		_input: &StateInputCompound,
		indent: u32
	) -> Result<StateOutput, StateError> {
		Ok(StateOutput::with(Box::new(DefaultState), indent))
	}

	fn process_tuple_item(
		&self,
		input: &StateInputCompoundItem,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let card = OutputCard {
			card: ParserCard::FieldNumber { 
				number: input.index + 1, 
				docs_field_number: "".to_string(),
				path_type: "".to_string(),
				docs_type: "".to_string()
			},
			indent
		};

		Ok(StateOutput { next_state: Box::new(DefaultState), cards: vec![card], indent })
	}

	fn process_variant(
		&self,
		input: &StateInputCompound,
		indent: u32
	) -> Result<StateOutput, StateError> {
		let card = OutputCard {
			card: ParserCard::EnumVariantName { 
				name: input.name.clone().unwrap_or_default(), 
				docs_enum_variant: "".to_string() 
			},
			indent
		};

		Ok(StateOutput { next_state: Box::new(DefaultState), cards: vec![card], indent: indent + 1 })
	}

	fn process_array(
		&self,
		_input: &StateInputCompound,
		indent: u32
	) -> Result<StateOutput, StateError> {
		Ok(StateOutput::with(Box::new(DefaultState), indent))
	}

	fn process_array_item(
		&self,
		_input: &StateInputCompoundItem,
		indent: u32
	) -> Result<StateOutput, StateError> {
		Ok(StateOutput::with(Box::new(DefaultState), indent))
	}
}

impl Clone for Box<dyn State> {
  fn clone(&self) -> Self {
    self.clone_box()
  }
}

#[derive(Debug, Clone, Default)]
pub struct DefaultState;

impl State for DefaultState {
    fn clone_box(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }
}