pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error. Internal error. {0}")]
    DbHandling(#[from] db_handling::Error),

    #[error("wrong password")]
    WrongPassword,

    #[error("crypto error")]
    CryptoError(sp_core::crypto::SecretStringError),

    #[error("other {0}")]
    Other(#[from] anyhow::Error),

    #[error("Wrong password.")]
    WrongPasswordNewChecksum(u32),
}
