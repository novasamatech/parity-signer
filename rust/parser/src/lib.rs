//! This crate is a transaction parser used by
//! [Signer](https://github.com/paritytech/parity-signer).
//!
//! # Scope
//!
//! Signer allows to sign only the transactions that were successfully parsed
//! and were approved by user after checking the transaction contents.
//!
//! Transactions are read by the Signer as QR codes with data having following
//! structure:
//!
//! <table>
//!     <tr>
//!         <td>prelude</td>
//!         <td>public key</td>
//!         <td>SCALE-encoded call data</td>
//!         <td>SCALE-encoded extensions</td>
//!         <td>network genesis hash</td>
//!     </tr>
//! </table>
//!
//! This crate deals with decoding and presenting in a readable format the call
//! data and extensions data, and processes only the
//!
//! <table>
//!     <tr>
//!         <td>SCALE-encoded call data</td>
//!         <td>SCALE-encoded extensions</td>
//!     </tr>
//! </table>
//!
//! part.
//!
//! The ultimate goal here is to show the contents of the call and the
//! extensions, not perform any operations on them, and this crate is not
//! intended to keep track of the types used in call generation.
//!
//! # Features
//!
//! Default feature `"standalone"` allows to operate the parser as a standalone
//! tool to parse the contents of transactions.
//!
//! Signer itself uses `parser` crate with default features disabled.
//!
//! # How the parser works
//!
//! Call data and extensions data in transactions are
//! [SCALE-encoded](https://docs.substrate.io/reference/scale-codec/).
//!
//! Parsing always starts with separating call data and extensions data.
//!
//! Call data is `Vec<u8>`. **SCALE-encoded** call data, that is a part of the
//! transaction data, is the same `Vec<u8>` prefixed with compact of the call
//! data length. After the first compact is found, the data gets cut into call
//! data and the extensions data, that get processed separately.
//!
//! In decoding both call and extensions data the `Vec<u8>` enters the decoder,
//! and gets pieces cut off byte-by-byte starting from the first. This
//! processing follows what the `decode` from the [`parity_scale_codec`] does,
//! however, the types that go into the decoder are found dynamically during the
//! decoding itself. Generally, first, the type is found from the metadata, then
//! the bytes corresponding to the encoded value are cut from the entered data,
//! decoded and transformed into displayable [`OutputCard`].
//!
//! Notably, the decoding operates differently for different
//! [`RuntimeMetadata`](frame_metadata::RuntimeMetadata) variants. Signer can
//! work with metadata `V12`, `V13` and `V14`. Of those, only `V14`, i.e.
//! [`RuntimeMetadataV14`] has types described inside the metadata itself.
//! `V12` and `V13` have only type text descriptors, and the types meaning has
//! to be inferred from elsewhere, therefore, additional types description
//! dataset is needed to parse transactions made with metadata runtime versions
//! below `V14`.
//!
//! ## Decoding extensions
//!
//! Decoding starts with the extensions, as the extensions contain the metadata
//! version that must match the version of the metadata used to decode both the
//! extensions and the call data.
//!
//! Extensions in metadata `V12` and `V13` are described in the metadata as
//! `ExtrinsicMetadata` in `extrinsic` field of the `RuntimeMetadataV12` and
//! `RuntimeMetadataV13`. Field `signed_extensions` contain text identifier set
//! for extensions, that in principle can vary between the metadata. Here for
//! older metadata static extension set is used, matching the only ever
//! encountered in Substrate networks extensions.
//!
//! In metadata `V14` the extensions are described in `extrinsic` field of the
//! `RuntimeMetadataV14`, and have types resolvable in the associated with the
//! metadata types registry. Set of extensions in `signed_extensions` field of
//! the [`ExtrinsicMetadata`](frame_metadata::v14::ExtrinsicMetadata) is scanned
//! twice: first, for the types in field `ty`, then for the types in the field
//! `additional_signed` of the
//! [`SignedExtensionsMetadata`](frame_metadata::v14::SignedExtensionMetadata).
//! The extensions for most `V14` metadata are matching the static ones used for
//! `V12` and `V13`, however, as the types could be easily interpreted,
//! potentially changeable construction is used here.
//!
//! ## Decoding call data
//!
//! Once the extensions are decoded and the metadata version in transaction is
//! asserted to match the metadata version from the metadata itself, the call
//! can be decoded.
//!
//! Both the call and the extensions decoding must use all the bytes that were
//! initially in the input.
//!
//! The call data always starts with pallet number, the index of the pallet in
//! which the call was created. In a sense, the call data is encoded enum data
//! with pallet being the enum variant used. Pallet index is the first byte
//! of the data, it is declared to be `u8` in `V12`
//! [`ModuleMetadata`](frame_metadata::v12::ModuleMetadata), `V13`
//! [`ModuleMetadata`](frame_metadata::v13::ModuleMetadata) and `V14`
//! [`PalletMetadata`](frame_metadata::v14::PalletMetadata).
//!
//! The pallet index is the decoding entry point for all runtime metadata
//! versions. The pallets available in the metadata are scanned to find the
//! pallet with correct index.
//!
//! ### Metadata `V14`
//!
//! For `V14` runtime metadata version the remaining data is then processed as
//! enum, with the type specified in `calls` field of the
//! [`PalletMetadata`](frame_metadata::v14::PalletMetadata). Enum variant has
//! field(s) with specified types, all of which are resolved in the types
//! registry, get bytes cut off from the remaining call data to decode with the
//! type found, and produce [`OutputCard`]s that are added to output set.
//!
//! ### Metadata `V12` and `V13`
//!
//! For `V12` and `V13` runtime metadata version the correct call variant and
//! its [`FunctionMetadata`](frame_metadata::v12::FunctionMetadata) in is found
//! by the ordinal number of the call in the vector in `calls` field of
//! `ModuleMetadata`. Arguments associated with the call (types and variable
//! names) are found in call-associated set of
//! [`FunctionArgumentMetadata`](frame_metadata::v12::FunctionMetadata) in
//! `arguments` field of the `FunctionMetadata`. Arguments are used in the same
//! order as they are listed.
//!
//! The text type descriptors are parsed using Regex (interpreting `Option`,
//! `Vec`, tuple fields etc) down to types that **have** to be known and then
//! those are used. The types information that is by default on record in the
//! Signer, contains description of the types that were used at the time of the
//! parser drafting in Westend. Polkadot, Kusama and Rococo networks, when those
//! still used metadata below `V14`. Types pre-`V14` were quite stable, so most
//! of the trivial transactions are expected to be parsed.
//!
//! If one of the encountered type is not described, Signer will not be able to
//! parse the transaction. In this case users are encouraged to update the types
//! information.
//!
//! For each argument an [`OutputCard`] is produces and added to the output set.
#![deny(unused_crate_dependencies)]

