use crate::{
  state_machine::{CallPalletState, DefaultState}, types::{CheckMetadataHashMode, IncludedInExtrinsic, IncludedInSignature, MetadataProof}, visitor::{CallCardsParser, TypeRegistry, TypeResolver}
};
use codec::Decode;
use parser::decoding_commons::OutputCard;
use scale_decode::{visitor::decode_with_visitor};

fn decode_special_included_in_extrinsic(identifier: &str, model: &mut IncludedInExtrinsic, data: &mut &[u8]) -> Result<bool, String> {
  match identifier {
      "CheckMetadataHash" => {
        let decoded = CheckMetadataHashMode::decode(data)
          .map_err(|e| format!("Failed to decode extension: {e}"))?;
        model.checkMetadataHashMode = Some(decoded);
        Ok(true)
      },
      _ => Ok(false)
  }
}

fn decode_special_included_in_signature(identifier: &str, model: &mut IncludedInSignature, data: &mut &[u8]) -> Result<bool, String> {
  match identifier {
      "CheckMetadataHash" => {
        let decoded = Option::decode(data)
          .map_err(|e| format!("Failed to decode extension: {e}"))?;
        model.metadataHash = decoded;
        Ok(true)
      },
      _ => Ok(false)
  }
}

pub fn decode_call(
  call: &mut &[u8],
  proof_metadata: &MetadataProof,
) -> Result<Vec<OutputCard>, String> {
  let type_resolver = TypeResolver::new(proof_metadata.proof.leaves.iter());
  let type_registry = TypeRegistry::new(proof_metadata.proof.leaves.iter());

	let visitor = CallCardsParser::new(&type_registry, proof_metadata.extra_info.clone(), CallPalletState::default());

  let result = decode_with_visitor(
		call,
		proof_metadata.extrinsic.call_ty,
		&type_resolver,
		visitor,
	)
	.map_err(|e| format!("Failed to decode call: {e}"))?;

  Ok(result.cards)
}

pub fn decode_included_in_extrinsic(
  data: &mut &[u8],
  proof_metadata: &MetadataProof,
) -> Result<IncludedInExtrinsic, String> {
  let type_resolver = TypeResolver::new(proof_metadata.proof.leaves.iter());
  let type_registry = TypeRegistry::new(proof_metadata.proof.leaves.iter());

  let mut included_in_extrinsic = IncludedInExtrinsic::default();

  for signed_ext in proof_metadata.extrinsic.signed_extensions.iter() {
    let is_decoded = decode_special_included_in_extrinsic(&signed_ext.identifier, &mut  included_in_extrinsic, data)?;

    if !is_decoded {
      let visitor = CallCardsParser::new(&type_registry, proof_metadata.extra_info.clone(), DefaultState::default());

      let mut result = decode_with_visitor(
        data,
        signed_ext.included_in_extrinsic,
        &type_resolver,
        visitor,
      )
      .map_err(|e| format!("Failed to decode call: {e}"))?;

      included_in_extrinsic.cards.append(&mut result.cards);
    }
  }

  Ok(included_in_extrinsic)
}

pub fn decode_included_in_signature(
  data: &mut &[u8],
  proof_metadata: &MetadataProof,
) -> Result<IncludedInSignature, String> {
  let type_resolver = TypeResolver::new(proof_metadata.proof.leaves.iter());
  let type_registry = TypeRegistry::new(proof_metadata.proof.leaves.iter());

  let mut included_in_signature = IncludedInSignature::default();

  for signed_ext in proof_metadata.extrinsic.signed_extensions.iter() {
    let is_decoded = decode_special_included_in_signature(&signed_ext.identifier, &mut included_in_signature, data)?;

    if !is_decoded {
      let visitor = CallCardsParser::new(&type_registry, proof_metadata.extra_info.clone(), DefaultState::default());

      let mut result = decode_with_visitor(
        data,
        signed_ext.included_in_signed_data,
        &type_resolver,
        visitor,
      )
      .map_err(|e| format!("Failed to decode call: {e}"))?;

      included_in_signature.cards.append(&mut result.cards);
    }
  }

  Ok(included_in_signature)
}