use crate::{
  types::MetadataProof, 
  visitor::{TypeResolver, CollectAccessedTypes},
};
use parser::decoding_commons::OutputCard;
use scale_decode::visitor::decode_with_visitor;

pub fn decode_call<'a>(
  call: &mut &[u8],
  proof_metadata: &MetadataProof,
) -> Result<Vec<OutputCard>, String> {
  let type_resolver = TypeResolver::new(proof_metadata.proof.leaves.iter());

	let visitor = CollectAccessedTypes::default();

  let mut visitor = decode_with_visitor(
		call,
		proof_metadata.extrinsic.call_ty,
		&type_resolver,
		visitor,
	)
	.map_err(|e| format!("Failed to decode call: {e}"))?;

  Ok(vec![])
}