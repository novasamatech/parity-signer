use db_handling::{
    db_transactions::TrDbColdStub,
    helpers::{
        genesis_hash_in_specs, get_general_verifier, open_db, try_get_valid_current_verifier,
    },
};
use definitions::{
    error::TransferContent,
    error_signer::GeneralVerifierForContent,
    history::{Event, MetaValuesDisplay},
    keyring::VerifierKey,
    metadata::MetaValues,
    navigation::{TransactionCard, TransactionCardSet},
    network_specs::{ValidCurrentVerifier, Verifier},
    qr_transfers::ContentLoadMeta,
};
use std::path::Path;

use crate::cards::{Card, Warning};
use crate::check_signature::pass_crypto;
use crate::error::{Error, Result};
use crate::helpers::accept_meta_values;
use crate::{StubNav, TransactionAction};

enum FirstCard {
    WarningCard(TransactionCard),
    VerifierCard(TransactionCard),
}

pub fn load_metadata<P>(data_hex: &str, db_path: P) -> Result<TransactionAction>
where
    P: AsRef<Path>,
{
    let checked_info = pass_crypto(data_hex, TransferContent::LoadMeta)?;
    let (meta, genesis_hash) = ContentLoadMeta::from_slice(&checked_info.message).meta_genhash()?;
    let meta_values = MetaValues::from_slice_metadata(&meta)?;
    let general_verifier = get_general_verifier(&db_path)?;
    let verifier_key = VerifierKey::from_parts(genesis_hash);
    let valid_current_verifier = try_get_valid_current_verifier(&verifier_key, &db_path)?.ok_or(
        Error::LoadMetaUnknownNetwork {
            name: meta_values.name.clone(),
        },
    )?;
    let specs_invariants = genesis_hash_in_specs(genesis_hash, &open_db(&db_path)?)?.ok_or(
        Error::LoadMetaNoSpecs {
            name: meta_values.name.clone(),
            valid_current_verifier: valid_current_verifier.clone(),
            general_verifier: general_verifier.clone(),
        },
    )?;
    if meta_values.name != specs_invariants.name {
        return Err(Error::LoadMetaWrongGenesisHash {
            name_metadata: meta_values.name,
            name_specs: specs_invariants.name,
            genesis_hash,
        });
    }
    if let Some(prefix_from_meta) = meta_values.optional_base58prefix {
        if prefix_from_meta != specs_invariants.base58prefix {
            return Err(
                definitions::error::MetadataError::Base58PrefixSpecsMismatch {
                    specs: specs_invariants.base58prefix,
                    meta: prefix_from_meta,
                }
                .into(),
            );
        }
    }
    let mut stub = TrDbColdStub::new();
    let mut index = 0;
    let optional_ext_warning = {
        if meta_values.warn_incomplete_extensions {
            stub = stub.new_history_entry(Event::Warning {
                warning: Warning::MetadataExtensionsIncomplete.show(),
            });
            Some(Card::Warning(Warning::MetadataExtensionsIncomplete).card(&mut index, 0))
        } else {
            None
        }
    };

    let first_card = match checked_info.verifier {
        Verifier { v: None } => {
            stub = stub.new_history_entry(Event::Warning {
                warning: Warning::NotVerified.show(),
            });
            match valid_current_verifier {
                ValidCurrentVerifier::Custom {
                    v: Verifier { v: None },
                } => (),
                ValidCurrentVerifier::Custom {
                    v:
                        Verifier {
                            v: Some(verifier_value),
                        },
                } => {
                    return Err(Error::NeedVerifier {
                        name: meta_values.name,
                        verifier_value,
                    })
                }
                ValidCurrentVerifier::General => match general_verifier {
                    Verifier { v: None } => (),
                    Verifier {
                        v: Some(verifier_value),
                    } => {
                        return Err(Error::NeedGeneralVerifier {
                            content: GeneralVerifierForContent::Network {
                                name: meta_values.name,
                            },
                            verifier_value,
                        })
                    }
                },
            }
            FirstCard::WarningCard(Card::Warning(Warning::NotVerified).card(&mut index, 0))
        }
        Verifier {
            v: Some(ref new_verifier_value),
        } => {
            match valid_current_verifier {
                ValidCurrentVerifier::Custom { v: a } => {
                    if checked_info.verifier != a {
                        match a {
                            Verifier { v: None } => {
                                return Err(Error::LoadMetaSetVerifier {
                                    name: meta_values.name,
                                    new_verifier_value: new_verifier_value.to_owned(),
                                })
                            }
                            Verifier {
                                v: Some(old_verifier_value),
                            } => {
                                return Err(Error::LoadMetaVerifierChanged {
                                    name: meta_values.name,
                                    old_verifier_value,
                                    new_verifier_value: new_verifier_value.to_owned(),
                                })
                            }
                        }
                    }
                }
                ValidCurrentVerifier::General => {
                    if checked_info.verifier != general_verifier {
                        match general_verifier {
                            Verifier { v: None } => {
                                return Err(Error::LoadMetaSetGeneralVerifier {
                                    name: meta_values.name,
                                    new_general_verifier_value: new_verifier_value.to_owned(),
                                })
                            }
                            Verifier {
                                v: Some(old_general_verifier_value),
                            } => {
                                return Err(Error::LoadMetaGeneralVerifierChanged {
                                    name: meta_values.name,
                                    old_general_verifier_value,
                                    new_general_verifier_value: new_verifier_value.to_owned(),
                                })
                            }
                        }
                    }
                }
            }
            FirstCard::VerifierCard(Card::Verifier(new_verifier_value).card(&mut index, 0))
        }
    };
    if accept_meta_values(&meta_values, &db_path)? {
        stub = stub.add_metadata(&meta_values);
        let checksum = stub.store_and_get_checksum(&db_path)?;
        let meta_display = MetaValuesDisplay::get(&meta_values);
        let meta_card = Card::Meta(meta_display).card(&mut index, 0);
        match first_card {
            FirstCard::WarningCard(warning_card) => match optional_ext_warning {
                Some(ext_warning) => Ok(TransactionAction::Stub {
                    s: Box::new(TransactionCardSet {
                        warning: Some(vec![ext_warning, warning_card]),
                        meta: Some(vec![meta_card]),
                        ..Default::default()
                    }),
                    u: checksum,
                    stub: StubNav::LoadMeta {
                        l: specs_invariants.first_network_specs_key,
                    },
                }),
                None => Ok(TransactionAction::Stub {
                    s: Box::new(TransactionCardSet {
                        warning: Some(vec![warning_card]),
                        meta: Some(vec![meta_card]),
                        ..Default::default()
                    }),
                    u: checksum,
                    stub: StubNav::LoadMeta {
                        l: specs_invariants.first_network_specs_key,
                    },
                }),
            },
            FirstCard::VerifierCard(verifier_card) => match optional_ext_warning {
                Some(ext_warning) => Ok(TransactionAction::Stub {
                    s: Box::new(TransactionCardSet {
                        warning: Some(vec![ext_warning]),
                        verifier: Some(vec![verifier_card]),
                        meta: Some(vec![meta_card]),
                        ..Default::default()
                    }),
                    u: checksum,
                    stub: StubNav::LoadMeta {
                        l: specs_invariants.first_network_specs_key,
                    },
                }),
                None => Ok(TransactionAction::Stub {
                    s: Box::new(TransactionCardSet {
                        verifier: Some(vec![verifier_card]),
                        meta: Some(vec![meta_card]),
                        ..Default::default()
                    }),
                    u: checksum,
                    stub: StubNav::LoadMeta {
                        l: specs_invariants.first_network_specs_key,
                    },
                }),
            },
        }
    } else {
        Err(Error::MetadataKnown {
            name: meta_values.name,
            version: meta_values.version,
        })
    }
}
