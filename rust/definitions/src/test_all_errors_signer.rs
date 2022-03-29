use anyhow::anyhow;
use sled::{transaction::TransactionError, IVec};
use sp_core::crypto::SecretStringError;
use sp_runtime::MultiSigner;

use crate::crypto::Encryption;
use crate::error::{AddressGeneration, AddressGenerationCommon, MetadataError, TransferContent};
use crate::error_signer::{
    DatabaseSigner, EntryDecodingSigner, ErrorSigner, ExtraAddressGenerationSigner,
    GeneralVerifierForContent, InputSigner, InterfaceSigner, KeyDecodingSignerDb,
    KeyDecodingSignerInterface, MismatchSigner, NotFoundSigner, NotHexSigner, ParserDecodingError,
    ParserError, ParserMetadataError, Signer,
};
use crate::keyring::{AddressKey, MetaKey, NetworkSpecsKey, Order, VerifierKey};
use crate::network_specs::{ValidCurrentVerifier, Verifier, VerifierValue};

const PUBLIC: [u8; 32] = [
    142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201,
    18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,
];

fn verifier_sr25519() -> Verifier {
    Verifier(Some(verifier_value_sr25519()))
}
fn verifier_value_sr25519() -> VerifierValue {
    VerifierValue::Standard(MultiSigner::Sr25519(sp_core::sr25519::Public::from_raw(
        PUBLIC,
    )))
}
fn verifier_value_ed25519() -> VerifierValue {
    VerifierValue::Standard(MultiSigner::Ed25519(sp_core::ed25519::Public::from_raw(
        PUBLIC,
    )))
}

fn not_hex_string() -> String {
    String::from("0xabracadabra")
}

fn address_key_bad() -> AddressKey {
    AddressKey::from_hex("0350e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779")
        .unwrap()
}

fn address_key_good() -> AddressKey {
    AddressKey::from_hex("0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779")
        .unwrap()
}

fn network_specs_key_bad() -> NetworkSpecsKey {
    NetworkSpecsKey::from_hex(
        "0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
    )
    .unwrap()
}

fn network_specs_key_good() -> NetworkSpecsKey {
    NetworkSpecsKey::from_hex(
        "0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
    )
    .unwrap()
}

fn meta_key() -> MetaKey {
    MetaKey::from_parts("westend", 9122)
}

fn verifier_key() -> VerifierKey {
    VerifierKey::from_parts(
        &hex::decode("853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd").unwrap(),
    )
}

fn genesis_hash() -> [u8; 32] {
    hex::decode("e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e")
        .unwrap()
        .try_into()
        .unwrap()
}

fn db_internal_error_set() -> Vec<sled::Error> {
    vec![
        sled::Error::CollectionNotFound(IVec::from(vec![1])),
        sled::Error::Unsupported(String::from("Something Unsupported.")),
        sled::Error::ReportableBug(String::from("Please report me")),
        sled::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "oh no!")),
        sled::Error::Corruption { at: None, bt: () },
    ]
}

fn metadata_error_set() -> Vec<MetadataError> {
    vec![
        MetadataError::VersionIncompatible,
        MetadataError::NoSystemPallet,
        MetadataError::NoVersionInConstants,
        MetadataError::RuntimeVersionNotDecodeable,
        MetadataError::Base58PrefixNotDecodeable,
        MetadataError::Base58PrefixSpecsMismatch{specs: 42, meta: 104},
        MetadataError::NotMeta,
        MetadataError::UnableToDecode,
    ]
}

fn secret_string_error_set() -> Vec<SecretStringError> {
    vec![
        SecretStringError::InvalidFormat,
        SecretStringError::InvalidPhrase,
        SecretStringError::InvalidPassword,
        SecretStringError::InvalidSeed,
        SecretStringError::InvalidSeedLength,
        SecretStringError::InvalidPath,
    ]
}

fn content_set() -> Vec<GeneralVerifierForContent> {
    vec![
        GeneralVerifierForContent::Network {
            name: String::from("westend"),
        },
        GeneralVerifierForContent::Types,
    ]
}

