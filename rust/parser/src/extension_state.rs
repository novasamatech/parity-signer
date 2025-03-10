use crate::{
    number_state::{NonceCardProducer, NumberState, TipCardProducer},
    state::{State, StateError, StateInputCompound, StateInputCompoundItem, StateOutput},
};

use merkleized_metadata::ExtraInfo;

#[derive(Debug, Clone, Default)]
pub struct NonceState;

impl State for NonceState {
    fn clone_box(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }

    fn process_composite(
        &self,
        _input: &StateInputCompound,
        indent: u32,
    ) -> Result<StateOutput, StateError> {
        Ok(StateOutput::with(Box::new(NonceState), indent))
    }

    fn process_field(
        &self,
        _input: &StateInputCompoundItem,
        indent: u32,
    ) -> Result<StateOutput, StateError> {
        Ok(StateOutput::with(
            Box::new(NumberState::<NonceCardProducer>::nonce_state()),
            indent,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct ChargeTransactionPaymentState(pub ExtraInfo);

impl State for ChargeTransactionPaymentState {
    fn clone_box(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }

    fn process_composite(
        &self,
        _input: &StateInputCompound,
        indent: u32,
    ) -> Result<StateOutput, StateError> {
        Ok(StateOutput::with(self.clone_box(), indent))
    }

    fn process_field(
        &self,
        _input: &StateInputCompoundItem,
        indent: u32,
    ) -> Result<StateOutput, StateError> {
        let next_state = Box::new(NumberState::<TipCardProducer>::tip_state(self.0.clone()));
        Ok(StateOutput::with(next_state, indent))
    }
}
