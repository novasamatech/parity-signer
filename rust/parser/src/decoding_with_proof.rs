use crate::{
    call_state::CallPalletState,
    decoding_commons::OutputCard,
    error::{Error, ParserDecodingError},
    extensions_decoder::{
        ChargeTransactionPaymentInExtrinsicHandler, CheckGenesisInSignatureHandler,
        CheckMetadataHashInExtrinsicHandler, CheckMetadataHashInSignatureHandler,
        CheckMortalityInExtrinsicHandler, CheckMortalityInSignatureHandler,
        CheckNonceInExtrinsicHandler, CheckSpecVersionInSignatureHandler,
        CheckTxVersionInSignatureHandler, DefaultExtensionHandler, ExtensionCompoundHandler,
        ExtensionHandling, ExtensionHandlingParams, ExtensionsOutput,
    },
    state_machine::{StateMachineParser, TypeRegistry},
    types::{CheckMetadataHashMode, MetadataProof},
};

use merkleized_metadata::{types::Hash, verify_metadata_digest, TypeResolver};

use parity_scale_codec::DecodeLimit;
use scale_decode::visitor::decode_with_visitor;

/// To avoid out of memory issues during scale decoding
/// one need to decode_all_with_depth_limit with a proper depth limit.
/// Typically stack has 1MB limit, choose the depth base on it.
const DECODING_DEPTH_LIMIT: u32 = 1000;

fn verify_decoded_metadata_hash(
    output: &ExtensionsOutput,
    metadata_proof: &MetadataProof,
) -> Result<(), Error> {
    let metadata_hash_mode = output
        .check_metadata_hash_mode
        .as_ref()
        .ok_or_else(|| Error::Decoding(ParserDecodingError::CheckMetadataHashModeExpected))?;

    match metadata_hash_mode {
        CheckMetadataHashMode::Enabled => {
            let expected_hash = output
                .metadata_hash
                .ok_or_else(|| Error::MetadataHashMissing)?;

            let extrinsic_metadata_hash = metadata_proof.extrinsic.hash();

            let is_valid = verify_metadata_digest(
                metadata_proof.proof.clone(),
                extrinsic_metadata_hash,
                metadata_proof.extra_info.clone(),
                expected_hash,
            );

            if is_valid {
                Ok(())
            } else {
                Err(Error::MetadataHashMismatch)
            }
        }
        CheckMetadataHashMode::Disabled => Err(Error::MetadataHashDisabled),
    }
}

fn verify_genesis_hash(
    output: &ExtensionsOutput,
    expected_genesis_hash: &Hash,
) -> Result<(), Error> {
    if let Some(genesis_hash) = output.genesis_hash {
        if *expected_genesis_hash != genesis_hash {
            return Err(Error::Decoding(ParserDecodingError::GenesisHashMismatch));
        }
    }

    Ok(())
}

pub fn decode_metadata_proof(data: &mut &[u8]) -> Result<MetadataProof, Error> {
    MetadataProof::decode_with_depth_limit(DECODING_DEPTH_LIMIT, data)
        .map_err(|_| Error::Decoding(ParserDecodingError::MetadataProofExpected))
}

pub fn decode_call(
    data: &mut &[u8],
    metadata_proof: &MetadataProof,
) -> Result<Vec<OutputCard>, Error> {
    let type_resolver = TypeResolver::new(metadata_proof.proof.leaves.iter());
    let type_registry = TypeRegistry::new(metadata_proof.proof.leaves.iter());

    let visitor = StateMachineParser::new(
        &type_registry,
        metadata_proof.extra_info.clone(),
        CallPalletState,
    );

    let result = decode_with_visitor(
        data,
        metadata_proof.extrinsic.call_ty,
        &type_resolver,
        visitor,
    )
    .map_err(|e| Error::Decoding(ParserDecodingError::StateMachine(e)))?;

    Ok(result.cards)
}

pub fn decode_extensions(
    data: &mut &[u8],
    metadata_proof: &MetadataProof,
    expected_genesis_hash: &Hash,
) -> Result<Vec<OutputCard>, Error> {
    let type_resolver = TypeResolver::new(metadata_proof.proof.leaves.iter());
    let type_registry = TypeRegistry::new(metadata_proof.proof.leaves.iter());

    let mut output = ExtensionsOutput::default();

    let ext_handler = ExtensionCompoundHandler {
        handlers: vec![
            Box::new(CheckMetadataHashInExtrinsicHandler),
            Box::new(CheckMortalityInExtrinsicHandler),
            Box::new(CheckNonceInExtrinsicHandler),
            Box::new(ChargeTransactionPaymentInExtrinsicHandler),
            Box::new(DefaultExtensionHandler),
        ],
    };

    let params = ExtensionHandlingParams {
        type_resolver: &type_resolver,
        type_registry: &type_registry,
        extra_info: metadata_proof.extra_info.clone(),
    };

    for signed_ext in metadata_proof.extrinsic.signed_extensions.iter() {
        ext_handler.decode_extension(
            data,
            &signed_ext.identifier,
            signed_ext.included_in_extrinsic,
            &params,
            &mut output,
        )?;
    }

    let sig_handler = ExtensionCompoundHandler {
        handlers: vec![
            Box::new(CheckMetadataHashInSignatureHandler),
            Box::new(CheckMortalityInSignatureHandler),
            Box::new(CheckGenesisInSignatureHandler),
            Box::new(CheckTxVersionInSignatureHandler),
            Box::new(CheckSpecVersionInSignatureHandler),
            Box::new(DefaultExtensionHandler),
        ],
    };

    for signed_ext in metadata_proof.extrinsic.signed_extensions.iter() {
        sig_handler.decode_extension(
            data,
            &signed_ext.identifier,
            signed_ext.included_in_signed_data,
            &params,
            &mut output,
        )?;
    }

    verify_decoded_metadata_hash(&output, metadata_proof)?;
    verify_genesis_hash(&output, expected_genesis_hash)?;

    Ok(output.cards)
}
