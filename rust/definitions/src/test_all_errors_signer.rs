//! Generating all [`ErrorSigner`] entries for tests
//!
//! This is used to proof-read the texts of the all possible errors, with some
//! mock content, if necessary, and to generate a complete set of errors that
//! is printed as cards in `transaction_parsing` and must be json-compatible.
//!
//! Crate `variant_count` is used to count the variants in each of the enums
//! and to make sure no entries are left missing in case of [`ErrorSigner`]
//! updating.

use std::str::FromStr;

#[cfg(test)]
use std::fmt::Write as _;

use anyhow::anyhow;
use sled::{transaction::TransactionError, IVec};
use sp_core::crypto::SecretStringError;
use sp_core::H256;
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
use crate::users::AddressDetails;

const PUBLIC: [u8; 32] = [
    142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201,
    18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,
];

/// `Verifier` mock value.
fn verifier_sr25519() -> Verifier {
    Verifier {
        v: Some(verifier_value_sr25519()),
    }
}

/// `VerifierValue` mock value.
fn verifier_value_sr25519() -> VerifierValue {
    VerifierValue::Standard {
        m: MultiSigner::Sr25519(sp_core::sr25519::Public::from_raw(PUBLIC)),
    }
}

/// Another `VerifierValue` mock value.
fn verifier_value_ed25519() -> VerifierValue {
    VerifierValue::Standard {
        m: MultiSigner::Ed25519(sp_core::ed25519::Public::from_raw(PUBLIC)),
    }
}

/// Mock non-hexadecimal `String`.
fn not_hex_string() -> String {
    String::from("0xabracadabra")
}

/// `AddressKey` mock value.
fn address_key_bad() -> AddressKey {
    AddressKey::from_hex("0350e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779")
        .unwrap()
}

/// Another `AddressKey` mock value.
fn address_key_good() -> AddressKey {
    AddressKey::from_hex("0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779")
        .unwrap()
}

/// `NetworkSpecsKey` mock value.
fn network_specs_key_bad() -> NetworkSpecsKey {
    NetworkSpecsKey::from_hex(
        "0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
    )
    .unwrap()
}

/// Another `NetworkSpecsKey` mock value.
fn network_specs_key_good() -> NetworkSpecsKey {
    NetworkSpecsKey::from_hex(
        "0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
    )
    .unwrap()
}

/// `MetaKey` mock value.
fn meta_key() -> MetaKey {
    MetaKey::from_parts("westend", 9122)
}

/// `VerifierKey` mock value.
fn verifier_key() -> VerifierKey {
    VerifierKey::from_parts(
        H256::from_str("853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd").unwrap(),
    )
}

/// `[u8; 32]` genesis hash mock value.
fn genesis_hash() -> H256 {
    H256::from_str("e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap()
}

/// Possible `sled::Error` errors (https://docs.rs/sled/0.34.6/sled/enum.Error.html).
fn db_internal_error_set() -> Vec<sled::Error> {
    vec![
        sled::Error::CollectionNotFound(IVec::from(vec![1])),
        sled::Error::Unsupported(String::from("Something Unsupported.")),
        sled::Error::ReportableBug(String::from("Please report me")),
        sled::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "oh no!")),
        sled::Error::Corruption { at: None, bt: () },
    ]
}

/// All possible [`MetadataError`] values.
fn metadata_error_set() -> Vec<MetadataError> {
    vec![
        MetadataError::VersionIncompatible,
        MetadataError::NoSystemPallet,
        MetadataError::NoVersionInConstants,
        MetadataError::RuntimeVersionNotDecodeable,
        MetadataError::Base58PrefixNotDecodeable,
        MetadataError::Base58PrefixSpecsMismatch {
            specs: 42,
            meta: 104,
        },
        MetadataError::NotMeta,
        MetadataError::UnableToDecode,
    ]
}

/// Possible `SecretStringError` errors (https://docs.rs/sp-core/6.0.0/sp_core/crypto/enum.SecretStringError.html).
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

/// All possible [`GeneralVerifierForContent`] values.
fn content_set() -> Vec<GeneralVerifierForContent> {
    vec![
        GeneralVerifierForContent::Network {
            name: String::from("westend"),
        },
        GeneralVerifierForContent::Types,
    ]
}