fn all_ext_parsing_failed_set() -> Vec<(u32, ParserError)> {
    vec![
        (
            9010,
            ParserError::WrongNetworkVersion {
                as_decoded: String::from("9122"),
                in_metadata: 9010,
            },
        ),
        (
            9000,
            ParserError::WrongNetworkVersion {
                as_decoded: String::from("9122"),
                in_metadata: 9000,
            },
        ),
    ]
}

/// Collecting all [`NotHexSigner`] errors.
fn not_hex_signer() -> Vec<NotHexSigner> {
    vec![
        NotHexSigner::NetworkSpecsKey {
            input: not_hex_string(),
        },
        NotHexSigner::InputContent,
        NotHexSigner::AddressKey {
            input: not_hex_string(),
        },
    ]
}

/// Collecting all [`KeyDecodingSignerInterface`] errors.
fn key_decoding_signer_interface() -> Vec<KeyDecodingSignerInterface> {
    vec![
        KeyDecodingSignerInterface::AddressKey(address_key_bad()),
        KeyDecodingSignerInterface::NetworkSpecsKey(network_specs_key_bad()),
    ]
}

/// Collecting all [`InterfaceSigner`] errors.
fn interface_signer() -> Vec<InterfaceSigner> {
    // [`NotHexSigner`] errors
    let mut out = not_hex_signer()
        .into_iter()
        .map(InterfaceSigner::NotHex)
        .collect::<Vec<InterfaceSigner>>();

    // [`KeyDecodingSignerInterface`] errors.
    out.append(
        &mut key_decoding_signer_interface()
            .into_iter()
            .map(InterfaceSigner::KeyDecoding)
            .collect::<Vec<InterfaceSigner>>(),
    );

    // All remaining [`InterfaceSigner`] errors.
    out.append(&mut vec![
        InterfaceSigner::PublicKeyLength,
        InterfaceSigner::HistoryPageOutOfRange {
            page_number: 14,
            total_pages: 10,
        },
        InterfaceSigner::SeedNameNotMatching {
            address_key: address_key_good(),
            expected_seed_name: String::from("Alice"),
            real_seed_name: String::from("ALICE"),
        },
        InterfaceSigner::LostPwd,
        InterfaceSigner::VersionNotU32(String::from("a505")),
        InterfaceSigner::IncNotU32(String::from("a505")),
        InterfaceSigner::OrderNotU32(String::from("a505")),
        InterfaceSigner::FlagNotBool(String::from("FALSE")),
    ]);

    out
}

/// Collecting all [`KeyDecodingSignerDb`] errors.
fn key_decoding_signer_db() -> Vec<KeyDecodingSignerDb> {
    vec![
        KeyDecodingSignerDb::AddressKey(address_key_bad()),
        KeyDecodingSignerDb::EntryOrder(vec![100, 4, 85]),
        KeyDecodingSignerDb::MetaKey(meta_key()),
        KeyDecodingSignerDb::NetworkSpecsKey(network_specs_key_bad()),
        KeyDecodingSignerDb::NetworkSpecsKeyAddressDetails {
            address_key: address_key_good(),
            network_specs_key: network_specs_key_bad(),
        },
    ]
}

/// Collecting all [`EntryDecodingSigner`] errors.
fn entry_decoding_signer() -> Vec<EntryDecodingSigner> {
    vec![
        EntryDecodingSigner::AddressDetails(address_key_good()),
        EntryDecodingSigner::CurrentVerifier(verifier_key()),
        EntryDecodingSigner::DangerStatus,
        EntryDecodingSigner::Derivations,
        EntryDecodingSigner::GeneralVerifier,
        EntryDecodingSigner::HistoryEntry(Order::from_number(135)),
        EntryDecodingSigner::NetworkSpecs(network_specs_key_good()),
        EntryDecodingSigner::Sign,
        EntryDecodingSigner::Stub,
        EntryDecodingSigner::Types,
    ]
}

