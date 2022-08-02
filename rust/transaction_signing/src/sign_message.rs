use parity_scale_codec::Encode;
use sp_core::{ecdsa, ed25519, sr25519, Pair};
use sp_runtime::MultiSigner;
use zeroize::Zeroize;

use crate::{Error, Result};
use db_handling::{
    db_transactions::TrDbCold,
    helpers::{get_meta_values_by_name_version, get_network_specs, prep_types},
    manage_history::events_to_batch,
};
use definitions::{
    crypto::SufficientCrypto,
    history::{Event, MetaValuesExport, NetworkSpecsExport, TypesExport},
    keyring::NetworkSpecsKey,
    navigation::{MSCContent, MSCNetworkInfo},
    qr_transfers::{ContentAddSpecs, ContentLoadMeta},
    users::AddressDetails,
};
use qrcode_static::{png_qr, DataType};

pub(crate) fn sign_as_address_key(
    to_sign: &[u8],
    multisigner: &MultiSigner,
    full_address: &str,
    pwd: Option<&str>,
) -> Result<SufficientCrypto> {
    match multisigner {
        MultiSigner::Ed25519(public) => {
            let ed25519_pair =
                ed25519::Pair::from_string(full_address, pwd).map_err(Error::CryptoError)?;
            if public != &ed25519_pair.public() {
                return Err(Error::WrongPassword);
            }
            // secret zeroize on drop, https://docs.rs/ed25519-dalek/1.0.1/src/ed25519_dalek/secret.rs.html#43
            let signature = ed25519_pair.sign(to_sign);
            Ok(SufficientCrypto::Ed25519 {
                public: public.to_owned(),
                signature,
            })
        }
        MultiSigner::Sr25519(public) => {
            let sr25519_pair =
                sr25519::Pair::from_string(full_address, pwd).map_err(Error::CryptoError)?;
            if public != &sr25519_pair.public() {
                return Err(Error::WrongPassword);
            }
            // pair zeroize on drop, https://docs.rs/schnorrkel/0.9.1/src/schnorrkel/keys.rs.html#680
            let signature = sr25519_pair.sign(to_sign);
            Ok(SufficientCrypto::Sr25519 {
                public: public.to_owned(),
                signature,
            })
        }
        MultiSigner::Ecdsa(public) => {
            let ecdsa_pair =
                ecdsa::Pair::from_string(full_address, pwd).map_err(Error::CryptoError)?;
            if public != &ecdsa_pair.public() {
                return Err(Error::WrongPassword);
            }
            let signature = ecdsa_pair.sign(to_sign);
            Ok(SufficientCrypto::Ecdsa {
                public: public.to_owned(),
                signature,
            })
        }
    }
}

/// Function to generate `sufficient crypto line` for given public key
fn sufficient_crypto(
    multisigner: &MultiSigner,
    address_details: &AddressDetails,
    to_sign: &[u8],
    seed_phrase: &str,
    pwd_entry: &str,
) -> Result<SufficientCrypto> {
    let pwd = {
        if address_details.has_pwd {
            Some(pwd_entry)
        } else {
            None
        }
    };
    let mut full_address = seed_phrase.to_owned() + &address_details.path;
    match sign_as_address_key(to_sign, multisigner, &full_address, pwd) {
        Ok(a) => {
            full_address.zeroize();
            Ok(a)
        }
        Err(e) => {
            full_address.zeroize();
            Err(e)
        }
    }
}

/// Function to generate hex line of qr data corresponding to `sufficient_crypto` for `load_types` message
pub(crate) fn sufficient_crypto_load_types(
    multisigner: &MultiSigner,
    address_details: &AddressDetails,
    database_name: &str,
    seed_phrase: &str,
    pwd_entry: &str,
) -> Result<(Vec<u8>, MSCContent)> {
    let types_content = prep_types(database_name)?;
    let sufficient = match sufficient_crypto(
        multisigner,
        address_details,
        &types_content.to_sign(),
        seed_phrase,
        pwd_entry,
    ) {
        Ok(s) => {
            TrDbCold::new()
                .set_history(events_to_batch(
                    database_name,
                    vec![Event::TypesSigned {
                        types_export: TypesExport::get(&types_content, &s.verifier_value()),
                    }],
                )?)
                .apply(database_name)?;
            qr_from_sufficient(s)?
        }
        Err(e) => {
            if let Error::WrongPassword = e {
                TrDbCold::new()
                    .set_history(events_to_batch(database_name, vec![Event::WrongPassword])?)
                    .apply(database_name)?;
            }
            return Err(e);
        }
    };
    let (types, pic) = types_content.show();
    Ok((sufficient, MSCContent::LoadTypes { types, pic }))
}

