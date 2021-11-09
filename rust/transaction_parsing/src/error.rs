use sled;
use definitions::{keyring::{NetworkSpecsKey, VerifierKey}, network_specs::Verifier};
use parser::error::ParserError;

#[derive(PartialEq)]
pub enum Error {
    AllParsingFailed(Vec<(String, u32, ParserError)>),
    Parser(ParserError),
    BadInputData(BadInputData),
    DatabaseError(DatabaseError),
    CryptoError(CryptoError),
}

#[derive(PartialEq)]
pub enum BadInputData {
    TooShort,
    NotSubstrate,
    NotHex,
    CryptoNotSupported,
    WrongPayloadType,
    LoadMetaUnknownNetwork(String),
    NotMeta,
    MetaVersionBelow12,
    MetaAlreadyThere,
    MetaTotalMismatch,
    VersionNotDecodeable,
    NoMetaVersion,
    UnableToDecodeMeta,
    SpecsAlreadyThere,
    UnableToDecodeTypes,
    TypesAlreadyThere,
    UnableToDecodeAddSpecsMessage,
    UnableToDecodeLoadMetadataMessage,
    ImportantSpecsChanged,
    MessageNotReadable,
}

#[derive(PartialEq)]
pub enum DatabaseError {
    Internal(sled::Error),
    DamagedChainSpecs,
    NoNetwork,
    DamagedAddressDetails,
    DamagedTypesDatabase,
    NoTypes,
    DamagedVersName,
    NoMetaAtAll,
    DamagedGeneralVerifier,
    NoGeneralVerifier,
    DamagedNetworkVerifier,
    NetworkSpecsKeyMismatch (NetworkSpecsKey),
    UnexpectedlyMetGenesisHash (Vec<u8>),
    DifferentNamesSameGenesisHash (Vec<u8>),
    Temporary(String),
    DeadVerifier(String),
    CustomVerifierIsGeneral(VerifierKey),
    NotMeta,
    MetaVersionBelow12,
    MetaMismatch,
    NoVersion,
    VersionNotDecodeable,
    UnableToDecodeMeta,
    RuntimeVersionIncompatible,
}

#[derive(PartialEq)]
pub enum CryptoError {
    BadSignature,
    AddSpecsVerifierChanged {network_name: String, old: Verifier, new: Verifier},
    VerifierDisappeared,
    GeneralVerifierChanged {old: Verifier, new: Verifier},
    GeneralVerifierDisappeared,
    LoadMetaUpdVerifier{network_name: String, new_verifier: Verifier},
    LoadMetaVerifierChanged{network_name: String, old: Verifier, new: Verifier},
    LoadMetaUpdGeneralVerifier{network_name: String, new_verifier: Verifier},
    LoadMetaGeneralVerifierChanged{network_name: String, old: Verifier, new: Verifier},
}

