use defaults::get_default_types;
use definitions::{metadata::VersionDecoded, network_specs::ShortSpecs, types::TypeEntry};
use frame_metadata::{RuntimeMetadata, v14::RuntimeMetadataV14};
use meta_reading::decode_metadata::get_meta_const_light;
use parity_scale_codec::Decode;
use printing_balance::{convert_balance_pretty};
use sp_runtime::generic::Era;

mod cards;
    use cards::MethodCard;
mod decoding_older;
    use decoding_older::process_as_call;
mod decoding_commons;
    use decoding_commons::{OutputCard, get_compact};
mod decoding_sci;
    use decoding_sci::decoding_sci_entry_point;
mod decoding_sci_ext;
    use decoding_sci_ext::{decode_ext_attempt, Ext};
mod error;
    use error::{ParserError, ArgumentsError, DecodingError, MetadataError, SystemError};
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
    if method_decoded.remaining_vector.len() != 0 {return Err(ParserError::Decoding(DecodingError::SomeDataNotUsedMethod))}
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

pub fn parse_extensions (extensions_data: Vec<u8>, metadata_bundle: &MetadataBundle, short_specs: &ShortSpecs) -> Result<Vec<OutputCard>, ParserError> {
    let indent = 0;
    match metadata_bundle {
        MetadataBundle::Older {older_meta: _, types: _, network_version} => {
            let ext = match <ExtValues>::decode(&mut &extensions_data[..]) {
                Ok(a) => a,
                Err(_) => return Err(ParserError::Decoding(DecodingError::ExtensionsOlder)),
            };
            if ext.genesis_hash != short_specs.genesis_hash {return Err(ParserError::Decoding(DecodingError::GenesisHashMismatch))}
            if let Era::Immortal = ext.era {if ext.genesis_hash != ext.block_hash {return Err(ParserError::Decoding(DecodingError::ImmortalHashMismatch))}}
            if network_version != &ext.metadata_version {return Err(ParserError::WrongNetworkVersion{as_decoded: ext.metadata_version.to_string(), in_metadata: network_version.to_owned()})}
            let tip = match convert_balance_pretty (&ext.tip.to_string(), short_specs.decimals, &short_specs.unit) {
                Ok(x) => x,
                Err(_) => return Err(ParserError::SystemError(SystemError::BalanceFail)),
            };
            Ok(vec![
                OutputCard{card: MethodCard::Era(ext.era), indent},
                OutputCard{card: MethodCard::Nonce(ext.nonce.to_string()), indent},
                OutputCard{card: MethodCard::Tip{number: tip.number.to_string(), units: tip.units.to_string()}, indent},
                OutputCard{card: MethodCard::NetworkName(short_specs.name.to_string()), indent},
                OutputCard{card: MethodCard::SpecVersion(network_version.to_string()), indent},
                OutputCard{card: MethodCard::TxVersion(ext.tx_version.to_string()), indent},
                OutputCard{card: MethodCard::BlockHash(ext.block_hash), indent},
            ])
        },
        MetadataBundle::Sci {meta_v14, network_version} => {
            let mut ext = Ext::init();
            let extensions_decoded = decode_ext_attempt (&extensions_data, &mut ext, &meta_v14, indent, short_specs)?;
            if let Some(genesis_hash) = ext.found_ext.genesis_hash {if genesis_hash != short_specs.genesis_hash {return Err(ParserError::Decoding(DecodingError::GenesisHashMismatch))}}
            let block_hash = match ext.found_ext.block_hash {
                Some(a) => a,
                None => return Err(ParserError::FundamentallyBadV14Metadata(MetadataError::NoBlockHash)),
            };
            match ext.found_ext.era {
                Some(era) => if let Era::Immortal = era {if short_specs.genesis_hash != block_hash {return Err(ParserError::Decoding(DecodingError::ImmortalHashMismatch))}},
                None => return Err(ParserError::FundamentallyBadV14Metadata(MetadataError::NoEra)),
            }
            match ext.found_ext.network_version_printed {
                Some(a) => if a != network_version.to_string() {return Err(ParserError::WrongNetworkVersion{as_decoded: a, in_metadata: network_version.to_owned()})},
                None => return Err(ParserError::FundamentallyBadV14Metadata(MetadataError::NoVersionExt)),
            }
            if extensions_decoded.remaining_vector.len() != 0 {return Err(ParserError::Decoding(DecodingError::SomeDataNotUsedExtensions))}
            Ok(extensions_decoded.fancy_out)
        },
    }
}

