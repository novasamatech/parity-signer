//! Errors occuring in Signer
//!
//! Signer works with cold database only.
//!
//! All errors [`ErrorSigner`] could be displayed to user:
//! - as part of card set, generated during transaction parsing  
//! - as individual text errors through `Error` modal in `navigator`
//! - as `anyhow` errors produced by the Signer device
//!
//! For all this, the text representation of the error is needed. Error text
//! must clearly indicate what happened and what user can try to do to fix the
//! issue. Error wording must be yet polished extensively.
//!
//! This module gathers all possible [`ErrorSigner`] errors in one place, so that
//! error management is easier.
use anyhow::anyhow;
use sp_core::{crypto::SecretStringError, H256};
use std::fmt::Write as _;
use time::error::Format;
#[cfg(feature = "test")]
use variant_count::VariantCount;

use crate::{
    crypto::Encryption,
    error::{
        bad_secret_string, AddressGeneration, AddressGenerationCommon, AddressKeySource,
        ErrorSource, MetadataError, MetadataSource, SpecsKeySource, TransferContent,
    },
    keyring::{AddressKey, MetaKey, NetworkSpecsKey, Order, VerifierKey},
    network_specs::{ValidCurrentVerifier, Verifier, VerifierValue},
};

/// Enum-marker indicating that error originates on the Signer side
#[derive(Debug)]
pub enum Signer {}

