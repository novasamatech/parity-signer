use crate::{
  default_state::DefaultState,
  call_state::CallPalletState,
  state_machine::{StateMachineParser, TypeRegistry},
  decoding_commons::OutputCard,
  cards::ParserCard,
  types::{IncludedInExtrinsic, IncludedInSignature, CheckMetadataHashMode, MetadataProof},
  error::{Error, ParserDecodingError}
};

use merkleized_metadata::{
  verify_metadata_digest,
  TypeResolver,
  types::Hash
};

use parity_scale_codec::Decode;
use scale_decode::visitor::decode_with_visitor;
use sp_core::H256;
use sp_runtime::generic::Era;

fn decode_special_included_in_extrinsic(identifier: &str, model: &mut IncludedInExtrinsic, data: &mut &[u8]) -> Result<bool, ParserDecodingError> {
  match identifier {
      "CheckMetadataHash" => {
        let decoded = CheckMetadataHashMode::decode(data)
          .map_err(|_| ParserDecodingError::CheckMetadataHashModeExpected)?;
        model.check_metadata_hash_mode = Some(decoded);
        Ok(true)
      },
      "CheckMortality" => {
        let era = Era::decode(data)
          .map_err(|_| ParserDecodingError::Era)?;
        
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

fn decode_special_included_in_signature(identifier: &str, model: &mut IncludedInSignature, data: &mut &[u8]) -> Result<bool, ParserDecodingError> {
  match identifier {
      "CheckMetadataHash" => {
        let decoded = Option::decode(data)
          .map_err(|_| ParserDecodingError::MetadataHashExpected)?;
        model.metadata_hash = decoded;
        Ok(true)
      },
      "CheckMortality" => {
        let block_hash = H256::decode(data)
          .map_err(|_| ParserDecodingError::BlockHashExpected)?;

        let card = OutputCard {
          card: ParserCard::BlockHash(block_hash),
          indent: 0
        };

        model.cards.push(card);

        Ok(true)
      },
      "CheckGenesis" => {
        let genesis_hash = Hash::decode(data)
          .map_err(|_| ParserDecodingError::GenesisHashExpected)?;

        model.genesis_hash = Some(genesis_hash);

        Ok(true)
      },
      _ => Ok(false)
  }
}

fn decode_included_in_extrinsic(
  data: &mut &[u8],
  metadata_proof: &MetadataProof,
) -> Result<IncludedInExtrinsic, ParserDecodingError> {
  let type_resolver = TypeResolver::new(metadata_proof.proof.leaves.iter());
  let type_registry = TypeRegistry::new(metadata_proof.proof.leaves.iter());

  let mut included_in_extrinsic = IncludedInExtrinsic::default();

  for signed_ext in metadata_proof.extrinsic.signed_extensions.iter() {
    let is_decoded = decode_special_included_in_extrinsic(&signed_ext.identifier, &mut  included_in_extrinsic, data)?;

    if !is_decoded {
      let visitor = StateMachineParser::new(&type_registry, metadata_proof.extra_info.clone(), DefaultState::default());

      let mut result = decode_with_visitor(
        data,
        signed_ext.included_in_extrinsic,
        &type_resolver,
        visitor,
      ).map_err(|e| ParserDecodingError::StateMachine(e))?;

      included_in_extrinsic.cards.append(&mut result.cards);
    }
  }

  Ok(included_in_extrinsic)
}

fn decode_included_in_signature(
  data: &mut &[u8],
  proof_metadata: &MetadataProof,
) -> Result<IncludedInSignature, ParserDecodingError> {
  let type_resolver = TypeResolver::new(proof_metadata.proof.leaves.iter());
  let type_registry = TypeRegistry::new(proof_metadata.proof.leaves.iter());

  let mut included_in_signature = IncludedInSignature::default();

  for signed_ext in proof_metadata.extrinsic.signed_extensions.iter() {
    let is_decoded = decode_special_included_in_signature(&signed_ext.identifier, &mut included_in_signature, data)?;

    if !is_decoded {
      let visitor = StateMachineParser::new(&type_registry, proof_metadata.extra_info.clone(), DefaultState::default());

      let mut result = decode_with_visitor(
        data,
        signed_ext.included_in_signed_data,
        &type_resolver,
        visitor,
      ).map_err(|e| ParserDecodingError::StateMachine(e))?;

      included_in_signature.cards.append(&mut result.cards);
    }
  }

  Ok(included_in_signature)
}

fn verify_decoded_metadata_hash(
  included_in_extrinsic: &IncludedInExtrinsic, 
  included_in_signature: &IncludedInSignature,
  metadata_proof: &MetadataProof
) -> Result<(), Error> {
  let metadata_hash_mode = included_in_extrinsic.check_metadata_hash_mode.as_ref()
    .ok_or_else(|| Error::Decoding(ParserDecodingError::CheckMetadataHashModeExpected))?;

  match metadata_hash_mode {
    CheckMetadataHashMode::Enabled => {
        let expected_hash = included_in_signature.metadata_hash
          .ok_or_else(|| Error::MetadataHashMissing)?;

        let extrinsic_metadata_hash = metadata_proof.extrinsic.hash();

        let is_valid = verify_metadata_digest(
          metadata_proof.proof.clone(), 
          extrinsic_metadata_hash, 
          metadata_proof.extra_info.clone(),
           expected_hash
        );

        if is_valid {
          Ok(())
        } else {
          Err(Error::MetadataHashMismatch)
        }
      },
      CheckMetadataHashMode::Disabled => {
        Err(Error::MetadataHashDisabled)
      }
  }
}

pub fn decode_metadata_proof(
  data: &mut &[u8]
) -> Result<MetadataProof, Error> {
  MetadataProof::decode(data).map_err(|_| Error::Decoding(ParserDecodingError::MetadataProofExpected))
}

pub fn decode_call(
  data: &mut &[u8],
  metadata_proof: &MetadataProof,
) -> Result<Vec<OutputCard>, Error> {
  let type_resolver = TypeResolver::new(metadata_proof.proof.leaves.iter());
  let type_registry = TypeRegistry::new(metadata_proof.proof.leaves.iter());

	let visitor = StateMachineParser::new(&type_registry, metadata_proof.extra_info.clone(), CallPalletState::default());
  
  let result = decode_with_visitor(
		data,
		metadata_proof.extrinsic.call_ty,
		&type_resolver,
		visitor,
	).map_err(|e| Error::Decoding(ParserDecodingError::StateMachine(e)))?;

  Ok(result.cards)
}

pub fn decode_extensions(
  data: &mut &[u8],
  metadata_proof: &MetadataProof
) -> Result<Vec<OutputCard>, Error> {
  let included_in_extrinsic = decode_included_in_extrinsic(data, metadata_proof)
    .map_err(|e| Error::Decoding(e))?;

  let mut included_in_signature = decode_included_in_signature(data, metadata_proof)
    .map_err(|e| Error::Decoding(e))?;

  verify_decoded_metadata_hash(&included_in_extrinsic, &included_in_signature, metadata_proof)?;

  let mut all_cards = included_in_extrinsic.cards;
  all_cards.append(&mut included_in_signature.cards);

  Ok(all_cards)
}