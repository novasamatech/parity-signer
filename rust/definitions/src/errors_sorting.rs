//use sled;

/// Trait to decsribe all errors occuring both on the hot side and on the
/// Signer side
pub trait Error {
    fn show(&self) -> String;
}

/// Enum listing all variants of errors from the hot side
pub enum ErrorHot {
    NotHex(NotHexHot),
    TransferContent(TransferContent),
}

impl Error for ErrorHot {
    fn show(&self) -> String {
        match &self {
            ErrorHot::NotHex(a) => a.show(),
            ErrorHot::TransferContent(a) => a.show(),
        }
    }
}

/// Enum listing all variants of errors from the Signer side
pub enum ErrorSigner {
    NotHex(NotHexSigner),
    TransferContent(TransferContent),
}

impl Error for ErrorSigner {
    fn show(&self) -> String {
        match &self {
            ErrorSigner::NotHex(a) => a.show(),
            ErrorSigner::TransferContent(a) => a.show(),
        }
    }
}

/// Trait to specify error details in transforming hexadecimal string into Vec<u8>.
/// NotHex errors may occur both on hot and on Signer side.
/// On hot side NotHex errors could be related to strings fetched form url,
/// input from command line, and processing of the default values.
/// On Signer side NotHex errors are caused by communication errors 
/// (since user interface should be sending valid hex strings into rust),
/// and generally should not be occuring.
pub trait NotHex {
    fn show(&self) -> String;
    fn to_error(self) -> Box<dyn Error>;
}

/// NotHex errors occuring on the hot side
pub enum NotHexHot {
    FetchedMetadata {url: String},
    FetchedGenesisHash {url: String},
    InputSufficientCrypto,
    InputPublicKey,
    InputSignature,
    DefaultMetadata {filename: String},
}

impl NotHex for NotHexHot {
    fn show(&self) -> String {
        let insert = match &self {
            NotHexHot::FetchedMetadata {url} => format!("Network metadata fetched from url {}", url),
            NotHexHot::FetchedGenesisHash {url} => format!("Network genesis hash fetched from url {}", url),
            NotHexHot::InputSufficientCrypto => String::from("Input sufficient crypto data"),
            NotHexHot::InputPublicKey => String::from("Input public key"),
            NotHexHot::InputSignature => String::from("Input signature"),
            NotHexHot::DefaultMetadata {filename} => format!("Default network metadata from file {}", filename),
        };
        format!("{} is not in hexadecimal format.", insert)
    }
    fn to_error(self) -> Box<dyn Error> {
        Box::new(ErrorHot::NotHex(self))
    }
}

/// NotHex errors occuring on the cold side
pub enum NotHexSigner { // 
    PublicKey {input: String},
    NetworkSpecsKey {input: String},
    InputContent, // double-check how data moves from qr reader into rust part of the signer
}

impl NotHex for NotHexSigner {
    fn show(&self) -> String {
        let insert = match &self {
            NotHexSigner::PublicKey {input} => format!("Input public key {}", input),
            NotHexSigner::NetworkSpecsKey {input} => format!("Input network specs key {}", input),
            NotHexSigner::InputContent => String::from("Input content"),
        };
        format!("{} is not in hexadecimal format.", insert)
    }
    fn to_error(self) -> Box<dyn Error> {
        Box::new(ErrorSigner::NotHex(self))
    }
}

/// Enum to specify errors occuring with decoding transfer content,
/// all of its variants could be encountered both on the hot side
/// (when checking the message content while signing it)
/// and on the cold side (when processing the received messages)
pub enum TransferContent {
    AddSpecs,
    LoadMeta,
    LoadTypes,
}

impl TransferContent {
    fn show(&self) -> String {
        let insert = match &self {
            TransferContent::AddSpecs => "`add_specs`",
            TransferContent::LoadMeta => "`load_meta`",
            TransferContent::LoadTypes => "`load_types`",
        };
        format!("Payload could not be decoded as {}.", insert)
    }
}

