use db_handling::{
    db_transactions::TrDbColdStub,
    helpers::{
        genesis_hash_in_specs, get_general_verifier, open_db, try_get_valid_current_verifier,
    },
};
use definitions::{
    error::{
        ErrorSigner, ErrorSource, GeneralVerifierForContent, IncomingMetadataSourceSigner,
        InputSigner, MetadataError, MetadataSource, Signer, TransferContent,
    },
    history::{Event, MetaValuesDisplay},
    keyring::VerifierKey,
    metadata::MetaValues,
    network_specs::{ValidCurrentVerifier, Verifier},
    qr_transfers::ContentLoadMeta,
};

use crate::cards::{Card, Warning};
use crate::check_signature::pass_crypto;
use crate::helpers::accept_meta_values;
use crate::{Action, StubNav};

enum FirstCard {
    WarningCard(String),
    VerifierCard(String),
}

pub fn load_metadata(data_hex: &str, database_name: &str) -> Result<Action, ErrorSigner> {
    let checked_info = pass_crypto(data_hex, TransferContent::LoadMeta)?;
    let (meta, genesis_hash) =
        ContentLoadMeta::from_vec(&checked_info.message).meta_genhash::<Signer>()?;
    let meta_values = match MetaValues::from_vec_metadata(&meta) {
        Ok(a) => a,
        Err(e) => {
            return Err(<Signer>::faulty_metadata(
                e,
                MetadataSource::Incoming(IncomingMetadataSourceSigner::ReceivedData),
            ))
        }
    };
    let general_verifier = get_general_verifier(database_name)?;
    let verifier_key = VerifierKey::from_parts(genesis_hash.as_ref());
    let valid_current_verifier =
        match try_get_valid_current_verifier(&verifier_key, database_name)? {
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
            stub = stub
                .new_history_entry(Event::Warning(Warning::MetadataExtensionsIncomplete.show()));
            Some(Card::Warning(Warning::MetadataExtensionsIncomplete).card(&mut index, 0))
        } else {
            None
        }
    };

    let first_card = match checked_info.verifier {
        Verifier(None) => {
            stub = stub.new_history_entry(Event::Warning(Warning::NotVerified.show()));
            match valid_current_verifier {
                ValidCurrentVerifier::Custom(Verifier(None)) => (),
                ValidCurrentVerifier::Custom(Verifier(Some(verifier_value))) => {
                    return Err(ErrorSigner::Input(InputSigner::NeedVerifier {
                        name: meta_values.name,
                        verifier_value,
                    }))
                }
                ValidCurrentVerifier::General => match general_verifier {
                    Verifier(None) => (),
                    Verifier(Some(verifier_value)) => {
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
        Verifier(Some(ref new_verifier_value)) => {
            match valid_current_verifier {
                ValidCurrentVerifier::Custom(a) => {
                    if checked_info.verifier != a {
                        match a {
                            Verifier(None) => {
                                return Err(ErrorSigner::Input(InputSigner::LoadMetaSetVerifier {
                                    name: meta_values.name,
                                    new_verifier_value: new_verifier_value.to_owned(),
                                }))
                            }
                            Verifier(Some(old_verifier_value)) => {
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
                            Verifier(None) => {
                                return Err(ErrorSigner::Input(
                                    InputSigner::LoadMetaSetGeneralVerifier {
                                        name: meta_values.name,
                                        new_general_verifier_value: new_verifier_value.to_owned(),
                                    },
                                ))
                            }
                            Verifier(Some(old_general_verifier_value)) => {
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
                Some(ext_warning) => Ok(Action::Stub(
                    format!(
                        "\"warning\":[{},{}],\"meta\":[{}]",
                        ext_warning, warning_card, meta_card
                    ),
                    checksum,
                    StubNav::LoadMeta(network_specs_key),
                )),
                None => Ok(Action::Stub(
                    format!("\"warning\":[{}],\"meta\":[{}]", warning_card, meta_card),
                    checksum,
                    StubNav::LoadMeta(network_specs_key),
                )),
            },
            FirstCard::VerifierCard(verifier_card) => match optional_ext_warning {
                Some(ext_warning) => Ok(Action::Stub(
                    format!(
                        "\"warning\":[{}],\"verifier\":[{}],\"meta\":[{}]",
                        ext_warning, verifier_card, meta_card
                    ),
                    checksum,
                    StubNav::LoadMeta(network_specs_key),
                )),
                None => Ok(Action::Stub(
                    format!("\"verifier\":[{}],\"meta\":[{}]", verifier_card, meta_card),
                    checksum,
                    StubNav::LoadMeta(network_specs_key),
                )),
            },
        }
    } else {
        Err(ErrorSigner::Input(InputSigner::MetadataKnown {
            name: meta_values.name,
            version: meta_values.version,
        }))
    }
}