/// Collecting all [`MismatchSigner`] errors.
fn mismatch_signer() -> Vec<MismatchSigner> {
    vec![
        MismatchSigner::Metadata {
            name_key: String::from("westend"),
            version_key: 1922,
            name_inside: String::from("westend"),
            version_inside: 9122,
        },
        MismatchSigner::SpecsGenesisHash {
            key: network_specs_key_good(),
            genesis_hash: genesis_hash().to_vec(),
        },
        MismatchSigner::SpecsEncryption {
            key: network_specs_key_good(),
            encryption: Encryption::Ecdsa,
        },
        MismatchSigner::AddressDetailsEncryption {
            key: address_key_good(),
            encryption: Encryption::Ecdsa,
        },
        MismatchSigner::AddressDetailsSpecsEncryption {
            address_key: address_key_good(),
            network_specs_key: network_specs_key_bad(),
        },
    ]
}

/// Collecting all [`DatabaseSigner`] errors.
fn database_signer() -> Vec<DatabaseSigner> {
    // [`KeyDecodingSignerDb`] errors.
    let mut out = key_decoding_signer_db()
        .into_iter()
        .map(DatabaseSigner::KeyDecoding)
        .collect::<Vec<DatabaseSigner>>();

    // `sled::Error` internal database errors.
    out.append(
        &mut db_internal_error_set()
            .into_iter()
            .map(DatabaseSigner::Internal)
            .collect::<Vec<DatabaseSigner>>(),
    );

    // `sled::transaction::Transaction` database errors.
    out.append(
        &mut db_internal_error_set()
            .into_iter()
            .map(|a| DatabaseSigner::Transaction(TransactionError::Storage(a)))
            .collect::<Vec<DatabaseSigner>>(),
    );

    // Checksum mismatch error
    out.push(DatabaseSigner::ChecksumMismatch);

    // [`EntryDecodingSigner`] errors.
    out.append(
        &mut entry_decoding_signer()
            .into_iter()
            .map(DatabaseSigner::EntryDecoding)
            .collect::<Vec<DatabaseSigner>>(),
    );

    // [`MismatchSigner`] errors.
    out.append(
        &mut mismatch_signer()
            .into_iter()
            .map(DatabaseSigner::Mismatch)
            .collect::<Vec<DatabaseSigner>>(),
    );

    // `FaultyMetadata` database errors.
    out.append(
        &mut metadata_error_set()
            .into_iter()
            .map(|error| DatabaseSigner::FaultyMetadata {
                name: String::from("westend"),
                version: 9000,
                error,
            })
            .collect::<Vec<DatabaseSigner>>(),
    );

    // All remaining [`DatabaseSigner`] errors.
    out.append(&mut vec![
        DatabaseSigner::UnexpectedGenesisHash {
            verifier_key: VerifierKey::from_parts(&genesis_hash().to_vec()),
            network_specs_key: network_specs_key_good(),
        },
        DatabaseSigner::SpecsCollision {
            name: String::from("westend"),
            encryption: Encryption::Sr25519,
        },
        DatabaseSigner::DifferentNamesSameGenesisHash {
            name1: String::from("westend"),
            name2: String::from("WeStEnD"),
            genesis_hash: genesis_hash().to_vec(),
        },
        DatabaseSigner::CustomVerifierIsGeneral(verifier_key()),
        DatabaseSigner::TwoRootKeys {
            seed_name: String::from("Alice"),
            encryption: Encryption::Sr25519,
        },
        DatabaseSigner::DifferentBase58Specs {
            genesis_hash: genesis_hash(),
            base58_1: 42,
            base58_2: 104,
        },
    ]);

    out
}

/// [`TransferContent`] errors.
fn transfer_content() -> Vec<TransferContent> {
    vec![
        TransferContent::AddSpecs,
        TransferContent::LoadMeta,
        TransferContent::LoadTypes,
    ]
}

