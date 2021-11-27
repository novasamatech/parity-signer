use bitvec::prelude::{BitVec, Lsb0};
use sled::IVec;
use definitions::{crypto::Encryption, history::MetaValuesDisplay, keyring::{NetworkSpecsKey, VerifierKey}, metadata::MetaValues, network_specs::{Verifier, VerifierValue, ChainSpecsToSend}, qr_transfers::ContentLoadTypes};
use hex;
use std::convert::TryInto;
use parser::{cards::ParserCard, error::{ParserError, DecodingError, MetadataError, SystemError}};
use sp_core;
use sp_runtime::{generic::Era, MultiSigner};

use crate::cards::{Card, Warning};
use crate::error::{Error, BadInputData, DatabaseError, CryptoError};
use crate::helpers::{GeneralHold, Hold};

const PUBLIC: [u8; 32] = [142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72];

fn verifier_sr25519() -> Verifier {
    Verifier(Some(VerifierValue::Standard(MultiSigner::Sr25519(sp_core::sr25519::Public::from_raw(PUBLIC)))))
}
    
fn verifier_ed25519() -> Verifier {
    Verifier(Some(VerifierValue::Standard(MultiSigner::Ed25519(sp_core::ed25519::Public::from_raw(PUBLIC)))))
}

/// Function to pring all types of cards.
/// Should be used to check how the cards are printed in the app.

