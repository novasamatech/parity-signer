use sp_core::crypto::{Ss58AddressFormat, Ss58Codec};
use sp_core::H256;
use sp_runtime::{generic::Era, MultiSigner};

use definitions::helpers::{make_identicon_from_account, make_identicon_from_id20, IdenticonStyle};
use definitions::keyring::{AddressKey, NetworkSpecsKey};

use definitions::derivations::SeedKeysPreview;
use definitions::navigation::MAddressCard;
use definitions::{
    crypto::Encryption,
    helpers::{make_identicon_from_multisigner, pic_meta, print_multisigner_as_base58_or_eth},
    history::MetaValuesDisplay,
    keyring::VerifierKey,
    navigation::{
        Address, Card as NavCard, MMetadataRecord, MSCCall, MSCCurrency, MSCEnumVariantName,
        MSCEraMortal, MSCFieldName, MSCFieldNumber, MSCId, MSCNameVersion, MSCNetworkInfo,
        MTypesInfo, MVerifierDetails, TransactionCard,
    },
    network_specs::{NetworkSpecs, OrderedNetworkSpecs, VerifierValue},
    qr_transfers::ContentLoadTypes,
    users::AddressDetails,
};
use parser::cards::ParserCard;

use crate::error::Error;
use crate::holds::{GeneralHold, Hold};