use frame_metadata::v14::RuntimeMetadataV14;
#[cfg(feature = "standalone")]
use frame_metadata::RuntimeMetadata;
use parity_scale_codec::{Decode, DecodeAll, Encode};
use printing_balance::AsBalance;
use sp_core::H256;
use sp_runtime::generic::Era;

#[cfg(feature = "standalone")]
use defaults::default_types_vec;
#[cfg(feature = "standalone")]
use definitions::metadata::info_from_metadata;
use definitions::{
    error_signer::{ParserDecodingError, ParserError, ParserMetadataError},
    network_specs::ShortSpecs,
    types::TypeEntry,
};

pub mod cards;
use cards::ParserCard;
mod decoding_older;
use decoding_older::process_as_call;
pub mod decoding_commons;
use decoding_commons::{get_compact, OutputCard};
mod decoding_sci;
use decoding_sci::decoding_sci_entry_point;
mod decoding_sci_ext;
use decoding_sci_ext::{decode_ext_attempt, Ext};
#[cfg(feature = "standalone")]
mod error;
#[cfg(feature = "standalone")]
use error::{ArgumentsError, Error};
pub mod method;
use method::OlderMeta;
#[cfg(feature = "standalone")]
#[cfg(test)]
mod tests;

/// Parse call data with suitable network [`MetadataBundle`] and [`ShortSpecs`].
pub fn parse_method(
    method_data: &mut Vec<u8>,
    metadata_bundle: &MetadataBundle,
    short_specs: &ShortSpecs,
) -> Result<Vec<OutputCard>, ParserError> {
    let start_indent = 0;
    let out = match metadata_bundle {
        MetadataBundle::Older {
            older_meta,
            types,
            network_version: _,
        } => process_as_call(method_data, older_meta, types, start_indent, short_specs)?,
        MetadataBundle::Sci {
            meta_v14,
            network_version: _,
        } => decoding_sci_entry_point(method_data, meta_v14, start_indent, short_specs)?,
    };
    if !method_data.is_empty() {
        return Err(ParserError::Decoding(
            ParserDecodingError::SomeDataNotUsedMethod,
        ));
    }
    Ok(out)
}

