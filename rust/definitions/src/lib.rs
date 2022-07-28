//! Definitions and common methods for types used in [Signer](https://github.com/paritytech/parity-signer)
//! and Signer-supporting ecosystem.  
//!
//! ## Features
//! Feature `"signer"` corresponds to everything related to Signer air-gapped
//! device.
//!
//! Feature `"active"` corresponds to all Signer-related things happening
//! **without** air-gap.
//!
//! Feature `"test"` includes both `"signer"` and `"active"` features, along
//! with some testing, and is the default one.  

#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod crypto;

pub mod danger;

pub mod error;

#[cfg(feature = "active")]
pub mod error_active;

#[cfg(feature = "signer")]
pub mod error_signer;

pub mod helpers;

pub mod history;

pub mod keyring;

pub mod metadata;

pub mod network_specs;

pub mod qr_transfers;

pub mod types;

pub mod users;

pub mod navigation;
