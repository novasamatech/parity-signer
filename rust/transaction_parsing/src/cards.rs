use sp_core::crypto::{Ss58AddressFormat, Ss58Codec};
use sp_runtime::{generic::Era, MultiSigner};

use definitions::{
    crypto::Encryption,
    error::ErrorSource,
    error_signer::{ErrorSigner, Signer},
    helpers::{make_identicon_from_multisigner, pic_meta, print_multisigner_as_base58},
    history::MetaValuesDisplay,
    keyring::VerifierKey,
    navigation::{
        Card as NavCard, MSCAuthorPlain, MSCCall, MSCCurrency, MSCEnumVariantName, MSCEraMortal,
        MSCFieldName, MSCFieldNumber, MSCId, MSCMetaSpecs, MSCNameVersion, MSCNetworkInfo,
        MTypesInfo, MVerifierDetails, TransactionAuthor, TransactionCard,
    },
    network_specs::{NetworkSpecs, NetworkSpecsToSend, VerifierValue},
    qr_transfers::ContentLoadTypes,
    users::AddressDetails,
};
use parser::cards::ParserCard;
use plot_icon::generate_png_scaled_default;

use crate::holds::{GeneralHold, Hold};

#[allow(clippy::enum_variant_names)]
pub(crate) enum Card<'a> {
    ParserCard(&'a ParserCard),
    Author {
        author: &'a MultiSigner,
        base58prefix: u16,
        address_details: &'a AddressDetails,
    },
    AuthorPlain {
        author: &'a MultiSigner,
        base58prefix: u16,
    },
    AuthorPublicKey(&'a MultiSigner),
    Verifier(&'a VerifierValue),
    Meta(MetaValuesDisplay),
    TypesInfo(ContentLoadTypes),
    NewSpecs(&'a NetworkSpecsToSend),
    NetworkInfo(&'a NetworkSpecs),
    NetworkGenesisHash(&'a [u8]),
    Derivations(&'a [String]),
    Warning(Warning<'a>),
    Error(ErrorSigner),
}

pub(crate) enum Warning<'a> {
    AuthorNotFound,
    NewerVersion {
        used_version: u32,
        latest_version: u32,
    },
    NoNetworkID,
    NotVerified,
    UpdatingTypes,
    TypesNotVerified,
    GeneralVerifierAppeared(&'a GeneralHold),
    VerifierChangingToGeneral {
        verifier_key: &'a VerifierKey,
        hold: &'a Hold,
    },
    VerifierChangingToCustom {
        verifier_key: &'a VerifierKey,
        hold: &'a Hold,
    },
    VerifierGeneralSuper {
        verifier_key: &'a VerifierKey,
        hold: &'a Hold,
    },
    TypesAlreadyThere,
    NetworkSpecsAlreadyThere(&'a str), // network title
    MetadataExtensionsIncomplete,
}

impl<'a> Warning<'a> {
    pub(crate) fn show(&self) -> String {
        match &self {
            Warning::AuthorNotFound => String::from("Transaction author public key not found."),
            Warning::NewerVersion {used_version, latest_version} => format!("Transaction uses outdated runtime version {}. Latest known available version is {}.", used_version, latest_version),
            Warning::NoNetworkID => String::from("Public key is on record, but not associated with the network used."),
            Warning::NotVerified => String::from("Received network information is not verified."),
            Warning::UpdatingTypes => String::from("Updating types (really rare operation)."),
            Warning::TypesNotVerified => String::from("Received types information is not verified."),
            Warning::GeneralVerifierAppeared(x) => format!("Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. {}", x.show()),
            Warning::VerifierChangingToGeneral{verifier_key, hold} => format!("Received message is verified by the general verifier. Current verifier for network with genesis hash {} is a custom one, and proceeding will update the network verifier to general. All previously acquired information associated with former custom verifier will be purged. {}", hex::encode(verifier_key.genesis_hash()), hold.show()),
            Warning::VerifierChangingToCustom{verifier_key, hold} => format!("Received message is verified. Currently no verifier is set for network with genesis hash {}. Proceeding will update the network verifier to custom verifier. All previously acquired network information that was received unverified will be purged. {}", hex::encode(verifier_key.genesis_hash()), hold.show()),
            Warning::VerifierGeneralSuper{verifier_key, hold} => format!("Received message is verified. Currently no verifier is set for network with genesis hash {} and no general verifier is set. Proceeding will update the network verifier to general. All previously acquired network information that was received unverified will be purged. {}", hex::encode(verifier_key.genesis_hash()), hold.show()),
            Warning::TypesAlreadyThere => String::from("Received types information is identical to the one that was in the database."),
            Warning::NetworkSpecsAlreadyThere (x) => format!("Received network specs information for {} is same as the one already in the database.", x),
            Warning::MetadataExtensionsIncomplete => String::from("Received metadata has incomplete set of signed extensions. As a result, Signer may be unable to parse signable transactions using this metadata."),
        }
    }
}

impl<'a> Card<'a> {
    pub(crate) fn card(&self, index: &mut u32, indent: u32) -> TransactionCard {
        let card = match &self {
            Card::ParserCard(m) => match m {
                ParserCard::Pallet(f) => NavCard::PalletCard { f: f.clone() },
                ParserCard::Method { method_name, docs } => NavCard::CallCard {
                    f: MSCCall {
                        method_name: method_name.clone(),
                        docs: hex::encode(docs.as_bytes()),
                    },
                },
                ParserCard::Varname(varname) => NavCard::VarNameCard { f: varname.clone() },
                ParserCard::Default(decoded_string) => NavCard::DefaultCard {
                    f: decoded_string.clone(),
                },
                ParserCard::Text(decoded_text) => NavCard::TextCard {
                    f: hex::encode(decoded_text.as_bytes()),
                },
                ParserCard::Id { id, base58prefix } => NavCard::IdCard {
                    f: MSCId {
                        base58: id
                            .to_ss58check_with_version(Ss58AddressFormat::custom(*base58prefix)),
                        identicon: hex::encode(generate_png_scaled_default(&<[u8; 32]>::from(
                            id.to_owned(),
                        ))),
                    },
                },
                ParserCard::None => NavCard::NoneCard,
                ParserCard::IdentityField(variant) => {
                    NavCard::IdentityFieldCard { f: variant.clone() }
                }
                ParserCard::BitVec(bv) => NavCard::BitVecCard { f: bv.clone() },
                ParserCard::Balance { number, units } => NavCard::BalanceCard {
                    f: MSCCurrency {
                        amount: number.clone(),
                        units: units.clone(),
                    },
                },
                ParserCard::FieldName {
                    name,
                    docs_field_name,
                    path_type,
                    docs_type,
                } => NavCard::FieldNameCard {
                    f: MSCFieldName {
                        name: name.clone(),
                        docs_field_name: hex::encode(docs_field_name.as_bytes()),
                        path_type: path_type.clone(),
                        docs_type: hex::encode(docs_type.as_bytes()),
                    },
                },
                ParserCard::FieldNumber {
                    number,
                    docs_field_number,
                    path_type,
                    docs_type,
                } => NavCard::FieldNumberCard {
                    f: MSCFieldNumber {
                        number: number.to_string(),
                        docs_field_number: hex::encode(docs_field_number.as_bytes()),
                        path_type: path_type.clone(),
                        docs_type: hex::encode(docs_type.as_bytes()),
                    },
                },
                ParserCard::EnumVariantName {
                    name,
                    docs_enum_variant,
                } => NavCard::EnumVariantNameCard {
                    f: MSCEnumVariantName {
                        name: name.clone(),
                        docs_enum_variant: hex::encode(docs_enum_variant.as_bytes()),
                    },
                },
                ParserCard::Era(era) => match era {
                    Era::Immortal => NavCard::EraImmortalCard,
                    Era::Mortal(period, phase) => NavCard::EraMortalCard {
                        f: MSCEraMortal {
                            era: "Mortal".to_string(),
                            phase: phase.to_string(),
                            period: period.to_string(),
                        },
                    },
                },
                ParserCard::Nonce(nonce) => NavCard::NonceCard { f: nonce.clone() },
                ParserCard::BlockHash(block_hash) => NavCard::BlockHashCard {
                    f: hex::encode(block_hash),
                },
                ParserCard::Tip { number, units } => NavCard::TipCard {
                    f: MSCCurrency {
                        amount: number.clone(),
                        units: units.clone(),
                    },
                },
                ParserCard::NetworkNameVersion { name, version } => NavCard::NameVersionCard {
                    f: MSCNameVersion {
                        name: name.clone(),
                        version: version.clone(),
                    },
                },
                ParserCard::TxVersion(x) => NavCard::TxSpecCard { f: x.clone() },
            },
            Card::Author {
                author,
                base58prefix,
                address_details,
            } => NavCard::AuthorCard {
                f: make_author_info(author, *base58prefix, address_details),
            },
            Card::AuthorPlain {
                author,
                base58prefix,
            } => NavCard::AuthorPlainCard {
                f: MSCAuthorPlain {
                    base58: print_multisigner_as_base58(author, Some(*base58prefix)),
                    identicon: hex::encode(make_identicon_from_multisigner(author)),
                },
            },
            Card::AuthorPublicKey(author) => {
                let identicon = hex::encode(make_identicon_from_multisigner(author));
                let (public_key, encryption) = match author {
                    MultiSigner::Ed25519(p) => (hex::encode(&p), Encryption::Ed25519.show()),
                    MultiSigner::Sr25519(p) => (hex::encode(&p), Encryption::Sr25519.show()),
                    MultiSigner::Ecdsa(p) => (hex::encode(&p), Encryption::Ecdsa.show()),
                };
                NavCard::AuthorPublicKeyCard {
                    f: MVerifierDetails {
                        public_key,
                        identicon,
                        encryption,
                    },
                }
            }
            Card::Verifier(x) => match x {
                VerifierValue::Standard { m } => {
                    let (public_key, encryption) = match m {
                        MultiSigner::Ed25519(p) => (hex::encode(&p), Encryption::Ed25519.show()),
                        MultiSigner::Sr25519(p) => (hex::encode(&p), Encryption::Sr25519.show()),
                        MultiSigner::Ecdsa(p) => (hex::encode(&p), Encryption::Ecdsa.show()),
                    };
                    NavCard::VerifierCard {
                        f: MVerifierDetails {
                            public_key,
                            identicon: hex::encode(make_identicon_from_multisigner(m)),
                            encryption,
                        },
                    }
                }
            },
            Card::Meta(x) => NavCard::MetaCard {
                f: MSCMetaSpecs {
                    specname: x.name.clone(),
                    spec_version: x.version.to_string(),
                    meta_hash: hex::encode(&x.meta_hash),
                    meta_id_pic: hex::encode(pic_meta(&x.meta_hash)),
                },
            },
            Card::TypesInfo(x) => {
                let (types_hash, types_id_pic) = x.show();
                NavCard::TypesInfoCard {
                    f: MTypesInfo {
                        types_on_file: false,
                        types_hash: Some(types_hash),
                        types_id_pic: Some(types_id_pic),
                    },
                }
            }
            Card::NewSpecs(x) => NavCard::NewSpecsCard { f: (*x).clone() },
            Card::NetworkInfo(x) => NavCard::NetworkInfoCard {
                f: MSCNetworkInfo {
                    network_title: x.title.clone(),
                    network_logo: x.logo.clone(),
                },
            },
            Card::NetworkGenesisHash(x) => NavCard::NetworkGenesisHashCard { f: hex::encode(x) },
            Card::Derivations(x) => NavCard::DerivationsCard {
                f: x.iter().cloned().collect(),
            },
            Card::Warning(warn) => NavCard::WarningCard { f: warn.show() },
            Card::Error(err) => NavCard::ErrorCard {
                f: <Signer>::show(err),
            },
        };

        let i = *index;
        *index += 1;
        TransactionCard {
            index: i,
            indent,
            card,
        }
    }
}

pub(crate) fn make_author_info(
    author: &MultiSigner,
    base58prefix: u16,
    address_details: &AddressDetails,
) -> TransactionAuthor {
    TransactionAuthor {
        base58: print_multisigner_as_base58(author, Some(base58prefix)),
        identicon: hex::encode(make_identicon_from_multisigner(author)),
        seed: address_details.seed_name.clone(),
        derivation_path: address_details.path.clone(),
    }
}