/// Collecting all [`InputSigner`] errors.
fn input_signer() -> Vec<InputSigner> {
    // [`TransferContent`] errors.
    let mut out = transfer_content()
        .into_iter()
        .map(InputSigner::TransferContent)
        .collect::<Vec<InputSigner>>();

    // `TransferDerivations` error.
    out.push(InputSigner::TransferDerivations);

    // Faulty metadata input content errors.
    out.append(
        &mut metadata_error_set()
            .into_iter()
            .map(InputSigner::FaultyMetadata)
            .collect::<Vec<InputSigner>>(),
    );

    // More [`InputSigner`] errors.
    out.append(&mut vec![
        InputSigner::TooShort,
        InputSigner::NotSubstrate(String::from("35")),
        InputSigner::PayloadNotSupported(String::from("0f")),
        InputSigner::SameNameVersionDifferentMeta {
            name: String::from("kusama"),
            version: 9110,
        },
        InputSigner::MetadataKnown {
            name: String::from("westend"),
            version: 9122,
        },
        InputSigner::ImportantSpecsChanged(network_specs_key_good()),
        InputSigner::DifferentBase58 {
            genesis_hash: genesis_hash(),
            base58_database: 42,
            base58_input: 104,
        },
        InputSigner::EncryptionNotSupported(String::from("03")),
        InputSigner::BadSignature,
        InputSigner::LoadMetaUnknownNetwork {
            name: String::from("kulupu"),
        },
        InputSigner::LoadMetaNoSpecs {
            name: String::from("westend"),
            valid_current_verifier: ValidCurrentVerifier::General,
            general_verifier: verifier_sr25519(),
        },
        InputSigner::NeedVerifier {
            name: String::from("kulupu"),
            verifier_value: verifier_value_ed25519(),
        },
    ]);

    // `NeedGeneralVerifier` errors.
    out.append(
        &mut content_set()
            .into_iter()
            .map(|content| InputSigner::NeedGeneralVerifier {
                content,
                verifier_value: verifier_value_sr25519(),
            })
            .collect::<Vec<InputSigner>>(),
    );

    // More [`InputSigner`] errors.
    out.append(&mut vec![
        InputSigner::LoadMetaSetVerifier {
            name: String::from("kulupu"),
            new_verifier_value: verifier_value_ed25519(),
        },
        InputSigner::LoadMetaVerifierChanged {
            name: String::from("kulupu"),
            old_verifier_value: verifier_value_sr25519(),
            new_verifier_value: verifier_value_ed25519(),
        },
        InputSigner::LoadMetaSetGeneralVerifier {
            name: String::from("westend"),
            new_general_verifier_value: verifier_value_sr25519(),
        },
        InputSigner::LoadMetaGeneralVerifierChanged {
            name: String::from("westend"),
            old_general_verifier_value: verifier_value_sr25519(),
            new_general_verifier_value: verifier_value_ed25519(),
        },
    ]);

    // `GeneralVerifierChanged` errors.
    out.append(
        &mut content_set()
            .into_iter()
            .map(|content| InputSigner::GeneralVerifierChanged {
                content,
                old_general_verifier_value: verifier_value_sr25519(),
                new_general_verifier_value: verifier_value_ed25519(),
            })
            .collect::<Vec<InputSigner>>(),
    );

    // More [`InputSigner`] errors.
    out.append(&mut vec![
        InputSigner::TypesKnown,
        InputSigner::MessageNotReadable,
        InputSigner::UnknownNetwork {
            genesis_hash: genesis_hash().to_vec(),
            encryption: Encryption::Sr25519,
        },
        InputSigner::NoMetadata {
            name: String::from("westend"),
        },
        InputSigner::SpecsKnown {
            name: String::from("westend"),
            encryption: Encryption::Sr25519,
        },
        InputSigner::AddSpecsVerifierChanged {
            name: String::from("kulupu"),
            old_verifier_value: verifier_value_sr25519(),
            new_verifier_value: verifier_value_ed25519(),
        },
        InputSigner::InvalidDerivation(String::from("//")),
        InputSigner::OnlyNoPwdDerivations,
        InputSigner::SeedNameExists(String::from("Alice")),
    ]);

    out
}

