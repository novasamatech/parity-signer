use crate::{
  proof_verifier::verify_metadata_proof, state_machine::{
    CallPalletState, 
    DefaultState}, types::{CheckMetadataHashMode, Hash, IncludedInExtrinsic, IncludedInSignature, MetadataProof}, visitor::{CallCardsParser, TypeRegistry, TypeResolver}
};
use codec::{Decode};
use parser::decoding_commons::OutputCard;
use parser::cards::ParserCard;
use scale_decode::{visitor::decode_with_visitor};
use sp_core::H256;
use sp_runtime::generic::Era;

fn decode_special_included_in_extrinsic(identifier: &str, model: &mut IncludedInExtrinsic, data: &mut &[u8]) -> Result<bool, String> {
  match identifier {
      "CheckMetadataHash" => {
        let decoded = CheckMetadataHashMode::decode(data)
          .map_err(|e| format!("Failed to decode extension: {e}"))?;
        model.check_metadata_hash_mode = Some(decoded);
        Ok(true)
      },
      "CheckMortality" => {
        let era = Era::decode(data)
          .map_err(|e| format!("Failed to decode check mortality: {e}"))?;
        
        model.mortality = Some(era);

        let card = OutputCard {
          card: ParserCard::Era(era),
          indent: 0
        };

        model.cards.push(card);

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
        model.metadata_hash = decoded;
        Ok(true)
      },
      "CheckMortality" => {
        let block_hash = H256::decode(data)
          .map_err(|e| format!("Failed to decode check mortality: {e}"))?;

        let card = OutputCard {
          card: ParserCard::BlockHash(block_hash),
          indent: 0
        };

        model.cards.push(card);

        Ok(true)
      },
      "CheckGenesis" => {
        let genesis_hash = Hash::decode(data)
          .map_err(|e| format!("Failed to decode check genesis: {e}"))?;

        model.genesis_hash = Some(genesis_hash);

        Ok(true)
      },
      _ => Ok(false)
  }
}

pub fn decode_metadata_proof(
  data: &mut &[u8]
) -> Result<MetadataProof, String> {
  MetadataProof::decode(data).map_err(|e| format!("Failed to decode metadata: {e}"))
}

pub fn decode_call(
  data: &mut &[u8],
  metadata_proof: &MetadataProof,
) -> Result<Vec<OutputCard>, String> {
  let type_resolver = TypeResolver::new(metadata_proof.proof.leaves.iter());
  let type_registry = TypeRegistry::new(metadata_proof.proof.leaves.iter());

	let visitor = CallCardsParser::new(&type_registry, metadata_proof.extra_info.clone(), CallPalletState::default());

  let result = decode_with_visitor(
		data,
		metadata_proof.extrinsic.call_ty,
		&type_resolver,
		visitor,
	)
	.map_err(|e| format!("Failed to decode call: {e}"))?;

  Ok(result.cards)
}

pub fn decode_and_verify_extensions(
  data: &mut &[u8],
  metadata_proof: &MetadataProof,
) -> Result<Vec<OutputCard>, String> {
  let mut included_in_extrinsic = decode_included_in_extrinsic(data, metadata_proof)?;
  let mut included_in_signature = decode_included_in_signature(data, metadata_proof)?;

  match included_in_extrinsic.check_metadata_hash_mode {
    Some(CheckMetadataHashMode::Enabled) => {
      let expected_hash = included_in_signature.metadata_hash.ok_or_else(|| "Missing metadata hash")?;

      verify_metadata_proof(metadata_proof, expected_hash)?;

      let mut cards = vec![];

      cards.append(&mut included_in_extrinsic.cards);
      cards.append(&mut included_in_signature.cards);

      Ok(cards)
    },
    _ => Err("Only enabled mode supported".to_string())
  }
}

pub fn decode_included_in_extrinsic(
  data: &mut &[u8],
  metadata_proof: &MetadataProof,
) -> Result<IncludedInExtrinsic, String> {
  let type_resolver = TypeResolver::new(metadata_proof.proof.leaves.iter());
  let type_registry = TypeRegistry::new(metadata_proof.proof.leaves.iter());

  let mut included_in_extrinsic = IncludedInExtrinsic::default();

  for signed_ext in metadata_proof.extrinsic.signed_extensions.iter() {
    let is_decoded = decode_special_included_in_extrinsic(&signed_ext.identifier, &mut  included_in_extrinsic, data)?;

    if !is_decoded {
      let visitor = CallCardsParser::new(&type_registry, metadata_proof.extra_info.clone(), DefaultState::default());

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