impl ErrorSource for Signer {
    type Error = ErrorSigner;
    type NotHex = NotHexSigner;
    type ExtraSpecsKeySource = ExtraSpecsKeySourceSigner;
    type ExtraAddressKeySource = ExtraAddressKeySourceSigner;
    type IncomingMetadataSource = IncomingMetadataSourceSigner;
    type ExtraAddressGeneration = ExtraAddressGenerationSigner;
    fn hex_to_error(what: Self::NotHex) -> Self::Error {
        ErrorSigner::Interface(InterfaceSigner::NotHex(what))
    }
    fn specs_key_to_error(
        network_specs_key: &NetworkSpecsKey,
        source: SpecsKeySource<Self>,
    ) -> Self::Error {
        match source {
            SpecsKeySource::SpecsTree => ErrorSigner::Database(DatabaseSigner::KeyDecoding(
                KeyDecodingSignerDb::NetworkSpecsKey(network_specs_key.to_owned()),
            )),
            SpecsKeySource::AddrTree(address_key) => ErrorSigner::Database(
                DatabaseSigner::KeyDecoding(KeyDecodingSignerDb::NetworkSpecsKeyAddressDetails {
                    address_key,
                    network_specs_key: network_specs_key.to_owned(),
                }),
            ),
            SpecsKeySource::Extra(ExtraSpecsKeySourceSigner::Interface) => {
                ErrorSigner::Interface(InterfaceSigner::KeyDecoding(
                    KeyDecodingSignerInterface::NetworkSpecsKey(network_specs_key.to_owned()),
                ))
            }
        }
    }
    fn address_key_to_error(
        address_key: &AddressKey,
        source: AddressKeySource<Self>,
    ) -> Self::Error {
        match source {
            AddressKeySource::AddrTree => ErrorSigner::Database(DatabaseSigner::KeyDecoding(
                KeyDecodingSignerDb::AddressKey(address_key.to_owned()),
            )),
            AddressKeySource::Extra(ExtraAddressKeySourceSigner::Interface) => {
                ErrorSigner::Interface(InterfaceSigner::KeyDecoding(
                    KeyDecodingSignerInterface::AddressKey(address_key.to_owned()),
                ))
            }
        }
    }
    fn meta_key_to_error(meta_key: &MetaKey) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::KeyDecoding(KeyDecodingSignerDb::MetaKey(
            meta_key.to_owned(),
        )))
    }
    fn metadata_mismatch(
        name_key: String,
        version_key: u32,
        name_inside: String,
        version_inside: u32,
    ) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::Mismatch(MismatchSigner::Metadata {
            name_key,
            version_key,
            name_inside,
            version_inside,
        }))
    }
    fn faulty_metadata(error: MetadataError, source: MetadataSource<Self>) -> Self::Error {
        match source {
            MetadataSource::Database { name, version } => {
                ErrorSigner::Database(DatabaseSigner::FaultyMetadata {
                    name,
                    version,
                    error,
                })
            }
            MetadataSource::Incoming(IncomingMetadataSourceSigner::ReceivedData) => {
                ErrorSigner::Input(InputSigner::FaultyMetadata(error))
            }
        }
    }
    fn specs_decoding(key: NetworkSpecsKey) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::EntryDecoding(
            EntryDecodingSigner::NetworkSpecs(key),
        ))
    }
    fn specs_genesis_hash_mismatch(key: NetworkSpecsKey, genesis_hash: H256) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::Mismatch(MismatchSigner::SpecsGenesisHash {
            key,
            genesis_hash,
        }))
    }
    fn specs_encryption_mismatch(key: NetworkSpecsKey, encryption: Encryption) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::Mismatch(MismatchSigner::SpecsEncryption {
            key,
            encryption,
        }))
    }
    fn address_details_decoding(key: AddressKey) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::EntryDecoding(
            EntryDecodingSigner::AddressDetails(key),
        ))
    }
    fn address_details_encryption_mismatch(key: AddressKey, encryption: Encryption) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::Mismatch(
            MismatchSigner::AddressDetailsEncryption { key, encryption },
        ))
    }
    fn address_details_specs_encryption_mismatch(
        address_key: AddressKey,
        network_specs_key: NetworkSpecsKey,
    ) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::Mismatch(
            MismatchSigner::AddressDetailsSpecsEncryption {
                address_key,
                network_specs_key,
            },
        ))
    }
    fn address_generation_common(error: AddressGenerationCommon) -> Self::Error {
        ErrorSigner::AddressGeneration(AddressGeneration::Common(error))
    }
    fn transfer_content_error(transfer_content: TransferContent) -> Self::Error {
        ErrorSigner::Input(InputSigner::TransferContent(transfer_content))
    }
    fn db_internal(error: sled::Error) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::Internal(error))
    }
    fn db_transaction(error: sled::transaction::TransactionError) -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::Transaction(error))
    }
    fn faulty_database_types() -> Self::Error {
        ErrorSigner::Database(DatabaseSigner::EntryDecoding(EntryDecodingSigner::Types))
    }
    fn types_not_found() -> Self::Error {
        ErrorSigner::NotFound(NotFoundSigner::Types)
    }
    fn metadata_not_found(name: String, version: u32) -> Self::Error {
        ErrorSigner::NotFound(NotFoundSigner::Metadata { name, version })
    }
    fn timestamp_format(error: time::error::Format) -> Self::Error {
        ErrorSigner::TimeFormat(error)
    }
    fn empty_seed() -> Self::Error {
        ErrorSigner::SeedPhraseEmpty
    }
    fn empty_seed_name() -> Self::Error {
        ErrorSigner::SeedNameEmpty
    }
    fn show(error: &Self::Error) -> String {
        match error {
            ErrorSigner::Interface(a) => {
                let insert = match a {
                    InterfaceSigner::NotHex(b) => {
                        let insert = match b {
                            NotHexSigner::NetworkSpecsKey {input} => format!("Network specs key {}", input),
                            NotHexSigner::InputContent => String::from("Input content"),
                            NotHexSigner::AddressKey {input} => format!("Address key {}", input),
                        };
                        format!("{} is not in hexadecimal format.", insert)
                    },
                    InterfaceSigner::KeyDecoding(b) => {
                        let (insert, key) = match b {
                            KeyDecodingSignerInterface::AddressKey(x) => ("address", x.key()),
                            KeyDecodingSignerInterface::NetworkSpecsKey(x) => ("network specs", x.key()),
                        };
                        format!("Unable to parse {} key {} passed through the interface.", insert, hex::encode(key))
                    },
                    InterfaceSigner::PublicKeyLength => String::from("Public key length does not match the encryption."),
                    InterfaceSigner::HistoryPageOutOfRange{page_number, total_pages} => format!("Requested history page {} does not exist. Total number of pages {}.", page_number, total_pages),
                    InterfaceSigner::SeedNameNotMatching{address_key, expected_seed_name, real_seed_name} => format!("Expected seed name {} for address key {}. Address details in database have {} name.", expected_seed_name, hex::encode(address_key.key()), real_seed_name),
                    InterfaceSigner::LostPwd => String::from("Derivation had password, then lost it."),
                    InterfaceSigner::VersionNotU32(x) => format!("Version {} could not be converted into u32.", x),
                    InterfaceSigner::IncNotU32(x) => format!("Increment {} could not be converted into u32.", x),
                    InterfaceSigner::OrderNotU32(x) => format!("Order {} could not be converted into u32", x),
                    InterfaceSigner::FlagNotBool(x) => format!("Flag {} could not be converted into bool.", x),
                };
                format!("Error on the interface. {}", insert)
            },
            ErrorSigner::Database(a) => {
                let insert = match a {
                    DatabaseSigner::KeyDecoding(b) => {
                        let insert = match b {
                            KeyDecodingSignerDb::AddressKey(x) => format!("address key {}", hex::encode(x.key())),
                            KeyDecodingSignerDb::EntryOrder(x) => format!("history entry order {}", hex::encode(x)),
                            KeyDecodingSignerDb::MetaKey(x) => format!("meta key {}", hex::encode(x.key())),
                            KeyDecodingSignerDb::NetworkSpecsKey(x) => format!("network specs key {}", hex::encode(x.key())),
                            KeyDecodingSignerDb::NetworkSpecsKeyAddressDetails{address_key, network_specs_key} => format!("network specs key {} from network id set of address book entry with key {}", hex::encode(network_specs_key.key()), hex::encode(address_key.key())),
                        };
                        format!("Unable to parse {} from the database.", insert)
                    },
                    DatabaseSigner::Internal(e) => format!("Internal error. {}", e),
                    DatabaseSigner::Transaction(e) => format!("Transaction error. {}", e),
                    DatabaseSigner::ChecksumMismatch => String::from("Checksum mismatch."),
                    DatabaseSigner::EntryDecoding(b) => {
                        let insert = match b {
                            EntryDecodingSigner::AddressDetails(x) => format!("address details entry for key {}.", hex::encode(x.key())),
                            EntryDecodingSigner::CurrentVerifier(x) => format!("current verifier entry for key {}.", hex::encode(x.key())),
                            EntryDecodingSigner::DangerStatus => String::from("danger status entry."),
                            EntryDecodingSigner::Derivations => String::from("temporary entry with information needed to import derivations."),
                            EntryDecodingSigner::GeneralVerifier => String::from("general verifier entry."),
                            EntryDecodingSigner::HistoryEntry(x) => format!("history entry for order {}.", x.stamp()),
                            EntryDecodingSigner::NetworkSpecs(x) => format!("network specs (NetworkSpecs) entry for key {}.", hex::encode(x.key())),
                            EntryDecodingSigner::Sign => String::from("temporary entry with information needed for signing approved transaction."),
                            EntryDecodingSigner::Stub => String::from("temporary entry with information needed for accepting approved information."),
                            EntryDecodingSigner::Types => String::from("types information."),
                        };
                        format!("Unable to decode {}", insert)
                    },
                    DatabaseSigner::Mismatch(b) => {
                        let insert = match b {
                            MismatchSigner::Metadata{name_key, version_key, name_inside, version_inside} => format!("Meta key corresponds to {}{}. Stored metadata is {}{}.", name_key, version_key, name_inside, version_inside),
                            MismatchSigner::SpecsGenesisHash{key, genesis_hash} => format!("Network specs (NetworkSpecs) entry with network specs key {} has not matching genesis hash {}.", hex::encode(key.key()), hex::encode(genesis_hash)),
                            MismatchSigner::SpecsEncryption{key, encryption} => format!("Network specs (NetworkSpecs) entry with network specs key {} has not matching encryption {}.", hex::encode(key.key()), encryption.show()),
                            MismatchSigner::AddressDetailsEncryption{key, encryption} => format!("Address details entry with address key {} has not matching encryption {}.", hex::encode(key.key()), encryption.show()),
                            MismatchSigner::AddressDetailsSpecsEncryption{address_key, network_specs_key} => format!("Address details entry with address key {} has associated network specs key {} with wrong encryption.", hex::encode(address_key.key()), hex::encode(network_specs_key.key())),
                        };
                        format!("Mismatch found. {}", insert)
                    },
                    DatabaseSigner::FaultyMetadata{name, version, error} => format!("Bad metadata for {}{}. {}", name, version, error.show()),
                    DatabaseSigner::UnexpectedGenesisHash{name, genesis_hash} => format!("Network {} with genesis hash {} has some network specs entries, while there is no verifier entry.", name, hex::encode(genesis_hash)),
                    DatabaseSigner::SpecsCollision{name, encryption} => format!("More than one entry for network specs with name {} and encryption {}.", name, encryption.show()),
                    DatabaseSigner::DifferentNamesSameGenesisHash{name1, name2, genesis_hash} => format!("Different network names ({}, {}) in database for same genesis hash {}.", name1, name2, hex::encode(genesis_hash)),
                    DatabaseSigner::CustomVerifierIsGeneral(key) => format!("Network with genesis hash {} verifier is set as a custom one. This custom verifier coinsides the database general verifier and not None. This should not have happened and likely indicates database corruption.", hex::encode(key.genesis_hash())),
                    DatabaseSigner::TwoRootKeys{seed_name, encryption} => format!("More than one seed key (i.e. with empty path and without password) found for seed name {} and encryption {}.", seed_name, encryption.show()),
                    DatabaseSigner::DifferentBase58Specs{genesis_hash, base58_1, base58_2} => format!("More than one base58 prefix in network specs database entries for network with genesis hash {}: {} and {}.", hex::encode(genesis_hash), base58_1, base58_2),
                };
                format!("Database error. {}", insert)
            },
            ErrorSigner::Input(a) => {
                let insert = match a {
                    InputSigner::TransferContent(a) => a.show(),
                    InputSigner::TransferDerivations => String::from("Payload could not be decoded as derivations transfer."),
                    InputSigner::FaultyMetadata(error) => format!("Received metadata is unsuitable. {}", error.show()),
                    InputSigner::TooShort => String::from("Input is too short."),
                    InputSigner::NotSubstrate(code) => format!("Only Substrate transactions are supported. Transaction is expected to start with 0x53, this one starts with 0x{}.", code),
                    InputSigner::PayloadNotSupported(code) => format!("Payload type with code 0x{} is not supported.", code),
                    InputSigner::SameNameVersionDifferentMeta{name, version} => format!("Metadata for {}{} is already in the database and is different from the one in received payload.", name, version),
                    InputSigner::MetadataKnown{name, version} => format!("Metadata for {}{} is already in the database.", name, version),
                    InputSigner::ImportantSpecsChanged(key) => format!("Similar network specs are already stored in the database under key {}. Network specs in received payload have different unchangeable values (base58 prefix, decimals, encryption, network name, unit).", hex::encode(key.key())),
                    InputSigner::AddSpecsDifferentBase58{genesis_hash, name, base58_database, base58_input} => format!("Network {} with genesis hash {} already has entries in the database with base58 prefix {}. Received network specs have same genesis hash and different base58 prefix {}.", name, hex::encode(genesis_hash), base58_database, base58_input),
                    InputSigner::AddSpecsDifferentName{genesis_hash, name_database, name_input} => format!("Network with genesis hash {} has name {} in the database. Received network specs have same genesis hash and name {}.", hex::encode(genesis_hash), name_database, name_input),
                    InputSigner::EncryptionNotSupported(code) => format!("Payload with encryption 0x{} is not supported.", code),
                    InputSigner::BadSignature => String::from("Received payload has bad signature."),
                    InputSigner::LoadMetaUnknownNetwork{name} => format!("Network {} is not in the database. Add network specs before loading the metadata.", name),
                    InputSigner::LoadMetaNoSpecs{name, valid_current_verifier, general_verifier} => {
                        let insert = match valid_current_verifier {
                            ValidCurrentVerifier::General => format!("{} (general verifier)", general_verifier.show_error()),
                            ValidCurrentVerifier::Custom{v} => format!("{} (custom verifier)", v.show_error()),
                        };
                        format!("Network {} was previously known to the database with verifier {}. However, no network specs are in the database at the moment. Add network specs before loading the metadata.", name, insert)
                    },
                    InputSigner::LoadMetaWrongGenesisHash {name_metadata, name_specs, genesis_hash} => format!("Update payload contains metadata for network {}. Genesis hash in payload ({}) matches database genesis hash for another network, {}.", name_metadata, hex::encode(genesis_hash), name_specs),
                    InputSigner::NeedVerifier{name, verifier_value} => format!("Saved network {} information was signed by verifier {}. Received information is not signed.", name, verifier_value.show_error()),
                    InputSigner::NeedGeneralVerifier{content, verifier_value} => {
                        let insert = match content {
                            GeneralVerifierForContent::Network{name} => format!("{} network information", name),
                            GeneralVerifierForContent::Types => String::from("types information"),
                        };
                        format!("General verifier in the database is {}. Received unsigned {} could be accepted only if signed by the general verifier.", verifier_value.show_error(), insert)
                    },
                    InputSigner::LoadMetaSetVerifier{name, new_verifier_value} => format!("Network {} currently has no verifier set up. Received load_metadata message is verified by {}. In order to accept verified metadata, first download properly verified network specs.", name, new_verifier_value.show_error()),
                    InputSigner::LoadMetaVerifierChanged{name, old_verifier_value, new_verifier_value} => format!("Network {} current verifier is {}. Received load_metadata message is verified by {}. Changing verifier for the network would require wipe and reset of Signer.", name, old_verifier_value.show_error(), new_verifier_value.show_error()),
                    InputSigner::LoadMetaSetGeneralVerifier{name, new_general_verifier_value} => format!("Network {} is set to be verified by the general verifier, however, general verifier is not yet established. Received load_metadata message is verified by {}. In order to accept verified metadata and set up the general verifier, first download properly verified network specs.", name, new_general_verifier_value.show_error()),
                    InputSigner::LoadMetaGeneralVerifierChanged{name, old_general_verifier_value, new_general_verifier_value} => format!("Network {} is verified by the general verifier which currently is {}. Received load_metadata message is verified by {}. Changing the general verifier or changing the network verifier to custom would require wipe and reset of Signer.", name, old_general_verifier_value.show_error(), new_general_verifier_value.show_error()),
                    InputSigner::GeneralVerifierChanged{content, old_general_verifier_value, new_general_verifier_value} => {
                        let insert = match content {
                            GeneralVerifierForContent::Network{name} => format!("network {} specs", name),
                            GeneralVerifierForContent::Types => String::from("types information"),
                        };
                        format!("General verifier in the database is {}. Received {} could be accepted only if verified by the same general verifier. Current message is verified by {}.", old_general_verifier_value.show_error(), insert, new_general_verifier_value.show_error())
                    },
                    InputSigner::TypesKnown => String::from("Exactly same types information is already in the database."),
                    InputSigner::MessageNoWrapper => String::from("Received message has no `<Bytes></Bytes>` wrapper."),
                    InputSigner::MessageNotValidUtf8 => String::from("Received message could not be represented as valid utf8 sequence."),
                    InputSigner::UnknownNetwork{genesis_hash, encryption} => format!("Input generated within unknown network and could not be processed. Add network with genesis hash {} and encryption {}.", hex::encode(genesis_hash), encryption.show()),
                    InputSigner::NoMetadata{name} => format!("Input transaction is generated in network {}. Currently there are no metadata entries for it, and transaction could not be processed. Add network metadata.", name),
                    InputSigner::SpecsKnown{name, encryption} => format!("Exactly same network specs for network {} with encryption {} are already in the database.", name, encryption.show()),
                    InputSigner::AddSpecsVerifierChanged {name, old_verifier_value, new_verifier_value} => format!("Network {} current verifier is {}. Received add_specs message is verified by {}, which is neither current network verifier not the general verifier. Changing the network verifier to another non-general one would require wipe and reset of Signer.", name, old_verifier_value.show_error(), new_verifier_value.show_error()),
                    InputSigner::InvalidDerivation(x) => format!("Derivation {} has invalid format.", x),
                    InputSigner::SeedNameExists(x) => format!("Seed name {} already exists.", x),
                };
                format!("Bad input data. {}", insert)
            },
            ErrorSigner::NotFound(a) => {
                match a {
                    NotFoundSigner::CurrentVerifier(verifier_key) => format!("Could not find current verifier for network with genesis hash {}.", hex::encode(verifier_key.genesis_hash())),
                    NotFoundSigner::GeneralVerifier => String::from("Could not find general verifier."),
                    NotFoundSigner::Types => String::from("Could not find types information."),
                    NotFoundSigner::NetworkSpecs(network_specs_key) => format!("Could not find network specs for network specs key {}.", hex::encode(network_specs_key.key())),
                    NotFoundSigner::NetworkSpecsForName(name) => format!("Could not find network specs for {}.", name),
                    NotFoundSigner::NetworkSpecsKeyForAddress{network_specs_key, address_key} => format!("Could not find network specs key {} in address details with key {}.", hex::encode(network_specs_key.key()), hex::encode(address_key.key())),
                    NotFoundSigner::AddressDetails(address_key) => format!("Could not find address details for address key {}.", hex::encode(address_key.key())),
                    NotFoundSigner::Metadata{name, version} => format!("Could not find metadata entry for {}{}.", name, version),
                    NotFoundSigner::DangerStatus => String::from("Could not find danger status information."),
                    NotFoundSigner::Stub => String::from("Could not find database temporary entry with information needed for accepting approved information."),
                    NotFoundSigner::Sign => String::from("Could not find database temporary entry with information needed for signing approved transaction."),
                    NotFoundSigner::Derivations => String::from("Could not find database temporary entry with information needed for importing derivations set."),
                    NotFoundSigner::HistoryEntry(x) => format!("Could not find history entry with order {}.", x.stamp()),
                    NotFoundSigner::HistoryNetworkSpecs{name, encryption} => format!("Could not find network specs for {} with encryption {} needed to decode historical transaction.", name, encryption.show()),
                    NotFoundSigner::HistoricalMetadata{name} => format!("Historical transaction was generated in network {} and processed. Currently there are no metadata entries for the network, and transaction could not be processed again. Add network metadata.", name),
                    NotFoundSigner::NetworkForDerivationsImport{genesis_hash, encryption} => format!("Unable to import derivations for network with genesis hash {} and encryption {}. Network is unknown. Please add corresponding network specs.", hex::encode(genesis_hash), encryption.show()),
                }
            },
            ErrorSigner::DeadVerifier(key) => format!("Network with genesis hash {} is disabled. It could be enabled again only after complete wipe and re-installation of Signer.", hex::encode(key.genesis_hash())),
            ErrorSigner::AddressGeneration(a) => {
                let insert = match a {
                    AddressGeneration::Common(a) => a.show(),
                    AddressGeneration::Extra(ExtraAddressGenerationSigner::RandomPhraseGeneration(e)) => format!("Could not create random phrase. {}", e),
                    AddressGeneration::Extra(ExtraAddressGenerationSigner::InvalidDerivation) =>  String::from("Invalid derivation format."),
                };
                format!("Error generating address. {}", insert)
            },
            ErrorSigner::Qr(e) => format!("Error generating qr code. {}", e),
            ErrorSigner::Parser(a) => format!("Error parsing incoming transaction content. {}", a.show()),
            ErrorSigner::AllExtensionsParsingFailed{network_name, errors} => {
                let mut insert = String::new();
                for (i,(version, parser_error)) in errors.iter().enumerate() {
                    if i>0 {insert.push(' ')}
                    let _ = write!(insert, "Parsing with {}{} metadata: {}", network_name, version, parser_error.show());
                }
                format!("Failed to decode extensions. Please try updating metadata for {} network. {}", network_name, insert)
            },
            ErrorSigner::AddressUse(e) => format!("Error with secret string of existing address: {}.", bad_secret_string(e)),
            ErrorSigner::WrongPassword => String::from("Wrong password."),
            ErrorSigner::WrongPasswordNewChecksum(_) => String::from("Wrong password."),
            ErrorSigner::NoNetworksAvailable => String::from("No networks available. Please load networks information to proceed."),
            ErrorSigner::TimeFormat(e) => format!("Unable to produce timestamp. {}", e),
            ErrorSigner::NoKnownSeeds => String::from("There are no seeds. Please create a seed first."),
            ErrorSigner::SeedPhraseEmpty => String::from("Signer expected seed phrase, but the seed phrase is empty. Please report this bug."),
            ErrorSigner::SeedNameEmpty => String::from("Signer expected seed name, but the seed name is empty. Please report this bug."),
        }
    }
}