/// Statically determined extensions for `V12` and `V13` metadata.
#[derive(Debug, Decode, Encode)]
struct ExtValues {
    era: Era,
    #[codec(compact)]
    nonce: u64,
    #[codec(compact)]
    tip: u128,
    metadata_version: u32,
    tx_version: u32,
    genesis_hash: H256,
    block_hash: H256,
}

/// Parse extensions.
pub fn parse_extensions(
    extensions_data: &mut Vec<u8>,
    metadata_bundle: &MetadataBundle,
    short_specs: &ShortSpecs,
    optional_mortal_flag: Option<bool>,
) -> Result<Vec<OutputCard>, ParserError> {
    let indent = 0;
    let (era, block_hash, cards) = match metadata_bundle {
        MetadataBundle::Older {
            older_meta: _,
            types: _,
            network_version,
        } => {
            let ext = match <ExtValues>::decode_all(&mut &extensions_data[..]) {
                Ok(a) => a,
                Err(_) => return Err(ParserError::Decoding(ParserDecodingError::ExtensionsOlder)),
            };
            if ext.genesis_hash != short_specs.genesis_hash {
                return Err(ParserError::Decoding(
                    ParserDecodingError::GenesisHashMismatch,
                ));
            }
            if network_version != &ext.metadata_version {
                return Err(ParserError::WrongNetworkVersion {
                    as_decoded: ext.metadata_version.to_string(),
                    in_metadata: network_version.to_owned(),
                });
            }
            let tip =
                <u128>::convert_balance_pretty(ext.tip, short_specs.decimals, &short_specs.unit);
            let cards = vec![
                OutputCard {
                    card: ParserCard::Era(ext.era),
                    indent,
                },
                OutputCard {
                    card: ParserCard::Nonce(ext.nonce.to_string()),
                    indent,
                },
                OutputCard {
                    card: ParserCard::Tip {
                        number: tip.number.to_string(),
                        units: tip.units,
                    },
                    indent,
                },
                OutputCard {
                    card: ParserCard::NetworkNameVersion {
                        name: short_specs.name.to_string(),
                        version: network_version.to_string(),
                    },
                    indent,
                },
                OutputCard {
                    card: ParserCard::TxVersion(ext.tx_version.to_string()),
                    indent,
                },
                OutputCard {
                    card: ParserCard::BlockHash(ext.block_hash),
                    indent,
                },
            ];
            (ext.era, ext.block_hash, cards)
        }
        MetadataBundle::Sci {
            meta_v14,
            network_version,
        } => {
            let mut ext = Ext::init();
            let extensions_decoded =
                decode_ext_attempt(extensions_data, &mut ext, meta_v14, indent, short_specs)?;
            if let Some(genesis_hash) = ext.found_ext.genesis_hash {
                if genesis_hash != short_specs.genesis_hash {
                    return Err(ParserError::Decoding(
                        ParserDecodingError::GenesisHashMismatch,
                    ));
                }
            }
            let block_hash = match ext.found_ext.block_hash {
                Some(a) => a,
                None => {
                    return Err(ParserError::FundamentallyBadV14Metadata(
                        ParserMetadataError::NoBlockHash,
                    ))
                }
            };
            let era = match ext.found_ext.era {
                Some(a) => a,
                None => {
                    return Err(ParserError::FundamentallyBadV14Metadata(
                        ParserMetadataError::NoEra,
                    ))
                }
            };
            match ext.found_ext.network_version_printed {
                Some(a) => {
                    if a != network_version.to_string() {
                        return Err(ParserError::WrongNetworkVersion {
                            as_decoded: a,
                            in_metadata: network_version.to_owned(),
                        });
                    }
                }
                None => {
                    return Err(ParserError::FundamentallyBadV14Metadata(
                        ParserMetadataError::NoVersionExt,
                    ))
                }
            }
            if !extensions_data.is_empty() {
                return Err(ParserError::Decoding(
                    ParserDecodingError::SomeDataNotUsedExtensions,
                ));
            }
            (era, block_hash, extensions_decoded)
        }
    };
    if let Era::Immortal = era {
        if short_specs.genesis_hash != block_hash {
            return Err(ParserError::Decoding(
                ParserDecodingError::ImmortalHashMismatch,
            ));
        }
        if let Some(true) = optional_mortal_flag {
            return Err(ParserError::Decoding(
                ParserDecodingError::UnexpectedImmortality,
            ));
        }
    }
    if let Era::Mortal(_, _) = era {
        if let Some(false) = optional_mortal_flag {
            return Err(ParserError::Decoding(
                ParserDecodingError::UnexpectedMortality,
            ));
        }
    }
    Ok(cards)
}

