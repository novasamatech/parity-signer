pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DbHandling(#[from] db_handling::Error),

    #[error(transparent)]
    Definitions(#[from] definitions::error::Error),

    #[error(transparent)]
    TransactionParsing(#[from] transaction_parsing::Error),

    #[error(transparent)]
    Hex(#[from] hex::FromHexError),

    #[error("DB not initialized.")]
    DbNotInitialized,

    #[error("Key not found {0}")]
    KeyNotFound(String),

    #[error("Mutex poisoned")]
    MutexPoisoned,

    #[error("Data packing error: {0}")]
    DataPacking(String),

    #[error("Tx Action not sign")]
    TxActionNotSign,

    #[error("Number of seeds provided does not match number of txs in a bulk <{0}>")]
    SeedsNumMismatch(String),

    #[error(transparent)]
    TransactionSigning(#[from] transaction_signing::Error),
}
