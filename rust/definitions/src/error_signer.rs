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
//! This module gathers all possible errors in one place, so that error
//! management is easier.
use anyhow::anyhow;
use sp_core::crypto::SecretStringError;

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
    fn specs_genesis_hash_mismatch(key: NetworkSpecsKey, genesis_hash: Vec<u8>) -> Self::Error {
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
                    DatabaseSigner::UnexpectedGenesisHash{verifier_key, network_specs_key} => format!("No verifier information found for network with genesis hash {}, however genesis hash is encountered in network specs entry with key {}.", hex::encode(verifier_key.genesis_hash()), hex::encode(network_specs_key.key())),
                    DatabaseSigner::SpecsCollision{name, encryption} => format!("More than one entry for network specs with name {} and encryption {}.", name, encryption.show()),
                    DatabaseSigner::DifferentNamesSameGenesisHash{name1, name2, genesis_hash} => format!("Different network names ({}, {}) in database for same genesis hash {}.", name1, name2, hex::encode(genesis_hash)),
                    DatabaseSigner::TwoTransactionsInEntry(x) => format!("Entry with order {} contains more than one transaction-related event. This should not be possible in current Signer and likely indicates database corruption.", x),
                    DatabaseSigner::CustomVerifierIsGeneral(key) => format!("Network with genesis hash {} verifier is set as a custom one. This custom verifier coinsides the database general verifier and not None. This should not have happened and likely indicates database corruption.", hex::encode(key.genesis_hash())),
                    DatabaseSigner::TwoRootKeys{seed_name, encryption} => format!("More than one root key (i.e. with empty path and without password) found for seed name {} and encryption {}.", seed_name, encryption.show()),
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
                    InputSigner::DifferentBase58{genesis_hash, base58_database, base58_input} => format!("Network with genesis hash {} already has entries in the database with base58 prefix {}. Received network specs have different base 58 prefix {}.", hex::encode(genesis_hash), base58_database, base58_input),
                    InputSigner::EncryptionNotSupported(code) => format!("Payload with encryption 0x{} is not supported.", code),
                    InputSigner::BadSignature => String::from("Received payload has bad signature."),
                    InputSigner::LoadMetaUnknownNetwork{name} => format!("Network {} is not in the database. Add network specs before loading the metadata.", name),
                    InputSigner::LoadMetaNoSpecs{name, valid_current_verifier, general_verifier} => {
                        let insert = match valid_current_verifier {
                            ValidCurrentVerifier::General => format!("{} (general verifier)", general_verifier.show_error()),
                            ValidCurrentVerifier::Custom(a) => format!("{} (custom verifier)", a.show_error()),
                        };
                        format!("Network {} was previously known to the database with verifier {}. However, no network specs are in the database at the moment. Add network specs before loading the metadata.", name, insert)
                    },
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
                    InputSigner::MessageNotReadable => String::from("Received message could not be read."),
                    InputSigner::UnknownNetwork{genesis_hash, encryption} => format!("Input generated within unknown network and could not be processed. Add network with genesis hash {} and encryption {}.", hex::encode(genesis_hash), encryption.show()),
                    InputSigner::NoMetadata{name} => format!("Input transaction is generated in network {}. Currently there are no metadata entries for it, and transaction could not be processed. Add network metadata.", name),
                    InputSigner::SpecsKnown{name, encryption} => format!("Exactly same network specs for network {} with encryption {} are already in the database.", name, encryption.show()),
                    InputSigner::AddSpecsVerifierChanged {name, old_verifier_value, new_verifier_value} => format!("Network {} current verifier is {}. Received add_specs message is verified by {}, which is neither current network verifier not the general verifier. Changing the network verifier to another non-general one would require wipe and reset of Signer.", name, old_verifier_value.show_error(), new_verifier_value.show_error()),
                    InputSigner::InvalidDerivation(x) => format!("Derivation {} has invalid format.", x),
                    InputSigner::OnlyNoPwdDerivations => String::from("Only derivations without passwords are allowed in bulk import."),
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
                    NotFoundSigner::TransactionEvent(x) => format!("Entry with order {} contains no transaction-related events.", x),
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
                    insert.push_str(&format!("Parsing with {}{} metadata: {}", network_name, version, parser_error.show()));
                }
                format!("Failed to decode extensions. Please try updating metadata for {} network. {}", network_name, insert)
            },
            ErrorSigner::AddressUse(e) => format!("Error with secret string of existing address: {}.", bad_secret_string(e)),
            ErrorSigner::WrongPassword => String::from("Wrong password."),
            ErrorSigner::WrongPasswordNewChecksum(_) => String::from("Wrong password."),
            ErrorSigner::NoNetworksAvailable => String::from("No networks available. Please load networks information to proceed."),
        }
    }
}

