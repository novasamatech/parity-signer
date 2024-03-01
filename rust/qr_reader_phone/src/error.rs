/// Qr code handling parsing result.
pub type Result<T> = std::result::Result<T, Error>;

/// QR code handling error type.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error parsing RaptorqFrame: {0}")]
    RaptorqFrame(String),

    #[error("Error parsing LegacyFrame: {0}")]
    LegacyFrame(String),

    #[error("Unexpected QR content: {0}")]
    UnexpectedData(String),

    #[error(transparent)]
    HexDecoding(#[from] hex::FromHexError),

    #[error("Banana Split password is wrong")]
    BananaSplitWrongPassword,

    #[error(transparent)]
    BananaSplitError(#[from] banana_recovery::Error),

    #[error(transparent)]
    TransactionParsingError(#[from] transaction_parsing::Error),

    #[error("Unable to decode on given dataset")]
    UnableToDecode,

    #[error("Was decoding fountain qr code with message length {0}, got interrupted by fountain qr code with message length {1}")]
    ConflictingPayloads(u32, u32),

    #[error("Was decoding legacy multi-element qr, and got interrupted by a fountain one.")]
    LegacyInterruptedByFountain,

    #[error("Was decoding legacy multi-element qr, and got interrupted by a banana recovery one.")]
    LegacyInterruptedByBanana,

    #[error(
        "Number of element in legacy multi-element qr sequence exceeds expected sequence length."
    )]
    LengthExceeded,

    #[error("Was decoding fountain qr code, and got interrupted by a legacy multi-element one.")]
    FountainInterruptedByLegacy,

    #[error("Was decoding legacy multi-element qr code with {0} elements, got interrupted by legacy multi-element qr code with {0} elements")]
    ConflictingLegacyLengths(u16, u16),

    #[error("Encountered two legacy multi-element qr code fragments with same number.")]
    SameNumber,

    #[error("Was reading dynamic qr, and got interrupted by a static one.")]
    DynamicInterruptedByStatic,

    #[error("Parsed mnemonic is invalid")]
    InvalidMnemonic,
}
