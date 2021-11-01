#[derive(PartialEq)]
pub enum ParserError {
    Arguments(ArgumentsError), // errors related to metadata and short_specs arguments input by user; this should not be encountered in internal signer decoding
    Decoding(DecodingError), // errors occuring during the decoding procedure
    FundamentallyBadV14Metadata(MetadataError), // errors occuring because the metadata is legit, but not acceptable in existing safety paradigm, for example, in V14 has no mention of network spec version in extrinsics
    SystemError(SystemError), // very much unexpected internal errors not related directly to parsing
    WrongNetworkVersion {as_decoded: String, in_metadata: u32},
}

#[derive(PartialEq)]
pub enum ArgumentsError {
    MetaSpecVersionNotDecodeable,
    NetworkNameMismatch {name_metadata: String, name_network_specs: String},
    NoMetaSpecVersion,
    NoTypes,
    RuntimeVersionIncompatible,
}

#[derive(PartialEq)]
pub enum DecodingError {
    GenesisHashMismatch,
    ImmortalHashMismatch,
    ExtensionsOlder,
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
    Era,
    SomeDataNotUsedMethod,
    SomeDataNotUsedExtensions,
}

#[derive(PartialEq)]
pub enum MetadataError {
    NoEra,
    NoBlockHash,
    NoVersionExt,
    EraTwice,
    GenesisHashTwice,
    BlockHashTwice,
    SpecVersionTwice,
}

#[derive(PartialEq)]
pub enum SystemError {
    BalanceFail,
    RegexError,
}

impl ParserError {
    pub fn show (&self) -> String {
        match &self {
            ParserError::Arguments(x) => {
                let insert = match x {
                    ArgumentsError::MetaSpecVersionNotDecodeable => String::from("Version constant from the metadata could not be decoded."),
                    ArgumentsError::NetworkNameMismatch {name_metadata, name_network_specs} => format!("Network name mismatch. In metadata: {}, in network specs: {}", name_metadata, name_network_specs),
                    ArgumentsError::NoMetaSpecVersion => String::from("No version in the metadata."),
                    ArgumentsError::NoTypes => String::from("Decoding transactions with metadata V12 and V13 uses pre-existing types info. Error generating default types info."),
                    ArgumentsError::RuntimeVersionIncompatible => String::from("Runtime metadata version is incompatible. Supported are V12, V13, and V14."),
                };
                format!("Arguments error. {}", insert)
            },
            ParserError::Decoding(x) => {
                let insert = match x {
                    DecodingError::GenesisHashMismatch => String::from("Genesis hash values from decoded extensions and from used network specs do not match."),
                    DecodingError::ImmortalHashMismatch => String::from("Block hash for immortal transaction not matching genesis hash for the network."),
                    DecodingError::ExtensionsOlder => String::from("Unable to decode extensions for V12/V13 metadata using standard extensions set."),
                    DecodingError::MethodNotFound{method_index, pallet_name} => format!("Method number {} not found in pallet {}.", method_index, pallet_name),
                    DecodingError::PalletNotFound(i) => format!("Pallet with index {} not found.", i),
                    DecodingError::MethodIndexTooHigh{method_index, pallet_index, total} => format!("Method number {} too high for pallet number {}. Only {} indices available.", method_index, pallet_index, total),
                    DecodingError::NoCallsInPallet(x) => format!("No calls found in pallet {}.", x),
                    DecodingError::V14TypeNotResolved => String::from("Referenced type could not be resolved in v14 metadata."),
                    DecodingError::ArgumentTypeError => String::from("Argument type error."),
                    DecodingError::ArgumentNameError => String::from("Argument name error."),
                    DecodingError::NotPrimitive(x) => format!("Expected primitive type. Found {}.", x),
                    DecodingError::NoCompact => String::from("Expected compact. Not found it."),
                    DecodingError::DataTooShort => String::from("Data too short for expected content."),
                    DecodingError::PrimitiveFailure(x) => format!("Unable to decode part of data as {}.", x),
                    DecodingError::UnexpectedOptionVariant => String::from("Encountered unexpected Option<_> variant."),
                    DecodingError::IdFields => String::from("IdentityField description error."),
                    DecodingError::Array => String::from("Unable to decode part of data as an array."),
                    DecodingError::BalanceNotDescribed => String::from("Unexpected type encountered for Balance"),
                    DecodingError::UnexpectedEnumVariant => String::from("Encountered unexpected enum variant."),
                    DecodingError::UnexpectedCompactInsides => String::from("Unexpected type inside compact."),
                    DecodingError::CompactNotPrimitive => String::from("Type claimed inside compact is not compactable."),
                    DecodingError::UnknownType(x) => format!("No description found for type {}.", x),
                    DecodingError::NotBitStoreType => String::from("Declared type is not suitable BitStore type for BitVec."),
                    DecodingError::NotBitOrderType => String::from("Declared type is not suitable BitOrder type for BitVec."),
                    DecodingError::BitVecFailure => String::from("Could not decode BitVec."),
                    DecodingError::Era => String::from("Could not decode Era."),
                    DecodingError::SomeDataNotUsedMethod => String::from("After decoding the method some data remained unused."),
                    DecodingError::SomeDataNotUsedExtensions => String::from("After decoding the extensions some data remained unused."),
                };
                format!("Decoding error. {}.", insert)
            },
            ParserError::FundamentallyBadV14Metadata(x) => {
                let insert = match x {
                    MetadataError::NoEra => String::from("Era information is missing."),
                    MetadataError::NoBlockHash => String::from("Block hash information is missing."),
                    MetadataError::NoVersionExt => String::from("Metadata spec version information is missing."),
                    MetadataError::EraTwice => String::from("Era information is encountered mora than once."),
                    MetadataError::GenesisHashTwice => String::from("Genesis hash is encountered more than once."),
                    MetadataError::BlockHashTwice => String::from("Block hash is encountered more than once."),
                    MetadataError::SpecVersionTwice => String::from("Metadata spec version is encountered more than once."),
                };
                format!("Signed extensions are not compatible with Signer (v14 metadata). {}", insert)
            },
            ParserError::SystemError(x) => {
                let insert = match x {
                    SystemError::BalanceFail => String::from("Balance printing failed."),
                    SystemError::RegexError => String::from("Unexpected regular expressions error."),
                };
                format!("System error. {}", insert)
            },
            ParserError::WrongNetworkVersion{as_decoded, in_metadata} => format!("Metadata network spec version ({}) differs from the version in extensions ({}).", as_decoded, in_metadata),
        }
    }
}