/// All possible errors that could occur on the Signer side
#[derive(Debug)]
pub enum ErrorSigner {
    /// Communication errors on the interface between native frontend and rust
    /// backend
    Interface(InterfaceSigner),

    /// Errors within Signer rust-managed database
    Database(DatabaseSigner),

    /// Errors in received input: either signable transaction or update
    Input(InputSigner),

    /// Something was expected to be known to Signer, but was not found
    NotFound(NotFoundSigner),

    /// User tried to interact with previously disabled network
    DeadVerifier(VerifierKey),

    /// Errors with address generation
    AddressGeneration(AddressGeneration<Signer>),

    /// Errors with static QR codes generation
    ///
    /// Signer can produce QR codes with signatures for transactions, with
    /// [`SufficientCrypto`](crate::crypto::SufficientCrypto) exports for
    /// user-verified updates, and with address public information exports
    /// for Signer companion.
    Qr(String),

    /// Errors parsing a signable transactions with a given version of the
    /// metadata for given network
    Parser(ParserError),

    /// Error parsing extensions of a signable transaction with all available
    /// versions of metadata for given network
    AllExtensionsParsingFailed {
        network_name: String,
        errors: Vec<(u32, ParserError)>,
    },

    /// Error with using address already stored in the database
    AddressUse(SecretStringError),

    /// User has entered a wrong password for a passworded address
    ///
    /// For cases when Signer database checksum is not verified.
    /// Signer log records that password was entered incorrectly.
    WrongPassword,

    /// User has entered a wrong password for a passworded address for cases
    /// when the Signer database checksum is verified
    ///
    /// Signer log records that password was entered incorrectly.
    /// This changes the database checksum, and for the next attempt it must be
    /// updated.
    WrongPasswordNewChecksum(u32),

    /// Signer has attempted an operation that requires at least one network to
    /// be loaded into Signer
    NoNetworksAvailable,
}

impl ErrorSigner {
    /// Signer side errors could be exported into native interface, before that
    /// they must be transformed into anyhow errors
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
/// Signer rust backend sends data into frontend as `json` strings. Frontend
/// parses these `json` strings, displays them, and can send back into rust
/// some parts of the data that the user has, for example, selected.
///
/// Data that user can type from keyboard is processed as is, it does not
/// cause errors on the interface.
#[derive(Debug)]
pub enum InterfaceSigner {
    /// Received string is not hexadecimal, and could not be transformed into
    /// `Vec<u8>`
    NotHex(NotHexSigner),

    /// Received database key could not be decoded
    KeyDecoding(KeyDecodingSignerInterface),

    /// Received public key length is different from the one expected for
    /// given encryption algorithm
    PublicKeyLength,

    /// Requested history page number exceeds the total number of pages
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
        address_key: AddressKey,
        expected_seed_name: String,
        real_seed_name: String,
    },

    /// User was creating the derivation with password, and thus moved into
    /// `PasswordConfirm` modal, however, the password was not found when
    /// cutting password from the path
    LostPwd,

    /// Received from interface network metadata version could not be parsed
    /// as `u32`
    ///
    /// Associated content is the received data as a string
    VersionNotU32(String),

    /// Received from interface increment for address generation could not
    /// be parsed as `u32`
    ///
    /// Associated content is the received data as a string
    IncNotU32(String),

    /// Received from interface history log order could not be parsed as `u32`
    ///
    /// Associated content is the received data as a string
    OrderNotU32(String),

    /// Received from interface boolean flag could not be parsed as `bool`
    ///
    /// Associated content is the received data as a string
    FlagNotBool(String),
}

/// `NotHex` errors occuring on the Signer side
///
/// Expected to receive hexadecimal string from the interface, got something
/// different. [`NotHexSigner`] specifies, what was expected.
#[derive(Debug)]
pub enum NotHexSigner {
    /// [`NetworkSpecsKey`] is not hexadecimal, associated data is input string
    /// as it is received
    NetworkSpecsKey { input: String },

    /// Received signable transaction or update are not hexadecimal
    InputContent,

    /// [`AddressKey`] is not hexadecimal, associated data is input string as
    /// it is received
    AddressKey { input: String },
}

