//! Definitions and common methods for types used in [Vault](https://github.com/paritytech/parity-signer)
//! and Vault-supporting ecosystem.
//!
//! ## Features
//! Feature `"signer"` corresponds to everything related to Vault air-gapped
//! device.
//!
//! Feature `"active"` corresponds to all Vault-related things happening
//! **without** air-gap.
//!
//! Feature `"test"` includes both `"signer"` and `"active"` features, along
//! with some testing, and is the default one.

#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod crypto;

pub mod danger;

pub mod error;

pub mod error_active;

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

pub mod derivations;

pub mod dynamic_derivations;
pub mod schema_version;