/// All possible errors that could occur on the Signer side.
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum ErrorSigner {
    /// Communication errors on the interface between native frontend and rust
    /// backend.
    ///
    /// Associated data is [`InterfaceSigner`] with more details.
    Interface(InterfaceSigner),

    /// Errors within Signer rust-managed database.
    ///
    /// Associated data is [`DatabaseSigner`] with more details.
    Database(DatabaseSigner),

    /// Errors in received input: signable transactions, updates,
    /// user-entered content.
    ///
    /// Associated data is [`InputSigner`] with more details.
    Input(InputSigner),

    /// Something was expected to be known to Signer, but was not found.
    ///
    /// Associated data is [`NotFoundSigner`] with more details.
    NotFound(NotFoundSigner),

    /// User tried to interact with previously disabled network.
    ///
    /// Associated data is the [`VerifierKey`] of the network.
    DeadVerifier(VerifierKey),

    /// Errors with address generation.
    ///
    /// Associated data is [`AddressGeneration`] with more details.
    AddressGeneration(AddressGeneration<Signer>),

    /// Errors with static QR codes generation.
    ///
    /// Signer can produce QR codes with signatures for transactions, with
    /// [`SufficientCrypto`](crate::crypto::SufficientCrypto) exports for
    /// user-verified updates, and with address public information exports
    /// for Signer companion.
    ///
    /// Associated data is text of the error produced by the QR generator.
    Qr(String),

    /// Errors parsing a signable transactions with a given version of the
    /// metadata for given network.
    ///
    /// Associated data is [`ParserError`] with more details.
    Parser(ParserError),

    /// Error parsing extensions of a signable transaction with all available
    /// versions of metadata for given network.
    AllExtensionsParsingFailed {
        /// network name
        network_name: String,

        /// set of errors, with network version and [`ParserError`] for each
        errors: Vec<(u32, ParserError)>,
    },

    /// Error with using address already stored in the database.
    ///
    /// Associated data is
    /// [`SecretStringError`](https://docs.rs/sp-core/6.0.0/sp_core/crypto/enum.SecretStringError.html).
    AddressUse(SecretStringError),

    /// User has entered a wrong password for a passworded address.
    ///
    /// For cases when Signer database checksum is not verified.
    /// Signer log records that password was entered incorrectly.
    WrongPassword,

    /// User has entered a wrong password for a passworded address for cases
    /// when the Signer database checksum is verified.
    ///
    /// Signer log records that password was entered incorrectly.
    /// This changes the database checksum, and for the next attempt it must be
    /// updated.
    ///
    /// Associated data is the new checksum.
    WrongPasswordNewChecksum(u32),

    /// Signer has attempted an operation that requires at least one network to
    /// be loaded into Signer.
    NoNetworksAvailable,

    /// Time formatting error
    TimeFormat(Format),

    /// Signer has no seeds in storage. User tried an action that needs at least
    /// one seed.
    NoKnownSeeds,

    /// Signer tried to use empty seed phrase, likely a bug on the interface
    SeedPhraseEmpty,

    /// Signer got empty seed name, likely a bug on the interface
    SeedNameEmpty,
}

impl ErrorSigner {
    /// Signer side errors could be exported into native interface, before that
    /// they must be transformed into anyhow errors.
    pub fn anyhow(&self) -> anyhow::Error {
        anyhow!(<Signer>::show(self))
    }
}

/// Communication errors on the interface between native frontend and rust
/// backend
///
/// [`InterfaceSigner`] error means that rust backend can not process the
/// information sent by the frontend.
///
/// Signer rust backend sends data into frontend as [`ActionResult`](crate::navigation::ActionResult)s. Frontend
/// displays them, and can send back into rust some parts of the data of
/// the user input.
///
/// Data that user can type from keyboard is processed as is, it does not
/// cause errors on the interface.
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum InterfaceSigner {
    /// Received string is not hexadecimal, and could not be transformed into
    /// `Vec<u8>`.
    ///
    /// Associated data is [`NotHexSigner`] with more details.
    NotHex(NotHexSigner),

    /// Received database key could not be decoded.
    ///
    /// Associated data is [`KeyDecodingSignerInterface`] with more details.
    KeyDecoding(KeyDecodingSignerInterface),

    /// Received public key length is different from the one expected for
    /// given encryption algorithm.
    PublicKeyLength,

    /// Requested history page number exceeds the total number of pages.
    // TODO: error possibly would become obsolete
    HistoryPageOutOfRange { page_number: u32, total_pages: u32 },

    /// To generate QR code with public address information export, Signer
    /// receives both seed name and
    /// [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)
    /// from the navigation state `Navstate`.
    /// `MultiSigner` gets transformed into [`AddressKey`] and corresponds to
    /// [`AddressDetails`](crate::users::AddressDetails) that are exported.
    /// `AddressDetails` also contain `seed_name` field, that must coincide
    /// with the one received directly from the navigator.
    /// This error appears if the seed names are different.
    SeedNameNotMatching {
        /// address key for which the export is done
        address_key: AddressKey,

        /// seed name, from the navigator
        expected_seed_name: String,

        /// seed name, from the `AddressDetails`
        real_seed_name: String,
    },

    /// User was creating the derivation with password, and thus moved into
    /// `PasswordConfirm` modal, however, the password was not found when
    /// cutting password from the path.
    LostPwd,

    /// Received from interface network metadata version could not be parsed
    /// as `u32`.
    ///
    /// Associated content is the received data as a string.
    VersionNotU32(String),

    /// Received from interface increment for address generation could not
    /// be parsed as `u32`.
    ///
    /// Associated content is the received data as a string.
    IncNotU32(String),

    /// Received from interface history log order could not be parsed as `u32`.
    ///
    /// Associated content is the received data as a string.
    OrderNotU32(String),

    /// Received from interface boolean flag could not be parsed as `bool`.
    ///
    /// Associated content is the received data as a string.
    FlagNotBool(String),
}

