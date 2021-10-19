use sled;
use definitions::{keyring::{NetworkSpecsKey, VerifierKey}, network_specs::Verifier};

#[derive(PartialEq)]
pub enum Error {
    BadInputData(BadInputData),
    UnableToDecode(UnableToDecode),
    DatabaseError(DatabaseError),
    SystemError(SystemError),
    CryptoError(CryptoError),
}

#[derive(PartialEq)]
pub enum BadInputData {
    TooShort,
    NotSubstrate,
    NotHex,
    CryptoNotSupported,
    UnexpectedImmortality,
    UnexpectedMortality,
    WrongPayloadType,
    GenesisHashMismatch,
    ImmortalHashMismatch,
    SomeDataNotUsed,
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
    EncryptionMismatch,
}

#[derive(PartialEq)]
pub enum UnableToDecode {
    MethodAndExtrinsicsFailure,
    NeedPalletAndMethod,
    NeedPallet,
    MethodNotFound{method_index: u8, pallet_name: String},
    PalletNotFound(u8),
    MethodIndexTooHigh{method_index: u8, pallet_index: u8, total: usize},
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
    NoMetaThisVersion,
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
}

#[derive(PartialEq)]
pub enum SystemError {
    BalanceFail,
    NotMeta,
    MetaVersionBelow12,
    MetaMismatch,
    NoVersion,
    VersionNotDecodeable,
    UnableToDecodeMeta,
    RegexError,
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
            Error::BadInputData(x) => {
                match x {
                    BadInputData::TooShort => String::from("Data is too short."),
                    BadInputData::NotSubstrate => String::from("Only Substrate transactions are supported. Transaction is expected to start with 0x53."),
                    BadInputData::NotHex => String::from("Input data not in hex format."),
                    BadInputData::CryptoNotSupported => String::from("Crypto type not supported."),
                    BadInputData::UnexpectedImmortality => String::from("Expected mortal transaction due to prelude format. Found immortal transaction."),
                    BadInputData::UnexpectedMortality => String::from("Expected immortal transaction due to prelude format. Found mortal transaction."),
                    BadInputData::WrongPayloadType => String::from("Wrong payload type, as announced by prelude."),
                    BadInputData::GenesisHashMismatch => String::from("Genesis hash from extrinsics not matching with genesis hash at the transaction end."),
                    BadInputData::ImmortalHashMismatch => String::from("Block hash for immortal transaction not matching genesis hash for the network."),
                    BadInputData::SomeDataNotUsed => String::from("After decoding some data remained unused."),
                    BadInputData::LoadMetaUnknownNetwork(x) => format!("Network {} in not in the database. Add network before loading the metadata.", x),
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
                    BadInputData::EncryptionMismatch => String::from("Encryption used in message is not supported by the network."),
                }
            },
            Error::UnableToDecode(x) => {
                match x {
                    UnableToDecode::MethodAndExtrinsicsFailure => String::from("Unable to separate transaction vector, extrinsics, and genesis hash."),
                    UnableToDecode::NeedPalletAndMethod => String::from("Error on decoding. Expected method and pallet information. Found data is shorter."),
                    UnableToDecode::NeedPallet => String::from("Error on decoding. Expected pallet information. Found data is shorter."),
                    UnableToDecode::MethodNotFound {method_index, pallet_name} => format!("Method number {} not found in pallet {}.", method_index, pallet_name),
                    UnableToDecode::PalletNotFound (i) => format!("Pallet with index {} not found.", i),
                    UnableToDecode::MethodIndexTooHigh {method_index, pallet_index, total} => format!("Method number {} too high for pallet number {}. Only {} indices available.", method_index, pallet_index, total),
                    UnableToDecode::NoCallsInPallet(x) => format!("No calls found in pallet {}.", x),
                    UnableToDecode::V14TypeNotResolved => String::from("Error decoding with v14 metadata. Referenced type could not be resolved."),
                    UnableToDecode::ArgumentTypeError => String::from("Argument type error."),
                    UnableToDecode::ArgumentNameError => String::from("Argument name error."),
                    UnableToDecode::NotPrimitive(x) => format!("Error decoding call contents. Expected primitive type. Found {}.", x),
                    UnableToDecode::NoCompact => String::from("Error decoding call contents. Expected compact. Not found it."),
                    UnableToDecode::DataTooShort => String::from("Error decoding call contents. Data too short for expected content."),
                    UnableToDecode::PrimitiveFailure(x) => format!("Error decoding call content. Unable to decode part of data as {}.", x),
                    UnableToDecode::UnexpectedOptionVariant => String::from("Error decoding call content. Encountered unexpected Option<_> variant."),
                    UnableToDecode::IdFields => String::from("Error decoding call content. IdentityField description error."),
                    UnableToDecode::Array => String::from("Error decoding call content. Unable to decode part of data as an [u8; 32] array."),
                    UnableToDecode::BalanceNotDescribed => String::from("Error decoding call content. Unexpected type encountered for Balance"),
                    UnableToDecode::UnexpectedEnumVariant => String::from("Error decoding call content. Encountered unexpected enum variant."),
                    UnableToDecode::UnexpectedCompactInsides => String::from("Error decoding call content. Unexpected type inside compact."),
                    UnableToDecode::CompactNotPrimitive => String::from("Error decoding call content. Type inside compact cound not be transformed into primitive."),
                    UnableToDecode::UnknownType(x) => format!("Error decoding call content. No description found for type {}.", x),
                    UnableToDecode::NotBitStoreType => String::from("Error decoding call content. Declared type is not suitable BitStore type for BitVec."),
                    UnableToDecode::NotBitOrderType => String::from("Error decoding call content. Declared type is not suitable BitOrder type for BitVec."),
                    UnableToDecode::BitVecFailure => String::from("Error decoding call content. Could not decode BitVec."),
                }
            },
            Error::DatabaseError(x) => {
                match x {
                    DatabaseError::Internal(e) => format!("Database internal error. {}", e),
                    DatabaseError::DamagedChainSpecs => String::from("ChainSpecs from database could not be decoded."),
                    DatabaseError::NoNetwork => String::from("Network not found. Please add the network."),
                    DatabaseError::DamagedAddressDetails => String::from("Address details from database could not be decoded."),
                    DatabaseError::DamagedTypesDatabase => String::from("Types database from database could not be decoded."),
                    DatabaseError::NoTypes => String::from("Types information not found in the database"),
                    DatabaseError::DamagedVersName => String::from("Network versioned name from metadata database could not be decoded."),
                    DatabaseError::NoMetaThisVersion => String::from("No metadata on file for this version."),
                    DatabaseError::NoMetaAtAll => String::from("No metadata on file for this network."),
                    DatabaseError::DamagedGeneralVerifier => String::from("General verifier information from database could not be decoded."),
                    DatabaseError::NoGeneralVerifier => String::from("No general verifier information in the database."),
                    DatabaseError::DamagedNetworkVerifier => String::from("Network verifier is damaged and could not be decoded."),
                    DatabaseError::NetworkSpecsKeyMismatch(x) => format!("Network specs stored under key {} do not match it.", hex::encode(x.key())),
                    DatabaseError::UnexpectedlyMetGenesisHash(x) => format!("No verifier information corresponding to genesis hash {}, however, genesis hash is encountered in network specs", hex::encode(x)),
                    DatabaseError::DifferentNamesSameGenesisHash(x) => format!("Different network names in database for same genesis hash {}.", hex::encode(x)),
                    DatabaseError::Temporary(x) => format!("Error setting stub into storage. {}", x),
                    DatabaseError::DeadVerifier(x) => format!("Network {} is disabled. It could be enabled again only after complete wipe and re-installation of Signer.", x),
                    DatabaseError::CustomVerifierIsGeneral(x) => format!("Custom verifier for VerifierKey {} is same as general verifier.", hex::encode(x.key())),
                }
            },
            Error::SystemError(x) => {
                match x {
                    SystemError::BalanceFail => format!("System error. Balance printing failed."),
                    SystemError::NotMeta => String::from("System error. First characters in metadata are expected to be 0x6d657461."),
                    SystemError::MetaVersionBelow12 => String::from("System error. Metadata could not be decoded. Runtime metadata version is below 12."),
                    SystemError::MetaMismatch => String::from("Network metadata entry corrupted in database. Please remove the entry and download the metadata for this network."),
                    SystemError::NoVersion => String::from("System error. No version in metadata."),
                    SystemError::VersionNotDecodeable => String::from("System error. Retrieved from metadata version constant could not be decoded."),
                    SystemError::UnableToDecodeMeta => String::from("System error. Unable to decode metadata."),
                    SystemError::RegexError => String::from("System error. Unexpected regular expressions error.")
                }
            },
            Error::CryptoError(x) => {
                match x {
                    CryptoError::BadSignature => String::from("Corrupted data. Bad signature."),
                    CryptoError::AddSpecsVerifierChanged {network_name, old, new} => format!("Network {} current verifier is {}. Received add_specs message is verified by {}, which is neither current network verifier not the general verifier. Changing the network verifier to another non-general one would require wipe and reset of Signer.", network_name, old.show_error(), new.show_error()),
                    CryptoError::VerifierDisappeared => String::from("Saved information for this network was signed by a verifier. Received information is not signed."),
                    CryptoError::GeneralVerifierChanged {old, new} => format!("Different general verifier was used previously. Previously used {}. Current attempt {}.", old.show_error(), new.show_error()),
                    CryptoError::GeneralVerifierDisappeared => String::from("General verifier information exists in the database. Received information could be accepted only from the same general verifier."),
                    CryptoError::LoadMetaUpdVerifier{network_name, new_verifier} => format!("Network {} currently has no verifier set up. Received load_metadata message is verified by {}. In order to accept verified metadata, first download properly verified network specs.", network_name, new_verifier.show_error()),
                    CryptoError::LoadMetaVerifierChanged{network_name, old, new} => format!("Network {} current verifier is {}. Received load_metadata message is verified by {}. Changing verifier for the network would require wipe and reset of Signer.", network_name, old.show_error(), new.show_error()),
                    CryptoError::LoadMetaUpdGeneralVerifier{network_name, new_verifier} => format!("Network {} is set to be verified by the general verifier, however, no general verifier is set up. Received load_metadata message is verified by {}. In order to accept verified metadata and set up the general verifier, first download properly verified network specs.", network_name, new_verifier.show_error()),
                    CryptoError::LoadMetaGeneralVerifierChanged{network_name, old, new} => format!("Network {} is verified by the general verifier which currently is {}. Received load_metadata message is verified by {}. Changing the general verifier or changing the network verifier to custom would require wipe and reset of Signer.", network_name, old.show_error(), new.show_error()),
                }
            },
        }
    }
}

