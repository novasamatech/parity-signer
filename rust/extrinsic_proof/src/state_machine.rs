use std::convert::TryInto;

use parser::decoding_commons::OutputCard;
use parser::cards::ParserCard;
use num_bigint::{BigInt, BigUint};
use printing_balance::convert_balance_pretty;
use scale_decode::visitor::DecodeError;
use sp_core::crypto::AccountId32;

use crate::types::ExtraInfo;

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
    fn with(state: Box<dyn State>, indent: u32) -> Self {
      StateOutput {
        next_state: state,
        cards: vec![],
        indent
      }
    }
}

// TODO: Refactor to util functions

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

fn field_type_name_is_balance(type_name: &str) -> bool {
  (type_name == "Balance")
      || (type_name == "T::Balance")
      || (type_name == "BalanceOf<T>")
      || (type_name == "ExtendedBalance")
      || (type_name == "BalanceOf<T, I>")
      || (type_name == "DepositBalance")
      || (type_name == "PalletBalanceOf<T>")
}

fn field_type_name_is_account(type_name: &str) -> bool {
  (type_name == "AccountId")
      || (type_name == "AccountId32")
      || (type_name == "AccountId20")
}

fn field_type_name_is_call(type_name: &str) -> bool {
  (type_name == "Call") || (type_name == "RuntimeCall")
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
	) -> Result<StateOutput, DecodeError> {
    Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::Bool))
  }

	fn process_char(
		&self,
		_input: char,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
    Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::Char))
  }

	fn process_u8(
		&self,
		_input: u8,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
    Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::U8))
  }

	fn process_u16(
		&self,
		_input: u16,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::U16))
	}

	fn process_u32(
		&self,
		_input: u32,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::U32))
	}

	fn process_u64(
		&self,
		_input: u64,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::U64))
	}

	fn process_u128(
		&self,
		_input: u128,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::U128))
	}

	fn process_u256<'a>(
		&self,
		_input: &'a [u8; 32],
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::U256))
	}

	fn process_i8(
		&self,
		_input: i8,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::I8))
	}

	fn process_i16<'scale, 'resolver>(
		&self,
		_input: i16,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::I16))
	}

	fn process_i32(
		&self,
		_input: i32,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::I32))
	}

	fn process_i64(
		&self,
		_input: i64,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::I64))
	}

	fn process_i128(
		&self,
		_input: i128,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::I128))
	}

	fn process_i256<'a>(
		&self,
		_input: &'a [u8; 32],
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::I256))
	}

  fn process_void(
		&self,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::I256))
	}

  fn process_str(
		&self,
		_input: String,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::Str))
	}

  fn process_bitsequence(
		&self,
		_input: Vec<bool>,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::Bitsequence))
	}

	fn process_sequence(
		&self,
    _input: &StateInputCompound,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::Sequence))
	}

  fn process_sequence_item(
    &self,
    _input: &StateInputCompoundItem,
    _indent: u32
  ) -> Result<StateOutput, DecodeError> {
    Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::Sequence))
  }

	fn process_composite(
		&self,
		_input: &StateInputCompound,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::Composite))
	}

  fn process_field(
    &self,
    _input: &StateInputCompoundItem,
    _indent: u32
  ) -> Result<StateOutput, DecodeError> {
    Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::Composite))
  }

	fn process_tuple(
		&self,
		_input: &StateInputCompound,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
    Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::Tuple))
	}

  fn process_tuple_item(
    &self,
    _input: &StateInputCompoundItem,
    _indent: u32
  ) -> Result<StateOutput, DecodeError> {
    Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::Tuple))
  }

  fn process_variant(
		&self,
		_input: &StateInputCompound,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
    Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::Variant))
	}

	fn process_array(
		&self,
		_input: &StateInputCompound,
    _indent: u32
	) -> Result<StateOutput, DecodeError> {
    Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::Array))
	}

  fn process_array_item(
    &self,
    _input: &StateInputCompoundItem,
    _indent: u32
  ) -> Result<StateOutput, DecodeError> {
		Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::Array))
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

    fn process_void(
      &self,
      indent: u32
    ) -> Result<StateOutput, DecodeError> {
      Ok(StateOutput::with(Box::new(DefaultState), indent))
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
      let maybe_field_type = &input.type_name;
  
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
            Box::new(BalanceState { extra_info: input.extra_info.clone() })
          },
          _ => Box::new(DefaultState)
      };
  
      Ok(StateOutput { next_state: next_state, cards, indent: next_indent })
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


