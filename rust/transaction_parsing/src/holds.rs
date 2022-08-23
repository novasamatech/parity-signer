use std::fmt::Write;
use std::path::Path;

use constants::{METATREE, SETTREE, SPECSTREE, TYPES, VERIFIERS};
use db_handling::{
    db_transactions::TrDbColdStub,
    helpers::{get_general_verifier, open_db, open_tree, prep_types},
};
use definitions::{
    history::Event,
    keyring::{MetaKeyPrefix, VerifierKey},
    metadata::MetaValues,
    network_specs::{CurrentVerifier, NetworkSpecs, ValidCurrentVerifier, Verifier},
};
use parity_scale_codec::Decode;
use sled::Tree;

use crate::cards::Warning;
use crate::error::Result;

fn print_affected(metadata_set: &[MetaValues], network_specs_set: &[NetworkSpecs]) -> String {
    let mut out_metadata = String::new();
    let mut out_network_specs = String::new();
    for (i, x) in metadata_set.iter().enumerate() {
        if i > 0 {
            out_metadata.push_str(", ");
        }
        let _ = write!(&mut out_metadata, "{}{}", x.name, x.version);
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

fn collect_set(
    verifier_key: &VerifierKey,
    chainspecs: &Tree,
    metadata: &Tree,
) -> Result<(Vec<MetaValues>, Vec<NetworkSpecs>)> {
    let mut metadata_set = Vec::new();
    let mut network_specs_set = Vec::new();
    let genesis_hash = verifier_key.genesis_hash();
    let mut name_found: Option<String> = None;
    for x in chainspecs.iter().flatten() {
        let network_specs = NetworkSpecs::from_entry_checked(x)?;
        if network_specs.genesis_hash.as_bytes() == &genesis_hash[..] {
            name_found = match name_found {
                Some(n) => {
                    if n != network_specs.name {
                        return Err(db_handling::Error::DifferentNamesSameGenesisHash {
                            name1: n,
                            name2: network_specs.name,
                            genesis_hash: network_specs.genesis_hash,
                        }
                        .into());
                    }
                    Some(n)
                }
                None => Some(network_specs.name.to_string()),
            };
            network_specs_set.push(network_specs);
        }
    }
    if let Some(name) = name_found {
        let meta_key_prefix = MetaKeyPrefix::from_name(&name);
        for y in metadata.scan_prefix(meta_key_prefix.prefix()).flatten() {
            metadata_set.push(MetaValues::from_entry_checked(y)?)
        }
    }
    metadata_set.sort_by(|a, b| a.version.cmp(&b.version));
    network_specs_set.sort_by(|a, b| a.title.cmp(&b.title));
    Ok((metadata_set, network_specs_set))
}

pub(crate) struct GeneralHold {
    pub(crate) metadata_set: Vec<MetaValues>,
    pub(crate) network_specs_set: Vec<NetworkSpecs>,
    pub(crate) types: bool,
}

impl GeneralHold {
    /// function to show entries depending on general verifier
    pub(crate) fn show(&self) -> String {
        let part = print_affected(&self.metadata_set, &self.network_specs_set);
        if self.types {
            format!("{} Types information is purged.", part)
        } else {
            part
        }
    }
    /// function to find all entries in the database that were verified by general verifier
    pub(crate) fn get<P>(db_path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut metadata_set = Vec::new();
        let mut network_specs_set = Vec::new(); // all are verified by general_verifier
        let mut verifier_set = Vec::new();

        let database = open_db(&db_path)?;
        let metadata = open_tree(&database, METATREE)?;
        let chainspecs = open_tree(&database, SPECSTREE)?;
        let settings = open_tree(&database, SETTREE)?;
        let verifiers = open_tree(&database, VERIFIERS)?;
        for (verifier_key_vec, current_verifier_encoded) in verifiers.iter().flatten() {
            let verifier_key = VerifierKey::from_ivec(&verifier_key_vec)?;
            let current_verifier = <CurrentVerifier>::decode(&mut &current_verifier_encoded[..])?;
            if let CurrentVerifier::Valid(ValidCurrentVerifier::General) = current_verifier {
                verifier_set.push(verifier_key)
            }
        }
        for verifier_key in verifier_set.iter() {
            let (new_metadata_set, new_network_specs_set) =
                collect_set(verifier_key, &chainspecs, &metadata)?;
            metadata_set.extend_from_slice(&new_metadata_set);
            network_specs_set.extend_from_slice(&new_network_specs_set);
        }
        let types = settings.contains_key(TYPES)?;
        metadata_set.sort_by(|a, b| a.name.cmp(&b.name));
        network_specs_set.sort_by(|a, b| a.title.cmp(&b.title));
        Ok(Self {
            metadata_set,
            network_specs_set,
            types,
        })
    }
    pub(crate) fn upd_stub<P>(
        &self,
        stub: TrDbColdStub,
        new_general_verifier: &Verifier,
        db_path: P,
    ) -> Result<TrDbColdStub>
    where
        P: AsRef<Path>,
    {
        let former_general_verifier = get_general_verifier(&db_path)?;
        let mut out = stub;
        out = out.new_history_entry(Event::Warning {
            warning: Warning::GeneralVerifierAppeared(self).show(),
        });
        for x in self.metadata_set.iter() {
            out = out.remove_metadata(x)
        }
        for x in self.network_specs_set.iter() {
            out = out.remove_network_specs(
                x,
                &ValidCurrentVerifier::General,
                &former_general_verifier,
            )
        }
        if self.types {
            out = out.remove_types(&prep_types(&db_path)?, &former_general_verifier)
        }
        out = out.new_general_verifier(new_general_verifier);
        Ok(out)
    }
}

pub(crate) struct Hold {
    pub(crate) metadata_set: Vec<MetaValues>,
    pub(crate) network_specs_set: Vec<NetworkSpecs>,
}

impl Hold {
    /// function to show entries depending on former verifier
    pub(crate) fn show(&self) -> String {
        print_affected(&self.metadata_set, &self.network_specs_set)
    }
    /// function to find all entries in the database corresponding to given `verifier_key`, that was used to store the former verifier
    pub(crate) fn get<P>(verifier_key: &VerifierKey, db_path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let database = open_db(&db_path)?;
        let metadata = open_tree(&database, METATREE)?;
        let chainspecs = open_tree(&database, SPECSTREE)?;
        let (metadata_set, network_specs_set) = collect_set(verifier_key, &chainspecs, &metadata)?;
        Ok(Self {
            metadata_set,
            network_specs_set,
        })
    }
    pub(crate) fn upd_stub<P>(
        &self,
        stub: TrDbColdStub,
        verifier_key: &VerifierKey,
        former_verifier: &Verifier,
        new_verifier: &ValidCurrentVerifier,
        hold_release: HoldRelease,
        db_path: P,
    ) -> Result<TrDbColdStub>
    where
        P: AsRef<Path>,
    {
        let general_verifier = get_general_verifier(&db_path)?;
        let mut out = stub;
        let warning = match hold_release {
            HoldRelease::General => Warning::VerifierChangingToGeneral {
                verifier_key,
                hold: self,
            }
            .show(),
            HoldRelease::Custom => Warning::VerifierChangingToCustom {
                verifier_key,
                hold: self,
            }
            .show(),
            HoldRelease::GeneralSuper => Warning::VerifierGeneralSuper {
                verifier_key,
                hold: self,
            }
            .show(),
        };
        out = out.new_history_entry(Event::Warning { warning });
        for x in self.metadata_set.iter() {
            out = out.remove_metadata(x)
        }
        for x in self.network_specs_set.iter() {
            out = out.remove_network_specs(
                x,
                &ValidCurrentVerifier::Custom {
                    v: former_verifier.to_owned(),
                },
                &general_verifier,
            )
        }
        out = out.new_network_verifier(verifier_key, new_verifier, &general_verifier);
        Ok(out)
    }
}

pub(crate) enum HoldRelease {
    General,
    Custom,
    GeneralSuper,
}
