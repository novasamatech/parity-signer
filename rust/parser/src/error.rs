//! Errors in standalone parser
//!
use definitions::{error::MetadataError, error_signer::ParserError};

pub enum Error {
    Parser(ParserError),
    Arguments(ArgumentsError), // errors related to metadata and short_specs arguments input by user
}

pub enum ArgumentsError {
    Metadata(MetadataError),
    NetworkNameMismatch {
        name_metadata: String,
        name_network_specs: String,
    },
    NoTypes,
    DefaultTypes,
}

impl Error {
    pub fn show(&self) -> String {
        match &self {
            Error::Parser(x) => x.show(),
            Error::Arguments(x) => {
                let insert = match x {
                    ArgumentsError::Metadata(e) => format!("Bad metadata. {}", e.show()),
                    ArgumentsError::NetworkNameMismatch {name_metadata, name_network_specs} => format!("Network name mismatch. In metadata: {}, in network specs: {}", name_metadata, name_network_specs),
                    ArgumentsError::NoTypes => String::from("Decoding transactions with metadata V12 and V13 uses pre-existing types info. Loaded default types info is empty."),
                    ArgumentsError::DefaultTypes => String::from("Decoding transactions with metadata V12 and V13 uses pre-existing types info. Error generating default types info."),
                };
                format!("Arguments error. {}", insert)
            }
        }
    }
}
