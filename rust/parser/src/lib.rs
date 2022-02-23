use defaults::get_default_types_vec;
use definitions::{error::{ParserError, ParserDecodingError, ParserMetadataError}, metadata::info_from_metadata, network_specs::ShortSpecs, types::TypeEntry};
use frame_metadata::{RuntimeMetadata, v14::RuntimeMetadataV14};
use parity_scale_codec::Decode;
use printing_balance::{convert_balance_pretty};
use sp_runtime::generic::Era;

pub mod cards;
    use cards::ParserCard;
mod decoding_older;
    use decoding_older::process_as_call;
pub mod decoding_commons;
    use decoding_commons::{OutputCard, get_compact};
mod decoding_sci;
    use decoding_sci::decoding_sci_entry_point;
mod decoding_sci_ext;
    use decoding_sci_ext::{decode_ext_attempt, Ext};
mod error;
    use error::{Error, ArgumentsError};
pub mod method;
    use method::OlderMeta;
mod tests;


/// Function intakes SCALE encoded method part of transaction as Vec<u8>,
/// network metadata and network specs.
/// 
pub fn parse_method (method_data: Vec<u8>, metadata_bundle: &MetadataBundle, short_specs: &ShortSpecs) -> Result<Vec<OutputCard>, ParserError> {
    let start_indent = 0;
    let method_decoded = match metadata_bundle {
        MetadataBundle::Older {older_meta, types, network_version: _} => process_as_call (method_data, &older_meta, &types, start_indent, &short_specs)?,
        MetadataBundle::Sci {meta_v14, network_version: _} => decoding_sci_entry_point (method_data, &meta_v14, start_indent, &short_specs)?,
    };
    if method_decoded.remaining_vector.len() != 0 {return Err(ParserError::Decoding(ParserDecodingError::SomeDataNotUsedMethod))}
    Ok(method_decoded.fancy_out)
}


/// Struct to decode pre-determined extensions for transactions with V12 and V13 metadata
#[derive(Debug, parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
struct ExtValues {
    era: Era,
#[codec(compact)]
    nonce: u64,
#[codec(compact)]
    tip: u128,
    metadata_version: u32,
    tx_version: u32,
    genesis_hash: [u8; 32],
    block_hash: [u8; 32],
}

pub fn parse_extensions (extensions_data: Vec<u8>, metadata_bundle: &MetadataBundle, short_specs: &ShortSpecs, optional_mortal_flag: Option<bool>) -> Result<Vec<OutputCard>, ParserError> {
    let indent = 0;
    let (era, block_hash, cards) = match metadata_bundle {
        MetadataBundle::Older {older_meta: _, types: _, network_version} => {
            let ext = match <ExtValues>::decode(&mut &extensions_data[..]) {
                Ok(a) => a,
                Err(_) => return Err(ParserError::Decoding(ParserDecodingError::ExtensionsOlder)),
            };
            if ext.genesis_hash != short_specs.genesis_hash {return Err(ParserError::Decoding(ParserDecodingError::GenesisHashMismatch))}
            if network_version != &ext.metadata_version {return Err(ParserError::WrongNetworkVersion{as_decoded: ext.metadata_version.to_string(), in_metadata: network_version.to_owned()})}
            let tip = convert_balance_pretty (&ext.tip.to_string(), short_specs.decimals, &short_specs.unit);
            let cards = vec![
                OutputCard{card: ParserCard::Era(ext.era), indent},
                OutputCard{card: ParserCard::Nonce(ext.nonce.to_string()), indent},
                OutputCard{card: ParserCard::Tip{number: tip.number.to_string(), units: tip.units.to_string()}, indent},
                OutputCard{card: ParserCard::NetworkNameVersion{name: short_specs.name.to_string(), version: network_version.to_string()}, indent},
                OutputCard{card: ParserCard::TxVersion(ext.tx_version.to_string()), indent},
                OutputCard{card: ParserCard::BlockHash(ext.block_hash), indent},
            ];
            (ext.era, ext.block_hash, cards)
        },
        MetadataBundle::Sci {meta_v14, network_version} => {
            let mut ext = Ext::init();
            let extensions_decoded = decode_ext_attempt (&extensions_data, &mut ext, &meta_v14, indent, short_specs)?;
            if let Some(genesis_hash) = ext.found_ext.genesis_hash {if genesis_hash != short_specs.genesis_hash {return Err(ParserError::Decoding(ParserDecodingError::GenesisHashMismatch))}}
            let block_hash = match ext.found_ext.block_hash {
                Some(a) => a,
                None => return Err(ParserError::FundamentallyBadV14Metadata(ParserMetadataError::NoBlockHash)),
            };
            let era = match ext.found_ext.era {
                Some(a) => a,
                None => return Err(ParserError::FundamentallyBadV14Metadata(ParserMetadataError::NoEra)),
            };
            match ext.found_ext.network_version_printed {
                Some(a) => if a != network_version.to_string() {return Err(ParserError::WrongNetworkVersion{as_decoded: a, in_metadata: network_version.to_owned()})},
                None => return Err(ParserError::FundamentallyBadV14Metadata(ParserMetadataError::NoVersionExt)),
            }
            if extensions_decoded.remaining_vector.len() != 0 {return Err(ParserError::Decoding(ParserDecodingError::SomeDataNotUsedExtensions))}
            (era, block_hash, extensions_decoded.fancy_out)
        },
    };
    if let Era::Immortal = era {
        if short_specs.genesis_hash != block_hash {return Err(ParserError::Decoding(ParserDecodingError::ImmortalHashMismatch))}
        if let Some(true) = optional_mortal_flag {return Err(ParserError::Decoding(ParserDecodingError::UnexpectedImmortality))}
    }
    if let Era::Mortal(_, _) = era {
        if let Some(false) = optional_mortal_flag {return Err(ParserError::Decoding(ParserDecodingError::UnexpectedMortality))}
    }
    Ok(cards)
}

