use crate::{
    cards::ParserCard,
    decoding_commons::OutputCard,
    error::ParserDecodingError,
    extension_state::{ChargeTransactionPaymentState, NonceState},
    number_state::{NumberState, SpecVersionCardProducer, TxVersionCardProducer},
    state::DefaultState,
    state_machine::{StateMachineParser, TypeRegistry},
    types::CheckMetadataHashMode,
};

use merkleized_metadata::{
    types::{Hash, TypeRef},
    ExtraInfo, TypeResolver,
};

use parity_scale_codec::Decode;
use scale_decode::visitor::decode_with_visitor;
use sp_core::H256;
use sp_runtime::generic::Era;

type ExtensionDecodingValue = Result<bool, ParserDecodingError>;

#[derive(Default, Debug)]
pub struct ExtensionsOutput {
    pub check_metadata_hash_mode: Option<CheckMetadataHashMode>,
    pub metadata_hash: Option<Hash>,
    pub genesis_hash: Option<Hash>,
    pub cards: Vec<OutputCard>,
}

pub struct ExtensionHandlingParams<'resolver, 'registry> {
    pub type_resolver: &'resolver TypeResolver,
    pub type_registry: &'registry TypeRegistry,
    pub extra_info: ExtraInfo,
}

pub trait ExtensionHandling {
    fn decode_extension(
        &self,
        data: &mut &[u8],
        identifier: &str,
        type_ref: TypeRef,
        params: &ExtensionHandlingParams,
        output: &mut ExtensionsOutput,
    ) -> ExtensionDecodingValue;
}

pub struct ExtensionCompoundHandler {
    pub handlers: Vec<Box<dyn ExtensionHandling>>,
}

