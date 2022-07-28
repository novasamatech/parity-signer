pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    DefinitionsError(#[from] definitions::error::Error),

    /// Base58 prefix from metadata (`meta`) does not match base58 prefix in specs (`specs`)
    #[error(
        "Base58 prefix {specs} from system pallet constants does \
        not match the base58 prefix from network specs {meta}."
    )]
    Base58PrefixSpecsMismatch { specs: u16, meta: u16 },

    /// Default metadata set contains metadata files that have no corresponding
    /// default network specs and address book entries.
    #[error(
        "Default metadata for {name} from {filename} \
        does not have corresponding default network specs."
    )]
    OrphanMetadata { name: String, filename: String },
}
