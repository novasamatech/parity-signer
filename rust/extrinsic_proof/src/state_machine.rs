use parser::decoding_commons::OutputCard;
use parser::cards::ParserCard;
use num_bigint::{BigInt, BigUint};
use scale_decode::visitor::DecodeError;

pub struct StateInputCompound<'a> {
  pub name: Option<String>,
  pub path: &'a Option<Vec<String>>,
  pub items_count: usize
}

pub struct StateInputCompoundItem<'a> {
  pub index: usize,
  pub name: Option<String>,
  pub parent_path: &'a Option<Vec<String>>,
  pub items_count: usize
}

pub struct StateOutput {
  pub next_state: Box<dyn State>,
  pub cards: Vec<OutputCard>,
  pub indent: u32
}

impl StateOutput {
    fn with(state: Box<dyn State>, indent: u32) -> Self {
      StateOutput {
        next_state: state,
        cards: vec![],
        indent
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
	) -> Result<StateOutput, DecodeError> {
    let output = self.get_default_output(input.to_string(), indent);

    return Ok(output);
  }

	fn process_char(
		&self,
		input: char,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
    let output = self.get_default_output(input.to_string(), indent);

    return Ok(output);
  }

	fn process_u8(
		&self,
		input: u8,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
    let output = self.get_default_output(input.to_string(), indent);

    return Ok(output);
  }

	fn process_u16(
		&self,
		input: u16,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string(), indent);

    return Ok(output);
	}

	fn process_u32(
		&self,
		input: u32,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string(), indent);

    return Ok(output);
	}

	fn process_u64(
		&self,
		input: u64,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string(), indent);

    return Ok(output);
	}

	fn process_u128(
		&self,
		input: u128,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string(), indent);

    return Ok(output);
	}

	fn process_u256<'a>(
		&self,
		input: &'a [u8; 32],
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let target_value = BigUint::from_bytes_le(input);
		let output = self.get_default_output(target_value.to_string(), indent);

    return Ok(output);
	}

	fn process_i8(
		&self,
		input: i8,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string(), indent);

    return Ok(output);
	}

	fn process_i16<'scale, 'resolver>(
		&self,
		input: i16,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string(), indent);

    return Ok(output);
	}

	fn process_i32(
		&self,
		input: i32,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string(), indent);

    return Ok(output);
	}

	fn process_i64(
		&self,
		input: i64,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string(), indent);

    return Ok(output);
	}

	fn process_i128(
		&self,
		input: i128,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input.to_string(), indent);

    return Ok(output);
	}

	fn process_i256<'a>(
		&self,
		input: &'a [u8; 32],
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let target_value = BigInt::from_signed_bytes_le(input);
		let output = self.get_default_output(target_value.to_string(), indent);

    return Ok(output);
	}

  fn process_str(
		&self,
		input: String,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.get_default_output(input, indent);

    return Ok(output);
	}

  fn process_bitsequence(
		&self,
		input: Vec<bool>,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
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

	fn process_sequence(
		&self,
    _input: &StateInputCompound,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		Ok(StateOutput::with(Box::new(DefaultState), indent))
	}

  fn process_sequence_item(
    &self,
    _input: &StateInputCompoundItem,
    indent: u32
  ) -> Result<StateOutput, DecodeError> {
    Ok(StateOutput::with(Box::new(DefaultState), indent))
  }

	fn process_composite(
		&self,
		_input: &StateInputCompound,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		Ok(StateOutput::with(Box::new(DefaultState), indent))
	}

  fn process_field(
    &self,
    input: &StateInputCompoundItem,
    indent: u32
  ) -> Result<StateOutput, DecodeError> {
    let maybe_field_type = &input.name;

    let full_path = path_from_parent(&input.parent_path, maybe_field_type);

    let (cards, next_indent) = match maybe_field_type {
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

		Ok(StateOutput { next_state: Box::new(DefaultState), cards, indent: next_indent })
  }

	fn process_tuple(
		&self,
		_input: &StateInputCompound,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
    Ok(StateOutput::with(Box::new(DefaultState), indent))
	}

  fn process_tuple_item(
    &self,
    input: &StateInputCompoundItem,
    indent: u32
  ) -> Result<StateOutput, DecodeError> {
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
	) -> Result<StateOutput, DecodeError> {
    let card = OutputCard {
			card: ParserCard::EnumVariantName { 
				name: input.name.clone().unwrap_or_default(), 
				docs_enum_variant: "".to_string() 
			},
			indent
		};

    Ok(StateOutput { next_state: Box::new(DefaultState), cards: vec![card], indent })
	}

	fn process_array(
		&self,
		_input: &StateInputCompound,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
    Ok(StateOutput::with(Box::new(DefaultState), indent))
	}

  fn process_array_item(
    &self,
    _input: &StateInputCompoundItem,
    indent: u32
  ) -> Result<StateOutput, DecodeError> {
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

#[derive(Debug, Clone, Default)]
pub struct CallPalletState;

impl State for CallPalletState {
  fn clone_box(&self) -> Box<dyn State> {
      Box::new(self.clone())
  }

  fn process_variant(
		&self,
		input: &StateInputCompound,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
    let card = OutputCard {
			card: ParserCard::Pallet( 
        input.name.clone().unwrap_or_default()
      ),
			indent
		};

    Ok(StateOutput { next_state: Box::new(Self), cards: vec![card], indent })
	}

  fn process_field(
    &self,
    _input: &StateInputCompoundItem,
    indent: u32
  ) -> Result<StateOutput, DecodeError> {
    // expecting single field as method variant inside pallet field

    Ok(StateOutput::with(Box::new(CallMethodState), indent + 1))
  }
}

#[derive(Debug, Clone, Default)]
pub struct CallMethodState;

impl State for CallMethodState {
  fn clone_box(&self) -> Box<dyn State> {
      Box::new(self.clone())
  }

  fn process_variant(
		&self,
		input: &StateInputCompound,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
    let card = OutputCard {
			card: ParserCard::Method {
        method_name: input.name.clone().unwrap_or_default(),
        docs: "".to_string()
      },
			indent
		};

    Ok(StateOutput { next_state: Box::new(DefaultState), cards: vec![card], indent: indent + 1 })
	}
}