/// Collecting all [`NotFoundSigner`] errors.
fn not_found_signer() -> Vec<NotFoundSigner> {
    vec![
        NotFoundSigner::CurrentVerifier(verifier_key()),
        NotFoundSigner::GeneralVerifier,
        NotFoundSigner::Types,
        NotFoundSigner::NetworkSpecs(network_specs_key_bad()),
        NotFoundSigner::NetworkSpecsForName(String::from("westend")),
        NotFoundSigner::NetworkSpecsKeyForAddress {
            network_specs_key: network_specs_key_good(),
            address_key: address_key_good(),
        },
        NotFoundSigner::AddressDetails(address_key_good()),
        NotFoundSigner::Metadata {
            name: String::from("westend"),
            version: 9120,
        },
        NotFoundSigner::DangerStatus,
        NotFoundSigner::Stub,
        NotFoundSigner::Sign,
        NotFoundSigner::Derivations,
        NotFoundSigner::HistoryEntry(Order::from_number(135)),
        NotFoundSigner::HistoryNetworkSpecs {
            name: String::from("westend"),
            encryption: Encryption::Ed25519,
        },
        NotFoundSigner::HistoricalMetadata {
            name: String::from("kulupu"),
        },
        NotFoundSigner::NetworkForDerivationsImport {
            genesis_hash: genesis_hash(),
            encryption: Encryption::Sr25519,
        },
    ]
}

/// Collecting all [`AddressGenerationCommon`] errors.
fn address_generation_common() -> Vec<AddressGenerationCommon> {
    // `KeyCollision` error.
    let mut out = vec![AddressGenerationCommon::KeyCollision {
        seed_name: String::from("Alice"),
    }];

    // `AddressGenerationCommon` errors.
    out.append(
        &mut secret_string_error_set()
            .into_iter()
            .map(AddressGenerationCommon::SecretString)
            .collect::<Vec<AddressGenerationCommon>>(),
    );

    out
}

/// Collecting all [`ExtraAddressGenerationSigner`] errors.
fn extra_address_generation_signer() -> Vec<ExtraAddressGenerationSigner> {
    vec![
        ExtraAddressGenerationSigner::RandomPhraseGeneration(anyhow!(
            "Seed phrase has invalid length."
        )),
        ExtraAddressGenerationSigner::InvalidDerivation,
    ]
}

/// Collecting all [`AddressGeneration`] errors.
fn address_generation() -> Vec<AddressGeneration<Signer>> {
    // `AddressGenerationCommon` errors.
    let mut out = address_generation_common()
        .into_iter()
        .map(AddressGeneration::Common)
        .collect::<Vec<AddressGeneration<Signer>>>();

    // `GeneralVerifierChanged` errors.
    out.append(
        &mut extra_address_generation_signer()
            .into_iter()
            .map(AddressGeneration::Extra)
            .collect::<Vec<AddressGeneration<Signer>>>(),
    );

    out
}

/// Collecting all [`ParserDecodingError`] errors.
fn parser_decoding_error() -> Vec<ParserDecodingError> {
    vec![
        ParserDecodingError::UnexpectedImmortality,
        ParserDecodingError::UnexpectedMortality,
        ParserDecodingError::GenesisHashMismatch,
        ParserDecodingError::ImmortalHashMismatch,
        ParserDecodingError::ExtensionsOlder,
        ParserDecodingError::MethodNotFound {
            method_index: 2,
            pallet_name: String::from("test_Pallet"),
        },
        ParserDecodingError::PalletNotFound(3),
        ParserDecodingError::NoCallsInPallet(String::from("test_pallet_v14")),
        ParserDecodingError::V14TypeNotResolved,
        ParserDecodingError::ArgumentTypeError,
        ParserDecodingError::ArgumentNameError,
        ParserDecodingError::NoCompact,
        ParserDecodingError::DataTooShort,
        ParserDecodingError::PrimitiveFailure(String::from("u32")),
        ParserDecodingError::UnexpectedOptionVariant,
        ParserDecodingError::IdFields,
        ParserDecodingError::BalanceNotDescribed,
        ParserDecodingError::UnexpectedEnumVariant,
        ParserDecodingError::UnexpectedCompactInsides,
        ParserDecodingError::UnknownType(String::from("SomeUnknownType")),
        ParserDecodingError::NotBitStoreType,
        ParserDecodingError::NotBitOrderType,
        ParserDecodingError::BitVecFailure,
        ParserDecodingError::Era,
        ParserDecodingError::SomeDataNotUsedMethod,
        ParserDecodingError::SomeDataNotUsedExtensions,
    ]
}