/// Function to generate hex line of qr data corresponding to `sufficient_crypto` for `load_metadata` message
pub(crate) fn sufficient_crypto_load_metadata(
    network_specs_key: &NetworkSpecsKey,
    network_version: u32,
    multisigner: &MultiSigner,
    address_details: &AddressDetails,
    database_name: &str,
    seed_phrase: &str,
    pwd_entry: &str,
) -> Result<(Vec<u8>, MSCContent)> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let meta_values =
        get_meta_values_by_name_version(database_name, &network_specs.name, network_version)?;
    let load_meta_content =
        ContentLoadMeta::generate(&meta_values.meta, &network_specs.genesis_hash);
    let sufficient = match sufficient_crypto(
        multisigner,
        address_details,
        &load_meta_content.to_sign(),
        seed_phrase,
        pwd_entry,
    ) {
        Ok(s) => {
            TrDbCold::new()
                .set_history(events_to_batch(
                    database_name,
                    vec![Event::MetadataSigned {
                        meta_values_export: MetaValuesExport::get(
                            &meta_values,
                            &s.verifier_value(),
                        ),
                    }],
                )?)
                .apply(database_name)?;
            qr_from_sufficient(s)?
        }
        Err(e) => {
            if let Error::WrongPassword = e {
                TrDbCold::new()
                    .set_history(events_to_batch(database_name, vec![Event::WrongPassword])?)
                    .apply(database_name)?;
            }
            return Err(e);
        }
    };
    Ok((
        sufficient,
        MSCContent::LoadMetadata {
            name: meta_values.name,
            version: meta_values.version,
        },
    ))
}

/// Function to generate hex line of qr data corresponding to `sufficient_crypto` for `add_specs` message
pub(crate) fn sufficient_crypto_add_specs(
    network_specs_key: &NetworkSpecsKey,
    multisigner: &MultiSigner,
    address_details: &AddressDetails,
    database_name: &str,
    seed_phrase: &str,
    pwd_entry: &str,
) -> Result<(Vec<u8>, MSCContent)> {
    let network_specs_to_send = get_network_specs(database_name, network_specs_key)?.to_send();
    let add_specs_content = ContentAddSpecs::generate(&network_specs_to_send);
    let sufficient = match sufficient_crypto(
        multisigner,
        address_details,
        &add_specs_content.to_sign(),
        seed_phrase,
        pwd_entry,
    ) {
        Ok(s) => {
            TrDbCold::new()
                .set_history(events_to_batch(
                    database_name,
                    vec![Event::NetworkSpecsSigned {
                        network_specs_export: NetworkSpecsExport::get(
                            &network_specs_to_send,
                            &s.verifier_value(),
                        ),
                    }],
                )?)
                .apply(database_name)?;
            qr_from_sufficient(s)?
        }
        Err(e) => {
            if let Error::WrongPassword = e {
                TrDbCold::new()
                    .set_history(events_to_batch(database_name, vec![Event::WrongPassword])?)
                    .apply(database_name)?;
            }
            return Err(e);
        }
    };
    Ok((
        sufficient,
        MSCContent::AddSpecs {
            f: MSCNetworkInfo {
                network_title: network_specs_to_send.title,
                network_logo: network_specs_to_send.logo,
            },
        },
    ))
}

fn qr_from_sufficient(sufficient: SufficientCrypto) -> Result<Vec<u8>> {
    Ok(png_qr(&sufficient.encode(), DataType::Regular)?)
}