/// Associated data for `ErrorSigner::AllExtensionsParsingFailed(_)` error.
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
            genesis_hash: genesis_hash(),
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
            name: String::from("westend"),
            genesis_hash: genesis_hash(),
        },
        DatabaseSigner::SpecsCollision {
            name: String::from("westend"),
            encryption: Encryption::Sr25519,
        },
        DatabaseSigner::DifferentNamesSameGenesisHash {
            name1: String::from("westend"),
            name2: String::from("WeStEnD"),
            genesis_hash: genesis_hash(),
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
        InputSigner::AddSpecsDifferentBase58 {
            genesis_hash: genesis_hash(),
            name: String::from("westend"),
            base58_database: 42,
            base58_input: 104,
        },
        InputSigner::AddSpecsDifferentName {
            genesis_hash: genesis_hash(),
            name_database: String::from("westend"),
            name_input: String::from("WeStEnD"),
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
        InputSigner::LoadMetaWrongGenesisHash {
            name_metadata: String::from("acala"),
            name_specs: String::from("westend"),
            genesis_hash: genesis_hash(),
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
        InputSigner::MessageNoWrapper,
        InputSigner::MessageNotValidUtf8,
        InputSigner::UnknownNetwork {
            genesis_hash: genesis_hash(),
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
    let mut out = vec![
        AddressGenerationCommon::KeyCollision {
            seed_name: String::from("Alice"),
        },
        AddressGenerationCommon::KeyCollisionBatch {
            seed_name_existing: String::from("Alice"),
            seed_name_new: String::from("Alice"),
            cropped_path_existing: String::from("//1"),
            cropped_path_new: String::from("//01"),
            in_this_network: true,
        },
    ];

    // `AddressGenerationCommon` errors.
    out.append(
        &mut secret_string_error_set()
            .into_iter()
            .map(AddressGenerationCommon::SecretString)
            .collect::<Vec<AddressGenerationCommon>>(),
    );

    // `DerivationExists` error.
    out.push(AddressGenerationCommon::DerivationExists(
        MultiSigner::Sr25519(sp_core::sr25519::Public::from_raw(PUBLIC)),
        AddressDetails {
            seed_name: String::from("Alice"),
            path: String::from("//Alice"),
            has_pwd: false,
            network_id: vec![network_specs_key_good()],
            encryption: Encryption::Sr25519,
        },
        network_specs_key_good(),
    ));

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

    // `TimeFormat` error.
    out.push(ErrorSigner::TimeFormat(
        time::error::Format::InvalidComponent("distance"),
    ));

    // `NoSeeds` error.
    out.push(ErrorSigner::NoKnownSeeds);

    // `SeedPhraseEmpty` error.
    out.push(ErrorSigner::SeedPhraseEmpty);

    // `SeedNameEmpty` error.
    out.push(ErrorSigner::SeedNameEmpty);

    out
}

#[cfg(feature = "test")]
#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ErrorSource;
    use crate::error_signer::Signer;

    /// check that no `MetadataError` entries are missed
    #[test]
    fn metadata_error_check() {
        assert_eq!(MetadataError::VARIANT_COUNT, metadata_error_set().len());
    }

    /// check that no `TransferContent` entries are missed
    #[test]
    fn transfer_content_check() {
        assert_eq!(TransferContent::VARIANT_COUNT, transfer_content().len());
    }

    /// check that no `NotHexSigner` entries are missed
    #[test]
    fn not_hex_signer_check() {
        assert_eq!(NotHexSigner::VARIANT_COUNT, not_hex_signer().len());
    }

    /// check that no `KeyDecodingSignerInterface` entries are missed
    #[test]
    fn key_decoding_signer_interface_check() {
        assert_eq!(
            KeyDecodingSignerInterface::VARIANT_COUNT,
            key_decoding_signer_interface().len()
        );
    }

    /// count all `InterfaceSigner` variants including the nested ones
    fn interface_signer_count() -> usize {
        InterfaceSigner::VARIANT_COUNT
            - 1 + not_hex_signer().len() // for nested variants in `InterfaceSigner::NotHex(_)`
            - 1 + key_decoding_signer_interface().len() // for nested variants `InterfaceSigner::KeyDecoding(_)`
    }

    /// check that no `InterfaceSigner` entries are missed
    #[test]
    fn interface_signer_check() {
        assert_eq!(interface_signer_count(), interface_signer().len(),);
    }

    /// check that no `KeyDecodingSignerDb` entries are missed
    #[test]
    fn key_decoding_signer_db_check() {
        assert_eq!(
            KeyDecodingSignerDb::VARIANT_COUNT,
            key_decoding_signer_db().len()
        );
    }

    /// check that no `EntryDecodingSigner` entries are missed
    #[test]
    fn entry_decoding_signer_check() {
        assert_eq!(
            EntryDecodingSigner::VARIANT_COUNT,
            entry_decoding_signer().len()
        );
    }

    /// check that no `MismatchSigner` entries are missed
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
        assert_eq!(database_signer_count(), database_signer().len(),);
    }

    /// check that no `GeneralVerifierForContent` entries are missed
    #[test]
    fn content_set_check() {
        assert_eq!(
            GeneralVerifierForContent::VARIANT_COUNT,
            content_set().len()
        );
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
        assert_eq!(input_signer_count(), input_signer().len(),);
    }

    /// check that no `NotFoundSigner` entries are missed
    #[test]
    fn not_found_signer_check() {
        assert_eq!(NotFoundSigner::VARIANT_COUNT, not_found_signer().len(),);
    }

    /// count all `AddressGenerationCommon` variants
    fn address_generation_common_count() -> usize {
        AddressGenerationCommon::VARIANT_COUNT - 1 + secret_string_error_set().len() // for nested variants in `AddressGenerationCommon::SecretString(_)`
    }

    /// check that no `AddressGenerationCommon` entries are missed
    #[test]
    fn address_generation_common_check() {
        assert_eq!(
            address_generation_common_count(),
            address_generation_common().len(),
        );
    }

    /// check that no `ExtraAddressGenerationSigner` entries are missed
    #[test]
    fn extra_address_generation_signer_check() {
        assert_eq!(
            ExtraAddressGenerationSigner::VARIANT_COUNT,
            extra_address_generation_signer().len(),
        );
    }

    /// count all `AddressGeneration` variants
    fn address_generation_count() -> usize {
        AddressGeneration::<Signer>::VARIANT_COUNT
            - 1 + address_generation_common_count() // for nested variants in `AddressGeneration::Common(_)`
            - 1 + extra_address_generation_signer().len() // for nested variants in `AddressGeneration::Extra(_)`
    }

    /// check that no `AddressGeneration` entries are missed
    #[test]
    fn address_generation_check() {
        assert_eq!(address_generation_count(), address_generation().len(),);
    }

    /// check that no `ParserDecodingError` entries are missed
    #[test]
    fn parser_decoding_error_check() {
        assert_eq!(
            ParserDecodingError::VARIANT_COUNT,
            parser_decoding_error().len(),
        );
    }

    /// check that no `ParserMetadataError` entries are missed
    #[test]
    fn parser_metadata_error_check() {
        assert_eq!(
            ParserMetadataError::VARIANT_COUNT,
            parser_metadata_error().len(),
        );
    }

    /// count all `ParserError` variants
    fn parser_error_count() -> usize {
        ParserError::VARIANT_COUNT
            - 1 + parser_decoding_error().len() // for nested variants in `ParserError::Decoding(_)`
            - 1 + parser_metadata_error().len() // for nested variants in `ParserError::FundamentallyBadV14Metadata(_)`
    }

    /// check that no `ParserError` entries are missed
    #[test]
    fn parser_error_check() {
        assert_eq!(parser_error_count(), parser_error().len(),);
    }

    /// count all `ErrorSigner` variants
    fn error_signer_count() -> usize {
        ErrorSigner::VARIANT_COUNT
            - 1 + interface_signer_count() // for nested variants in `ErrorSigner::Interface(_)`
            - 1 + database_signer_count() // for nested variants in `ErrorSigner::Database(_)`
            - 1 + input_signer_count() // for nested variants in `ErrorSigner::Input(_)`
            - 1 + not_found_signer().len() // for nested variants in `ErrorSigner::NotFound(_)`
            - 1 + address_generation_count() // for nested variants in `ErrorSigner::AddressGeneration(_)`
            - 1 + parser_error_count() // for nested variants in `ErrorSigner::Parser(_)`
            - 1 + secret_string_error_set().len() // for nested variants in `ErrorSigner::AddressUse(_)`
    }

    /// check that no `ErrorSigner` entries are missed, this covers all signer errors
    #[test]
    fn error_signer_check() {
        assert_eq!(error_signer_count(), error_signer().len(),);
    }

    #[test]
    fn print_signer_errors_nicely() {
        let mut print = String::from("\n");
        for e in error_signer().iter() {
            let _ = write!(print, "\"{}\"", <Signer>::show(e));
            print.push('\n');
        }
        let print_expected = r#"
"Error on the interface. Network specs key 0xabracadabra is not in hexadecimal format."
"Error on the interface. Input content is not in hexadecimal format."
"Error on the interface. Address key 0xabracadabra is not in hexadecimal format."
"Error on the interface. Unable to parse address key 0350e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779 passed through the interface."
"Error on the interface. Unable to parse network specs key 0350e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e passed through the interface."
"Error on the interface. Public key length does not match the encryption."
"Error on the interface. Requested history page 14 does not exist. Total number of pages 10."
"Error on the interface. Expected seed name Alice for address key 0150e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779. Address details in database have ALICE name."
"Error on the interface. Derivation had password, then lost it."
"Error on the interface. Version a505 could not be converted into u32."
"Error on the interface. Increment a505 could not be converted into u32."
"Error on the interface. Order a505 could not be converted into u32"
"Error on the interface. Flag FALSE could not be converted into bool."
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
"Database error. Unable to decode temporary entry with information needed to import derivations."
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
"Database error. Bad metadata for westend9000. Base58 prefix is found in system pallet constants, but could not be decoded."
"Database error. Bad metadata for westend9000. Base58 prefix 104 from system pallet constants does not match the base58 prefix from network specs 42."
"Database error. Bad metadata for westend9000. Metadata vector does not start with 0x6d657461."
"Database error. Bad metadata for westend9000. Runtime metadata could not be decoded."
"Database error. Network westend with genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e has some network specs entries, while there is no verifier entry."
"Database error. More than one entry for network specs with name westend and encryption sr25519."
"Database error. Different network names (westend, WeStEnD) in database for same genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e."
"Database error. Network with genesis hash 853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd verifier is set as a custom one. This custom verifier coinsides the database general verifier and not None. This should not have happened and likely indicates database corruption."
"Database error. More than one seed key (i.e. with empty path and without password) found for seed name Alice and encryption sr25519."
"Database error. More than one base58 prefix in network specs database entries for network with genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e: 42 and 104."
"Bad input data. Payload could not be decoded as `add_specs`."
"Bad input data. Payload could not be decoded as `load_meta`."
"Bad input data. Payload could not be decoded as `load_types`."
"Bad input data. Payload could not be decoded as derivations transfer."
"Bad input data. Received metadata is unsuitable. Runtime metadata version is incompatible. Currently supported are v12, v13, and v14."
"Bad input data. Received metadata is unsuitable. No system pallet in runtime metadata."
"Bad input data. Received metadata is unsuitable. No runtime version in system pallet constants."
"Bad input data. Received metadata is unsuitable. Runtime version from system pallet constants could not be decoded."
"Bad input data. Received metadata is unsuitable. Base58 prefix is found in system pallet constants, but could not be decoded."
"Bad input data. Received metadata is unsuitable. Base58 prefix 104 from system pallet constants does not match the base58 prefix from network specs 42."
"Bad input data. Received metadata is unsuitable. Metadata vector does not start with 0x6d657461."
"Bad input data. Received metadata is unsuitable. Runtime metadata could not be decoded."
"Bad input data. Input is too short."
"Bad input data. Only Substrate transactions are supported. Transaction is expected to start with 0x53, this one starts with 0x35."
"Bad input data. Payload type with code 0x0f is not supported."
"Bad input data. Metadata for kusama9110 is already in the database and is different from the one in received payload."
"Bad input data. Metadata for westend9122 is already in the database."
"Bad input data. Similar network specs are already stored in the database under key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e. Network specs in received payload have different unchangeable values (base58 prefix, decimals, encryption, network name, unit)."
"Bad input data. Network westend with genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e already has entries in the database with base58 prefix 42. Received network specs have same genesis hash and different base58 prefix 104."
"Bad input data. Network with genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e has name westend in the database. Received network specs have same genesis hash and name WeStEnD."
"Bad input data. Payload with encryption 0x03 is not supported."
"Bad input data. Received payload has bad signature."
"Bad input data. Network kulupu is not in the database. Add network specs before loading the metadata."
"Bad input data. Network westend was previously known to the database with verifier public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519 (general verifier). However, no network specs are in the database at the moment. Add network specs before loading the metadata."
"Bad input data. Update payload contains metadata for network acala. Genesis hash in payload (e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e) matches database genesis hash for another network, westend."
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
"Bad input data. Received message has no `<Bytes></Bytes>` wrapper."
"Bad input data. Received message could not be represented as valid utf8 sequence."
"Bad input data. Input generated within unknown network and could not be processed. Add network with genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e and encryption sr25519."
"Bad input data. Input transaction is generated in network westend. Currently there are no metadata entries for it, and transaction could not be processed. Add network metadata."
"Bad input data. Exactly same network specs for network westend with encryption sr25519 are already in the database."
"Bad input data. Network kulupu current verifier is public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: sr25519. Received add_specs message is verified by public key: 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48, encryption: ed25519, which is neither current network verifier not the general verifier. Changing the network verifier to another non-general one would require wipe and reset of Signer."
"Bad input data. Derivation // has invalid format."
"Bad input data. Seed name Alice already exists."
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
"Could not find database temporary entry with information needed for importing derivations set."
"Could not find history entry with order 135."
"Could not find network specs for westend with encryption ed25519 needed to decode historical transaction."
"Historical transaction was generated in network kulupu and processed. Currently there are no metadata entries for the network, and transaction could not be processed again. Add network metadata."
"Unable to import derivations for network with genesis hash e143f23803ca50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e and encryption sr25519. Network is unknown. Please add corresponding network specs."
"Network with genesis hash 853faffbfc6713c1f899bf16547fcfbf733ae8361b8ca0129699d01d4f2181fd is disabled. It could be enabled again only after complete wipe and re-installation of Signer."
"Error generating address. Address key collision for seed name Alice"
"Error generating address. Tried to create colliding addresses within same network. Address for seed name Alice and path //01 has same public key as address for seed name Alice and path //1."
"Error generating address. Bad secret string: invalid overall format."
"Error generating address. Bad secret string: invalid bip39 phrase."
"Error generating address. Bad secret string: invalid password."
"Error generating address. Bad secret string: invalid seed."
"Error generating address. Bad secret string: invalid seed length."
"Error generating address. Bad secret string: invalid path."
"Error generating address. Seed Alice already has derivation //Alice for network specs key 0150e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e, public key 8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48."
"Error generating address. Could not create random phrase. Seed phrase has invalid length."
"Error generating address. Invalid derivation format."
"Error generating qr code. Qr generation failed."
"Error parsing incoming transaction content. Unable to separate transaction method and extensions."
"Error parsing incoming transaction content. Expected mortal transaction due to prelude format. Found immortal transaction."
"Error parsing incoming transaction content. Expected immortal transaction due to prelude format. Found mortal transaction."
"Error parsing incoming transaction content. Genesis hash values from decoded extensions and from used network specs do not match."
"Error parsing incoming transaction content. Block hash for immortal transaction not matching genesis hash for the network."
"Error parsing incoming transaction content. Unable to decode extensions for V12/V13 metadata using standard extensions set."
"Error parsing incoming transaction content. Method number 2 not found in pallet test_Pallet."
"Error parsing incoming transaction content. Pallet with index 3 not found."
"Error parsing incoming transaction content. No calls found in pallet test_pallet_v14."
"Error parsing incoming transaction content. Referenced type could not be resolved in v14 metadata."
"Error parsing incoming transaction content. Argument type error."
"Error parsing incoming transaction content. Argument name error."
"Error parsing incoming transaction content. Expected compact. Not found it."
"Error parsing incoming transaction content. Data too short for expected content."
"Error parsing incoming transaction content. Unable to decode part of data as u32."
"Error parsing incoming transaction content. Encountered unexpected Option<_> variant."
"Error parsing incoming transaction content. IdentityField description error."
"Error parsing incoming transaction content. Unexpected type encountered for Balance"
"Error parsing incoming transaction content. Encountered unexpected enum variant."
"Error parsing incoming transaction content. Unexpected type inside compact."
"Error parsing incoming transaction content. No description found for type SomeUnknownType."
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
"Failed to decode extensions. Please try updating metadata for westend network. Parsing with westend9010 metadata: Network spec version decoded from extensions (9122) differs from the version in metadata (9010). Parsing with westend9000 metadata: Network spec version decoded from extensions (9122) differs from the version in metadata (9000)."
"Error with secret string of existing address: invalid overall format."
"Error with secret string of existing address: invalid bip39 phrase."
"Error with secret string of existing address: invalid password."
"Error with secret string of existing address: invalid seed."
"Error with secret string of existing address: invalid seed length."
"Error with secret string of existing address: invalid path."
"Wrong password."
"Wrong password."
"No networks available. Please load networks information to proceed."
"Unable to produce timestamp. The distance component cannot be formatted into the requested format."
"There are no seeds. Please create a seed first."
"Signer expected seed phrase, but the seed phrase is empty. Please report this bug."
"Signer expected seed name, but the seed name is empty. Please report this bug."
"#;
        assert!(print == print_expected, "\nReceived: {}", print);
    }
}