/// Collecting all [`ParserMetadataError`] errors.
fn parser_metadata_error() -> Vec<ParserMetadataError> {
    vec![
        ParserMetadataError::NoEra,
        ParserMetadataError::NoBlockHash,
        ParserMetadataError::NoVersionExt,
        ParserMetadataError::EraTwice,
        ParserMetadataError::GenesisHashTwice,
        ParserMetadataError::BlockHashTwice,
        ParserMetadataError::SpecVersionTwice,
    ]
}

/// Collecting all [`ParserError`] errors.
fn parser_error() -> Vec<ParserError> {
    // `SeparateMethodExtensions` error.
    let mut out = vec![ParserError::SeparateMethodExtensions];

    // `ParserDecodingError` errors.
    out.append(
        &mut parser_decoding_error()
            .into_iter()
            .map(ParserError::Decoding)
            .collect::<Vec<ParserError>>(),
    );

    // `ParserMetadataError` errors.
    out.append(
        &mut parser_metadata_error()
            .into_iter()
            .map(ParserError::FundamentallyBadV14Metadata)
            .collect::<Vec<ParserError>>(),
    );

    out.push(ParserError::WrongNetworkVersion {
        as_decoded: String::from("9122"),
        in_metadata: 9010,
    });

    out
}

/// Collecting all [`ErrorSigner`] errors.
pub fn error_signer() -> Vec<ErrorSigner> {
    // [`InterfaceSigner`] errors.
    let mut out = interface_signer()
        .into_iter()
        .map(ErrorSigner::Interface)
        .collect::<Vec<ErrorSigner>>();

    // [`DatabaseSigner`] errors.
    out.append(
        &mut database_signer()
            .into_iter()
            .map(ErrorSigner::Database)
            .collect::<Vec<ErrorSigner>>(),
    );

    // [`InputSigner`] errors.
    out.append(
        &mut input_signer()
            .into_iter()
            .map(ErrorSigner::Input)
            .collect::<Vec<ErrorSigner>>(),
    );

    // [`NotFoundSigner`] errors.
    out.append(
        &mut not_found_signer()
            .into_iter()
            .map(ErrorSigner::NotFound)
            .collect::<Vec<ErrorSigner>>(),
    );

    // `DeadVerifier` error.
    out.push(ErrorSigner::DeadVerifier(verifier_key()));

    // `AddressGeneration` errors.
    out.append(
        &mut address_generation()
            .into_iter()
            .map(ErrorSigner::AddressGeneration)
            .collect::<Vec<ErrorSigner>>(),
    );

    // `Qr` error.
    out.push(ErrorSigner::Qr(String::from("Qr generation failed.")));

    // `ParserError` errors.
    out.append(
        &mut parser_error()
            .into_iter()
            .map(ErrorSigner::Parser)
            .collect::<Vec<ErrorSigner>>(),
    );

    // `AllExtensionsParsingFailed` error.
    out.push(ErrorSigner::AllExtensionsParsingFailed {
        network_name: String::from("westend"),
        errors: all_ext_parsing_failed_set(),
    });

    // `AddressUse` errors.
    out.append(
        &mut secret_string_error_set()
            .into_iter()
            .map(ErrorSigner::AddressUse)
            .collect::<Vec<ErrorSigner>>(),
    );

    // `WrongPassword` error.
    out.push(ErrorSigner::WrongPassword);

    // `WrongPasswordNewChecksum(_)` error.
    out.push(ErrorSigner::WrongPasswordNewChecksum(32167));

    // `NoNetworksAvailable` error.
    out.push(ErrorSigner::NoNetworksAvailable);

    out
}