pub fn parse_set (data: Vec<u8>, metadata_bundle: &MetadataBundle, short_specs: &ShortSpecs) -> Result<(Vec<OutputCard>, Vec<OutputCard>), ParserError> {
    let pre_method = get_compact::<u32>(&data)?;
    let method_length = pre_method.compact_found as usize;
    let (method_data, extensions_data) = match pre_method.start_next_unit {
        Some(start) => {
            match data.get(start..start+method_length) {
                Some(a) => (a.to_vec(), data[start+method_length..].to_vec()),
                None => {return Err(ParserError::Decoding(DecodingError::DataTooShort))}
            }
        },
        None => {
            if method_length != 0 {return Err(ParserError::Decoding(DecodingError::DataTooShort))}
            (Vec::new(), data)
        },
    };
    let extensions_cards = parse_extensions (extensions_data, metadata_bundle, short_specs)?;
    let method_cards = parse_method (method_data, metadata_bundle, short_specs)?;
    Ok((method_cards, extensions_cards))
}

pub fn parse_and_display_set (data: Vec<u8>, metadata: &RuntimeMetadata, short_specs: &ShortSpecs) -> String {
    let (network_name, network_version) = match get_meta_const_light(&metadata) {
        Ok(x) => {
            match VersionDecoded::decode(&mut &x[..]) {
                Ok(y) => (y.specname, y.spec_version),
                Err(_) => return ParserError::Arguments(ArgumentsError::MetaSpecVersionNotDecodeable).show()
            }
        },
        Err(_) => return ParserError::Arguments(ArgumentsError::NoMetaSpecVersion).show()
    };
    if network_name != short_specs.name {return ParserError::Arguments(ArgumentsError::NetworkNameMismatch {name_metadata: network_name, name_network_specs: short_specs.name.to_string()}).show()}
    let metadata_bundle = match metadata {
        RuntimeMetadata::V12(_)|RuntimeMetadata::V13(_) => {
            let older_meta = match metadata {
                RuntimeMetadata::V12(meta_v12) => {OlderMeta::V12(meta_v12)},
                RuntimeMetadata::V13(meta_v13) => {OlderMeta::V13(meta_v13)},
                _ => unreachable!(),
            };
            let types = match get_default_types() {
                Ok(a) => {
                    let out = a.types().expect("just generated types set");
                    if out.len() == 0 {return ParserError::Arguments(ArgumentsError::NoTypes).show()}
                    out
                },
                Err(_) => return ParserError::Arguments(ArgumentsError::NoTypes).show()
            };
            MetadataBundle::Older{older_meta, types, network_version}
        },
        RuntimeMetadata::V14(meta_v14) => MetadataBundle::Sci{meta_v14, network_version},
        _ => return ParserError::Arguments(ArgumentsError::RuntimeVersionIncompatible).show(),
    };
    match parse_set (data, &metadata_bundle, short_specs) {
        Ok((method_cards, extensions_cards)) => {
            let mut method = String::new();
            let mut extensions = String::new();
            for (i, x) in method_cards.iter().enumerate() {
                if i>0 {method.push_str(",\n");}
                method.push_str(&x.card.show_no_docs(x.indent));
            }
            for (i, x) in extensions_cards.iter().enumerate() {
                if i>0 {extensions.push_str(",\n");}
                extensions.push_str(&x.card.show_no_docs(x.indent));
            }
            format!("\nMethod:\n\n{}\n\n\nExtensions:\n\n{}", method, extensions)
        },
        Err(e) => e.show(),
    }
}

pub enum MetadataBundle <'a> {
    Older {older_meta: OlderMeta <'a>, types: Vec<TypeEntry>, network_version: u32},
    Sci {meta_v14: &'a RuntimeMetadataV14, network_version: u32},
}