/// Source of damaged [`NetworkSpecsKey`], exclusive for the Signer side
#[derive(Debug)]
pub enum ExtraSpecsKeySourceSigner {
    /// Damaged [`NetworkSpecsKey`] is from the interface
    Interface,
}

/// Source of damaged [`AddressKey`], exclusive for the Signer side
#[derive(Debug)]
pub enum ExtraAddressKeySourceSigner {
    /// Damaged [`AddressKey`] is from the interface
    Interface,
}

/// Source of damaged metadata, exclusive for the Signer side
#[derive(Debug)]
pub enum IncomingMetadataSourceSigner {
    /// Damaged metadata is received through QR code update
    ReceivedData,
}

/// Errors decoding database keys received from the frontend on the Signer
/// side
#[derive(Debug)]
pub enum KeyDecodingSignerInterface {
    /// [`AddressKey`] received from the frontend was a valid hexadecimal, but
    /// turned out to be a damaged database key with invalid content.
    ///
    /// This error indicates that [`AddressKey`] content could not be processed
    /// to get an associated
    /// [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)
    /// value.
    AddressKey(AddressKey),

    /// [`NetworkSpecsKey`] received from the frontend was a valid hexadecimal, but
    /// turned out to be a damaged database key with invalid content.
    ///
    /// This error indicates that [`NetworkSpecsKey`] content could not be processed
    /// to get an associated [`Encryption`] and network genesis hash.
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
pub enum DatabaseSigner {
    /// Key used in one of the database trees has invalid content, and could
    /// not be decoded
    KeyDecoding(KeyDecodingSignerDb),

    /// Database [`Error`](https://docs.rs/sled/0.34.6/sled/enum.Error.html)
    ///
    /// Could happen, for example, when opening the database, loading trees,
    /// reading values etc.
    Internal(sled::Error),

    /// Database
    /// [`TransactionError`](https://docs.rs/sled/0.34.6/sled/transaction/enum.TransactionError.html)
    ///
    /// Could happen when making transactions in multiple trees simultaneously.
    Transaction(sled::transaction::TransactionError),

    /// Database checksum does not match the expected value
    ChecksumMismatch,

    /// Value found in one of the database trees has invalid content, and could
    /// not be decoded
    EntryDecoding(EntryDecodingSigner),

    /// Data retrieved from the database contains some internal contradictions,
    /// could not have been written in the database this way, and is therefore
    /// likely indicating the database corruption.
    Mismatch(MismatchSigner),

    /// Network metadata that already is in the database, is damaged.
    ///
    /// Unsuitable metadata could not be put in the database in the first place,
    /// finding one would mean the database got corrupted
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
    /// remain, so this indicates the database got corrupted
    UnexpectedGenesisHash {
        verifier_key: VerifierKey,
        network_specs_key: NetworkSpecsKey,
    },

    /// More than one entry found for network specs with given `name` and
    /// `encryption`, when trying to parse transaction from historical record.
    // TODO: goes obsolete if we add `genesis_hash` field to `SignDisplay`
    SpecsCollision {
        name: String,
        encryption: Encryption,
    },

    /// While searching for all networks with same genesis hash, found that
    /// there are networks with same genesis hash, but different names.
    // TODO: is this really an error? if it is, add error to data loading too.
    DifferentNamesSameGenesisHash {
        name1: String,
        name2: String,
        genesis_hash: Vec<u8>,
    },

    /// History log [`Entry`](crate::history::Entry) contains two events with
    /// a signable transactions
    ///
    /// This is error and indication of the database corruption, because only
    /// one at a time signable transaction [`Event`](crate::history::Event)
    /// could be added.
    TwoTransactionsInEntry(u32),

    /// Network [`CurrentVerifier`](crate::network_specs::CurrentVerifier) is
    /// `ValidCurrentVerifier::Custom(_)`, but the custom verifier value
    /// coincides with the general verifier.
    CustomVerifierIsGeneral(VerifierKey),

    /// Database has two root addresses (i.e. with empty derivation path and no
    /// password) for same seed name and [`Encryption`]
    ///
    /// This indicates the database corruption, since the encrypion method,
    /// seed name and derivation path strictly determine the public key.
    TwoRootKeys {
        seed_name: String,
        encryption: Encryption,
    },

