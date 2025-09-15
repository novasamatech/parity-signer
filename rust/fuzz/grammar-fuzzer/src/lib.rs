//! # Merkleized metadata
//!
//! This crate provides the merkleization of [`frame_metadata`] as described in
//! [RFC78](https://polkadot-fellows.github.io/RFCs/approved/0078-merkleized-metadata.html).
//!
//! It exports the following main functions:
//!
//! - [`generate_metadata_digest`]: This generates the [`MetadataDigest`] for the given `metadata`.
//!   The hash of this digest is what is called the "metadata hash" in the RFC.
//!
//! - [`generate_proof_for_extrinsic`]: This generates a proof that contains all the types required
//!   for decoding the given `extrinsic`.
//!
//! - [`generate_proof_for_extrinsic_parts`]: Similar to the previous one, but doesn't require the
//!   full extrinsic to be passed. However, this may increases the proof size, see the documentation
//!   of the function for more information.
//!
//! These functions are the main entry point providing all the functionality expected from this
//! crate. The crate itself is by default `no_std` compatible.

extern crate alloc;

use alloc::string::String;
use codec::{Decode, Encode};
use extrinsic_decoder::{
    decode_extrinsic_and_collect_type_ids, decode_extrinsic_parts_and_collect_type_ids,
};
use frame_metadata::RuntimeMetadata;
use from_frame_metadata::FrameMetadataPrepared;
use merkle_tree::MerkleTree;
pub use merkle_tree::Proof;
use types::MetadataDigest;

mod extrinsic_decoder;
mod from_frame_metadata;
pub mod merkle_tree;
pub mod types;

/// Extra information that is required to generate the [`MetadataDigest`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, autarkie::Grammar, Encode, Decode)]
pub struct ExtraInfo {
    /// The spec version of the runtime.
    pub spec_version: u32,
    /// The spec name of the runtime.
    pub spec_name: String,
    /// The base58 prefix for addresses.
    pub base58_prefix: u16,
    /// The number of decimals of the primary token.
    pub decimals: u8,
    /// The token symbol of the primary token.
    pub token_symbol: String,
}

/// Generate the [`MetadataDigest`] using the given `extra_info`.
pub fn generate_metadata_digest(
    metadata: &RuntimeMetadata,
    extra_info: ExtraInfo,
) -> Result<MetadataDigest, String> {
    let prepared = FrameMetadataPrepared::prepare(metadata)?;

    let type_information = prepared.as_type_information()?;

    let tree_root = MerkleTree::new(type_information.types).root();

    Ok(MetadataDigest::V1 {
        types_tree_root: tree_root,
        extrinsic_metadata_hash: type_information.extrinsic_metadata.hash(),
        spec_version: extra_info.spec_version,
        spec_name: extra_info.spec_name,
        base58_prefix: extra_info.base58_prefix,
        decimals: extra_info.decimals,
        token_symbol: extra_info.token_symbol,
    })
}

/// Generate a proof for the given `extrinsic` using the given `metadata`.
///
/// If `additional_data` is `Some(_)`, it will be decoded as well and the
/// required type information are included in the proof.
///
/// If the full `extrinsic` is not available, [`generate_proof_for_extrinsic_parts`]
/// is maybe the better option as it only requires the `call` and the
/// `additional_data`.
pub fn generate_proof_for_extrinsic(
    mut extrinsic: &[u8],
    additional_signed: Option<&[u8]>,
    metadata: &RuntimeMetadata,
) -> Result<Proof, String> {
    let prepared = FrameMetadataPrepared::prepare(metadata)?;
    let type_information = prepared.as_type_information()?;

    let extrinsic = &mut extrinsic;

    let accessed_types = decode_extrinsic_and_collect_type_ids(
        extrinsic,
        additional_signed,
        &type_information,
        type_information.types.values(),
    )?;

    if !extrinsic.is_empty() {
        return Err("Bytes left in `extrinsic` after decoding".into());
    }

    MerkleTree::new(type_information.types).build_proof(accessed_types)
}

/// Verify that the given `proof` can be used to decode the given `extrinsic`.
///
/// This is mainly useful for tests/fuzzing.
pub fn verify_proof(
    mut extrinsic: &[u8],
    additional_signed: Option<&[u8]>,
    metadata: &RuntimeMetadata,
    proof: &Proof,
) -> Result<(), String> {
    let prepared = FrameMetadataPrepared::prepare(metadata)?;
    let type_information = prepared.as_type_information()?;

    decode_extrinsic_and_collect_type_ids(
        &mut extrinsic,
        additional_signed,
        &type_information,
        proof.leaves.iter(),
    )
    .map(drop)
}

