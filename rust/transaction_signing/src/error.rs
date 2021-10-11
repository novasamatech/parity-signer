use sled;
use anyhow::anyhow;

#[derive(PartialEq)]
pub enum Error {
    InternalDatabaseError(sled::Error),
    ChecksumMismatch,
    NoAction(ActionFailure),
    BadActionDecode(ActionFailure),
    AddressDetailsNotFound,
    Interpretation(Interpretation),
    CryptoError(CryptoError),
    AddressKeyDecoding,
    AddressKeyGeneration(String),
    EncryptionMismatch,
}

#[derive(PartialEq)]
pub enum ActionFailure {
    LoadMeta,
    AddVerifier,
    AddNetwork,
    LoadTypes,
    AddGeneralVerifier,
    SignTransaction,
}

#[derive(PartialEq)]
pub enum Interpretation {
    ChecksumMissing,
    ChecksumNotU32,
    ActionMissing,
    UnsupportedAction,
    BadActionLine,
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
            Error::ChecksumMismatch => anyhow!("Database checksum mismatch."),
            Error::NoAction(e) => match e {
                ActionFailure::LoadMeta => anyhow!("No approved load_metadata message found."),
                ActionFailure::AddVerifier => anyhow!("No approved add_metadata_verifier message found."),
                ActionFailure::AddNetwork => anyhow!("No approved add_network message found."),
                ActionFailure::LoadTypes => anyhow!("No approved load_types message found."),
                ActionFailure::AddGeneralVerifier => anyhow!("No approved add_general_verifier message found."),
                ActionFailure::SignTransaction => anyhow!("No approved sign_transaction message found."),
            },
            Error::BadActionDecode(e) => match e {
                ActionFailure::LoadMeta => anyhow!("Found load_metadata message could not be decoded."),
                ActionFailure::AddVerifier => anyhow!("Found add_metadata_verifier message could not be decoded."),
                ActionFailure::AddNetwork => anyhow!("Found add_network message could not be decoded."),
                ActionFailure::LoadTypes => anyhow!("Found load_types message could not be decoded."),
                ActionFailure::AddGeneralVerifier => anyhow!("Found add_general_verifier message could not be decoded."),
                ActionFailure::SignTransaction => anyhow!("Found sign_transaction message could not be decoded."),
            },
            Error::AddressDetailsNotFound => anyhow!("Identity not found."),
            Error::Interpretation(e) => match e {
                Interpretation::ChecksumMissing => anyhow!("Checksum is missing in action line."),
                Interpretation::ChecksumNotU32 => anyhow!("Checksum in action line does not fit in u32."),
                Interpretation::ActionMissing => anyhow!("Action type is missing in action line."),
                Interpretation::UnsupportedAction => anyhow!("Action type not supported."),
                Interpretation::BadActionLine => anyhow!("Unrecognized action line format."),
            },
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
