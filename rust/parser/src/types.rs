use parity_scale_codec::{Encode, Decode};
use crate::decoding_commons::OutputCard;
use sp_runtime::generic::Era;

use merkleized_metadata::{
	Proof, 
	ExtraInfo, 
	types::{Hash, ExtrinsicMetadata}
};

#[derive(Debug, Clone, Encode, Decode)]
pub struct MetadataProof {
    pub proof: Proof,
    pub extrinsic: ExtrinsicMetadata,
    pub extra_info: ExtraInfo,
}

#[derive(Debug, Clone, Decode, Encode)]
pub enum CheckMetadataHashMode {
	Disabled,
	Enabled,
}

#[derive(Default, Debug)]
pub struct IncludedInExtrinsic {
	pub check_metadata_hash_mode: Option<CheckMetadataHashMode>,
	pub mortality: Option<Era>,
	pub nonce: Option<u32>,
	pub cards: Vec<OutputCard>
}

#[derive(Default, Debug)]
pub struct IncludedInSignature {
	pub metadata_hash: Option<Hash>,
	pub genesis_hash: Option<Hash>,
	pub cards: Vec<OutputCard>
}