/// `NotHex` errors occuring on the Signer side
///
/// Expected to receive hexadecimal string from the interface, got something
/// different. [`NotHexSigner`] specifies, what was expected.
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum NotHexSigner {
    /// [`NetworkSpecsKey`] is not hexadecimal, associated data is input string
    /// as it is received.
    NetworkSpecsKey { input: String },

    /// Received signable transaction or update are not hexadecimal.
    InputContent,

    /// [`AddressKey`] is not hexadecimal, associated data is input string as
    /// it is received.
    AddressKey { input: String },
}

/// Source of damaged [`NetworkSpecsKey`], exclusive for the Signer side
#[derive(Debug)]
pub enum ExtraSpecsKeySourceSigner {
    /// Damaged [`NetworkSpecsKey`] is from the interface.
    Interface,
}

/// Source of damaged [`AddressKey`], exclusive for the Signer side
#[derive(Debug)]
pub enum ExtraAddressKeySourceSigner {
    /// Damaged [`AddressKey`] is from the interface.
    Interface,
}

/// Source of damaged metadata, exclusive for the Signer side
#[derive(Debug)]
pub enum IncomingMetadataSourceSigner {
    /// Damaged metadata is received through QR code update.
    ReceivedData,
}

/// Errors decoding database keys received from the frontend on the Signer
/// side
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum KeyDecodingSignerInterface {
    /// [`AddressKey`] received from the frontend was a valid hexadecimal, but
    /// turned out to be a damaged database key with invalid content.
    ///
    /// This error indicates that [`AddressKey`] content could not be processed
    /// to get
    /// [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)
    /// value.
    ///
    /// Associated data is the damaged `AddressKey`.
    AddressKey(AddressKey),

    /// [`NetworkSpecsKey`] received from the frontend was a valid hexadecimal, but
    /// turned out to be a damaged database key with invalid content.
    ///
    /// This error indicates that [`NetworkSpecsKey`] content could not be processed
    /// to get from it [`Encryption`] and network genesis hash.
    ///
    /// Associated data is the damaged `NetworkSpecsKey`.
    NetworkSpecsKey(NetworkSpecsKey),
}

/// Errors in the database content on the Signer side
///
/// Describes errors with already existing database content (e.g. damaged keys,
/// damaged values, various mismatches, data that could not have been added to
/// the database in the first place etc).
///
/// Note that [`NotFoundSigner`] is a separate set of errors. Things **not
/// found** are kept separately here from things **damaged**.
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum DatabaseSigner {
    /// Key used in one of the database trees has invalid content, and could
    /// not be decoded.
    ///
    /// Associated data is [`KeyDecodingSignerDb`] with more details.
    KeyDecoding(KeyDecodingSignerDb),

    /// Database [`Error`](https://docs.rs/sled/0.34.6/sled/enum.Error.html).
    ///
    /// Could happen, for example, when opening the database, loading trees,
    /// reading values etc.
    Internal(sled::Error),

    /// Database
    /// [`TransactionError`](https://docs.rs/sled/0.34.6/sled/transaction/enum.TransactionError.html).
    ///
    /// Could happen when making transactions in multiple trees simultaneously.
    Transaction(sled::transaction::TransactionError),

    /// Database checksum does not match the expected value.
    ChecksumMismatch,

    /// Value found in one of the database trees has invalid content, and could
    /// not be decoded.
    ///
    /// Associated data is [`EntryDecodingSigner`] with more details.
    EntryDecoding(EntryDecodingSigner),

    /// Data retrieved from the database contains some internal contradictions,
    /// could not have been written in the database this way, and is therefore
    /// likely indicating the database corruption.
    ///
    /// Associated data is [`MismatchSigner`] with more details.
    Mismatch(MismatchSigner),

    /// Network metadata that already is in the database, is damaged.
    ///
    /// Unsuitable metadata could not be put in the database in the first place,
    /// finding one would mean the database got corrupted.
    FaultyMetadata {
        /// network name, from [`MetaKey`]
        name: String,

        /// network version, from [`MetaKey`]
        version: u32,

        /// what exactly is wrong with the metadata
        error: MetadataError,
    },

    /// Network has no entry for
    /// [`CurrentVerifier`](crate::network_specs::CurrentVerifier) under
    /// `verifier_key` in `VERIFIERS` tree of the database, however, the
    /// corresponding genesis hash is encountered in a
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) entry under
    /// `network_specs_key` in `SPECSTREE` tree of the database.
    /// No network specs record can get into database without the verifier
    /// entry, and the verifier could not be removed while network specs still
    /// remain, so this indicates the database got corrupted.
    UnexpectedGenesisHash {
        /// network name
        name: String,

        /// network genesis hash
        genesis_hash: H256,
    },

    /// More than one entry found for network specs with given `name` and
    /// `encryption`, when trying to parse transaction from historical record.
    // TODO: goes obsolete if we add `genesis_hash` field to `SignDisplay`
    SpecsCollision {
        /// network name
        name: String,

        /// network supported encryption
        encryption: Encryption,
    },

    /// While searching for all networks with same genesis hash, found that
    /// there are networks with same genesis hash, but different names.
    DifferentNamesSameGenesisHash {
        name1: String,
        name2: String,
        genesis_hash: H256,
    },

    /// Network [`CurrentVerifier`](crate::network_specs::CurrentVerifier) is
    /// `ValidCurrentVerifier::Custom(_)`, but the custom verifier value
    /// coincides with the general verifier.
    ///
    /// Associated data is [`VerifierKey`] corresponding to faulty entry.
    CustomVerifierIsGeneral(VerifierKey),

    /// Database has two seed addresses (i.e. with empty derivation path and no
    /// password) for same seed name and [`Encryption`]
    ///
    /// This indicates the database corruption, since the encrypion method,
    /// seed name and derivation path strictly determine the public key.
    TwoRootKeys {
        /// seed name
        seed_name: String,

        /// encryption algorithm for which two seed keys were found
        encryption: Encryption,
    },

    /// Network specs entries have same genesis hash, but different base58 prefix
    DifferentBase58Specs {
        genesis_hash: H256,
        base58_1: u16,
        base58_2: u16,
    },
}

/// Errors decoding database keys on the Signer side
///
/// `IVec` value of the database key could be unfallably transformed into the
/// contents of the corresponding key from the [`keyring`](crate::keyring)
/// module. All these keys were, however, generated using certain information,
/// and if this information could not be extracted from the key, it indicates
/// that the database is damaged and results in [`KeyDecodingSignerDb`] error.
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum KeyDecodingSignerDb {
    /// [`AddressKey`] from the database could not be processed.
    ///
    /// Associated data is the damaged `AddressKey`.
    AddressKey(AddressKey),

    /// [`Order`](crate::keyring::Order) could not be generated from the
    /// database key.
    ///
    /// Associated data is the damaged database key in `Vec<u8>` format.
    EntryOrder(Vec<u8>),

    /// [`MetaKey`] from the database could not be processed.
    ///
    /// Associated data is the damaged `MetaKey`.
    MetaKey(MetaKey),

    /// [`NetworkSpecsKey`] encountered as a key in database tree `SPECSTREE`
    /// could not be processed.
    ///
    /// Associated data is the damaged `NetworkSpecsKey`.
    NetworkSpecsKey(NetworkSpecsKey),

    /// [`NetworkSpecsKey`] encountered as one of the entries in `network_id`
    /// field of the [`AddressDetails`](crate::users::AddressDetails) could not
    /// be processed.
    NetworkSpecsKeyAddressDetails {
        /// [`AddressKey`] corresponding to `AddressDetails` that contain the
        /// damaged `NetworkSpecsKey`
        address_key: AddressKey,

        /// damaged `NetworkSpecsKey`
        network_specs_key: NetworkSpecsKey,
    },
}

