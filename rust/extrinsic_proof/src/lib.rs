extern crate alloc;

mod types;
mod visitor;
mod extrinsic_decoder;
mod state_machine;
mod proof_verifier;

pub use types::MetadataProof;
pub use extrinsic_decoder::decode_call;

#[cfg(test)]
mod tests;