use parser::decoding_commons::OutputCard;
use parser::cards::ParserCard;
use num_bigint::{BigInt, BigUint};
use crate::types::EnumerationVariant;
use scale_decode::visitor::DecodeError;
use std::clone::Clone;

pub struct StateInputCompoundValue {
  name: Option<String>,
  path: Option<String>
}

pub struct StateInputCompositeValue {
  pub name: Option<String>,
  pub path: Option<Vec<String>>,
  pub field_count: usize
}

pub struct StateInputFieldValue {
  pub index: usize,
  pub name: Option<String>
}

pub struct StateInputVariantValue {
  name: String,
  path: Option<Vec<String>>,
  variants: Vec<EnumerationVariant>,
  selected: EnumerationVariant
}

pub struct StateOutput {
  pub cards: Vec<OutputCard>
}

impl StateOutput {
    fn empty() -> Self {
      StateOutput {
        cards: vec![]
      }
    }
}

fn path_to_string<'a>(path: Vec<String>) -> String {
	path.join(" >> ")
}

fn path_from_parent<'a>(parent_path: &Option<Vec<String>>, maybe_type_name: &Option<String>) -> String {
  match maybe_type_name {
    Some(type_name) => {
      let new_path = parent_path
        .as_ref()
        .unwrap_or(&vec![])
        .into_iter()
        .chain(std::iter::once(type_name))
        .cloned()
        .collect();
      path_to_string(new_path)
    }
    None => {
      "".to_string()
    }
  }
}

pub trait PushStateMachine {
    fn push_state(&mut self);
    fn set_state(&mut self, state: Box<dyn State>);
    fn pop_state(&mut self);
    fn can_pop_state(&self) -> bool;

    fn pop_if_possible(&mut self) {
      if self.can_pop_state() {
        self.pop_state();
      }
  }
}

pub trait State: Send + Sync {
  fn clone_box(&self) -> Box<dyn State>;

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

  fn process_bool(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: bool
	) -> Result<StateOutput, DecodeError> {
    let output = self.get_default_output(input.to_string());
    state_machine.pop_if_possible();

    return Ok(output);
  }

	fn process_char(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: char
	) -> Result<StateOutput, DecodeError> {
    let output = self.get_default_output(input.to_string());
    state_machine.pop_if_possible();

    return Ok(output);
  }

	fn process_u8(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: u8
	) -> Result<StateOutput, DecodeError> {
    let output = self.get_default_output(input.to_string());
    state_machine.pop_if_possible();

    return Ok(output);
  }

	fn process_u16(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: u16
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string());
    state_machine.pop_if_possible();