/// Errors decoding database entry content on the Signer side
///
/// Database stores most of the values SCALE-encoded, and to be used they must
/// be decoded. If the decoding fails, it indicates that the database is
/// damaged.
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum EntryDecodingSigner {
    /// Database entry could not be decoded as
    /// [`AddressDetails`](crate::users::AddressDetails).
    ///
    /// Associated data is the corresponding [`AddressKey`].
    AddressDetails(AddressKey),

    /// Database entry could not be decoded as
    /// [`CurrentVerifier`](crate::network_specs::CurrentVerifier).
    ///
    /// Associated data is the corresponding [`VerifierKey`].
    CurrentVerifier(VerifierKey),

    /// Database entry in `SETTREE` tree of the Signer database stored under
    /// the key `DANGER` could not be processed as a valid
    /// [`DangerRecord`](crate::danger::DangerRecord).
    DangerStatus,

    /// Temporary database entry in `TRANSACTION` tree of the Signer database
    /// under the key `DRV`, used to store the derivation import data,
    /// could not be decoded.
    Derivations,

    /// General verifier information, i.e. encoded
    /// [`Verifier`](crate::network_specs::Verifier) stored in `SETTREE` tree
    /// of the Signer database under the key `GENERALVERIFIER`,
    /// could not be decoded.
    GeneralVerifier,

    /// History log [`Entry`](crate::history::Entry) stored in `HISTORY` tree
    /// of the Signer database under the key [`Order`](crate::keyring::Order)
    /// could not be decoded.
    ///
    /// Associated data is the corresponding [`Order`].
    HistoryEntry(Order),

    /// Database entry could not be decoded as
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs).
    ///
    /// Associated data is the corresponding [`NetworkSpecsKey`].
    NetworkSpecs(NetworkSpecsKey),

    /// Temporary database entry in `TRANSACTION` tree of the Signer database
    /// under the key `SIGN`, used to store the signable transaction data
    /// awaiting for the user approval, could not be decoded.
    Sign,

    /// Temporary database entry in `TRANSACTION` tree of the Signer database
    /// under the key `STUB`, used to store the update data awaiting for the
    /// user approval, could not be decoded.
    Stub,

    /// Types information, i.e. encoded `Vec<TypeEntry>` stored in `SETTREE`
    /// tree of the Signer database under the key `TYPES`, could not be decoded.
    Types,
}

/// Mismatch errors within database on the Signer side
///
/// Data could be recorded in the Signer database only in ordered fasion, i.e.
/// with keys corresponding to the data stored in the encoded values etc.
/// If the data retrieved from the database contains some internal
/// contradictions, it indicates the database corruption.
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum MismatchSigner {
    /// Network name and/or network version in [`MetaKey`] do not match the
    /// network name and network version from `Version` constant, `System`
    /// pallet of the metadata stored under this `MetaKey`.
    Metadata {
        /// network name as it is in the key
        name_key: String,

        /// network version as it is in the key
        version_key: u32,

        /// network name as it is in the metadata
        name_inside: String,

        /// network version as it is in the metadata
        version_inside: u32,
    },

    /// [`NetworkSpecsKey`] is built using network genesis hash and [`Encryption`].
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) entry stored under
    /// this `NetworkSpecsKey` contains `genesis_hash` field with a different
    /// genesis hash.
    SpecsGenesisHash {
        /// [`NetworkSpecsKey`] corresponding to mismatching data
        key: NetworkSpecsKey,

        /// genesis hash as it is in the `NetworkSpecs`
        // TODO: could be [u8; 32] array here; we may want to decide and fix in
        // several places if the genesis hash may at all be something different
        // from [u8;32] array
        genesis_hash: H256,
    },

    /// [`NetworkSpecsKey`] is built using network genesis hash and [`Encryption`].
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) entry stored under
    /// this `NetworkSpecsKey` contains `encryption` field with a different
    /// [`Encryption`].
    SpecsEncryption {
        /// [`NetworkSpecsKey`] corresponding to mismatching data
        key: NetworkSpecsKey,

        /// [`Encryption`] as it is in the `NetworkSpecs`
        encryption: Encryption,
    },

    /// [`AddressKey`] has an associated [`Encryption`].
    /// [`AddressDetails`](crate::users::AddressDetails) entry stored under
    /// this `AddressKey` contains `encryption` field with a different
    /// [`Encryption`].
    AddressDetailsEncryption {
        /// [`AddressKey`] corresponding to mismatching data
        key: AddressKey,

        /// [`Encryption`] as it is in the `AddressDetails`
        encryption: Encryption,
    },

    /// [`AddressKey`] has an associated [`Encryption`].
    /// [`AddressDetails`](crate::users::AddressDetails) entry stored under
    /// this `AddressKey` contains `network_id` field with a set of
    /// [`NetworkSpecsKey`] values corresponding to networks in which this
    /// address exists. [`NetworkSpecsKey`] is built using network genesis hash
    /// and `Encryption`.
    ///
    /// If the `Encryption` value from one of `NetworkSpecsKey` values is
    /// different from `Encryption` assocaited with `AddressKey`, this error
    /// appears.
    AddressDetailsSpecsEncryption {
        /// [`AddressKey`] corresponding to mismatching data
        address_key: AddressKey,

        /// [`NetworkSpecsKey`] having `Encryption` different from the one
        /// associated with `AddressKey`
        network_specs_key: NetworkSpecsKey,
    },
}

