use crate::{
  state::{State, StateInputCompound, StateInputCompoundItem, StateOutput, StateError},
  cards::ParserCard,
  decoding_commons::OutputCard,
  default_state::DefaultState
};

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
	) -> Result<StateOutput, StateError> {
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
    input: &StateInputCompoundItem,
    indent: u32
  ) -> Result<StateOutput, StateError> {
    // expecting single field as method variant inside pallet field
    if input.items_count == 1 {
      Ok(StateOutput::with(Box::new(CallMethodState), indent + 1))
    } else {
      Ok(StateOutput::with(Box::new(DefaultState), indent + 1))
    }
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
	) -> Result<StateOutput, StateError> {
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