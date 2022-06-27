//! Data sets verified in Signer by a given verifier
//!
//! A single verifier can be responsible for whole data sets in Signer.
//!
//! If the verifier is no longer trusted or if the verifier gets changed, all
//! the data that was previously verified by it, must be removed from Signer.
//!
//! [`GeneralHold`] represents data verified by the general verifier that will
//! be removed if a new general verifier gets accepted. It can include network
//! specs and metadata for networks verified with the general verifier, and also
//! types information.
//!
//! [`Hold`] represents data verified by a custom verifier. It could be all data
//! related to the certain network if the network gets a new, stronger verifier,
//! or all data related to the custom verifier (possibly multiple networks, with
//! all specs and metadata), if the custom verifier gets compromised.
use crate::{metadata::MetaValues, network_specs::NetworkSpecs};

/// Signer data verified by the general verifier
pub struct GeneralHold {
    /// network metadata
    pub metadata_set: Vec<MetaValues>,

    /// network specs
    pub network_specs_set: Vec<NetworkSpecs>,

    /// types information flag, `true` if types information is in the database
    pub types: bool,
}

impl std::fmt::Display for GeneralHold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let part = print_affected(&self.metadata_set, &self.network_specs_set);
        let complete = {
            if self.types {
                format!("{} Types information is purged.", part)
            } else {
                part
            }
        };
        write!(f, "{}", complete)
    }
}

/// Signer data verified by a custom verifier, or its subset
pub struct Hold {
    /// network metadata
    pub metadata_set: Vec<MetaValues>,

    /// network specs
    pub network_specs_set: Vec<NetworkSpecs>,
}

impl std::fmt::Display for Hold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            print_affected(&self.metadata_set, &self.network_specs_set)
        )
    }
}

/// Print network specs and metadata identifiers into readable format.
fn print_affected(metadata_set: &[MetaValues], network_specs_set: &[NetworkSpecs]) -> String {
    let mut out_metadata = String::new();
    let mut out_network_specs = String::new();
    for (i, x) in metadata_set.iter().enumerate() {
        if i > 0 {
            out_metadata.push_str(", ");
        }
        out_metadata.push_str(&format!("{}{}", x.name, x.version));
    }
    for (i, x) in network_specs_set.iter().enumerate() {
        if i > 0 {
            out_network_specs.push_str(", ");
        }
        out_network_specs.push_str(&x.title);
    }
    if out_network_specs.is_empty() {
        out_network_specs = String::from("none");
    }
    if out_metadata.is_empty() {
        out_metadata = String::from("none");
    }
    format!(
        "Affected network specs entries: {}; affected metadata entries: {}.",
        out_network_specs, out_metadata
    )
}