#[allow(clippy::enum_variant_names)]
pub(crate) enum Card<'a> {
    ParserCard(&'a ParserCard),
    Author {
        author: &'a MultiSigner,
        base58prefix: u16,
        genesis_hash: H256,
        address_details: &'a AddressDetails,
    },
    AuthorPlain {
        author: &'a MultiSigner,
        base58prefix: u16,
    },
    Verifier(&'a VerifierValue),
    Meta(MetaValuesDisplay),
    TypesInfo(ContentLoadTypes),
    NewSpecs(&'a NetworkSpecs),
    NetworkInfo(&'a OrderedNetworkSpecs),
    Derivations(&'a [SeedKeysPreview]),
    Warning(Warning<'a>),
    Error(Error),
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

impl Warning<'_> {
    pub(crate) fn show(&self) -> String {
        match &self {
            Warning::AuthorNotFound => String::from("Transaction author public key not found."),
            Warning::NewerVersion {used_version, latest_version} => format!("Transaction uses outdated runtime version {used_version}. Latest known available version is {latest_version}."),
            Warning::NoNetworkID => String::from("Public key is on record, but not associated with the network used."),
            Warning::NotVerified => String::from("Received network information is not verified."),
            Warning::UpdatingTypes => String::from("Updating types (really rare operation)."),
            Warning::TypesNotVerified => String::from("Received types information is not verified."),
            Warning::GeneralVerifierAppeared(x) => format!("Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. {}", x.show()),
            Warning::VerifierChangingToGeneral{verifier_key, hold} => format!("Received message is verified by the general verifier. Current verifier for network with genesis hash {} is a custom one, and proceeding will update the network verifier to general. All previously acquired information associated with former custom verifier will be purged. {}", hex::encode(verifier_key.genesis_hash()), hold.show()),
            Warning::VerifierChangingToCustom{verifier_key, hold} => format!("Received message is verified. Currently no verifier is set for network with genesis hash {}. Proceeding will update the network verifier to custom verifier. All previously acquired network information that was received unverified will be purged. {}", hex::encode(verifier_key.genesis_hash()), hold.show()),
            Warning::VerifierGeneralSuper{verifier_key, hold} => format!("Received message is verified. Currently no verifier is set for network with genesis hash {} and no general verifier is set. Proceeding will update the network verifier to general. All previously acquired network information that was received unverified will be purged. {}", hex::encode(verifier_key.genesis_hash()), hold.show()),
            Warning::TypesAlreadyThere => String::from("Received types information is identical to the one that was in the database."),
            Warning::NetworkSpecsAlreadyThere (x) => format!("Received network specs information for {x} is same as the one already in the database."),
            Warning::MetadataExtensionsIncomplete => String::from("Received metadata has incomplete set of signed extensions. As a result, Vault may be unable to parse signable transactions using this metadata."),
        }
    }
}

impl Card<'_> {
    pub(crate) fn card(&self, index: &mut u32, indent: u32) -> TransactionCard {
        let card = match &self {
            Card::ParserCard(m) => match m {
                ParserCard::Pallet(f) => NavCard::PalletCard { f: f.clone() },
                ParserCard::Method { method_name, docs } => NavCard::CallCard {
                    f: MSCCall {
                        method_name: method_name.clone(),
                        docs: docs.clone(),
                    },
                },
                ParserCard::Varname(varname) => NavCard::VarNameCard { f: varname.clone() },
                ParserCard::Default(decoded_string) => NavCard::DefaultCard {
                    f: decoded_string.clone(),
                },
                ParserCard::Text(decoded_text) => NavCard::TextCard {
                    f: decoded_text.clone(),
                },
                ParserCard::Id { id, base58prefix } => NavCard::IdCard {
                    f: MSCId {
                        base58: id
                            .to_ss58check_with_version(Ss58AddressFormat::custom(*base58prefix)),
                        identicon: make_identicon_from_account(id.to_owned()),
                    },
                },
                ParserCard::Id20 {
                    id,
                    base58prefix: _,
                } => NavCard::IdCard {
                    f: MSCId {
                        base58: format!("0x{}", hex::encode(id)),
                        identicon: make_identicon_from_id20(id),
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
                        docs_field_name: docs_field_name.clone(),
                        path_type: path_type.clone(),
                        docs_type: docs_type.clone(),
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
                        docs_field_number: docs_field_number.clone(),
                        path_type: path_type.clone(),
                        docs_type: docs_type.clone(),
                    },
                },
                ParserCard::EnumVariantName {
                    name,
                    docs_enum_variant,
                } => NavCard::EnumVariantNameCard {
                    f: MSCEnumVariantName {
                        name: name.clone(),
                        docs_enum_variant: docs_enum_variant.clone(),
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
                genesis_hash,
                address_details,
            } => NavCard::AuthorCard {
                f: make_author_info(author, *base58prefix, *genesis_hash, address_details),
            },
            Card::AuthorPlain {
                author,
                base58prefix,
            } => NavCard::AuthorPlainCard {
                f: MSCId {
                    base58: print_multisigner_as_base58_or_eth(
                        author,
                        Some(*base58prefix),
                        Encryption::Sr25519,
                    ),
                    identicon: make_identicon_from_multisigner(author, IdenticonStyle::Dots),
                },
            },
            Card::Verifier(x) => match x {
                VerifierValue::Standard { m } => {
                    let (public_key, encryption) = match m {
                        MultiSigner::Ed25519(p) => (hex::encode(p), Encryption::Ed25519.show()),
                        MultiSigner::Sr25519(p) => (hex::encode(p), Encryption::Sr25519.show()),
                        MultiSigner::Ecdsa(p) => (hex::encode(p), Encryption::Ecdsa.show()),
                    };
                    NavCard::VerifierCard {
                        f: MVerifierDetails {
                            public_key,
                            identicon: make_identicon_from_multisigner(m, IdenticonStyle::Dots),
                            encryption,
                        },
                    }
                }
            },
            Card::Meta(x) => NavCard::MetaCard {
                f: MMetadataRecord {
                    specname: x.name.clone(),
                    specs_version: x.version.to_string(),
                    meta_hash: hex::encode(x.meta_hash),
                    meta_id_pic: pic_meta(x.meta_hash.as_bytes()),
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
                    network_title: x.specs.title.clone(),
                    network_logo: x.specs.logo.clone(),
                    network_specs_key: hex::encode(
                        NetworkSpecsKey::from_parts(&x.specs.genesis_hash, &x.specs.encryption)
                            .key(),
                    ),
                },
            },
            Card::Derivations(x) => NavCard::DerivationsCard { f: x.to_vec() },
            Card::Warning(warn) => NavCard::WarningCard { f: warn.show() },
            Card::Error(err) => NavCard::ErrorCard {
                f: format!("Bad input data. {err}"),
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
    genesis_hash: H256,
    address_details: &AddressDetails,
) -> MAddressCard {
    let base58 =
        print_multisigner_as_base58_or_eth(author, Some(base58prefix), address_details.encryption);
    let address_key = hex::encode(AddressKey::new(author.clone(), Some(genesis_hash)).key());
    MAddressCard {
        base58,
        address_key,
        address: Address {
            identicon: make_identicon_from_multisigner(author, address_details.identicon_style()),
            seed_name: address_details.seed_name.clone(),
            path: address_details.path.clone(),
            has_pwd: address_details.has_pwd,
            secret_exposed: address_details.secret_exposed,
        },
    }
}
