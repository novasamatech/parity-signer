pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("State Mutex poisoned")]
    StatePoisoned,

    #[error(transparent)]
    DbHandling(#[from] db_handling::Error),

    #[error(transparent)]
    Definitions(#[from] definitions::error::Error),

    #[error(transparent)]
    TransactionParsing(#[from] transaction_parsing::Error),

    #[error("DB not initialized.")]
    DbNotInitialized,

    #[error("Key not found {0}")]
    KeyNotFound(String),

    #[error("Seed name exists in db")]
    SeedNameExists,

    #[error("Wrong new state")]
    WrongNewState,
}
