use parity_scale_codec::{Encode, Decode};

use merkleized_metadata::{
	Proof, 
	ExtraInfo, 
	types::ExtrinsicMetadata
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