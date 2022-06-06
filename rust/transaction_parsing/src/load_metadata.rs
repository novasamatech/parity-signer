use db_handling::{
    db_transactions::TrDbColdStub,
    helpers::{
        genesis_hash_in_specs, get_general_verifier, open_db, try_get_valid_current_verifier,
    },
};
use definitions::{
    error::{ErrorSource, MetadataError, MetadataSource, TransferContent},
    error_signer::{
        ErrorSigner, GeneralVerifierForContent, IncomingMetadataSourceSigner, InputSigner, Signer,
    },
    history::{Event, MetaValuesDisplay},
    keyring::VerifierKey,
    metadata::MetaValues,
    navigation::{TransactionCard, TransactionCardSet},
    network_specs::{ValidCurrentVerifier, Verifier},
    qr_transfers::ContentLoadMeta,
};

use crate::cards::{Card, Warning};
use crate::check_signature::pass_crypto;
use crate::helpers::accept_meta_values;
use crate::{StubNav, TransactionAction};

enum FirstCard {
    WarningCard(TransactionCard),
    VerifierCard(TransactionCard),
}

pub fn load_metadata(
    data_hex: &str,
    database_name: &str,
) -> Result<TransactionAction, ErrorSigner> {
    let checked_info = pass_crypto(data_hex, TransferContent::LoadMeta)?;
    let (meta, genesis_hash) =
        ContentLoadMeta::from_slice(&checked_info.message).meta_genhash::<Signer>()?;
    let meta_values = match MetaValues::from_slice_metadata(&meta) {
        Ok(a) => a,
        Err(e) => {
            return Err(<Signer>::faulty_metadata(
                e,
                MetadataSource::Incoming(IncomingMetadataSourceSigner::ReceivedData),
            ))
        }
    };
    let general_verifier = get_general_verifier(database_name)?;
    let verifier_key = VerifierKey::from_parts(genesis_hash);
    let valid_current_verifier = match try_get_valid_current_verifier(&verifier_key, database_name)?
    {
        Some(a) => a,
        None => {
            return Err(ErrorSigner::Input(InputSigner::LoadMetaUnknownNetwork {
                name: meta_values.name,
            }))
        }
    };
    let (network_specs_key, network_specs) =
        match genesis_hash_in_specs(&verifier_key, &open_db::<Signer>(database_name)?)? {
            Some(a) => a,
            None => {
                return Err(ErrorSigner::Input(InputSigner::LoadMetaNoSpecs {
                    name: meta_values.name,
                    valid_current_verifier,
                    general_verifier,
                }))
            }
        };
    if let Some(prefix_from_meta) = meta_values.optional_base58prefix {
        if prefix_from_meta != network_specs.base58prefix {
            return Err(<Signer>::faulty_metadata(
                MetadataError::Base58PrefixSpecsMismatch {
                    specs: network_specs.base58prefix,
                    meta: prefix_from_meta,
                },
                MetadataSource::Incoming(IncomingMetadataSourceSigner::ReceivedData),
            ));
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
                    return Err(ErrorSigner::Input(InputSigner::NeedVerifier {
                        name: meta_values.name,
                        verifier_value,
                    }))
                }
                ValidCurrentVerifier::General => match general_verifier {
                    Verifier { v: None } => (),
                    Verifier {
                        v: Some(verifier_value),
                    } => {
                        return Err(ErrorSigner::Input(InputSigner::NeedGeneralVerifier {
                            content: GeneralVerifierForContent::Network {
                                name: meta_values.name,
                            },
                            verifier_value,
                        }))
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
                                return Err(ErrorSigner::Input(InputSigner::LoadMetaSetVerifier {
                                    name: meta_values.name,
                                    new_verifier_value: new_verifier_value.to_owned(),
                                }))
                            }
                            Verifier {
                                v: Some(old_verifier_value),
                            } => {
                                return Err(ErrorSigner::Input(
                                    InputSigner::LoadMetaVerifierChanged {
                                        name: meta_values.name,
                                        old_verifier_value,
                                        new_verifier_value: new_verifier_value.to_owned(),
                                    },
                                ))
                            }
                        }
                    }
                }
                ValidCurrentVerifier::General => {
                    if checked_info.verifier != general_verifier {
                        match general_verifier {
                            Verifier { v: None } => {
                                return Err(ErrorSigner::Input(
                                    InputSigner::LoadMetaSetGeneralVerifier {
                                        name: meta_values.name,
                                        new_general_verifier_value: new_verifier_value.to_owned(),
                                    },
                                ))
                            }
                            Verifier {
                                v: Some(old_general_verifier_value),
                            } => {
                                return Err(ErrorSigner::Input(
                                    InputSigner::LoadMetaGeneralVerifierChanged {
                                        name: meta_values.name,
                                        old_general_verifier_value,
                                        new_general_verifier_value: new_verifier_value.to_owned(),
                                    },
                                ))
                            }
                        }
                    }
                }
            }
            FirstCard::VerifierCard(Card::Verifier(new_verifier_value).card(&mut index, 0))
        }
    };
    if accept_meta_values(&meta_values, database_name)? {
        stub = stub.add_metadata(&meta_values);
        let checksum = stub.store_and_get_checksum(database_name)?;
        let meta_display = MetaValuesDisplay::get(&meta_values);
        let meta_card = Card::Meta(meta_display).card(&mut index, 0);
        match first_card {
            FirstCard::WarningCard(warning_card) => match optional_ext_warning {
                Some(ext_warning) => Ok(TransactionAction::Stub {
                    s: TransactionCardSet {
                        warning: Some(vec![ext_warning, warning_card]),
                        meta: Some(vec![meta_card]),
                        ..Default::default()
                    },
                    u: checksum,
                    stub: StubNav::LoadMeta {
                        l: network_specs_key,
                    },
                }),
                None => Ok(TransactionAction::Stub {
                    s: TransactionCardSet {
                        warning: Some(vec![warning_card]),
                        meta: Some(vec![meta_card]),
                        ..Default::default()
                    },
                    u: checksum,
                    stub: StubNav::LoadMeta {
                        l: network_specs_key,
                    },
                }),
            },
            FirstCard::VerifierCard(verifier_card) => match optional_ext_warning {
                Some(ext_warning) => Ok(TransactionAction::Stub {
                    s: TransactionCardSet {
                        warning: Some(vec![ext_warning]),
                        verifier: Some(vec![verifier_card]),
                        meta: Some(vec![meta_card]),
                        ..Default::default()
                    },
                    u: checksum,
                    stub: StubNav::LoadMeta {
                        l: network_specs_key,
                    },
                }),
                None => Ok(TransactionAction::Stub {
                    s: TransactionCardSet {
                        verifier: Some(vec![verifier_card]),
                        meta: Some(vec![meta_card]),
                        ..Default::default()
                    },
                    u: checksum,
                    stub: StubNav::LoadMeta {
                        l: network_specs_key,
                    },
                }),
            },
        }
    } else {
        Err(ErrorSigner::Input(InputSigner::MetadataKnown {
            name: meta_values.name,
            version: meta_values.version,
        }))
    }
}