/// Errors in the input received by the Signer
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum InputSigner {
    /// Updating transaction with `add_specs`, `load_metadata` or `load_types`
    /// content, as declared by the prelude, could not be decoded.
    ///
    /// Associated data is [`TransferContent`] specifying what content was
    /// expected to be.
    TransferContent(TransferContent),

    /// Updating transaction with `derivation` content, as declared by the
    /// prelude, could not be decoded.
    TransferDerivations,

    /// Network metadata received in `load_metadata` is not suitable for use
    /// in Signer.
    ///
    /// Associated data is [`MetadataError`] specifying what exactly is
    /// unacceplable with incoming metadata.
    FaultyMetadata(MetadataError),

    /// Received transaction is unexpectedly short, more bytes were expected.
    TooShort,

    /// All transactions are expected to be the Substrate ones, starting with
    /// hexadecimal `53`.
    ///
    /// Associated data is the first two elements of the hexadecimal string in
    /// received transaction.
    NotSubstrate(String),

    /// There is a limited number of payloads supported by the Signer. Payload
    /// type is declared in the transaction prelude `53xxyy` in `yy` part.
    ///
    /// Currently supported payloads are:
    ///
    /// - `00` mortal signable transaction
    /// - `02` immortal signable transaction
    /// - `03` text message
    /// - `80` `load_metadata` update
    /// - `81` `load_types` update
    /// - `c1` `add_specs` update
    /// - `de` `derivations` update
    /// - `f0` print all available cards (testing tool)
    ///
    /// Other codes are not supported, the error associated data contains the
    /// hexadecimal string with unsupported payload code.
    PayloadNotSupported(String),

    /// Network name and version from metadata received in `load_metadata`
    /// message already have a corresponding entry in `METATREE` tree of the
    /// Signer database. However, the received metadata is different from
    /// the one already stored in the database.
    SameNameVersionDifferentMeta {
        /// network name (identical for received and for stored metadata)
        name: String,

        /// network version (identical for received and for stored metadata)
        version: u32,
    },

    /// Network name and version from metadata received in `load_metadata`
    /// message already have a corresponding entry in `METATREE` tree of the
    /// Signer database, with exactly same metadata.
    ///
    /// Not exactly an error, but Signer can't do anything and complains.
    MetadataKnown {
        /// network name (identical for received and for stored metadata)
        name: String,

        /// network version (identical for received and for stored metadata)
        version: u32,
    },

    /// [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend)
    /// received in `add_specs` payload are for a network that already has
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) entry in
    /// the `SPECSTREE` tree of the Signer database with **same**
    /// [`NetworkSpecsKey`], and the permanent components of the network
    /// specs stores and received are different.
    ///
    /// The components that could not be changed by an update, without removing
    /// the network completely, are:
    ///
    /// - `base58prefix`, network-associated base58 prefix  
    /// - `decimals`  
    /// - `name`, network name, as it appears in the network metadata  
    /// - `unit`
    ImportantSpecsChanged(NetworkSpecsKey),

    /// [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend)
    /// received in `add_specs` payload are for a network that already has
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) entry in
    /// the `SPECSTREE` tree of the Signer database with not necessarily
    /// same encryption, i.e. **possibly different** [`NetworkSpecsKey`],
    /// and base58 prefix in stored network specs is different from the base58
    /// prefix in the received ones.
    AddSpecsDifferentBase58 {
        genesis_hash: H256,
        name: String,
        base58_database: u16,
        base58_input: u16,
    },

    /// [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecsToSend)
    /// received in `add_specs` payload are for a network that already has
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) entry in
    /// the `SPECSTREE` tree of the Signer database with not necessarily
    /// same encryption, i.e. **possibly different** [`NetworkSpecsKey`],
    /// and network name in stored network specs is different from the network
    /// name in the received ones.
    AddSpecsDifferentName {
        genesis_hash: H256,
        name_database: String,
        name_input: String,
    },

    /// There is a limited number of encryption algorithms supported by the
    /// Signer. Encryption algorithm is declared in the transaction prelude
    /// `53xxyy` in `xx` part.
    ///
    /// For signable transactions (i.e. with prelude `53xx00`, `53xx02` and
    /// `53xx03`) currently supported encryption algorithms are:
    ///
    /// - `00` for `Ed25519`
    /// - `01` for `Sr25519`
    /// - `02` for `Ecdsa`
    ///
    /// In signable transaction the encryption algorithm corresponds to the
    /// encryption associated with the address that generated the transaction
    /// and can sign it (and thus to the encryption supported by the network
    /// in which the transaction is generated).
    ///
    /// Update transactions have currently supported encryption codes:
    ///
    /// - `00` for `Ed25519`
    /// - `01` for `Sr25519`
    /// - `02` for `Ecdsa`
    /// - `ff` for unsigned update transactions
    ///
    /// In signed update transactions the encryption code indicates which
    /// algorithm to use for update signature verification.
    ///
    /// Unsigned update transactions have no associated signature, are not
    /// checked and are strongly discouraged.
    ///
    /// Other encryption codes are not supported, the error associated data
    /// contains the hexadecimal string with unsupported encryption code.
    EncryptionNotSupported(String),

    /// Update payload signature is invalid for given public key, encryption
    /// algorithm and payload content
    BadSignature,

    /// User attempted to load into Signer the metadata for the network that
    /// has no [`CurrentVerifier`](crate::network_specs::CurrentVerifier) entry
    /// in the `VERIFIERS` tree of the Signer database.
    LoadMetaUnknownNetwork {
        /// network name as it is in the received metadata
        name: String,
    },

    /// User attempted to load into Signer the metadata for the network that
    /// has no associated [`NetworkSpecs`](crate::network_specs::NetworkSpecs)
    /// entries in the `SPECSTREE` tree of the Signer database, although it has
    /// an associated
    /// [`ValidCurrentVerifier`](crate::network_specs::ValidCurrentVerifier),
    /// i.e. it was known to user at some point and never disabled.
    LoadMetaNoSpecs {
        /// network name as it is in the received metadata
        name: String,

        /// network-associated
        /// [`ValidCurrentVerifier`](crate::network_specs::ValidCurrentVerifier)
        valid_current_verifier: ValidCurrentVerifier,

        /// Signer general verifier
        general_verifier: Verifier,
    },

    /// User attempted to load into Signer the metadata for the network that
    /// has a [`NetworkSpecs`](crate::network_specs::NetworkSpecs) entry in the
    /// `SPECSTREE` tree of the Signer database, but specs have a different
    /// network name.
    ///
    /// Most likely, wrong genesis hash was attached to the metadata update.
    ///
    /// Since the network metadata in `METATREE` is identified by network name,
    /// and verifier is identified by the genesis hash, this should be checked
    /// on `load_metadata`.
    LoadMetaWrongGenesisHash {
        /// network name as it is in the received metadata
        name_metadata: String,

        /// network name as it is in the network specs for genesis hash
        name_specs: String,

        /// genesis hash from the `load_metadata` payload, that was used to find
        /// the network specs and verifier information
        genesis_hash: H256,
    },

    /// Received `add_specs` or `load_metadata` update payload is not verified.
    ///
    /// Network, however, was verified previuosly by verifier with certain
    /// [`VerifierValue`] and corresponding entry in `VERIFIERS` tree of the
    /// database is
    /// `CurrentVerifier::Valid(ValidCurrentVerifier::Custom(Some(verifier_value)))`.
    ///
    /// Signer does not allow downgrading the verifiers.
    NeedVerifier {
        /// network name
        name: String,

        /// expected verifier for this network
        verifier_value: VerifierValue,
    },

    /// Received update payload is not verified, although the verification by
    /// currently used general verifier with certain [`VerifierValue`] was
    /// expected.
    ///
    /// Network has entry in `VERIFIERS` tree of the database with
    /// `CurrentVerifier::Valid(ValidCurrentVerifier::General)`.
    NeedGeneralVerifier {
        /// payload that requires general verifier
        content: GeneralVerifierForContent,

        /// [`VerifierValue`] currently associated with the general verifier,
        /// expected verifier for the data
        verifier_value: VerifierValue,
    },

    /// Received `load_metadata` update payload is signed.
    ///
    /// Network has entry in `VERIFIERS` tree of the database with
    /// `CurrentVerifier::Valid(ValidCurrentVerifier::Custom(None))`, i.e. it was
    /// never verified previously and its network specs were loaded unverified.
    ///
    /// Verified `add_specs` must be loaded before any verified `load_metadata`.
    LoadMetaSetVerifier {
        /// network name
        name: String,

        /// [`VerifierValue`] that has signed the update payload
        new_verifier_value: VerifierValue,
    },

    /// Received `load_metadata` update payload is signed by `new_verifier_value`.
    ///
    /// Network has entry in `VERIFIERS` tree of the database with
    /// `CurrentVerifier::Valid(ValidCurrentVerifier::Custom(Some(old_verifier_value)))`,
    /// but `new_verifier_value` and `old_verifier_value` are different, and
    /// `new_verifier_value` is not the general verifier.
    ///
    /// Custom verifier could be upgraded only to general one, see
    /// [here](crate::network_specs), and during that network specs must be
    /// updated prior to loading the metadata.
    LoadMetaVerifierChanged {
        /// network name
        name: String,

        /// [`VerifierValue`] for the network in the database
        old_verifier_value: VerifierValue,

        /// [`VerifierValue`] for the payload
        new_verifier_value: VerifierValue,
    },

    /// Received `load_metadata` update payload is signed by
    /// `new_general_verifier_value`.
    ///
    /// Network has entry in `VERIFIERS` tree of the database with
    /// `CurrentVerifier::Valid(ValidCurrentVerifier::General)`,
    /// and database value for general verifier is `None`, i.e. the network
    /// specs for this network are not verified.
    ///
    /// Verified `add_specs` must be loaded before any verified `load_metadata`.
    LoadMetaSetGeneralVerifier {
        /// network name
        name: String,

        /// [`VerifierValue`] that has signed the payload instead of the
        /// known general verifier
        new_general_verifier_value: VerifierValue,
    },

    /// Received `load_metadata` update payload is signed by
    /// `new_general_verifier_value`.
    ///
    /// Network has entry in `VERIFIERS` tree of the database with
    /// `CurrentVerifier::Valid(ValidCurrentVerifier::General)`,
    /// and database value for general verifier is
    /// `Some(old_general_verifier_value)`.
    ///
    /// General verifier with assigned [`VerifierValue`] could not be changed
    /// without Signer wipe. If the Signer is reset with no general verifier,
    /// and the network in question is the default one (currently Pokadot,
    /// Kusama, and Westend), the network will still be recorded as the one
    /// verified by the general verifier and accepting verified `add_specs` for
    /// it would result in setting the general verifier. If the network is not
    /// the default one and if by the time its `add_specs` are loaded the
    /// general verifier already has an associated `VerifierValue`, loading
    /// verified `add_specs` would result in the network having custom verifier.
    LoadMetaGeneralVerifierChanged {
        /// network name
        name: String,

        /// general verifier associated `VerifierValue` in the database
        old_general_verifier_value: VerifierValue,

        /// `VerifierValue` that was used to sign the update
        new_general_verifier_value: VerifierValue,
    },

    /// Received `add_specs` or `load_types` is signed by
    /// `new_general_verifier_value`.
    // TODO: maybe combine with the LoadMetaGeneralVerifierChanged,
    // modify GeneralVerifierForContent into 3 variants
    GeneralVerifierChanged {
        /// payload that requires general verifier
        content: GeneralVerifierForContent,

        /// general verifier associated `VerifierValue` in the database
        old_general_verifier_value: VerifierValue,

        /// `VerifierValue` that was used to sign the update
        new_general_verifier_value: VerifierValue,
    },

    /// Types information received in the `load_types` payload is exactly
    /// same, as the one already stored in the `SETTREE` tree of the database
    /// under the key `TYPES`.
    ///
    /// Not exactly an error, but Signer can't do anything and complains.
    TypesKnown,

    /// Text message received as a part of signable transaction with `53xx03`
    /// does not have `<Bytes></Bytes>` wrapper
    MessageNoWrapper,

    /// Text message received as a part of signable transaction with `53xx03`
    /// prelude could not be transformed into a valid `String`, because there
    /// are invalid utf8 symbols.
    MessageNotValidUtf8,

    /// Received signable transaction (with prelude `53xx00`, `53xx02` or
    /// `53xx03`) is generated in the network that has no corresponding
    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) entry in the
    /// `SPECSTREE` tree of the database.
    UnknownNetwork {
        /// network genesis hash
        genesis_hash: H256,

        /// encryption algorithm supported by the network
        encryption: Encryption,
    },

    /// Received transaction that should be parsed prior to approval (with
    /// prelude `53xx00` or `53xx02`) is generated in the network that has no
    /// metadata entries in the `METATREE` tree of the database.
    ///
    /// Without metadata the transaction could not be decoded.
    NoMetadata { name: String },

    /// [`NetworkSpecsToSend`](crate::network_specs::NetworkSpecs) from the
    /// received `add_specs` payload already have an entry in `SPECSTREE` tree
    /// of the database.
    ///
    /// Not exactly an error, but Signer can't do anything and complains.
    SpecsKnown {
        /// network name
        name: String,

        /// network [`Encryption`]
        encryption: Encryption,
    },

    /// Received `add_specs` update payload is signed by `new_verifier_value`.
    ///
    /// Network has entry in `VERIFIERS` tree of the database with
    /// `CurrentVerifier::Valid(ValidCurrentVerifier::Custom(Some(old_verifier_value)))`,
    /// but `new_verifier_value` and `old_verifier_value` are different, and
    /// `new_verifier_value` is not the general verifier.
    ///
    /// Custom verifier could be upgraded only to general one, see
    /// [here](crate::network_specs).
    AddSpecsVerifierChanged {
        /// network name
        name: String,

        /// [`VerifierValue`] for the network in the database
        old_verifier_value: VerifierValue,

        /// [`VerifierValue`] for the payload
        new_verifier_value: VerifierValue,
    },

    /// Received `derivations` update payload contains an invalid derivation.
    ///
    /// Associated data is the derivation that could not be used as a `String`.
    InvalidDerivation(String),

    /// User has tried to create new seed with already existing seed name.
    ///
    /// Note: this is only input error that is caused by the user-typed input.
    ///
    /// Associated data is the proposed seed name that is already known to Signer.
    SeedNameExists(String),
}

/// Content that should have been verified by the general verifier
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum GeneralVerifierForContent {
    /// Network data.
    /// Associated data is the network name.
    Network { name: String },

    /// Types information.
    Types,
}