pub fn cut_method_extensions(data: &Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), ParserError> {
    let pre_method = get_compact::<u32>(data)?;
    let method_length = pre_method.compact_found as usize;
    match pre_method.start_next_unit {
        Some(start) => {
            match data.get(start..start+method_length) {
                Some(a) => Ok((a.to_vec(), data[start+method_length..].to_vec())),
                None => {return Err(ParserError::Decoding(ParserDecodingError::DataTooShort))}
            }
        },
        None => {
            if method_length != 0 {return Err(ParserError::Decoding(ParserDecodingError::DataTooShort))}
            Ok((Vec::new(), data.to_vec()))
        },
    }
}

pub fn parse_set (data: &Vec<u8>, metadata_bundle: &MetadataBundle, short_specs: &ShortSpecs, optional_mortal_flag: Option<bool>) -> Result<(Result<Vec<OutputCard>, ParserError>, Vec<OutputCard>, Vec<u8>, Vec<u8>), ParserError> {
    // if unable to separate method date and extensions, then some fundamental flaw is in transaction itself
    let (method_data, extensions_data) = match cut_method_extensions(data) {
        Ok(a) => a,
        Err(_) => return Err(ParserError::SeparateMethodExtensions),
    };
    // try parsing extensions, if is works, the version and extensions are correct
    let extensions_cards = parse_extensions (extensions_data.to_vec(), metadata_bundle, short_specs, optional_mortal_flag)?;
    // try parsing method
    let method_cards_result = parse_method (method_data.to_vec(), metadata_bundle, short_specs);
    Ok((method_cards_result, extensions_cards, method_data, extensions_data))
}

pub fn parse_and_display_set (data: &Vec<u8>, metadata: &RuntimeMetadata, short_specs: &ShortSpecs) -> Result<String, String> {
    let meta_info = match info_from_metadata(&metadata) {
        Ok(x) => x,
        Err(e) => return Err(Error::Arguments(ArgumentsError::Metadata(e)).show())
    };
    if meta_info.name != short_specs.name {return Err(Error::Arguments(ArgumentsError::NetworkNameMismatch {name_metadata: meta_info.name.to_string(), name_network_specs: short_specs.name.to_string()}).show())}
    let metadata_bundle = match metadata {
        RuntimeMetadata::V12(_)|RuntimeMetadata::V13(_) => {
            let older_meta = match metadata {
                RuntimeMetadata::V12(meta_v12) => {OlderMeta::V12(meta_v12)},
                RuntimeMetadata::V13(meta_v13) => {OlderMeta::V13(meta_v13)},
                _ => unreachable!(),
            };
            let types = match get_default_types_vec() {
                Ok(a) => {
                    if a.len() == 0 {return Err(Error::Arguments(ArgumentsError::NoTypes).show())}
                    a
                },
                Err(_) => return Err(Error::Arguments(ArgumentsError::DefaultTypes).show())
            };
            MetadataBundle::Older{older_meta, types, network_version: meta_info.version}
        },
        RuntimeMetadata::V14(meta_v14) => MetadataBundle::Sci{meta_v14, network_version: meta_info.version},
        _ => unreachable!(), // just checked in the info_from_metadata function if the metadata is acceptable one
    };
    match parse_set (data, &metadata_bundle, short_specs, None) {
        Ok((method_cards_result, extensions_cards, _, _)) => {
            let mut method = String::new();
            let mut extensions = String::new();
            match method_cards_result {
                Ok(method_cards) => {
                    for (i, x) in method_cards.iter().enumerate() {
                        if i>0 {method.push_str(",\n");}
                        method.push_str(&x.card.show_no_docs(x.indent));
                    }
                },
                Err(e) => method = e.show(),
            }
            for (i, x) in extensions_cards.iter().enumerate() {
                if i>0 {extensions.push_str(",\n");}
                extensions.push_str(&x.card.show_no_docs(x.indent));
            }
            Ok(format!("\nMethod:\n\n{}\n\n\nExtensions:\n\n{}", method, extensions))
        },
        Err(e) => return Err(Error::Parser(e).show()),
    }
}

pub enum MetadataBundle <'a> {
    Older {older_meta: OlderMeta <'a>, types: Vec<TypeEntry>, network_version: u32},
    Sci {meta_v14: &'a RuntimeMetadataV14, network_version: u32},
}