/// Data that is required for a signed extrinsic.
pub struct SignedExtrinsicData<'a> {
    /// The data that is directly included in the extrinsic.
    pub included_in_extrinsic: &'a [u8],
    /// The data that is included in the signed data.
    pub included_in_signed_data: &'a [u8],
}

/// Generate a proof for the given extrinsic parts using the given `metadata`.
///
/// This generates a proof that contains all the types required to decode an
/// extrinsic that is build using the given `call` and `signed_ext_data`. When
/// `signed_ext_data` is `Some(_)` it is assumed that the extrinsic is signed
/// and thus, all the signed extension types are included in the proof as well.
/// The same applies for the `sigature` and `address` types which are only
/// included when `signed_ext_data` is `Some(_)`.
pub fn generate_proof_for_extrinsic_parts(
    mut call: &[u8],
    signed_ext_data: Option<SignedExtrinsicData>,
    metadata: &RuntimeMetadata,
) -> Result<Proof, String> {
    let prepared = FrameMetadataPrepared::prepare(metadata)?;
    let type_information = prepared.as_type_information()?;

    let call = &mut call;

    let accessed_types = decode_extrinsic_parts_and_collect_type_ids(
        call,
        signed_ext_data,
        &type_information,
        type_information.types.values(),
    )?;

    if !call.is_empty() {
        return Err("Bytes left in `call` after decoding".into());
    }

    MerkleTree::new(type_information.types).build_proof(accessed_types)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::frame_metadata::RuntimeMetadataPrefixed;
    use array_bytes::{Dehexify, Hexify};
    use codec::Decode;
    use std::fs;

    const FIXTURES: &[(&str, &str)] = &[
        (
            "rococo_metadata_v15",
            "0x6619a31025a9a14086a34da4ede7ed61258b9f55c12baae8bc801317869d2dfb",
        ),
        (
            "polkadot_metadata_v15",
            "0x72b3e70cb722edeb45a9380720ecad79b09b4113ab2dee5f5d974f170fb77a7e",
        ),
        (
            "kusama_metadata_v15",
            "0x23d7a31034edf29f4a5977ffc3075aba8087c422026e9bf4aaea8bc8192d6a23",
        ),
        (
            "acala_metadata_v15",
            "0xbd64dee496517c5288c47014fe0f57c2e12e42a7d627caeafa95e9f992e7e774",
        ),
        (
            "moonbeam_metadata_v15",
            "0x1339dc558887eb12f454586ef324c36bd3a1990000e17fbba6311f6ae55af676",
        ),
        (
            "hydradx_metadata_v15",
            "0xa11f4b8cb2515bf5dc0f8f7c04c0602d72e97892c562dafc3bb1d526d36ab838",
        ),
    ];

    #[test]
    fn calculate_metadata_digest_works() {
        let extra_info = ExtraInfo {
            spec_version: 1,
            spec_name: "nice".into(),
            base58_prefix: 1,
            decimals: 1,
            token_symbol: "lol".into(),
        };

        for (fixture, expected_hash) in FIXTURES {
            println!("Processing: {fixture}");

            let metadata = String::from_utf8(
                fs::read(format!("{}/fixtures/{fixture}", env!("CARGO_MANIFEST_DIR"))).unwrap(),
            )
            .unwrap();

            let metadata = Option::<Vec<u8>>::decode(
                &mut &Vec::<u8>::dehexify(metadata.strip_suffix("\n").unwrap()).unwrap()[..],
            )
            .unwrap()
            .unwrap();

            let metadata = RuntimeMetadataPrefixed::decode(&mut &metadata[..])
                .unwrap()
                .1;

            let digest = generate_metadata_digest(&metadata, extra_info.clone()).unwrap();
            assert_eq!(*expected_hash, digest.hash().hexify_prefixed());

            let prepared = FrameMetadataPrepared::prepare(&metadata).unwrap();

            let type_information = prepared.as_type_information().unwrap();
            type_information
                .types
                .values()
                .fold(None, |p, v| match p {
                    None => Some(v.clone()),
                    Some(p) => {
                        if p.type_id.0 < v.type_id.0
                            || p.type_def
                                .as_enumeration()
                                .and_then(|p| v.type_def.as_enumeration().map(|v| (p, v)))
                                .map_or(false, |(p, v)| p.index.0 < v.index.0)
                        {
                            Some(v.clone())
                        } else {
                            panic!("Invalid: {:?} < {:?}", p, v)
                        }
                    }
                })
                .unwrap();
        }
    }
}