    /// Network specs entries have same genesis hash, but different base58 prefix
    DifferentBase58Specs {
        genesis_hash: [u8; 32],
        base58_1: u16,
        base58_2: u16,
    },
}

/// Enum listing possible errors in decoding keys from the database on the Signer side
#[derive(Debug)]
pub enum KeyDecodingSignerDb {
    AddressKey(AddressKey),
    EntryOrder(Vec<u8>),
    MetaKey(MetaKey),
    NetworkSpecsKey(NetworkSpecsKey),
    NetworkSpecsKeyAddressDetails {
        address_key: AddressKey,
        network_specs_key: NetworkSpecsKey,
    },
}

/// Enum listing possible errors in decoding database entry content on the Signer side
#[derive(Debug)]
pub enum EntryDecodingSigner {
    AddressDetails(AddressKey),
    CurrentVerifier(VerifierKey),
    DangerStatus,
    Derivations,
    GeneralVerifier,
    HistoryEntry(Order),
    NetworkSpecs(NetworkSpecsKey),
    Sign,
    Stub,
    Types,
}

#[derive(Debug)]
/// Enum listing possible mismatch within database on the Signer side
pub enum MismatchSigner {
    Metadata {
        name_key: String,
        version_key: u32,
        name_inside: String,
        version_inside: u32,
    },
    SpecsGenesisHash {
        key: NetworkSpecsKey,
        genesis_hash: Vec<u8>,
    },
    SpecsEncryption {
        key: NetworkSpecsKey,
        encryption: Encryption,
    },
    AddressDetailsEncryption {
        key: AddressKey,
        encryption: Encryption,
    },
    AddressDetailsSpecsEncryption {
        address_key: AddressKey,
        network_specs_key: NetworkSpecsKey,
    },
}

/// Enum listing errors with input received by the Signer
#[derive(Debug)]
pub enum InputSigner {
    TransferContent(TransferContent),
    TransferDerivations,
    FaultyMetadata(MetadataError),
    TooShort,
    NotSubstrate(String),
    PayloadNotSupported(String),
    SameNameVersionDifferentMeta {
        name: String,
        version: u32,
    },
    MetadataKnown {
        name: String,
        version: u32,
    },
    ImportantSpecsChanged(NetworkSpecsKey),
    DifferentBase58 {
        genesis_hash: [u8; 32],
        base58_database: u16,
        base58_input: u16,
    },
    EncryptionNotSupported(String),
    BadSignature,
    LoadMetaUnknownNetwork {
        name: String,
    },
    LoadMetaNoSpecs {
        name: String,
        valid_current_verifier: ValidCurrentVerifier,
        general_verifier: Verifier,
    },
    NeedVerifier {
        name: String,
        verifier_value: VerifierValue,
    },
    NeedGeneralVerifier {
        content: GeneralVerifierForContent,
        verifier_value: VerifierValue,
    },
    LoadMetaSetVerifier {
        name: String,
        new_verifier_value: VerifierValue,
    },
    LoadMetaVerifierChanged {
        name: String,
        old_verifier_value: VerifierValue,
        new_verifier_value: VerifierValue,
    },
    LoadMetaSetGeneralVerifier {
        name: String,
        new_general_verifier_value: VerifierValue,
    },
    LoadMetaGeneralVerifierChanged {
        name: String,
        old_general_verifier_value: VerifierValue,
        new_general_verifier_value: VerifierValue,
    },
    GeneralVerifierChanged {
        content: GeneralVerifierForContent,
        old_general_verifier_value: VerifierValue,
        new_general_verifier_value: VerifierValue,
    },
    TypesKnown,
    MessageNotReadable,
    UnknownNetwork {
        genesis_hash: Vec<u8>,
        encryption: Encryption,
    },
    NoMetadata {
        name: String,
    },
    SpecsKnown {
        name: String,
        encryption: Encryption,
    },
    AddSpecsVerifierChanged {
        name: String,
        old_verifier_value: VerifierValue,
        new_verifier_value: VerifierValue,
    },
    InvalidDerivation(String),
    OnlyNoPwdDerivations,
    SeedNameExists(String),
}

#[derive(Debug)]
pub enum GeneralVerifierForContent {
    Network { name: String },
    Types,
}

/// Enum listing errors for cases when something was needed from the Signer database
/// and was not found.
/// Not necessarily the database failure, could be just not updated Signer as well
#[derive(Debug)]
pub enum NotFoundSigner {
    CurrentVerifier(VerifierKey),
    GeneralVerifier,
    Types,
    NetworkSpecs(NetworkSpecsKey),
    NetworkSpecsForName(String),
    NetworkSpecsKeyForAddress {
        network_specs_key: NetworkSpecsKey,
        address_key: AddressKey,
    },
    AddressDetails(AddressKey),
    Metadata {
        name: String,
        version: u32,
    },
    DangerStatus,
    Stub,
    Sign,
    Derivations,
    HistoryEntry(Order),
    HistoryNetworkSpecs {
        name: String,
        encryption: Encryption,
    },
    TransactionEvent(u32),
    HistoricalMetadata {
        name: String,
    },
    NetworkForDerivationsImport {
        genesis_hash: [u8; 32],
        encryption: Encryption,
    },
}

/// Enum listing errors that can happen when generating address only on the Signer side
#[derive(Debug)]
pub enum ExtraAddressGenerationSigner {
    RandomPhraseGeneration(anyhow::Error),
    InvalidDerivation,
}

/// Enum listing errors that occur during the transaction parsing
#[derive(Debug)]
pub enum ParserError {
    SeparateMethodExtensions, // can not separate method from extensions, bad transaction
    Decoding(ParserDecodingError), // errors occuring during the decoding procedure
    FundamentallyBadV14Metadata(ParserMetadataError), // errors occuring because the metadata is legit, but not acceptable in existing safety paradigm, for example, in V14 has no mention of network spec version in extrinsics
    WrongNetworkVersion {
        as_decoded: String,
        in_metadata: u32,
    },
}

/// Enum listing errors directly related to transaction parsing
#[derive(Debug)]
pub enum ParserDecodingError {
    UnexpectedImmortality,
    UnexpectedMortality,
    GenesisHashMismatch,
    ImmortalHashMismatch,
    ExtensionsOlder,
    MethodNotFound {
        method_index: u8,
        pallet_name: String,
    },
    PalletNotFound(u8),
    MethodIndexTooHigh {
        method_index: u8,
        pallet_index: u8,
        total: usize,
    },
    NoCallsInPallet(String),
    V14TypeNotResolved,
    ArgumentTypeError,
    ArgumentNameError,
    NotPrimitive(String),
    NoCompact,
    DataTooShort,
    PrimitiveFailure(String),
    UnexpectedOptionVariant,
    IdFields,
    Array,
    BalanceNotDescribed,
    UnexpectedEnumVariant,
    UnexpectedCompactInsides,
    CompactNotPrimitive,
    UnknownType(String),
    NotBitStoreType,
    NotBitOrderType,
    BitVecFailure,
    Era,
    SomeDataNotUsedMethod,
    SomeDataNotUsedExtensions,
}

/// Not every V14 metadata are suitable for transaction parsing.
/// At the very least the extensions must include Era (once), BlockHash (once),
/// Version (once) and at most once the network genesis hash.
/// If the metadata does not follow those criteria, transactons could not be read,
/// and therefore, could not be signed.
#[derive(Debug)]
pub enum ParserMetadataError {
    NoEra,
    NoBlockHash,
    NoVersionExt,
    EraTwice,
    GenesisHashTwice,
    BlockHashTwice,
    SpecVersionTwice,
}

impl ParserError {
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
                    ParserDecodingError::MethodIndexTooHigh{method_index, pallet_index, total} => format!("Method number {} too high for pallet number {}. Only {} indices available.", method_index, pallet_index, total),
                    ParserDecodingError::NoCallsInPallet(x) => format!("No calls found in pallet {}.", x),
                    ParserDecodingError::V14TypeNotResolved => String::from("Referenced type could not be resolved in v14 metadata."),
                    ParserDecodingError::ArgumentTypeError => String::from("Argument type error."),
                    ParserDecodingError::ArgumentNameError => String::from("Argument name error."),
                    ParserDecodingError::NotPrimitive(x) => format!("Expected primitive type. Found {}.", x),
                    ParserDecodingError::NoCompact => String::from("Expected compact. Not found it."),
                    ParserDecodingError::DataTooShort => String::from("Data too short for expected content."),
                    ParserDecodingError::PrimitiveFailure(x) => format!("Unable to decode part of data as {}.", x),
                    ParserDecodingError::UnexpectedOptionVariant => String::from("Encountered unexpected Option<_> variant."),
                    ParserDecodingError::IdFields => String::from("IdentityField description error."),
                    ParserDecodingError::Array => String::from("Unable to decode part of data as an array."),
                    ParserDecodingError::BalanceNotDescribed => String::from("Unexpected type encountered for Balance"),
                    ParserDecodingError::UnexpectedEnumVariant => String::from("Encountered unexpected enum variant."),
                    ParserDecodingError::UnexpectedCompactInsides => String::from("Unexpected type inside compact."),
                    ParserDecodingError::CompactNotPrimitive => String::from("Type claimed inside compact is not compactable."),
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