#[cfg(feature = "test")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ErrorSource;
    use crate::error_signer::Signer;

    #[test]
    fn metadata_error_check() {
        assert_eq!(MetadataError::VARIANT_COUNT, metadata_error_set().len());
    }
    
    #[test]
    fn transfer_content_check() {
        assert_eq!(TransferContent::VARIANT_COUNT, transfer_content().len());
    }
    
    #[test]
    fn not_hex_signer_check() {
        assert_eq!(NotHexSigner::VARIANT_COUNT, not_hex_signer().len());
    }
    
    #[test]
    fn key_decoding_signer_interface_check() {
        assert_eq!(KeyDecodingSignerInterface::VARIANT_COUNT, key_decoding_signer_interface().len());
    }
    
    /// count all `InterfaceSigner` variants including the nested ones
    fn interface_signer_count() -> usize {
        InterfaceSigner::VARIANT_COUNT
            - 1 + not_hex_signer().len() // for nested variants in `InterfaceSigner::NotHex(_)`
            - 1 + key_decoding_signer_interface().len() // for nested variants `InterfaceSigner::KeyDecoding(_)`
    }
    
    #[test]
    fn interface_signer_check() {
        assert_eq!(
            interface_signer_count(), 
            interface_signer().len(),
        );
    }
    
    #[test]
    fn key_decoding_signer_db_check() {
        assert_eq!(KeyDecodingSignerDb::VARIANT_COUNT, key_decoding_signer_db().len());
    }
    
    #[test]
    fn entry_decoding_signer_check() {
        assert_eq!(EntryDecodingSigner::VARIANT_COUNT, entry_decoding_signer().len());
    }
    
    #[test]
    fn mismatch_signer_check() {
        assert_eq!(MismatchSigner::VARIANT_COUNT, mismatch_signer().len());
    }
    
    /// count all `DatabaseSigner` variants including the nested ones
    fn database_signer_count() -> usize {
        DatabaseSigner::VARIANT_COUNT
            - 1 + key_decoding_signer_db().len() // for nested variants in `DatabaseSigner::KeyDecoding(_)`
            - 1 + db_internal_error_set().len() // for nested variants `DatabaseSigner::Internal(_)`
            - 1 + db_internal_error_set().len() // for nested variants `DatabaseSigner::Transaction(_)` except user-provided error, since there is none
            - 1 + entry_decoding_signer().len() // for nested variants in `DatabaseSigner::EntryDecoding(_)`
            - 1 + mismatch_signer().len() // for nested variants in `DatabaseSigner::Mismatch(_)`
            - 1 + metadata_error_set().len() // for nested variants in `DatabaseSigner::FaultyMetadata(_)`
    }
    
    /// check that no `DatabaseSigner` entries are missed
    #[test]
    fn database_signer_check() {
        assert_eq!(
            database_signer_count(), 
            database_signer().len(),
        );
    }
    
    /// check that no `GeneralVerifierForContent` entries are missed
    #[test]
    fn content_set_check() {
        assert_eq!(GeneralVerifierForContent::VARIANT_COUNT, content_set().len());
    }
    
    /// count all `InputSigner` variants including the nested ones
    fn input_signer_count() -> usize {
        InputSigner::VARIANT_COUNT
            - 1 + transfer_content().len() // for nested variants in `InputSigner::TransferContent(_)`
            - 1 + metadata_error_set().len() // for nested variants in `InputSigner::FaultyMetadata(_)`
            - 1 + content_set().len() // for nested variants in `InputSigner::NeedGeneralVerifier(_)`
            - 1 + content_set().len() // for nested variants in `InputSigner::GeneralVerifierChanged(_)`
    }
    
    /// check that no `InputSigner` entries are missed
    #[test]
    fn input_signer_check() {
        assert_eq!(
            input_signer_count(), 
            input_signer().len(),
        );
    }
    
    /// count all `NotFoundSigner` variants
    fn not_found_signer_count() -> usize {
        NotFoundSigner::VARIANT_COUNT
    }
    
    /// check that no `NotFoundSigner` entries are missed
    #[test]
    fn not_found_signer_check() {
        assert_eq!(
            not_found_signer_count(), 
            not_found_signer().len(),
        );
    }
    
    
    
    #[test]
    fn print_signer_errors_nicely() {
        let mut print = String::from("\n");
        let signer_errors = error_signer();
        assert!(
            signer_errors.len() == 154,
            "Different error set length: {}",
            signer_errors.len()
        );
        for e in signer_errors.iter() {
            print.push_str(&format!("\"{}\"", <Signer>::show(e)));
            print.push_str("\n");
        }
        let print_expected = r#""#;
        assert!(print == print_expected, "\nReceived: {}", print);
    }
}