// Account is expected as [u8, 32] sequence
#[derive(Debug, Clone)]
pub struct AccountState {
  len: Option<usize>,
  partial: Vec<u8>,
  extra_info: ExtraInfo
}

impl AccountState {
    fn new(extra_info: ExtraInfo) -> Self {
      Self {
        len: None,
        partial: vec![],
        extra_info: extra_info
      }
    }
}

impl State for AccountState {
  fn clone_box(&self) -> Box<dyn State> {
      Box::new(self.clone())
  }

  fn process_composite(
		&self,
		_input: &StateInputCompound,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		Ok(StateOutput::with(Box::new(AccountState::new(self.extra_info.clone())), indent))
	}

  fn process_variant(
		&self,
		_input: &StateInputCompound,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		Ok(StateOutput::with(Box::new(AccountState::new(self.extra_info.clone())), indent))
	}

  fn process_field(
    &self,
    _input: &StateInputCompoundItem,
    indent: u32
  ) -> Result<StateOutput, DecodeError> {
    Ok(StateOutput::with(Box::new(AccountState::new(self.extra_info.clone())), indent))
  }

  fn process_array(
      &self,
      input: &StateInputCompound,
      indent: u32
    ) -> Result<StateOutput, DecodeError> {
      Ok(StateOutput::with(Box::new(AccountState { 
        len: Some(input.items_count), 
        partial: vec![], 
        extra_info: self.extra_info.clone() 
      }), indent))
  }

  fn process_array_item(
      &self,
      input: &StateInputCompoundItem,
      indent: u32
    ) -> Result<StateOutput, DecodeError> {
      Ok(StateOutput::with(Box::new(AccountState { 
        len: Some(input.items_count), 
        partial: self.partial.clone(), 
        extra_info: self.extra_info.clone() 
      }), indent))
  }

  fn process_u8(
		&self,
		input: u8,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
    let current_len = self.len.ok_or(DecodeError::Unexpected(scale_decode::visitor::Unexpected::U16))?;

    let mut new_partial = self.partial.clone();
    new_partial.push(input);

    if new_partial.len() == current_len {
      let card = match  current_len {
          32 => {
            let account_id: [u8; 32] = new_partial.try_into()
                                          .map_err(|_err| DecodeError::Unexpected(scale_decode::visitor::Unexpected::U8))?;
            OutputCard {
              card: ParserCard::Id { id: AccountId32::new(account_id), base58prefix: self.extra_info.base58_prefix }, 
              indent
            }
          },
          20 => {
            let account_id: [u8; 20] = new_partial.try_into()
                                          .map_err(|_err| DecodeError::Unexpected(scale_decode::visitor::Unexpected::U8))?;
            OutputCard {
              card: ParserCard::Id20 { id: account_id, base58prefix: self.extra_info.base58_prefix }, 
              indent
            }
          },
          _ => return Err(DecodeError::Unexpected(scale_decode::visitor::Unexpected::U8))
      };

      Ok(StateOutput { next_state: Box::new(DefaultState), cards: vec![card], indent: indent })
    } else {
      Ok(StateOutput::with(Box::new(AccountState { len: self.len, partial: new_partial, extra_info: self.extra_info.clone() }), indent))
    }
  }
}

#[derive(Debug, Clone)]
pub struct BalanceState {
  extra_info: ExtraInfo
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
	) -> Result<StateOutput, DecodeError> {
    let output = self.process(input.to_string(), indent);

    return Ok(output);
  }

	fn process_u16(
		&self,
		input: u16,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_u32(
		&self,
		input: u32,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_u64(
		&self,
		input: u64,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_u128(
		&self,
		input: u128,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_u256<'a>(
		&self,
		input: &'a [u8; 32],
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let target_value = BigUint::from_bytes_le(input);
		let output = self.process(target_value.to_string(), indent);

    return Ok(output);
	}

	fn process_i8(
		&self,
		input: i8,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_i16<'scale, 'resolver>(
		&self,
		input: i16,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_i32(
		&self,
		input: i32,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_i64(
		&self,
		input: i64,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_i128(
		&self,
		input: i128,
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let output = self.process(input.to_string(), indent);

    return Ok(output);
	}

	fn process_i256<'a>(
		&self,
		input: &'a [u8; 32],
    indent: u32
	) -> Result<StateOutput, DecodeError> {
		let target_value = BigInt::from_signed_bytes_le(input);
		let output = self.process(target_value.to_string(), indent);

    return Ok(output);
	}
}