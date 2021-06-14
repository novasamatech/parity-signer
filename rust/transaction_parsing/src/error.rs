use sled;

#[derive(PartialEq)]
pub enum Error {
    BadInputData(BadInputData),
    UnableToDecode(UnableToDecode),
    DatabaseError(DatabaseError),
    SystemError(SystemError),
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
}

#[derive(PartialEq)]
pub enum UnableToDecode {
    MethodAndExtrinsicsFailure,
    NeedPalletAndMethod,
    MethodNotFound{method_index: u8, pallet_name: String},
    PalletNotFound(u8),
    MethodIndexTooHigh{method_index: u8, pallet_index: u8, total: usize},
    ArgumentTypeError,
    ArgumentNameError,
    NotPrimitive(String),
    NoCompact,
    DataTooShort,
    PrimitiveFailure(String),
    UnexpectedOptionVariant,
    IdFields,
    Array,
    BalanceNotDescribed(String),
    UnexpectedEnumVariant,
    UnexpectedCompactInsides,
    CompactNotGetsPrimitive,
    UnknownType(String),
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
}

#[derive(PartialEq)]
pub enum SystemError {
    BalanceFail,
    MetaVersionBelow12,
    MetaMismatch,
    NoVersion,
    UnableToDecodeMeta,
    RegexError,
}

impl Error {
    pub fn show (&self) -> String {
        match &self {
            Error::BadInputData(x) => {
                match x {
                    BadInputData::TooShort => String::from("Data is too short."),
                    BadInputData::NotSubstrate => String::from("Only Substrate transactions are supported. Transaction is expected to start with '53'."),
                    BadInputData::NotHex => String::from("Input data not in hex format."),
                    BadInputData::CryptoNotSupported => String::from("Crypto type not supported."),
                    BadInputData::UnexpectedImmortality => String::from("Expected mortal transaction due to prelude format. Found immortal transaction."),
                    BadInputData::UnexpectedMortality => String::from("Expected immortal transaction due to prelude format. Found mortal transaction."),
                    BadInputData::WrongPayloadType => String::from("Wrong payload type, as announced by prelude."),
                    BadInputData::GenesisHashMismatch => String::from("Genesis hash from extrinsics not matching with genesis hash at the transaction end."),
                    BadInputData::ImmortalHashMismatch => String::from("Block hash for immortal transaction not matching genesis hash for the network."),
                    BadInputData::SomeDataNotUsed => String::from("After decoding some data remained unused."),
                }
            },
            Error::UnableToDecode(x) => {
                match x {
                    UnableToDecode::MethodAndExtrinsicsFailure => String::from("Unable to separate transaction vector, extrinsics, and genesis hash."),
                    UnableToDecode::NeedPalletAndMethod => String::from("Error on decoding. Expected method and pallet information. Found data is shorter."),
                    UnableToDecode::MethodNotFound {method_index, pallet_name} => format!("Method number {} not found in pallet \"{}\".", method_index, pallet_name),
                    UnableToDecode::PalletNotFound (i) => format!("Pallet with index {} not found.", i),
                    UnableToDecode::MethodIndexTooHigh {method_index, pallet_index, total} => format!("Method number {} too high for pallet number {}. Only {} indices available.", method_index, pallet_index, total),
                    UnableToDecode::ArgumentTypeError => String::from("Argument type error."),
                    UnableToDecode::ArgumentNameError => String::from("Argument name error."),
                    UnableToDecode::NotPrimitive(x) => format!("Error decoding call contents. Expected primitive type. Found {}.", x),
                    UnableToDecode::NoCompact => String::from("Error decoding call contents. Expected compact. Not found it."),
                    UnableToDecode::DataTooShort => String::from("Error decoding call contents. Data too short for expected content."),
                    UnableToDecode::PrimitiveFailure(x) => format!("Error decoding call content. Unable to decode part of data as {}.", x),
                    UnableToDecode::UnexpectedOptionVariant => String::from("Error decoding call content. Encountered unexpected Option<_> variant."),
                    UnableToDecode::IdFields => String::from("Error decoding call content. IdentityField description error."),
                    UnableToDecode::Array => String::from("Error decoding call content. Unable to decode part of data as an [u8; 32] array."),
                    UnableToDecode::BalanceNotDescribed(x) => format!("Error decoding call content. Balance type {} used is not described.", x),
                    UnableToDecode::UnexpectedEnumVariant => String::from("Error decoding call content. Encountered unexpected enum variant."),
                    UnableToDecode::UnexpectedCompactInsides => String::from("Error decoding call content. Unexpected type inside compact."),
                    UnableToDecode::CompactNotGetsPrimitive => String::from("Error decoding call content. Type inside compact cound not be transformed into primitive."),
                    UnableToDecode::UnknownType(x) => format!("Error decoding call content. No description found for type \"{}\".", x),
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
                }
            },
            Error::SystemError(x) => {
                match x {
                    SystemError::BalanceFail => format!("System error. Balance printing failed."),
                    SystemError::MetaVersionBelow12 => String::from("System error. Metadata could not be decoded. Runtime metadata version is below 12."),
                    SystemError::MetaMismatch => String::from("Network metadata entry corrupted in database. Please remove the entry and download the metadata for this network."),
                    SystemError::NoVersion => String::from("System error. No version in metadata."),
                    SystemError::UnableToDecodeMeta => String::from("System error. Unable to decode metadata."),
                    SystemError::RegexError => String::from("System error. Expected single argument in regex capture, should not get here.")
                }
            },
        }
    }
}

