/// Transaction signing result.
pub type Result<T> = std::result::Result<T, Error>;

/// Transaction signing error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A DB error.
    #[error("Database error. Internal error. {0}")]
    DbHandling(#[from] db_handling::Error),

    #[error("An error occured: {0}.")]
    Other(#[from] anyhow::Error),

    /// User has entered a wrong password for a passworded address.
    ///
    /// For cases when Signer database checksum is not verified.
    /// Signer log records that password was entered incorrectly.
    #[error("Wrong password.")]
    WrongPassword,

    /// Error in [`SecretString`](https://docs.rs/sp-core/6.0.0/sp_core/crypto/type.SecretString.html).
    ///
    /// `SecretString` consists of combined seed phrase and derivation.
    ///
    /// Associated error content is
    /// [`SecretStringError`](https://docs.rs/sp-core/6.0.0/sp_core/crypto/enum.SecretStringError.html).
    #[error("Secret string error: {}.", format!("{:?}", .0))]
    CryptoError(sp_core::crypto::SecretStringError),

    /// User has entered a wrong password for a passworded address for cases
    /// when the Signer database checksum is verified.
    ///
    /// Signer log records that password was entered incorrectly.
    /// This changes the database checksum, and for the next attempt it must be
    /// updated.
    ///
    /// Associated data is the new checksum.
    #[error("Wrong password.")]
    WrongPasswordNewChecksum(u32),
}