    return Ok(output);
	}

	fn process_u32(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string());
    state_machine.pop_if_possible();

    return Ok(output);
	}

	fn process_u64(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: u64
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string());
    state_machine.pop_if_possible();

    return Ok(output);
	}

	fn process_u128(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: u128
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string());
    state_machine.pop_if_possible();

    return Ok(output);
	}

	fn process_u256<'a>(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: &'a [u8; 32]
	) -> Result<StateOutput, DecodeError> {
		let target_value = BigUint::from_bytes_le(input);
		let output = self.get_default_output(target_value.to_string());
    state_machine.pop_if_possible();

    return Ok(output);
	}

	fn process_i8(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: i8
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string());
    state_machine.pop_if_possible();

    return Ok(output);
	}

	fn process_i16<'scale, 'resolver>(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: i16
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string());
    state_machine.pop_if_possible();

    return Ok(output);
	}

	fn process_i32(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: i32
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string());
    state_machine.pop_if_possible();

    return Ok(output);
	}

	fn process_i64(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: i64
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string());
    state_machine.pop_if_possible();

    return Ok(output);
	}

	fn process_i128(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: i128
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string());
    state_machine.pop_if_possible();

    return Ok(output);
	}

	fn process_i256<'a>(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: &'a [u8; 32]
	) -> Result<StateOutput, DecodeError> {
		let target_value = BigInt::from_signed_bytes_le(input);
		let output = self.get_default_output(target_value.to_string());
    state_machine.pop_if_possible();

    return Ok(output);
	}

  fn process_str(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: String
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input);
    state_machine.pop_if_possible();

    return Ok(output);
	}

  fn process_bitsequence(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: Vec<bool>
	) -> Result<StateOutput, DecodeError> {
		let string_repr = input.into_iter().map(|b| if b  { '1' }  else { '0' }).collect();

		let card = OutputCard {
			card: ParserCard::BitVec(string_repr),
			indent: self.get_indent()
		};

    let output = StateOutput {
      cards: vec![card]
    };

    state_machine.pop_if_possible();

		Ok(output)
	}

	fn start_sequence(
		&self,
    _state_machine:&mut dyn PushStateMachine,
    _input: StateInputCompoundValue
	) -> Result<StateOutput, DecodeError> {
		Ok(StateOutput::empty())
	}

  fn complete_sequence(
    &self,
    _state_machine:&mut dyn PushStateMachine,
    _input: StateInputCompoundValue
  ) -> Result<StateOutput, DecodeError> {
		Ok(StateOutput::empty())
  }

	fn start_composite(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: StateInputCompositeValue
	) -> Result<StateOutput, DecodeError> {
    let next_state = Box::new(FieldState {
      indent: self.get_indent(),
      parent_path: input.path,
      parent_fields_count: input.field_count
    });

    state_machine.set_state(next_state);

		Ok(StateOutput::empty())
	}

  fn process_field(
    &self,
    _state_machine:&mut dyn PushStateMachine,
    _input: StateInputFieldValue
  ) -> Result<StateOutput, DecodeError> {
		Ok(StateOutput::empty())
  }

  fn complete_composite(
		&self,
    _state_machine:&mut dyn PushStateMachine,
		_input: StateInputCompositeValue
	) -> Result<StateOutput, DecodeError> {
		Ok(StateOutput::empty())
	}

	fn start_tuple(
		&self,
    _state_machine:&mut dyn PushStateMachine,
		_input: StateInputCompoundValue
	) -> Result<StateOutput, DecodeError> {
    Ok(StateOutput::empty())
	}

  fn complete_tuple(
    &self,
    _state_machine:&mut dyn PushStateMachine,
    _input: StateInputCompoundValue
  ) -> Result<StateOutput, DecodeError> {
    Ok(StateOutput::empty())
  }

  fn start_variant(
		&self,
    state_machine:&mut dyn PushStateMachine,
		input: StateInputVariantValue
	) -> Result<StateOutput, DecodeError> {
    let card = OutputCard {
			card: ParserCard::EnumVariantName { 
				name: input.name, 
				docs_enum_variant: "".to_string() 
			},
			indent: self.get_indent()
		};

		let next_state = Box::new(FieldState {
      indent: self.get_indent(),
      parent_path: input.path,
      parent_fields_count: input.selected.fields.iter().count()
    });

    state_machine.set_state(next_state);

    Ok(StateOutput { cards: vec![card] })
	}

  fn complete_variant(
    &self,
    _state_machine:&mut dyn PushStateMachine,
    _input: StateInputVariantValue
  ) -> Result<StateOutput, DecodeError> {
    Ok(StateOutput::empty())
  }

	fn start_array(
		&self,
    _state_machine:&mut dyn PushStateMachine,
		_input: StateInputCompoundValue
	) -> Result<StateOutput, DecodeError> {
    Ok(StateOutput::empty())
	}

  fn complete_array(
    &self,
    _state_machine:&mut dyn PushStateMachine,
    _input: StateInputCompoundValue
  ) -> Result<StateOutput, DecodeError> {
    Ok(StateOutput::empty())
  }
}

impl Clone for Box<dyn State> {
  fn clone(&self) -> Self {
    self.clone_box()
  }
}

#[derive(Debug, Clone)]
pub struct DefaultState {
  pub indent: u32
}

impl State for DefaultState {
    fn clone_box(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }

    fn get_indent(&self) -> u32 {
        self.indent
    }
}

#[derive(Debug, Clone)]
struct FieldState {
  indent: u32,
  parent_path: Option<Vec<String>>,
  parent_fields_count: usize
}

impl State for FieldState {
  fn clone_box(&self) -> Box<dyn State> {
    Box::new(self.clone())
  }

  fn get_indent(&self) -> u32 {
      self.indent
  }

  fn process_field(
    &self,
    state_machine:&mut dyn PushStateMachine,
    input: StateInputFieldValue
  ) -> Result<StateOutput, DecodeError> {
    state_machine.push_state();

    let current_indent = self.get_indent();
    let maybe_field_type = input.name;

    let full_path = path_from_parent(&self.parent_path, &maybe_field_type);

    let (cards, next_indent) = match maybe_field_type {
      Some(field_name) => {
        let card = OutputCard {
          card: ParserCard::FieldName {
              name: field_name.to_string(),
              docs_field_name: "".to_string(),
              path_type: full_path,
              docs_type: "".to_string(),
          },
          indent: current_indent,
        };

        (vec![card], current_indent + 1)
      }
      None => {
        if self.parent_fields_count > 1 {
          let card = OutputCard {
            card: ParserCard::FieldNumber { 
              number: input.index, 
              docs_field_number: "".to_string(),
              path_type: full_path,
              docs_type: "".to_string()
            },
            indent: current_indent
          };
          
          (vec![card], current_indent + 1)
        } else {
          (vec![], current_indent)
        }
      }
    };

    let next_state = Box::new(DefaultState { indent: next_indent });
    state_machine.set_state(next_state);

		Ok(StateOutput { cards })
  }
}