pub fn make_all_cards() -> String {
    
    let mut index = 0;
    let mut all_cards: Vec<String> = Vec::new();
    let network_specs_westend = ChainSpecsToSend {
        base58prefix: 42,
        color: String::from("#660D35"),
        decimals: 12,
        encryption: Encryption::Sr25519,
        genesis_hash: hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").expect("known value").try_into().expect("known value"),
        logo: String::from("westend"),
        name: String::from("westend"),
        path_id: String::from("//westend"),
        secondary_color: String::from("#262626"),
        title: String::from("Westend"),
        unit: String::from("WND"),
    };
    let bv: BitVec<Lsb0, u8> = BitVec::from_vec(vec![32, 4, 155]);
    
    all_cards.push(Card::ParserCard(&ParserCard::Pallet("test_pallet".to_string())).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::Method{method_name: "test_method".to_string(), docs: "verbose \ndescription \nof \nthe \nmethod".to_string()}).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::Varname("test_Varname".to_string())).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::Default("12345".to_string())).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::Text("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string())).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::Id("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string())).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::None).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::IdentityField("Twitter".to_string())).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::BitVec(bv.to_string())).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::Balance{number: "300.000000".to_string(), units: "KULU".to_string()}).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::FieldName{name: "test_FieldName".to_string(), docs_field_name: "a very special field".to_string(), path_type: "field >> path >> TypePath".to_string(), docs_type: "type is difficult to describe".to_string()}).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::FieldNumber{number: 1, docs_field_number: "less special field".to_string(), path_type: "field >> path >> TypePath".to_string(), docs_type: "type is just as difficult to describe".to_string()}).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::EnumVariantName{name: "test_EnumVariantName".to_string(), docs_enum_variant: "".to_string()}).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::Era(Era::Immortal)).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::Era(Era::Mortal(64, 31))).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::Nonce("15".to_string())).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::BlockHash(hex::decode("a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd").expect("checked value").try_into().expect("checked value"))).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::Tip{number: "0".to_string(), units: "pWND".to_string()}).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::NetworkNameVersion{name: "westend".to_string(), version: "9110".to_string()}).card(&mut index,0));
    all_cards.push(Card::ParserCard(&ParserCard::TxVersion("5".to_string())).card(&mut index,0));
    
    all_cards.push(Card::Author{base58_author: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", seed_name: "Alice", path: "//Alice", has_pwd: false, name: ""}.card(&mut index,0));
    all_cards.push(Card::AuthorPlain("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty").card(&mut index,0));
    all_cards.push(Card::AuthorPublicKey{author_public_key: PUBLIC.to_vec(), encryption: Encryption::Sr25519}.card(&mut index,0));
    all_cards.push(Card::Verifier(&verifier_sr25519()).card(&mut index,0));
    all_cards.push(Card::Meta(MetaValuesDisplay::get(&MetaValues{name: String::from("westend"), version: 9100, meta: Vec::new()})).card(&mut index,0));
    all_cards.push(Card::TypesInfo(ContentLoadTypes::generate(&Vec::new())).card(&mut index,0));
    all_cards.push(Card::NewSpecs(&network_specs_westend).card(&mut index,0));
//    all_cards.push(Card::Message("Hello!").card(&mut index,0));
    all_cards.push(Card::NetworkName("westend").card(&mut index,0));
    all_cards.push(Card::NetworkGenesisHash(&network_specs_westend.genesis_hash.to_vec()).card(&mut index,0));

    all_cards.push(Card::Warning(Warning::AuthorNotFound).card(&mut index,0));
    all_cards.push(Card::Warning(Warning::NewerVersion{used_version: 50, latest_version: 9010}).card(&mut index,0));
    all_cards.push(Card::Warning(Warning::NoNetworkID).card(&mut index,0));
    all_cards.push(Card::Warning(Warning::NotVerified).card(&mut index,0));
    all_cards.push(Card::Warning(Warning::UpdatingTypes).card(&mut index,0));
    all_cards.push(Card::Warning(Warning::TypesNotVerified).card(&mut index,0));
    all_cards.push(Card::Warning(Warning::GeneralVerifierAppeared(&GeneralHold{metadata_set: Vec::new(), network_specs_set: Vec::new(), types: true})).card(&mut index,0));
    all_cards.push(Card::Warning(Warning::VerifierChangingToGeneral{verifier_key: &VerifierKey::from_parts(&network_specs_westend.genesis_hash.to_vec()), hold: &Hold{metadata_set: Vec::new(), network_specs_set: Vec::new()}}).card(&mut index,0));
    all_cards.push(Card::Warning(Warning::VerifierChangingToCustom{verifier_key: &VerifierKey::from_parts(&network_specs_westend.genesis_hash.to_vec()), hold: &Hold{metadata_set: Vec::new(), network_specs_set: Vec::new()}}).card(&mut index,0));
    all_cards.push(Card::Warning(Warning::VerifierGeneralSuper{verifier_key: &VerifierKey::from_parts(&network_specs_westend.genesis_hash.to_vec()), hold: &Hold{metadata_set: Vec::new(), network_specs_set: Vec::new()}}).card(&mut index,0));
    all_cards.push(Card::Warning(Warning::TypesAlreadyThere).card(&mut index,0));
    all_cards.push(Card::Warning(Warning::NetworkSpecsAlreadyThere(&network_specs_westend.title)).card(&mut index,0));
    
    all_cards.push(Card::Error(Error::AllParsingFailed(vec![("westend".to_string(), 9120, ParserError::WrongNetworkVersion{as_decoded: "50".to_string(), in_metadata: 9120}), ("westend".to_string(), 9010, ParserError::WrongNetworkVersion{as_decoded: "50".to_string(), in_metadata: 9010})])).card(&mut index,0));
    
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::UnexpectedImmortality))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::UnexpectedMortality))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::GenesisHashMismatch))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::ImmortalHashMismatch))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::ExtensionsOlder))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::MethodNotFound{method_index: 2, pallet_name: "test_Pallet".to_string()}))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::PalletNotFound(3)))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::MethodIndexTooHigh{method_index: 5, pallet_index: 3, total: 4}))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::NoCallsInPallet("test_pallet_v14".to_string())))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::V14TypeNotResolved))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::ArgumentTypeError))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::ArgumentNameError))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::NotPrimitive(String::from("Option<u8>"))))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::NoCompact))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::DataTooShort))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::PrimitiveFailure("u32".to_string())))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::UnexpectedOptionVariant))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::IdFields))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::Array))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::BalanceNotDescribed))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::UnexpectedEnumVariant))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::UnexpectedCompactInsides))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::CompactNotPrimitive))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::UnknownType("T::SomeUnknownType".to_string())))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::NotBitStoreType))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::NotBitOrderType))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::BitVecFailure))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::Era))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::SomeDataNotUsedMethod))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::Decoding(DecodingError::SomeDataNotUsedExtensions))).card(&mut index,0));
    
    all_cards.push(Card::Error(Error::Parser(ParserError::FundamentallyBadV14Metadata(MetadataError::NoEra))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::FundamentallyBadV14Metadata(MetadataError::NoBlockHash))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::FundamentallyBadV14Metadata(MetadataError::NoVersionExt))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::FundamentallyBadV14Metadata(MetadataError::EraTwice))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::FundamentallyBadV14Metadata(MetadataError::GenesisHashTwice))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::FundamentallyBadV14Metadata(MetadataError::BlockHashTwice))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::FundamentallyBadV14Metadata(MetadataError::SpecVersionTwice))).card(&mut index,0));
    
    all_cards.push(Card::Error(Error::Parser(ParserError::SystemError(SystemError::BalanceFail))).card(&mut index,0));
    all_cards.push(Card::Error(Error::Parser(ParserError::SystemError(SystemError::RegexError))).card(&mut index,0));
    
    all_cards.push(Card::Error(Error::Parser(ParserError::WrongNetworkVersion{as_decoded: "50".to_string(), in_metadata: 9120})).card(&mut index,0));
    
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::TooShort)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::NotSubstrate)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::NotHex)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::CryptoNotSupported)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::WrongPayloadType)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::LoadMetaUnknownNetwork(network_specs_westend.name.to_string()))).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::NotMeta)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::MetaVersionBelow12)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::MetaAlreadyThere)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::MetaTotalMismatch)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::VersionNotDecodeable)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::NoMetaVersion)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::UnableToDecodeMeta)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::SpecsAlreadyThere)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::UnableToDecodeTypes)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::TypesAlreadyThere)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::UnableToDecodeAddSpecsMessage)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::UnableToDecodeLoadMetadataMessage)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::ImportantSpecsChanged)).card(&mut index,0));
    
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::Internal(sled::Error::CollectionNotFound(IVec::from(vec![1]))))).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::Internal(sled::Error::Unsupported(String::from("Something Unsupported."))))).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::Internal(sled::Error::ReportableBug(String::from("Please report me"))))).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::Internal(sled::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))))).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::Internal(sled::Error::Corruption{at: None, bt: ()}))).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::DamagedChainSpecs)).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::NoNetwork)).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::DamagedAddressDetails)).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::DamagedTypesDatabase)).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::NoTypes)).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::DamagedVersName)).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::NoMetaAtAll)).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::DamagedGeneralVerifier)).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::NoGeneralVerifier)).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::DamagedNetworkVerifier)).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::NetworkSpecsKeyMismatch(NetworkSpecsKey::from_parts(&network_specs_westend.genesis_hash.to_vec(), &network_specs_westend.encryption)))).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::UnexpectedlyMetGenesisHash(network_specs_westend.genesis_hash.to_vec()))).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::DifferentNamesSameGenesisHash(network_specs_westend.genesis_hash.to_vec()))).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::Temporary(String::from("This error should not be here.")))).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::DeadVerifier(network_specs_westend.name.to_string()))).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::CustomVerifierIsGeneral(VerifierKey::from_parts(&network_specs_westend.genesis_hash.to_vec())))).card(&mut index,0));
    
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::NotMeta)).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::MetaVersionBelow12)).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::MetaMismatch)).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::NoVersion)).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::VersionNotDecodeable)).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::UnableToDecodeMeta)).card(&mut index,0));
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::RuntimeVersionIncompatible)).card(&mut index,0));
    
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::BadSignature)).card(&mut index,0));
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::AddSpecsVerifierChanged{network_name: network_specs_westend.name.to_string(), old: verifier_sr25519(), new: verifier_ed25519()})).card(&mut index,0));
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::VerifierDisappeared)).card(&mut index,0));
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::GeneralVerifierChanged{old: verifier_sr25519(), new: verifier_ed25519()})).card(&mut index,0));
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::GeneralVerifierDisappeared)).card(&mut index,0));
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::LoadMetaUpdVerifier{network_name: network_specs_westend.name.to_string(), new_verifier: verifier_sr25519()})).card(&mut index,0));
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::LoadMetaVerifierChanged{network_name: network_specs_westend.name.to_string(), old: verifier_sr25519(), new: verifier_ed25519()})).card(&mut index,0));
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::LoadMetaUpdGeneralVerifier{network_name: network_specs_westend.name.to_string(), new_verifier: verifier_sr25519()})).card(&mut index,0));
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::LoadMetaGeneralVerifierChanged{network_name: network_specs_westend.name.to_string(), old: verifier_sr25519(), new: verifier_ed25519()})).card(&mut index,0));
    
    let mut output_cards = String::from("{\"method\":[");
    
    for (i,x) in all_cards.iter().enumerate() {
        if i > 0 {output_cards.push_str(",")}
        output_cards.push_str(&x);
    }
    
    output_cards.push_str("]}");
    output_cards
}
