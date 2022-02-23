use anyhow::anyhow;
use hex;
use sled::{IVec, transaction::TransactionError};
use sp_core::crypto::SecretStringError;
use sp_runtime::MultiSigner;

use crate::crypto::Encryption;
use crate::error::{AddressGeneration, AddressGenerationCommon, DatabaseSigner, EntryDecodingSigner, ErrorSigner, ExtraAddressGenerationSigner, GeneralVerifierForContent, InputSigner, InterfaceSigner, KeyDecodingSignerDb, KeyDecodingSignerInterface, MetadataError, MismatchSigner, NotFoundSigner, NotHexSigner, ParserDecodingError, ParserError, ParserMetadataError, TransferContent};
use crate::keyring::{AddressKey, MetaKey, NetworkSpecsKey, VerifierKey};
use crate::network_specs::{ValidCurrentVerifier, Verifier, VerifierValue};

const PUBLIC: [u8; 32] = [142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72];

fn verifier_sr25519() -> Verifier {Verifier(Some(verifier_value_sr25519()))}
fn verifier_value_sr25519() -> VerifierValue {VerifierValue::Standard(MultiSigner::Sr25519(sp_core::sr25519::Public::from_raw(PUBLIC)))}
fn verifier_value_ed25519() -> VerifierValue {VerifierValue::Standard(MultiSigner::Ed25519(sp_core::ed25519::Public::from_raw(PUBLIC)))}

fn db_internal_error_set() -> Vec<sled::Error> {
    let mut out: Vec<sled::Error> = Vec::new();
    out.push(sled::Error::CollectionNotFound(IVec::from(vec![1])));
    out.push(sled::Error::Unsupported(String::from("Something Unsupported.")));
    out.push(sled::Error::ReportableBug(String::from("Please report me")));
    out.push(sled::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "oh no!")));
    out.push(sled::Error::Corruption{at: None, bt: ()});
    out
}

fn metadata_error_set() -> Vec<MetadataError> {
    let mut out: Vec<MetadataError> = Vec::new();
    out.push(MetadataError::VersionIncompatible);
    out.push(MetadataError::NoSystemPallet);
    out.push(MetadataError::NoVersionInConstants);
    out.push(MetadataError::RuntimeVersionNotDecodeable);
    out.push(MetadataError::NotMeta);
    out.push(MetadataError::UnableToDecode);
    out
}

fn secret_string_error_set() -> Vec<SecretStringError> {
    let mut out: Vec<SecretStringError> = Vec::new();
    out.push(SecretStringError::InvalidFormat);
    out.push(SecretStringError::InvalidPhrase);
    out.push(SecretStringError::InvalidPassword);
    out.push(SecretStringError::InvalidSeed);
    out.push(SecretStringError::InvalidSeedLength);
    out.push(SecretStringError::InvalidPath);
    out
}

fn content() -> Vec<GeneralVerifierForContent> {
    vec![
        GeneralVerifierForContent::Network{name: String::from("westend")},
        GeneralVerifierForContent::Types,
    ]
}

fn parser_metadata_error_set() -> Vec<ParserMetadataError> {
    let mut out: Vec<ParserMetadataError> = Vec::new();
    out.push(ParserMetadataError::NoEra);
    out.push(ParserMetadataError::NoBlockHash);
    out.push(ParserMetadataError::NoVersionExt);
    out.push(ParserMetadataError::EraTwice);
    out.push(ParserMetadataError::GenesisHashTwice);
    out.push(ParserMetadataError::BlockHashTwice);
    out.push(ParserMetadataError::SpecVersionTwice);
    out
}

fn all_ext_parsing_failed_set() -> Vec<(u32, ParserError)> {
    vec![
        (9010, ParserError::WrongNetworkVersion {as_decoded: String::from("9122"), in_metadata: 9010}),
        (9000, ParserError::WrongNetworkVersion {as_decoded: String::from("9122"), in_metadata: 9000}),
    ]
}