/// Errors when something was needed from the Signer database and was not found
///
/// Not necessarily the database failure, could be just not updated Signer
/// as well.
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum NotFoundSigner {
    /// [`CurrentVerifier`](crate::network_specs::CurrentVerifier) for a
    /// network in `VERIFIERS` tree of the Signer database.
    ///
    /// Associated data is the [`VerifierKey`] used for the search.
    CurrentVerifier(VerifierKey),

    /// General verifier [`Verifier`](crate::network_specs::Verifier) information
    /// stored in `SETTREE` tree of the database under key `GENERALVERIFIER`.
    ///
    /// Missing general verifier always indicates the database corruption.
    GeneralVerifier,

    /// Types information stored in `SETTREE` tree of the database under key
    /// `TYPES`.
    ///
    /// Could be missing if user has deleted it.
    Types,

    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) for a network
    /// in `SPECSTREE` tree of the Signer database, searched by
    /// [`NetworkSpecsKey`].
    ///
    /// Associated data is the `NetworkSpecsKey` used for the search.
    NetworkSpecs(NetworkSpecsKey),

    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) for a network
    /// in `SPECSTREE` tree of the Signer database, searched by
    /// network name.
    ///
    /// Associated data is the network name.
    NetworkSpecsForName(String),

    /// [`NetworkSpecsKey`] of a network in `network_id` field of the
    /// [`AddressDetails`](crate::users::AddressDetails) corresponding to
    /// [`AddressKey`].
    ///
    /// This happens when the derivation is created in some other network(s), but
    /// not in the given network. This way the `AddressKey` is in the database,
    /// but the address in the network is not.
    NetworkSpecsKeyForAddress {
        /// [`NetworkSpecsKey`] of the network that is not available
        network_specs_key: NetworkSpecsKey,

        /// [`AddressKey`] for which the address in the network was expected to
        /// exist
        address_key: AddressKey,
    },

    /// [`AddressDetails`](crate::users::AddressDetails) for [`AddressKey`] in
    /// `ADDRTREE` tree of the Signer database.
    ///
    /// Associated data is the `AddressKey` used for search.
    AddressDetails(AddressKey),

    /// Network metadata in `METATREE` tree of the Signer database, for network
    /// name and version combination.
    Metadata {
        /// network name
        name: String,

        /// network version
        version: u32,
    },

    /// [`DangerRecord`](crate::danger::DangerRecord) information
    /// stored in `SETTREE` tree of the database under key `DANGER`.
    ///
    /// Missing `DangerRecord` always indicates the database corruption.
    DangerStatus,

    /// Temporary database entry in `TRANSACTION` tree of the Signer database
    /// under the key `STUB`, used to store the update data awaiting for the
    /// user approval.
    ///
    /// Missing `Stub` when it is expected always indicates the database
    /// corruption.
    Stub,

    /// Temporary database entry in `TRANSACTION` tree of the Signer database
    /// under the key `SIGN`, used to store the signable transaction data
    /// awaiting for the user approval.
    ///
    /// Missing `Sign` when it is expected always indicates the database
    /// corruption.
    Sign,

    /// Temporary database entry in `TRANSACTION` tree of the Signer database
    /// under the key `DRV`, used to store the derivation import data.
    ///
    /// Missing `Derivations` when it is expected always indicates the database
    /// corruption.
    Derivations,

    /// History log [`Entry`](crate::history::Entry) from `HISTORY` tree of the
    /// Signer database.
    ///
    /// Associated data is the [`Order`] used for search.
    HistoryEntry(Order),

    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) needed to parse
    /// historical transactions saved into history log, searched by network
    /// name and encryption.
    HistoryNetworkSpecs {
        /// network name
        name: String,

        /// network supported [`Encryption`]
        encryption: Encryption,
    },

    /// Network metadata needed to parse historical transaction, no entries at
    /// all for a given network name.
    HistoricalMetadata {
        /// network name used for the search
        name: String,
    },

    /// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) for network in
    /// which the imported derivations are user to create addresses.
    NetworkForDerivationsImport {
        /// network genesis hash
        genesis_hash: H256,

        /// network supported encryption
        encryption: Encryption,
    },
}

/// Errors in generating address, exclusive for the Signer side
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum ExtraAddressGenerationSigner {
    /// Error generating random phrase.
    ///
    /// Associated data is an error produced by `bip39`.
    RandomPhraseGeneration(anyhow::Error),

    /// Invalid derivation used for address generation.
    InvalidDerivation,
}

/// Errors in transaction parsing
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum ParserError {
    /// Can not separate method from extensions, bad transaction.
    SeparateMethodExtensions,

    /// Errors occuring during the decoding procedure.
    Decoding(ParserDecodingError),

    /// Errors occuring because the metadata
    /// [`RuntimeMetadataV14`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/v14/struct.RuntimeMetadataV14.html)
    /// has extensions not acceptable in existing safety paradigm for
    /// signable transactions.
    FundamentallyBadV14Metadata(ParserMetadataError),

    /// While parsing transaction with certain version of network metadata,
    /// found that the version found in signable extensions does not match
    /// the version of the metadata used for parsing.
    ///
    /// Transaction parsing in Signer is done by consecutively checking all
    /// available metadata for a given network name, starting with the highest
    /// available version, and looking for a matching network version in the
    /// parsed extensions.
    ///
    /// For `RuntimeMetadataV12` and `RuntimeMetadataV13` the extensions set
    /// is a fixed one, whereas for `RuntimeMetadataV14` is may vary and is
    /// determined by the metadata itself.
    WrongNetworkVersion {
        /// metadata version from transaction extensions, as found through
        /// parsing process
        as_decoded: String,

        /// metadata version actually used for parsing, from the `Version`
        /// constant in `System` pallet of the metadata
        in_metadata: u32,
    },
}

/// Errors directly related to transaction parsing
///
/// Signable transactions are differentiated based on prelude:
///
/// - `53xx00` mortal transactions
/// - `53xx02` immortal transactions
/// - `53xx03` text message transactions
///
/// `53xx00` and `53xx02` transactions contain encoded transaction data, and
/// are parsed prior to signing using the network metadata. Transaction is
/// generated in client, for certain address and within certain network.
/// To parse the transaction and to generate the signature, Signer must
/// have the network information (network specs and correct network metadata)
/// and the public address-associated information in its database.
///
/// `53xx00` and `53xx02` transcations consist of:
///
/// - prelude, `53xx00` or `53xx02`, where `xx` stands for the encryption
/// algorithm associated with address and network used
/// - public key corresponding to the address that can sign the transaction
/// - encoded call data, the body of the transaction
/// - extensions, as set in the network metadata
/// - genesis hash of the network in which the transaction was generated
///
/// Parsing process first separates the prelude, public key, genesis hash and
/// the combined call + extensions data.
///
/// The call information is SCALE-encoded into `Vec<u8>` bytes and then those
/// bytes are SCALE-encoded again, so that the call data contained in the
/// transaction consists of `compact` with encoded call length in bytes
/// followed by the `Vec<u8>` with the encoded data.
///
/// Call and extensions are cut based on the call length declared at the start
/// of the combined call + extensions data.
///
/// Then the extensions are decoded, and it is checked that the metadata version
/// in extensions coincides with the metadata version used for the decoding.
///
/// Decoding the extensions for metadata with `RuntimeMetadataV12` or
/// `RuntimeMetadataV13` is using a static set of extensions, namely:
///
/// - [`Era`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/generic/enum.Era.html)
/// - nonce, compact `u64`
/// - transaction tip, compact `u128`
/// - metadata version, `u32`
/// - tx version, `u32`
/// - network genesis hash, `[u8; 32]`
/// - block hash, `[u8; 32]`
///
/// Decoding the extensions for metadata with `RuntimeMetadataV14` uses
/// dynamically acquired set of extensions from the metadata itself.
///
/// After the extensions, the call data itself is decoded using the network
/// metadata. Each call first byte is the index of the pallet.
///
/// Metadata with `RuntimeMetadataV12` or `RuntimeMetadataV13` has only type
/// names associated with call arguments. Signer finds what the types really
/// are and how to decode them by using the types information that must be in
/// Signer database.
/// For `RuntimeMetadataV12` or `RuntimeMetadataV13` the second byte in call is
/// the index of the method within the pallet, and thes Signer finds the types
/// used by the method and proceeds to decode the call data piece by piece.
///
/// Metadata with `RuntimeMetadataV14` has types data in-built in the metadata
/// itself, and the types needed to decode the call are resolved during the
/// decoding. For `RuntimeMetadataV14` the second byte in call is also
/// the index of the method within the pallet, but this already goes into the
/// type resolver.
///
/// Calls may contain nested calls, for `RuntimeMetadataV12` or
/// `RuntimeMetadataV13` metadata the call decoding always starts with pallet
/// and method combination processing. For `RuntimeMetadataV14` metadata the
/// nested calls are processed through the type resolver, i.e. the pallet index
/// is processed independently only on the start of the decoding.
///
/// `53xx03` transaction consists of:
///
/// - prelude `53xx03`, where `xx` stands for the encryption algorithm
/// associated with address and network used
/// - public key corresponding to the address that can sign the transaction
/// - SCALE-encoded `String` contents of the message
/// - genesis hash of the network in which the transaction was generated
///
/// Signer assumes that every byte of the transaction will be processed, and
/// shows an error if this is not the case.
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum ParserDecodingError {
    /// Transaction was announced by the prelude to be mortal (`53xx00`),
    /// but has `Era::Immortal` in extensions
    UnexpectedImmortality,

    /// Transaction was announced by the prelude to be immortal (`53xx02`),
    /// but has `Era::Mortal(_, _)` in extensions
    UnexpectedMortality,

    /// Genesis hash cut from the end of the transaction doen not match the one
    /// found in the extensions
    GenesisHashMismatch,

    /// In immortal transaction the block hash from the extensions is the
    /// network genesis hash.
    ///
    /// This error happens when block hash is different with the genesis hash
    /// cut from the end of the transaction.
    ImmortalHashMismatch,

    /// Error decoding the extensions using metadata with `RuntimeMetadataV12`
    /// or `RuntimeMetadataV13`, with default extensions set.
    ExtensionsOlder,

    /// Used only for `RuntimeMetadataV12` or `RuntimeMetadataV13`,
    /// indicates that method index (second byte of the call data) is not valid
    /// for the pallet with found name.
    MethodNotFound {
        /// index of the method, second byte of the call data
        method_index: u8,

        /// name of the pallet, found from the first byte of the call data
        pallet_name: String,
    },

    /// Used only for all calls in `RuntimeMetadataV12` or `RuntimeMetadataV13`,
    /// and for entry call in `RuntimeMetadataV14` metadata. First byte of the
    /// call data is not a valid pallet index.
    ///
    /// Associated data is what was thought to be a pallet index.
    PalletNotFound(u8),

    /// Only for entry call in `RuntimeMetadataV14`. Pallet found via first byte
    /// of the call has no associated calls.
    ///
    /// Associated data is the pallet name.
    NoCallsInPallet(String),

    /// Only for `RuntimeMetadataV14`. Found type index could not be resolved
    /// in types registry
    V14TypeNotResolved,

    /// Only for `RuntimeMetadataV12` and `RuntimeMetadataV13`. Argument type
    /// could not be taken out of `DecodeDifferent` construction.
    ArgumentTypeError,

    /// Only for `RuntimeMetadataV12` and `RuntimeMetadataV13`. Argument name
    /// could not be taken out of `DecodeDifferent` construction.
    ArgumentNameError,

    /// Parser was trying to find an encoded
    /// [`compact`](https://docs.rs/parity-scale-codec/latest/parity_scale_codec/struct.Compact.html),
    /// in the bytes sequence, but was unable to.
    NoCompact,

    /// Parser was expecting more data.
    DataTooShort,

    /// Parser was unable to decode the data piece into a primitive type.
    ///
    /// Associated data is primitive identifier.
    PrimitiveFailure(String),

    /// SCALE-encoded `Option<_>` can have as a first byte:
    ///
    /// - `0` if the value is `None`
    /// - `1` if the value is `Some`
    /// - `2` if the value is `Some(false)` for `Option<bool>` encoding
    ///
    /// This error appears if the parser encounters something unexpected in the
    /// first byte of encoded `Option<_>` instead.
    UnexpectedOptionVariant,

    /// Only for `RuntimeMetadataV12` and `RuntimeMetadataV13`.
    /// Decoding
    /// [`IdentityFields`](https://docs.substrate.io/rustdocs/latest/pallet_identity/struct.IdentityFields.html)
    /// requires having correct type information for
    /// [`IdentityField`](https://docs.substrate.io/rustdocs/latest/pallet_identity/enum.IdentityField.html)
    /// in types information. If types information has no entry for
    /// `IdentityFields` or it is not an enum, this error appears.
    IdFields,

    /// Parser processes certain types as balance (i.e. transforms the data
    /// into appropriate float using decimals and units provided).
    /// For some types the balance representation is not possible, this error
    /// occurs if the parser tried to process as a balance some type not
    /// suitable for it.
    BalanceNotDescribed,

    /// SCALE-encoded enum can have as a first byte only correct index of the
    /// variant used.
    ///
    /// This error appears if the first byte is an invalid variant index.
    UnexpectedEnumVariant,

    /// Parser found that type declared as a
    /// [`compact`](https://docs.rs/parity-scale-codec/latest/parity_scale_codec/struct.Compact.html)
    /// has inner type that could not be encoded as a `compact`
    UnexpectedCompactInsides,

    /// Only for `RuntimeMetadataV12` and `RuntimeMetadataV13`.
    /// Parser has encountered a type that could not be interpreted using the
    /// existing types information.
    ///
    /// Associated data is the type description as it was received by parser
    /// from the metadata.
    UnknownType(String),

    /// Only for `RuntimeMetadataV14`.
    /// While decoding
    /// [`BitVec<T,O>`](https://docs.rs/bitvec/1.0.0/bitvec/vec/struct.BitVec.html),
    /// parser encountered `T` type not implementing
    /// [`BitStore`](https://docs.rs/bitvec/1.0.0/bitvec/store/trait.BitStore.html).
    NotBitStoreType,

    /// Only for `RuntimeMetadataV14`.
    /// While decoding
    /// [`BitVec<T,O>`](https://docs.rs/bitvec/1.0.0/bitvec/vec/struct.BitVec.html),
    /// parser encountered `O` type not implementing
    /// [`BitOrder`](https://docs.rs/bitvec/1.0.0/bitvec/order/trait.BitOrder.html).
    NotBitOrderType,

    /// Only for `RuntimeMetadataV14`.
    /// Parser failed to decode
    /// [`BitVec<T,O>`](https://docs.rs/bitvec/1.0.0/bitvec/vec/struct.BitVec.html),
    /// even though `T` and `O` types were suitable.
    BitVecFailure,

    /// Only for `RuntimeMetadataV14`.
    /// Parser failed to decode data slice as
    /// [`Era`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/generic/enum.Era.html).
    Era,

    /// Parser expects to use all data in decoding. This error appears if some
    /// data was not used in parsing of the method.
    SomeDataNotUsedMethod,

    /// Only for `RuntimeMetadataV14`.
    /// Parser expects to use all data in decoding. This error appears if some
    /// data from extensions is not used in the decoding.
    SomeDataNotUsedExtensions,
}

