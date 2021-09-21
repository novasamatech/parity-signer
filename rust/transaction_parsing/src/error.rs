use sled;

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
    NotMeta,
    MetaVersionBelow12,
    MetaMismatch,
    MetaAlreadyThere,
    MetaTotalMismatch,
    VersionNotDecodeable,
    NoMetaVersion,
    UnableToDecodeMeta,
    UnableToDecodeTypes,
    TypesAlreadyThere,
    UnableToDecodeAddNetworkMessage,
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
    NotRangeIndex,
    RangeFailure,
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
    NoNetworkVerifier ([u8; 32]),
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
    VerifierChanged {old_show: String, new_show: String},
    VerifierDisappeared,
    GeneralVerifierChanged {old_show: String, new_show: String},
    GeneralVerifierDisappeared,
    NetworkExistsVerifierDisappeared,
    
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
                    BadInputData::NotMeta => String::from("First characters in metadata are expected to be 0x6d657461."),
                    BadInputData::MetaVersionBelow12 => String::from("Received metadata could not be decoded. Runtime metadata version is below 12."),
                    BadInputData::MetaMismatch => String::from("Received metadata specname does not match."),
                    BadInputData::MetaAlreadyThere => String::from("Metadata already in database."),
                    BadInputData::MetaTotalMismatch => String::from("Attempt to load different metadata for same name and version."),
                    BadInputData::VersionNotDecodeable => String::from("Received metadata version could not be decoded."),
                    BadInputData::NoMetaVersion => String::from("No version in received metadata."),
                    BadInputData::UnableToDecodeMeta => String::from("Unable to decode received metadata."),
                    BadInputData::UnableToDecodeTypes => String::from("Unable to decode received types information."),
                    BadInputData::TypesAlreadyThere => String::from("Types information already in database."),
                    BadInputData::UnableToDecodeAddNetworkMessage => String::from("Unable to decode received add network message."),
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
                    UnableToDecode::NotRangeIndex => String::from("Error decoding call content. Declared type is not suitable index type for Range."),
                    UnableToDecode::RangeFailure => String::from("Error decoding call content. Could not decode Range."),
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
                    DatabaseError::NoNetworkVerifier(x) => format!("No network verifier information in the database for genesis hash {}.", hex::encode(x)),
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
                    CryptoError::VerifierChanged {old_show, new_show} => format!("Different verifier was used for this network previously. Previously used {}. Current attempt {}.", old_show, new_show),
                    CryptoError::VerifierDisappeared => String::from("Saved metadata for this network was signed by a verifier. This metadata is not."),
                    CryptoError::GeneralVerifierChanged {old_show, new_show} => format!("Different general verifier was used previously. Previously used {}. Current attempt {}.", old_show, new_show),
                    CryptoError::GeneralVerifierDisappeared => String::from("General verifier information exists in the database. Received information could be accepted only from the same general verifier."),
                    CryptoError::NetworkExistsVerifierDisappeared => String::from("Network already has specs recorded in database. Received add network message is not signed, previously this network information was signed."),
                }
            },
        }
    }
}