pub fn signer_errors() -> Vec<ErrorSigner> {
    
    let address_key_bad = AddressKey::from_hex("0350e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779").unwrap();
    let address_key_good = AddressKey::from_hex("0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779").unwrap();
    let entry_order_vec: Vec<u8> = vec![100, 4, 85];
    let genesis_hash: Vec<u8> = hex::decode("e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap();
    let meta_key = MetaKey::from_parts("westend", 9122);
    let network_specs_key_bad = NetworkSpecsKey::from_hex("0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap();
    let network_specs_key_good = NetworkSpecsKey::from_hex("0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap();
    let not_hex_string = String::from("0xabracadabra");
    let verifier_key = VerifierKey::from_parts(&hex::decode("853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd").unwrap());
    let valid_current_verifier = ValidCurrentVerifier::General;
    
    let mut error_set: Vec<ErrorSigner> = Vec::new();
    
    error_set.push(ErrorSigner::Interface(InterfaceSigner::NotHex(NotHexSigner::NetworkSpecsKey{input: not_hex_string.to_string()})));
    error_set.push(ErrorSigner::Interface(InterfaceSigner::NotHex(NotHexSigner::InputContent)));
    error_set.push(ErrorSigner::Interface(InterfaceSigner::NotHex(NotHexSigner::AddressKey{input: not_hex_string.to_string()})));
    error_set.push(ErrorSigner::Interface(InterfaceSigner::KeyDecoding(KeyDecodingSignerInterface::AddressKey(address_key_bad.to_owned()))));
    error_set.push(ErrorSigner::Interface(InterfaceSigner::KeyDecoding(KeyDecodingSignerInterface::NetworkSpecsKey(network_specs_key_bad.to_owned()))));
    error_set.push(ErrorSigner::Interface(InterfaceSigner::PublicKeyLength));
    error_set.push(ErrorSigner::Interface(InterfaceSigner::HistoryPageOutOfRange{page_number: 14, total_pages: 10}));
    
    error_set.push(ErrorSigner::Database(DatabaseSigner::KeyDecoding(KeyDecodingSignerDb::AddressKey(address_key_bad.to_owned()))));
    error_set.push(ErrorSigner::Database(DatabaseSigner::KeyDecoding(KeyDecodingSignerDb::EntryOrder(entry_order_vec.to_owned()))));
    error_set.push(ErrorSigner::Database(DatabaseSigner::KeyDecoding(KeyDecodingSignerDb::MetaKey(meta_key.to_owned()))));
    error_set.push(ErrorSigner::Database(DatabaseSigner::KeyDecoding(KeyDecodingSignerDb::NetworkSpecsKey(network_specs_key_bad.to_owned()))));
    error_set.push(ErrorSigner::Database(DatabaseSigner::KeyDecoding(KeyDecodingSignerDb::NetworkSpecsKeyAddressDetails{address_key: address_key_good.to_owned(), network_specs_key: network_specs_key_bad.to_owned()})));
    for e in db_internal_error_set().into_iter() {error_set.push(ErrorSigner::Database(DatabaseSigner::Internal(e)));}
    for e in db_internal_error_set().into_iter(){error_set.push(ErrorSigner::Database(DatabaseSigner::Transaction(TransactionError::Storage(e))));}
    error_set.push(ErrorSigner::Database(DatabaseSigner::ChecksumMismatch));
    error_set.push(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::AddressDetails(address_key_good.to_owned()))));
    error_set.push(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::CurrentVerifier(verifier_key.to_owned()))));
    error_set.push(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::DangerStatus)));
    error_set.push(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::GeneralVerifier)));
    error_set.push(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::HistoryEntry(135))));
    error_set.push(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::NetworkSpecs(network_specs_key_good.to_owned()))));
    error_set.push(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::Sign)));
    error_set.push(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::Stub)));
    error_set.push(ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::Types)));
    error_set.push(ErrorSigner::Database(DatabaseSigner::Mismatch(MismatchSigner::Metadata{name_key: String::from("westend"), version_key: 1922, name_inside: String::from("westend"), version_inside: 9122})));
    error_set.push(ErrorSigner::Database(DatabaseSigner::Mismatch(MismatchSigner::SpecsGenesisHash{key: network_specs_key_good.to_owned(), genesis_hash: genesis_hash.to_vec()})));
    error_set.push(ErrorSigner::Database(DatabaseSigner::Mismatch(MismatchSigner::SpecsEncryption{key: network_specs_key_good.to_owned(), encryption: Encryption::Ecdsa})));
    error_set.push(ErrorSigner::Database(DatabaseSigner::Mismatch(MismatchSigner::AddressDetailsEncryption{key: address_key_good.to_owned(), encryption: Encryption::Ecdsa})));
    error_set.push(ErrorSigner::Database(DatabaseSigner::Mismatch(MismatchSigner::AddressDetailsSpecsEncryption{address_key: address_key_good.to_owned(), network_specs_key: network_specs_key_bad.to_owned()})));
    for error in metadata_error_set().into_iter() {error_set.push(ErrorSigner::Database(DatabaseSigner::FaultyMetadata{name: String::from("westend"), version: 9000, error}));}
    error_set.push(ErrorSigner::Database(DatabaseSigner::UnexpectedGenesisHash{verifier_key: VerifierKey::from_parts(&genesis_hash), network_specs_key: network_specs_key_good.to_owned()}));
    error_set.push(ErrorSigner::Database(DatabaseSigner::SpecsCollision{name: String::from("westend"), encryption: Encryption::Sr25519}));
    error_set.push(ErrorSigner::Database(DatabaseSigner::DifferentNamesSameGenesisHash{name1: String::from("westend"), name2: String::from("WeStEnD"), genesis_hash: genesis_hash.to_vec()}));
    error_set.push(ErrorSigner::Database(DatabaseSigner::TwoTransactionsInEntry(135)));
    error_set.push(ErrorSigner::Database(DatabaseSigner::CustomVerifierIsGeneral(verifier_key.to_owned())));
    
    error_set.push(ErrorSigner::Input(InputSigner::TransferContent(TransferContent::AddSpecs)));
    error_set.push(ErrorSigner::Input(InputSigner::TransferContent(TransferContent::LoadMeta)));
    error_set.push(ErrorSigner::Input(InputSigner::TransferContent(TransferContent::LoadTypes)));
    for e in metadata_error_set().into_iter() {error_set.push(ErrorSigner::Input(InputSigner::FaultyMetadata(e)));}
    error_set.push(ErrorSigner::Input(InputSigner::TooShort));
    error_set.push(ErrorSigner::Input(InputSigner::NotSubstrate(String::from("35"))));
    error_set.push(ErrorSigner::Input(InputSigner::PayloadNotSupported(String::from("0f"))));
    error_set.push(ErrorSigner::Input(InputSigner::SameNameVersionDifferentMeta{name: String::from("kusama"), version: 9110}));
    error_set.push(ErrorSigner::Input(InputSigner::MetadataKnown{name: String::from("westend"), version: 9122}));
    error_set.push(ErrorSigner::Input(InputSigner::ImportantSpecsChanged(network_specs_key_good.to_owned())));
    error_set.push(ErrorSigner::Input(InputSigner::EncryptionNotSupported(String::from("03"))));
    error_set.push(ErrorSigner::Input(InputSigner::BadSignature));
    error_set.push(ErrorSigner::Input(InputSigner::LoadMetaUnknownNetwork{name: String::from("kulupu")}));
    error_set.push(ErrorSigner::Input(InputSigner::LoadMetaNoSpecs{name: String::from("westend"), valid_current_verifier: valid_current_verifier.to_owned(), general_verifier: verifier_sr25519()}));
    error_set.push(ErrorSigner::Input(InputSigner::NeedVerifier{name: String::from("kulupu"), verifier_value: verifier_value_ed25519()}));
    for content in content().into_iter() {error_set.push(ErrorSigner::Input(InputSigner::NeedGeneralVerifier{content, verifier_value: verifier_value_sr25519()}));}
    error_set.push(ErrorSigner::Input(InputSigner::LoadMetaSetVerifier{name: String::from("kulupu"), new_verifier_value: verifier_value_ed25519()}));
    error_set.push(ErrorSigner::Input(InputSigner::LoadMetaVerifierChanged{name: String::from("kulupu"), old_verifier_value: verifier_value_sr25519(), new_verifier_value: verifier_value_ed25519()}));
    error_set.push(ErrorSigner::Input(InputSigner::LoadMetaSetGeneralVerifier{name: String::from("westend"), new_general_verifier_value: verifier_value_sr25519()}));
    error_set.push(ErrorSigner::Input(InputSigner::LoadMetaGeneralVerifierChanged{name: String::from("westend"), old_general_verifier_value: verifier_value_sr25519(), new_general_verifier_value: verifier_value_ed25519()}));
    for content in content().into_iter() {error_set.push(ErrorSigner::Input(InputSigner::GeneralVerifierChanged{content, old_general_verifier_value: verifier_value_sr25519(), new_general_verifier_value: verifier_value_ed25519()}));}
    error_set.push(ErrorSigner::Input(InputSigner::TypesKnown));
    error_set.push(ErrorSigner::Input(InputSigner::MessageNotReadable));
    error_set.push(ErrorSigner::Input(InputSigner::UnknownNetwork{genesis_hash: genesis_hash.to_vec(), encryption: Encryption::Sr25519}));
    error_set.push(ErrorSigner::Input(InputSigner::NoMetadata{name: String::from("westend")}));
    error_set.push(ErrorSigner::Input(InputSigner::SpecsKnown{name: String::from("westend"), encryption: Encryption::Sr25519}));
    error_set.push(ErrorSigner::Input(InputSigner::AddSpecsVerifierChanged {name: String::from("kulupu"), old_verifier_value: verifier_value_sr25519(), new_verifier_value: verifier_value_ed25519()}));
    
    error_set.push(ErrorSigner::NotFound(NotFoundSigner::CurrentVerifier(verifier_key.to_owned())));
    error_set.push(ErrorSigner::NotFound(NotFoundSigner::GeneralVerifier));
    error_set.push(ErrorSigner::NotFound(NotFoundSigner::Types));
    error_set.push(ErrorSigner::NotFound(NotFoundSigner::NetworkSpecs(network_specs_key_bad.to_owned())));
    error_set.push(ErrorSigner::NotFound(NotFoundSigner::NetworkSpecsForName(String::from("westend"))));
    error_set.push(ErrorSigner::NotFound(NotFoundSigner::NetworkSpecsKeyForAddress{network_specs_key: network_specs_key_good.to_owned(), address_key: address_key_good.to_owned()}));
    error_set.push(ErrorSigner::NotFound(NotFoundSigner::AddressDetails(address_key_good.to_owned())));
    error_set.push(ErrorSigner::NotFound(NotFoundSigner::Metadata{name: String::from("westend"), version: 9120}));
    error_set.push(ErrorSigner::NotFound(NotFoundSigner::DangerStatus));
    error_set.push(ErrorSigner::NotFound(NotFoundSigner::Stub));
    error_set.push(ErrorSigner::NotFound(NotFoundSigner::Sign));
    error_set.push(ErrorSigner::NotFound(NotFoundSigner::HistoryEntry(135)));
    error_set.push(ErrorSigner::NotFound(NotFoundSigner::HistoryNetworkSpecs{name: String::from("westend"), encryption: Encryption::Ed25519}));
    error_set.push(ErrorSigner::NotFound(NotFoundSigner::TransactionEvent(280)));
    error_set.push(ErrorSigner::NotFound(NotFoundSigner::HistoricalMetadata{name: String::from("kulupu")}));
    
    error_set.push(ErrorSigner::DeadVerifier(verifier_key.to_owned()));
    
    error_set.push(ErrorSigner::AddressGeneration(AddressGeneration::Common(AddressGenerationCommon::EncryptionMismatch{network_encryption: Encryption::Sr25519, seed_object_encryption: Encryption::Ed25519})));
    error_set.push(ErrorSigner::AddressGeneration(AddressGeneration::Common(AddressGenerationCommon::KeyCollision{seed_name: String::from("Alice super secret seed")})));
    for e in secret_string_error_set().into_iter() {error_set.push(ErrorSigner::AddressGeneration(AddressGeneration::Common(AddressGenerationCommon::SecretString(e))));}
    error_set.push(ErrorSigner::AddressGeneration(AddressGeneration::Extra(ExtraAddressGenerationSigner::RandomPhraseGeneration(anyhow!("Mnemonic generator refuses to work with a valid excuse.")))));
    error_set.push(ErrorSigner::AddressGeneration(AddressGeneration::Extra(ExtraAddressGenerationSigner::InvalidDerivation)));
    
    error_set.push(ErrorSigner::Qr(String::from("QR generator refuses to work with a valid excuse.")));
    
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::UnexpectedImmortality)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::UnexpectedMortality)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::GenesisHashMismatch)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::ImmortalHashMismatch)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::ExtensionsOlder)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::MethodNotFound{method_index: 2, pallet_name: "test_Pallet".to_string()})));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::PalletNotFound(3))));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::MethodIndexTooHigh{method_index: 5, pallet_index: 3, total: 4})));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::NoCallsInPallet("test_pallet_v14".to_string()))));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::V14TypeNotResolved)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::ArgumentTypeError)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::ArgumentNameError)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::NotPrimitive(String::from("Option<u8>")))));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::NoCompact)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::DataTooShort)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::PrimitiveFailure("u32".to_string()))));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::UnexpectedOptionVariant)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::IdFields)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::Array)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::BalanceNotDescribed)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::UnexpectedEnumVariant)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::UnexpectedCompactInsides)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::CompactNotPrimitive)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::UnknownType("T::SomeUnknownType".to_string()))));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::NotBitStoreType)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::NotBitOrderType)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::BitVecFailure)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::Era)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::SomeDataNotUsedMethod)));
    error_set.push(ErrorSigner::Parser(ParserError::Decoding(ParserDecodingError::SomeDataNotUsedExtensions)));
    for e in parser_metadata_error_set().into_iter() {error_set.push(ErrorSigner::Parser(ParserError::FundamentallyBadV14Metadata(e)));}
    error_set.push(ErrorSigner::Parser(ParserError::WrongNetworkVersion {as_decoded: String::from("9122"), in_metadata: 9010}));
    
    error_set.push(ErrorSigner::AllExtensionsParsingFailed{network_name: String::from("westend"), errors: all_ext_parsing_failed_set()});
    
    for e in secret_string_error_set().into_iter() {error_set.push(ErrorSigner::AddressUse(e));}
    
    error_set.push(ErrorSigner::WrongPassword);
    
    error_set
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{ErrorSource, Signer};
    
    #[test]
    fn print_signer_errors_nicely() {
        let mut print = String::from("\n");
        let signer_errors = signer_errors();
        assert!(signer_errors.len() == 155, "Different error set length: {}", signer_errors.len());
        for e in signer_errors.iter() {
            print.push_str(&format!("\"{}\"", <Signer>::show(e)));
            print.push_str("\n");
        }
        let print_expected = r#"
"Error on the interface. Network specs key 0xabracadabra is not in hexadecimal format."
"Error on the interface. Input content is not in hexadecimal format."
"Error on the interface. Address key 0xabracadabra is not in hexadecimal format."
"Error on the interface. Unable to parse address key 0350e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779 passed through the interface."
"Error on the interface. Unable to parse network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e passed through the interface."
"Error on the interface. Public key length does not match the encryption."
"Error on the interface. Requested history page 14 does not exist. Total number of pages 10."
"Database error. Unable to parse address key 0350e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779 from the database."
"Database error. Unable to parse history entry order 640455 from the database."
"Database error. Unable to parse meta key 1c77657374656e64a2230000 from the database."
"Database error. Unable to parse network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e from the database."
"Database error. Unable to parse network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e from network id set of address book entry with key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779 from the database."
"Database error. Internal error. Collection [1] does not exist"
"Database error. Internal error. Unsupported: Something Unsupported."
"Database error. Internal error. Unexpected bug has happened: Please report me. PLEASE REPORT THIS BUG!"
"Database error. Internal error. IO error: oh no!"
"Database error. Internal error. Read corrupted data at file offset None backtrace ()"
"Database error. Transaction error. Collection [1] does not exist"
"Database error. Transaction error. Unsupported: Something Unsupported."
"Database error. Transaction error. Unexpected bug has happened: Please report me. PLEASE REPORT THIS BUG!"
"Database error. Transaction error. IO error: oh no!"
"Database error. Transaction error. Read corrupted data at file offset None backtrace ()"
"Database error. Checksum mismatch."
"Database error. Unable to decode address details entry for key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779."
"Database error. Unable to decode current verifier entry for key 853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd."
"Database error. Unable to decode danger status entry."
"Database error. Unable to decode general verifier entry."
"Database error. Unable to decode history entry for order 135."
"Database error. Unable to decode network specs (NetworkSpecs) entry for key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."
"Database error. Unable to decode temporary entry with information needed for signing approved transaction."
"Database error. Unable to decode temporary entry with information needed for accepting approved information."
"Database error. Unable to decode types information."
"Database error. Mismatch found. Meta key corresponds to westend1922. Stored metadata is westend9122."
"Database error. Mismatch found. Network specs (NetworkSpecs) entry with network specs key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e has not matching genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."
"Database error. Mismatch found. Network specs (NetworkSpecs) entry with network specs key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e has not matching encryption ecdsa."
"Database error. Mismatch found. Address details entry with address key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779 has not matching encryption ecdsa."
"Database error. Mismatch found. Address details entry with address key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779 has associated network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e with wrong encryption."
"Database error. Bad metadata for westend9000. Runtime metadata version is incompatible. Currently supported are v12, v13, and v14."
"Database error. Bad metadata for westend9000. No system pallet in runtime metadata."
"Database error. Bad metadata for westend9000. No runtime version in system pallet constants."
"Database error. Bad metadata for westend9000. Runtime version from system pallet constants could not be decoded."
"Database error. Bad metadata for westend9000. Metadata vector does not start with 0x6d657461."
"Database error. Bad metadata for westend9000. Runtime metadata could not be decoded."
"Database error. No verifier information found for network with genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e, however genesis hash is encountered in network specs entry with key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."
"Database error. More than one entry for network specs with name westend and encryption sr25519."
"Database error. Different network names (westend, WeStEnD) in database for same genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."
"Database error. Entry with order 135 contains more than one transaction-related event. This should not be possible in current Signer and likely indicates database corruption."
"Database error. Network with genesis hash 853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd verifier is set as a custom one. This custom verifier coinsides the database general verifier and not None. This should not have happened and likely indicates database corruption."
"Bad input data. Payload could not be decoded as `add_specs`."
"Bad input data. Payload could not be decoded as `load_meta`."
"Bad input data. Payload could not be decoded as `load_types`."
"Bad input data. Received metadata is unsuitable. Runtime metadata version is incompatible. Currently supported are v12, v13, and v14."
"Bad input data. Received metadata is unsuitable. No system pallet in runtime metadata."
"Bad input data. Received metadata is unsuitable. No runtime version in system pallet constants."
"Bad input data. Received metadata is unsuitable. Runtime version from system pallet constants could not be decoded."
"Bad input data. Received metadata is unsuitable. Metadata vector does not start with 0x6d657461."
"Bad input data. Received metadata is unsuitable. Runtime metadata could not be decoded."
"Bad input data. Input is too short."
"Bad input data. Only Substrate transactions are supported. Transaction is expected to start with 0x53, this one starts with 0x35."
"Bad input data. Payload type with code 0x0f is not supported."
"Bad input data. Metadata for kusama9110 is already in the database and is different from the one in received payload."
"Bad input data. Metadata for westend9122 is already in the database."
"Bad input data. Similar network specs are already stored in the database under key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e. Network specs in received payload have different unchangeable values (base58 prefix, decimals, encryption, network name, unit)."
"Bad input data. Payload with encryption 0x03 is not supported."
"Bad input data. Received payload has bad signature."
"Bad input data. Network kulupu is not in the database. Add network specs before loading the metadata."
"Bad input data. Network westend was previously known to the database with verifier public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519 (general verifier). However, no network specs are in the database at the moment. Add network specs before loading the metadata."
"Bad input data. Saved network kulupu information was signed by verifier public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519. Received information is not signed."
"Bad input data. General verifier in the database is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received unsigned westend network information could be accepted only if signed by the general verifier."
"Bad input data. General verifier in the database is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received unsigned types information could be accepted only if signed by the general verifier."
"Bad input data. Network kulupu currently has no verifier set up. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519. In order to accept verified metadata, first download properly verified network specs."
"Bad input data. Network kulupu current verifier is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519. Changing verifier for the network would require wipe and reset of Signer."
"Bad input data. Network westend is set to be verified by the general verifier, however, general verifier is not yet established. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. In order to accept verified metadata and set up the general verifier, first download properly verified network specs."
"Bad input data. Network westend is verified by the general verifier which currently is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received load_metadata message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519. Changing the general verifier or changing the network verifier to custom would require wipe and reset of Signer."
"Bad input data. General verifier in the database is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received network westend specs could be accepted only if verified by the same general verifier. Current message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519."
"Bad input data. General verifier in the database is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received types information could be accepted only if verified by the same general verifier. Current message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519."
"Bad input data. Exactly same types information is already in the database."
"Bad input data. Received message could not be read."
"Bad input data. Input generated within unknown network and could not be processed. Add network with genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e and encryption sr25519."
"Bad input data. Input transaction is generated in network westend. Currently there are no metadata entries for it, and transaction could not be processed. Add network metadata."
"Bad input data. Exactly same network specs for network westend with encryption sr25519 are already in the database."
"Bad input data. Network kulupu current verifier is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received add_specs message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519, which is neither current network verifier not the general verifier. Changing the network verifier to another non-general one would require wipe and reset of Signer."
"Could not find current verifier for network with genesis hash 853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd."
"Could not find general verifier."
"Could not find types information."
"Could not find network specs for network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."
"Could not find network specs for westend."
"Could not find network specs key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e in address details with key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779."
"Could not find address details for address key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779."
"Could not find metadata entry for westend9120."
"Could not find danger status information."
"Could not find database temporary entry with information needed for accepting approved information."
"Could not find database temporary entry with information needed for signing approved transaction."
"Could not find history entry with order 135."
"Could not find network specs for westend with encryption ed25519 needed to decode historical transaction."
"Entry with order 280 contains no transaction-related events."
"Historical transaction was generated in network kulupu and processed. Currently there are no metadata entries for the network, and transaction could not be processed again. Add network metadata."
"Network with genesis hash 853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd is disabled. It could be enabled again only after complete wipe and re-installation of Signer."
"Error generating address. Network encryption sr25519 is different from seed object encryption ed25519."
"Error generating address. Address key collision for seed name Alice super secret seed"
"Error generating address. Bad secret string: invalid overall format."
"Error generating address. Bad secret string: invalid bip39 phrase."
"Error generating address. Bad secret string: invalid password."
"Error generating address. Bad secret string: invalid seed."
"Error generating address. Bad secret string: invalid seed length."
"Error generating address. Bad secret string: invalid path."
"Error generating address. Could not create random phrase. Mnemonic generator refuses to work with a valid excuse."
"Error generating address. Invalid derivation format."
"Error generating qr code. QR generator refuses to work with a valid excuse."
"Error parsing incoming transaction content. Expected mortal transaction due to prelude format. Found immortal transaction."
"Error parsing incoming transaction content. Expected immortal transaction due to prelude format. Found mortal transaction."
"Error parsing incoming transaction content. Genesis hash values from decoded extensions and from used network specs do not match."
"Error parsing incoming transaction content. Block hash for immortal transaction not matching genesis hash for the network."
"Error parsing incoming transaction content. Unable to decode extensions for V12/V13 metadata using standard extensions set."
"Error parsing incoming transaction content. Method number 2 not found in pallet test_Pallet."
"Error parsing incoming transaction content. Pallet with index 3 not found."
"Error parsing incoming transaction content. Method number 5 too high for pallet number 3. Only 4 indices available."
"Error parsing incoming transaction content. No calls found in pallet test_pallet_v14."
"Error parsing incoming transaction content. Referenced type could not be resolved in v14 metadata."
"Error parsing incoming transaction content. Argument type error."
"Error parsing incoming transaction content. Argument name error."
"Error parsing incoming transaction content. Expected primitive type. Found Option<u8>."
"Error parsing incoming transaction content. Expected compact. Not found it."
"Error parsing incoming transaction content. Data too short for expected content."
"Error parsing incoming transaction content. Unable to decode part of data as u32."
"Error parsing incoming transaction content. Encountered unexpected Option<_> variant."
"Error parsing incoming transaction content. IdentityField description error."
"Error parsing incoming transaction content. Unable to decode part of data as an array."
"Error parsing incoming transaction content. Unexpected type encountered for Balance"
"Error parsing incoming transaction content. Encountered unexpected enum variant."
"Error parsing incoming transaction content. Unexpected type inside compact."
"Error parsing incoming transaction content. Type claimed inside compact is not compactable."
"Error parsing incoming transaction content. No description found for type T::SomeUnknownType."
"Error parsing incoming transaction content. Declared type is not suitable BitStore type for BitVec."
"Error parsing incoming transaction content. Declared type is not suitable BitOrder type for BitVec."
"Error parsing incoming transaction content. Could not decode BitVec."
"Error parsing incoming transaction content. Could not decode Era."
"Error parsing incoming transaction content. After decoding the method some data remained unused."
"Error parsing incoming transaction content. After decoding the extensions some data remained unused."
"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Era information is missing."
"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Block hash information is missing."
"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Metadata spec version information is missing."
"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Era information is encountered mora than once."
"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Genesis hash is encountered more than once."
"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Block hash is encountered more than once."
"Error parsing incoming transaction content. Metadata signed extensions are not compatible with Signer (v14 metadata). Metadata spec version is encountered more than once."
"Error parsing incoming transaction content. Network spec version decoded from extensions (9122) differs from the version in metadata (9010)."
"Failed to decode extensions. Try updating metadata for westend network. Parsing with westend9010 metadata: Network spec version decoded from extensions (9122) differs from the version in metadata (9010). Parsing with westend9000 metadata: Network spec version decoded from extensions (9122) differs from the version in metadata (9000)."
"Error with secret string of existing address: invalid overall format."
"Error with secret string of existing address: invalid bip39 phrase."
"Error with secret string of existing address: invalid password."
"Error with secret string of existing address: invalid seed."
"Error with secret string of existing address: invalid seed length."
"Error with secret string of existing address: invalid path."
"Wrong password."
"#;
        assert!(print == print_expected, "\nReceived: {}", print);
    }
}