impl Error {
    pub fn show (&self) -> String {
        match &self {
            Error::AllParsingFailed(errors) => {
                let mut insert = String::new();
                for (i,(name, version, parser_error)) in errors.iter().enumerate() {
                    if i>0 {insert.push_str(" ")}
                    insert.push_str(&format!("Parsing with {}{} metadata: {}", name, version, parser_error.show()));
                }
                format!("All parsing attempts failed with following errors. {}", insert)
            },
            Error::Parser(x) => x.show(),
            Error::BadInputData(x) => {
                let insert = match x {
                    BadInputData::TooShort => String::from("Data is too short."),
                    BadInputData::NotSubstrate => String::from("Only Substrate transactions are supported. Transaction is expected to start with 0x53."),
                    BadInputData::NotHex => String::from("Input data not in hex format."),
                    BadInputData::CryptoNotSupported => String::from("Crypto type not supported."),
                    BadInputData::WrongPayloadType => String::from("Wrong payload type, as announced by prelude."),
                    BadInputData::LoadMetaUnknownNetwork(x) => format!("Network {} is not in the database. Add network before loading the metadata.", x),
                    BadInputData::NotMeta => String::from("First characters in metadata are expected to be 0x6d657461."),
                    BadInputData::MetaVersionBelow12 => String::from("Received metadata could not be decoded. Runtime metadata version is below 12."),
                    BadInputData::MetaAlreadyThere => String::from("Metadata already in database."),
                    BadInputData::MetaTotalMismatch => String::from("Attempt to load different metadata for same name and version."),
                    BadInputData::VersionNotDecodeable => String::from("Received metadata version could not be decoded."),
                    BadInputData::NoMetaVersion => String::from("No version in received metadata."),
                    BadInputData::UnableToDecodeMeta => String::from("Unable to decode received metadata."),
                    BadInputData::SpecsAlreadyThere => String::from("Network specs from the message are already in the database."),
                    BadInputData::UnableToDecodeTypes => String::from("Unable to decode received types information."),
                    BadInputData::TypesAlreadyThere => String::from("Types information already in database."),
                    BadInputData::UnableToDecodeAddSpecsMessage => String::from("Unable to decode received add specs message."),
                    BadInputData::UnableToDecodeLoadMetadataMessage => String::from("Unable to decode received load metadata message."),
                    BadInputData::ImportantSpecsChanged => String::from("Network already has entries. Important chainspecs in received add network message are different."),
                    BadInputData::MessageNotReadable => String::from("Received message could not be read."),
                };
                format!("Bad input data. {}", insert)
            },
            Error::DatabaseError(x) => {
                let insert = match x {
                    DatabaseError::Internal(e) => format!("Internal error. {}", e),
                    DatabaseError::DamagedChainSpecs => String::from("ChainSpecs could not be decoded."),
                    DatabaseError::NoNetwork => String::from("Network not found. Please add the network."),
                    DatabaseError::DamagedAddressDetails => String::from("Address details could not be decoded."),
                    DatabaseError::DamagedTypesDatabase => String::from("Types information could not be decoded."),
                    DatabaseError::NoTypes => String::from("Types information not found."),
                    DatabaseError::DamagedVersName => String::from("Network versioned name could not be decoded."),
                    DatabaseError::NoMetaAtAll => String::from("No metadata on file for this network."),
                    DatabaseError::DamagedGeneralVerifier => String::from("General verifier information could not be decoded."),
                    DatabaseError::NoGeneralVerifier => String::from("No general verifier information in the database."),
                    DatabaseError::DamagedNetworkVerifier => String::from("Network verifier is damaged and could not be decoded."),
                    DatabaseError::NetworkSpecsKeyMismatch(x) => format!("Network specs stored under key {} do not match it.", hex::encode(x.key())),
                    DatabaseError::UnexpectedlyMetGenesisHash(x) => format!("No verifier information corresponding to genesis hash {}, however, genesis hash is encountered in network specs", hex::encode(x)),
                    DatabaseError::DifferentNamesSameGenesisHash(x) => format!("Different network names in database for same genesis hash {}.", hex::encode(x)),
                    DatabaseError::Temporary(x) => format!("Error setting stub into storage. {}", x),
                    DatabaseError::DeadVerifier(x) => format!("Network {} is disabled. It could be enabled again only after complete wipe and re-installation of Signer.", x),
                    DatabaseError::CustomVerifierIsGeneral(x) => format!("Custom verifier for VerifierKey {} is same as general verifier.", hex::encode(x.key())),
                    DatabaseError::NotMeta => String::from("Metadata entry does not start with 0x6d657461."),
                    DatabaseError::MetaVersionBelow12 => String::from("Metadata could not be decoded. Runtime metadata version is below 12."),
                    DatabaseError::MetaMismatch => String::from("Network metadata entry corrupted in database, name and/or version in meta_key do not match the ones in metadata itself. Please remove the entry and download the metadata for this network."),
                    DatabaseError::NoVersion => String::from("Metadata in storage has no version."),
                    DatabaseError::VersionNotDecodeable => String::from("Version block of metadata in storage could not be decoded."),
                    DatabaseError::UnableToDecodeMeta => String::from("Metadata in storage could not be decoded."),
                    DatabaseError::RuntimeVersionIncompatible => String::from("Metadata runtime version in database is not v12, v13, or v14."),
                };
                format!("Database error. {}", insert)
            },
            Error::CryptoError(x) => {
                let insert = match x {
                    CryptoError::BadSignature => String::from("Corrupted data. Bad signature."),
                    CryptoError::AddSpecsVerifierChanged {network_name, old, new} => format!("Network {} current verifier is {}. Received add_specs message is verified by {}, which is neither current network verifier not the general verifier. Changing the network verifier to another non-general one would require wipe and reset of Signer.", network_name, old.show_error(), new.show_error()),
                    CryptoError::VerifierDisappeared => String::from("Saved information for this network was signed by a verifier. Received information is not signed."),
                    CryptoError::GeneralVerifierChanged {old, new} => format!("Different general verifier was used previously. Previously used {}. Current attempt {}.", old.show_error(), new.show_error()),
                    CryptoError::GeneralVerifierDisappeared => String::from("General verifier information exists in the database. Received information could be accepted only from the same general verifier."),
                    CryptoError::LoadMetaUpdVerifier{network_name, new_verifier} => format!("Network {} currently has no verifier set up. Received load_metadata message is verified by {}. In order to accept verified metadata, first download properly verified network specs.", network_name, new_verifier.show_error()),
                    CryptoError::LoadMetaVerifierChanged{network_name, old, new} => format!("Network {} current verifier is {}. Received load_metadata message is verified by {}. Changing verifier for the network would require wipe and reset of Signer.", network_name, old.show_error(), new.show_error()),
                    CryptoError::LoadMetaUpdGeneralVerifier{network_name, new_verifier} => format!("Network {} is set to be verified by the general verifier, however, no general verifier is set up. Received load_metadata message is verified by {}. In order to accept verified metadata and set up the general verifier, first download properly verified network specs.", network_name, new_verifier.show_error()),
                    CryptoError::LoadMetaGeneralVerifierChanged{network_name, old, new} => format!("Network {} is verified by the general verifier which currently is {}. Received load_metadata message is verified by {}. Changing the general verifier or changing the network verifier to custom would require wipe and reset of Signer.", network_name, old.show_error(), new.show_error()),
                };
                format!("Safety-related error. {}", insert)
            },
        }
    }
}

