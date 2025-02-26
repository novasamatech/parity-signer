extern crate alloc;

mod types;
mod visitor;
mod extrinsic_decoder;
mod state_machine;
mod proof_verifier;

pub use types::MetadataProof;
pub use extrinsic_decoder::{decode_metadata_proof, decode_call, decode_and_verify_extensions};

#[cfg(test)]
mod tests;