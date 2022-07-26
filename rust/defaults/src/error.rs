pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("base58 prefi mismatch: specs: {}, meta: {}", .specs, .meta)]
    Base58PrefixSpecsMismatch { specs: u16, meta: u16 },

    #[error(transparent)]
    DefinitionsError(#[from] definitions::error::Error),

    #[error("orphan metadata {} {}", .name, .filename)]
    OrphanMetadata { name: String, filename: String },
}
