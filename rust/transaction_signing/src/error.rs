use sled;
use anyhow::anyhow;

#[derive(PartialEq)]
pub enum Error {
    InternalDatabaseError(sled::Error),
    AddressDetailsNotFound,
    ChecksumNotU32,
    CryptoError(CryptoError),
    AddressKeyDecoding,
    AddressKeyGeneration(String),
    EncryptionMismatch,
}

#[derive(PartialEq)]
pub enum CryptoError {
    KeyGenEd25519,
    KeyGenSr25519,
    KeyGenEcdsa,
    WrongPassword,
}

impl Error {
    pub fn show (&self) -> anyhow::Error {
        match &self {
            Error::InternalDatabaseError(e) => anyhow!("Database internal error. {}", e),
            Error::AddressDetailsNotFound => anyhow!("Identity not found."),
            Error::ChecksumNotU32 => anyhow!("Provided checksum is not u32."),
            Error::CryptoError(e) => match e {
                CryptoError::KeyGenEd25519 => anyhow!("Error generating keys for ed25519 crypto."),
                CryptoError::KeyGenSr25519 => anyhow!("Error generating keys for sr25519 crypto."),
                CryptoError::KeyGenEcdsa => anyhow!("Error generating keys for ecdsa crypto."),
                CryptoError::WrongPassword => anyhow!("Wrong password."),
            },
            Error::AddressKeyDecoding => anyhow!("Address key could not be decoded."),
            Error::AddressKeyGeneration(e) => anyhow!("Address key could not be generated. {}", e),
            Error::EncryptionMismatch => anyhow!("Suggested encryption does not correspond to account details."),
        }
    }
}
