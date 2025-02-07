extern crate alloc;

mod types;

pub use types::MetadataProof;

#[cfg(test)]
mod tests;

mod visitor;
mod extrinsic_decoder;