/// Errors occuring because the network metadata
/// [`RuntimeMetadataV14`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/v14/struct.RuntimeMetadataV14.html)
/// has extensions not compatible with Signer.
///
/// To be compatible with signable transactions, metadata extensions must
/// include:
///
/// - `Era` (once)
/// - block hash (once)
/// - metadata version (once)
/// - network genesis hash (at most, once)
///
/// If the metadata does not follow those criteria, transactons could not be
/// parsed, and therefore, could not be signed.
#[derive(Debug)]
#[cfg_attr(feature = "test", derive(VariantCount))]
pub enum ParserMetadataError {
    /// Metadata extensions have no `Era`
    NoEra,

    /// Metadata extensions have no block hash
    NoBlockHash,

    /// Metadata extensions have no network metadata version
    NoVersionExt,

    /// Metadata extensions have more than one `Era`
    EraTwice,

    /// Metadata extensions have more than one genesis hash
    GenesisHashTwice,

    /// Metadata extensions have more than one block hash
    BlockHashTwice,

    /// Metadata extensions have more than one network metadata version
    SpecVersionTwice,
}

impl ParserError {
    /// Printing [`ParserError`] in readable format.
    pub fn show(&self) -> String {
        match &self {
            ParserError::SeparateMethodExtensions => String::from("Unable to separate transaction method and extensions."),
            ParserError::Decoding(x) => {
                match x {
                    ParserDecodingError::UnexpectedImmortality => String::from("Expected mortal transaction due to prelude format. Found immortal transaction."),
                    ParserDecodingError::UnexpectedMortality => String::from("Expected immortal transaction due to prelude format. Found mortal transaction."),
                    ParserDecodingError::GenesisHashMismatch => String::from("Genesis hash values from decoded extensions and from used network specs do not match."),
                    ParserDecodingError::ImmortalHashMismatch => String::from("Block hash for immortal transaction not matching genesis hash for the network."),
                    ParserDecodingError::ExtensionsOlder => String::from("Unable to decode extensions for V12/V13 metadata using standard extensions set."),
                    ParserDecodingError::MethodNotFound{method_index, pallet_name} => format!("Method number {} not found in pallet {}.", method_index, pallet_name),
                    ParserDecodingError::PalletNotFound(i) => format!("Pallet with index {} not found.", i),
                    ParserDecodingError::NoCallsInPallet(x) => format!("No calls found in pallet {}.", x),
                    ParserDecodingError::V14TypeNotResolved => String::from("Referenced type could not be resolved in v14 metadata."),
                    ParserDecodingError::ArgumentTypeError => String::from("Argument type error."),
                    ParserDecodingError::ArgumentNameError => String::from("Argument name error."),
                    ParserDecodingError::NoCompact => String::from("Expected compact. Not found it."),
                    ParserDecodingError::DataTooShort => String::from("Data too short for expected content."),
                    ParserDecodingError::PrimitiveFailure(x) => format!("Unable to decode part of data as {}.", x),
                    ParserDecodingError::UnexpectedOptionVariant => String::from("Encountered unexpected Option<_> variant."),
                    ParserDecodingError::IdFields => String::from("IdentityField description error."),
                    ParserDecodingError::BalanceNotDescribed => String::from("Unexpected type encountered for Balance"),
                    ParserDecodingError::UnexpectedEnumVariant => String::from("Encountered unexpected enum variant."),
                    ParserDecodingError::UnexpectedCompactInsides => String::from("Unexpected type inside compact."),
                    ParserDecodingError::UnknownType(x) => format!("No description found for type {}.", x),
                    ParserDecodingError::NotBitStoreType => String::from("Declared type is not suitable BitStore type for BitVec."),
                    ParserDecodingError::NotBitOrderType => String::from("Declared type is not suitable BitOrder type for BitVec."),
                    ParserDecodingError::BitVecFailure => String::from("Could not decode BitVec."),
                    ParserDecodingError::Era => String::from("Could not decode Era."),
                    ParserDecodingError::SomeDataNotUsedMethod => String::from("After decoding the method some data remained unused."),
                    ParserDecodingError::SomeDataNotUsedExtensions => String::from("After decoding the extensions some data remained unused."),
                }
            },
            ParserError::FundamentallyBadV14Metadata(x) => {
                let insert = match x {
                    ParserMetadataError::NoEra => String::from("Era information is missing."),
                    ParserMetadataError::NoBlockHash => String::from("Block hash information is missing."),
                    ParserMetadataError::NoVersionExt => String::from("Metadata spec version information is missing."),
                    ParserMetadataError::EraTwice => String::from("Era information is encountered mora than once."),
                    ParserMetadataError::GenesisHashTwice => String::from("Genesis hash is encountered more than once."),
                    ParserMetadataError::BlockHashTwice => String::from("Block hash is encountered more than once."),
                    ParserMetadataError::SpecVersionTwice => String::from("Metadata spec version is encountered more than once."),
                };
                format!("Metadata signed extensions are not compatible with Signer (v14 metadata). {}", insert)
            },
            ParserError::WrongNetworkVersion{as_decoded, in_metadata} => format!("Network spec version decoded from extensions ({}) differs from the version in metadata ({}).", as_decoded, in_metadata),
        }
    }
}
