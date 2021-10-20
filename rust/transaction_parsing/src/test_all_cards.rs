use bitvec::prelude::{BitVec, Lsb0};
use sled::IVec;
use definitions::{crypto::Encryption, history::MetaValuesDisplay, keyring::{NetworkSpecsKey, VerifierKey}, metadata::MetaValues, network_specs::{Verifier, ChainSpecsToSend}, qr_transfers::ContentLoadTypes};
use hex;
use std::convert::TryInto;

use crate::cards::{Card, Warning};
use crate::error::{Error, BadInputData, UnableToDecode, DatabaseError, SystemError, CryptoError};
use crate::helpers::{GeneralHold, Hold};

const PUBLIC: [u8; 32] = [142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72];

/// Function to pring all types of cards.
/// Should be used to check how the cards are printed in the app.

pub fn make_all_cards() -> String {
    
    let mut index = 0;
    let mut all_cards: Vec<String> = Vec::new();
    
    all_cards.push(Card::Call{pallet: "test_Pallet", method: "test_Method", docs: "test docs description"}.card(&mut index,0));
    all_cards.push(Card::Pallet{pallet_name: "test_pallet_v14", path: "test >> test_test >> TestTest", docs: "test docs"}.card(&mut index,0));
    all_cards.push(Card::Varname("test_Varname").card(&mut index,0));
    all_cards.push(Card::Default("12345").card(&mut index,0));
    all_cards.push(Card::Id("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty").card(&mut index,0));
    all_cards.push(Card::None.card(&mut index,0));
    all_cards.push(Card::IdentityField("Twitter").card(&mut index,0));
    
    let bv: BitVec<Lsb0, u8> = BitVec::from_vec(vec![32, 4, 155]);
    all_cards.push(Card::BitVec(bv.to_string()).card(&mut index,0));
    
    all_cards.push(Card::Balance{number: "300.000000", units: "KULU"}.card(&mut index,0));
    all_cards.push(Card::FieldName{name: "test_FieldName", docs_field_name: "a very special field", path_type: "field >> path >> TypePath", docs_type: "type is difficult to describe"}.card(&mut index,0));
    all_cards.push(Card::FieldNumber{number: 1, docs_field_number: "less special field", path_type: "field >> path >> TypePath", docs_type: "type is just as difficult to describe"}.card(&mut index,0));
    all_cards.push(Card::EnumVariantName{name: "test_EnumVariantName", docs_enum_variant: ""}.card(&mut index,0));
    all_cards.push(Card::EraImmortalNonce(4980).card(&mut index,0));
    all_cards.push(Card::EraMortalNonce{phase: 55, period: 64, nonce: 89}.card(&mut index,0));
    all_cards.push(Card::Tip{number: "0", units: "pWND"}.card(&mut index,0));
    all_cards.push(Card::TipPlain(8800).card(&mut index,0));
    all_cards.push(Card::BlockHash("a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd").card(&mut index,0));
    all_cards.push(Card::TxSpec{network: "westend", version: 50, tx_version: 5}.card(&mut index,0));
    all_cards.push(Card::TxSpecPlain{gen_hash: "a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd", version: 50, tx_version: 5}.card(&mut index,0));
    all_cards.push(Card::Author{base58_author: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", seed_name: "Alice", path: "//Alice", has_pwd: false, name: ""}.card(&mut index,0));
    all_cards.push(Card::AuthorPlain("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty").card(&mut index,0));
    all_cards.push(Card::AuthorPublicKey{author_public_key: PUBLIC.to_vec(), encryption: Encryption::Sr25519}.card(&mut index,0));
    all_cards.push(Card::Verifier(&Verifier::Sr25519(PUBLIC)).card(&mut index,0));
    all_cards.push(Card::Meta(MetaValuesDisplay::get(&MetaValues{name: String::from("westend"), version: 9100, meta: Vec::new()})).card(&mut index,0));
    all_cards.push(Card::TypesInfo(ContentLoadTypes::generate(&Vec::new())).card(&mut index,0));

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
    all_cards.push(Card::NewSpecs(&network_specs_westend).card(&mut index,0));

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
    
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::TooShort)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::NotSubstrate)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::NotHex)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::CryptoNotSupported)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::UnexpectedImmortality)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::UnexpectedMortality)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::WrongPayloadType)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::GenesisHashMismatch)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::ImmortalHashMismatch)).card(&mut index,0));
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::SomeDataNotUsed)).card(&mut index,0));
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
    all_cards.push(Card::Error(Error::BadInputData(BadInputData::EncryptionMismatch)).card(&mut index,0));
    
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::MethodAndExtrinsicsFailure)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::NeedPalletAndMethod)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::NeedPallet)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::MethodNotFound{method_index: 2, pallet_name: "test_Pallet".to_string()})).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::PalletNotFound(3))).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::MethodIndexTooHigh{method_index: 5, pallet_index: 3, total: 4})).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::NoCallsInPallet("test_pallet_v14".to_string()))).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::V14TypeNotResolved)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::ArgumentTypeError)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::ArgumentNameError)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::NotPrimitive(String::from("Option<u8>")))).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::NoCompact)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::DataTooShort)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::PrimitiveFailure(String::from("u32")))).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::UnexpectedOptionVariant)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::IdFields)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::Array)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::BalanceNotDescribed)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::UnexpectedEnumVariant)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::UnexpectedCompactInsides)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::CompactNotPrimitive)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::UnknownType(String::from("T::SomeUnknownType")))).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::NotBitStoreType)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::NotBitOrderType)).card(&mut index,0));
    all_cards.push(Card::Error(Error::UnableToDecode(UnableToDecode::BitVecFailure)).card(&mut index,0));
    
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
    all_cards.push(Card::Error(Error::DatabaseError(DatabaseError::NoMetaThisVersion)).card(&mut index,0));
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

    all_cards.push(Card::Error(Error::SystemError(SystemError::BalanceFail)).card(&mut index,0));
    all_cards.push(Card::Error(Error::SystemError(SystemError::NotMeta)).card(&mut index,0));
    all_cards.push(Card::Error(Error::SystemError(SystemError::MetaVersionBelow12)).card(&mut index,0));
    all_cards.push(Card::Error(Error::SystemError(SystemError::MetaMismatch)).card(&mut index,0));
    all_cards.push(Card::Error(Error::SystemError(SystemError::NoVersion)).card(&mut index,0));
    all_cards.push(Card::Error(Error::SystemError(SystemError::VersionNotDecodeable)).card(&mut index,0));
    all_cards.push(Card::Error(Error::SystemError(SystemError::UnableToDecodeMeta)).card(&mut index,0));
    all_cards.push(Card::Error(Error::SystemError(SystemError::RegexError)).card(&mut index,0));
    
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::BadSignature)).card(&mut index,0));
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::AddSpecsVerifierChanged{network_name: network_specs_westend.name.to_string(), old: Verifier::Sr25519(PUBLIC), new: Verifier::Ed25519(PUBLIC)})).card(&mut index,0));
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::VerifierDisappeared)).card(&mut index,0));
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::GeneralVerifierChanged{old: Verifier::Sr25519(PUBLIC), new: Verifier::Ed25519(PUBLIC)})).card(&mut index,0));
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::GeneralVerifierDisappeared)).card(&mut index,0));
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::LoadMetaUpdVerifier{network_name: network_specs_westend.name.to_string(), new_verifier: Verifier::Sr25519(PUBLIC)})).card(&mut index,0));
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::LoadMetaVerifierChanged{network_name: network_specs_westend.name.to_string(), old: Verifier::Sr25519(PUBLIC), new: Verifier::Ed25519(PUBLIC)})).card(&mut index,0));
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::LoadMetaUpdGeneralVerifier{network_name: network_specs_westend.name.to_string(), new_verifier: Verifier::Sr25519(PUBLIC)})).card(&mut index,0));
    all_cards.push(Card::Error(Error::CryptoError(CryptoError::LoadMetaGeneralVerifierChanged{network_name: network_specs_westend.name.to_string(), old: Verifier::Sr25519(PUBLIC), new: Verifier::Ed25519(PUBLIC)})).card(&mut index,0));
    
    let mut output_cards = String::from("{\"method\":[");
    
    for (i,x) in all_cards.iter().enumerate() {
        if i > 0 {output_cards.push_str(",")}
        output_cards.push_str(&x);
    }
    
    output_cards.push_str("]}");
    output_cards
}