impl ExtensionHandling for ExtensionCompoundHandler {
    fn decode_extension(
        &self,
        data: &mut &[u8],
        identifier: &str,
        type_ref: TypeRef,
        params: &ExtensionHandlingParams,
        output: &mut ExtensionsOutput,
    ) -> ExtensionDecodingValue {
        for handler in self.handlers.iter() {
            if handler.decode_extension(data, identifier, type_ref, params, output)? {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

pub struct DefaultExtensionHandler;

impl ExtensionHandling for DefaultExtensionHandler {
    fn decode_extension(
        &self,
        data: &mut &[u8],
        _identifier: &str,
        type_ref: TypeRef,
        params: &ExtensionHandlingParams,
        output: &mut ExtensionsOutput,
    ) -> ExtensionDecodingValue {
        let visitor = StateMachineParser::new(
            params.type_registry,
            params.extra_info.clone(),
            DefaultState::default(),
        );

        let mut result = decode_with_visitor(data, type_ref, params.type_resolver, visitor)
            .map_err(|e| ParserDecodingError::StateMachine(e))?;

        output.cards.append(&mut result.cards);

        Ok(true)
    }
}

const CHECK_METADATAHASH_EXTENSION: &str = "CheckMetadataHash";

pub struct CheckMetadataHashInExtrinsicHandler;

impl ExtensionHandling for CheckMetadataHashInExtrinsicHandler {
    fn decode_extension(
        &self,
        data: &mut &[u8],
        identifier: &str,
        _type_ref: TypeRef,
        _params: &ExtensionHandlingParams,
        output: &mut ExtensionsOutput,
    ) -> ExtensionDecodingValue {
        if identifier != CHECK_METADATAHASH_EXTENSION {
            return Ok(false);
        }

        let decoded = CheckMetadataHashMode::decode(data)
            .map_err(|_| ParserDecodingError::CheckMetadataHashModeExpected)?;

        output.check_metadata_hash_mode = Some(decoded);

        Ok(true)
    }
}

pub struct CheckMetadataHashInSignatureHandler;

impl ExtensionHandling for CheckMetadataHashInSignatureHandler {
    fn decode_extension(
        &self,
        data: &mut &[u8],
        identifier: &str,
        _type_ref: TypeRef,
        _params: &ExtensionHandlingParams,
        output: &mut ExtensionsOutput,
    ) -> ExtensionDecodingValue {
        if identifier != CHECK_METADATAHASH_EXTENSION {
            return Ok(false);
        }

        let decoded =
            Option::decode(data).map_err(|_| ParserDecodingError::MetadataHashExpected)?;

        output.metadata_hash = decoded;

        Ok(true)
    }
}

const CHECK_MORTALITY_EXTENSION: &str = "CheckMortality";

pub struct CheckMortalityInExtrinsicHandler;

impl ExtensionHandling for CheckMortalityInExtrinsicHandler {
    fn decode_extension(
        &self,
        data: &mut &[u8],
        identifier: &str,
        _type_ref: TypeRef,
        _params: &ExtensionHandlingParams,
        output: &mut ExtensionsOutput,
    ) -> ExtensionDecodingValue {
        if identifier != CHECK_MORTALITY_EXTENSION {
            return Ok(false);
        }

        let era = Era::decode(data).map_err(|_| ParserDecodingError::Era)?;

        output.cards.push(OutputCard {
            card: ParserCard::Era(era),
            indent: 0,
        });

        Ok(true)
    }
}

pub struct CheckMortalityInSignatureHandler;

impl ExtensionHandling for CheckMortalityInSignatureHandler {
    fn decode_extension(
        &self,
        data: &mut &[u8],
        identifier: &str,
        _type_ref: TypeRef,
        _params: &ExtensionHandlingParams,
        output: &mut ExtensionsOutput,
    ) -> ExtensionDecodingValue {
        if identifier != CHECK_MORTALITY_EXTENSION {
            return Ok(false);
        }

        let block_hash = H256::decode(data).map_err(|_| ParserDecodingError::BlockHashExpected)?;

        output.cards.push(OutputCard {
            card: ParserCard::BlockHash(block_hash),
            indent: 0,
        });

        Ok(true)
    }
}

const CHECK_GENESIS_EXTENSION: &str = "CheckGenesis";

pub struct CheckGenesisInSignatureHandler;

impl ExtensionHandling for CheckGenesisInSignatureHandler {
    fn decode_extension(
        &self,
        data: &mut &[u8],
        identifier: &str,
        _type_ref: TypeRef,
        _params: &ExtensionHandlingParams,
        output: &mut ExtensionsOutput,
    ) -> ExtensionDecodingValue {
        if identifier != CHECK_GENESIS_EXTENSION {
            return Ok(false);
        }

        let genesis_hash =
            Hash::decode(data).map_err(|_| ParserDecodingError::GenesisHashExpected)?;

        output.genesis_hash = Some(genesis_hash);

        Ok(true)
    }
}

const CHECK_NONCE_EXTENSION: &str = "CheckNonce";

pub struct CheckNonceInExtrinsicHandler;

impl ExtensionHandling for CheckNonceInExtrinsicHandler {
    fn decode_extension(
        &self,
        data: &mut &[u8],
        identifier: &str,
        type_ref: TypeRef,
        params: &ExtensionHandlingParams,
        output: &mut ExtensionsOutput,
    ) -> ExtensionDecodingValue {
        if identifier != CHECK_NONCE_EXTENSION {
            return Ok(false);
        }

        let visitor =
            StateMachineParser::new(params.type_registry, params.extra_info.clone(), NonceState);

        let mut result = decode_with_visitor(data, type_ref, params.type_resolver, visitor)
            .map_err(|e| ParserDecodingError::StateMachine(e))?;

        output.cards.append(&mut result.cards);

        Ok(true)
    }
}

const CHECK_TX_VERSION_EXTENSION: &str = "CheckTxVersion";

pub struct CheckTxVersionInSignatureHandler;

impl ExtensionHandling for CheckTxVersionInSignatureHandler {
    fn decode_extension(
        &self,
        data: &mut &[u8],
        identifier: &str,
        type_ref: TypeRef,
        params: &ExtensionHandlingParams,
        output: &mut ExtensionsOutput,
    ) -> ExtensionDecodingValue {
        if identifier != CHECK_TX_VERSION_EXTENSION {
            return Ok(false);
        }

        let visitor = StateMachineParser::new(
            params.type_registry,
            params.extra_info.clone(),
            NumberState::<TxVersionCardProducer>::tx_version_state(),
        );

        let mut result = decode_with_visitor(data, type_ref, params.type_resolver, visitor)
            .map_err(|e| ParserDecodingError::StateMachine(e))?;

        output.cards.append(&mut result.cards);

        Ok(true)
    }
}

const CHECK_SPEC_VERSION_EXTENSION: &str = "CheckSpecVersion";

pub struct CheckSpecVersionInSignatureHandler;

impl ExtensionHandling for CheckSpecVersionInSignatureHandler {
    fn decode_extension(
        &self,
        data: &mut &[u8],
        identifier: &str,
        type_ref: TypeRef,
        params: &ExtensionHandlingParams,
        output: &mut ExtensionsOutput,
    ) -> ExtensionDecodingValue {
        if identifier != CHECK_SPEC_VERSION_EXTENSION {
            return Ok(false);
        }

        let visitor = StateMachineParser::new(
            params.type_registry,
            params.extra_info.clone(),
            NumberState::<SpecVersionCardProducer>::spec_version_state(params.extra_info.clone()),
        );

        let mut result = decode_with_visitor(data, type_ref, params.type_resolver, visitor)
            .map_err(|e| ParserDecodingError::StateMachine(e))?;

        output.cards.append(&mut result.cards);

        Ok(true)
    }
}

const CHARGE_TRANSACTION_PAYMENT_EXTENSION: &str = "ChargeTransactionPayment";

pub struct ChargeTransactionPaymentInExtrinsicHandler;

impl ExtensionHandling for ChargeTransactionPaymentInExtrinsicHandler {
    fn decode_extension(
        &self,
        data: &mut &[u8],
        identifier: &str,
        type_ref: TypeRef,
        params: &ExtensionHandlingParams,
        output: &mut ExtensionsOutput,
    ) -> ExtensionDecodingValue {
        if identifier != CHARGE_TRANSACTION_PAYMENT_EXTENSION {
            return Ok(false);
        }

        let visitor = StateMachineParser::new(
            params.type_registry,
            params.extra_info.clone(),
            ChargeTransactionPaymentState(params.extra_info.clone()),
        );

        let mut result = decode_with_visitor(data, type_ref, params.type_resolver, visitor)
            .map_err(|e| ParserDecodingError::StateMachine(e))?;

        output.cards.append(&mut result.cards);

        Ok(true)
    }
}
