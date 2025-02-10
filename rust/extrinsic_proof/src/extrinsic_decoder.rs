use std::default;

use crate::{
  types::MetadataProof, 
  visitor::{CallCardsParser, TypeResolver},
};
use parser::decoding_commons::OutputCard;
use scale_decode::visitor::decode_with_visitor;

pub fn decode_call(
  call: &mut &[u8],
  proof_metadata: &MetadataProof,
) -> Result<Vec<OutputCard>, String> {
  let type_resolver = TypeResolver::new(proof_metadata.proof.leaves.iter());

	let visitor = CallCardsParser::default();

  let result = decode_with_visitor(
		call,
		proof_metadata.extrinsic.call_ty,
		&type_resolver,
		visitor,
	)
	.map_err(|e| format!("Failed to decode call: {e}"))?;

  Ok(result.cards)
}

pub fn decode_extensions(
  data: &mut &[u8],
  proof_metadata: &MetadataProof,
) -> Result<Vec<OutputCard>, String> {
  let type_resolver = TypeResolver::new(proof_metadata.proof.leaves.iter());

  let mut cards: Vec<OutputCard> = Vec::new();

  for signed_ext in proof_metadata.extrinsic.signed_extensions.iter() {
    let visitor = CallCardsParser::default();

    let mut result = decode_with_visitor(
      data,
      signed_ext.included_in_extrinsic,
      &type_resolver,
      visitor,
    )
    .map_err(|e| format!("Failed to decode call: {e}"))?;

    cards.append(&mut result.cards);
  }

  for signed_ext in proof_metadata.extrinsic.signed_extensions.iter() {
    let visitor = CallCardsParser::default();

    let mut result = decode_with_visitor(
      data,
      signed_ext.included_in_signed_data,
      &type_resolver,
      visitor,
    )
    .map_err(|e| format!("Failed to decode call: {e}"))?;

    cards.append(&mut result.cards);
  }

  Ok(cards)
}