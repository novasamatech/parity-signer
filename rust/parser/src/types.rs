use parity_scale_codec::{Decode, Encode};

use merkleized_metadata::{types::ExtrinsicMetadata, ExtraInfo, Proof};

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