/// Separate call data and extensions data based on the call data length
/// declared as a compact.
pub fn cut_method_extensions(data: &[u8]) -> Result<(Vec<u8>, Vec<u8>), ParserError> {
    let pre_method = get_compact::<u32>(data).map_err(|_| ParserError::SeparateMethodExtensions)?;
    let method_length = pre_method.compact_found as usize;
    match pre_method.start_next_unit {
        Some(start) => match data.get(start..start + method_length) {
            Some(a) => Ok((a.to_vec(), data[start + method_length..].to_vec())),
            None => Err(ParserError::SeparateMethodExtensions),
        },
        None => {
            if method_length != 0 {
                return Err(ParserError::SeparateMethodExtensions);
            }
            Ok((Vec::new(), data.to_vec()))
        }
    }
}

#[cfg(feature = "standalone")]
/// Parse transaction with given metadata and network specs. For standalone
/// parser.
pub fn parse_and_display_set(
    data: &[u8],
    metadata: &RuntimeMetadata,
    short_specs: &ShortSpecs,
) -> Result<String, String> {
    let meta_info = match info_from_metadata(metadata) {
        Ok(x) => x,
        Err(e) => return Err(Error::Arguments(ArgumentsError::Metadata(e)).show()),
    };
    if meta_info.name != short_specs.name {
        return Err(Error::Arguments(ArgumentsError::NetworkNameMismatch {
            name_metadata: meta_info.name,
            name_network_specs: short_specs.name.to_string(),
        })
        .show());
    }
    let metadata_bundle = match metadata {
        RuntimeMetadata::V12(_) | RuntimeMetadata::V13(_) => {
            let older_meta = match metadata {
                RuntimeMetadata::V12(meta_v12) => OlderMeta::V12(meta_v12),
                RuntimeMetadata::V13(meta_v13) => OlderMeta::V13(meta_v13),
                _ => unreachable!(),
            };
            let types = match default_types_vec() {
                Ok(a) => {
                    if a.is_empty() {
                        return Err(Error::Arguments(ArgumentsError::NoTypes).show());
                    }
                    a
                }
                Err(_) => return Err(Error::Arguments(ArgumentsError::DefaultTypes).show()),
            };
            MetadataBundle::Older {
                older_meta,
                types,
                network_version: meta_info.version,
            }
        }
        RuntimeMetadata::V14(meta_v14) => MetadataBundle::Sci {
            meta_v14,
            network_version: meta_info.version,
        },
        _ => unreachable!(), // just checked in the info_from_metadata function if the metadata is acceptable one
    };

    // if unable to separate method date and extensions, then some fundamental flaw is in transaction itself
    let (mut method_data, mut extensions_data) =
        cut_method_extensions(data).map_err(|e| Error::Parser(e).show())?;

    // try parsing extensions, if is works, the version and extensions are correct
    let extensions_cards =
        parse_extensions(&mut extensions_data, &metadata_bundle, short_specs, None)
            .map_err(|e| Error::Parser(e).show())?;

    let mut extensions = String::new();
    for (i, x) in extensions_cards.iter().enumerate() {
        if i > 0 {
            extensions.push_str(",\n");
        }
        extensions.push_str(&x.card.show_no_docs(x.indent));
    }

    // try parsing method
    let method = match parse_method(&mut method_data, &metadata_bundle, short_specs) {
        Ok(method_cards) => {
            let mut collect = String::new();
            for (i, x) in method_cards.iter().enumerate() {
                if i > 0 {
                    collect.push_str(",\n");
                }
                collect.push_str(&x.card.show_no_docs(x.indent));
            }
            collect
        }
        Err(e) => e.show(),
    };
    Ok(format!(
        "\nMethod:\n\n{}\n\n\nExtensions:\n\n{}",
        method, extensions
    ))
}

/// Metadata information sufficient for transaction decoding.
pub enum MetadataBundle<'a> {
    /// Metadata before `V14`, requiring additional types information
    Older {
        older_meta: OlderMeta<'a>,
        types: Vec<TypeEntry>,
        network_version: u32,
    },

    /// `scale-info` supporting metadata `V14`
    Sci {
        meta_v14: &'a RuntimeMetadataV14,
        network_version: u32,
    },
}
