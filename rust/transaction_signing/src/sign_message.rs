use db_handling::{
    db_transactions::TrDbCold,
    helpers::{get_meta_values_by_name_version, get_network_specs},
    manage_history::events_to_batch,
    prep_messages::{get_genesis_hash, prep_types},
};
use definitions::{
    crypto::SufficientCrypto,
    error::{ErrorSigner, Signer},
    history::{Event, MetaValuesDisplay, MetaValuesExport, NetworkSpecsExport, TypesExport},
    keyring::NetworkSpecsKey,
    qr_transfers::{ContentAddSpecs, ContentLoadMeta},
    users::AddressDetails,
};

use parity_scale_codec::Encode;
use qrcode_static::png_qr;
use sp_core::{ecdsa, ed25519, sr25519, Pair};
use sp_runtime::MultiSigner;
use zeroize::Zeroize;

pub(crate) fn sign_as_address_key(
    to_sign: &[u8],
    multisigner: &MultiSigner,
    full_address: &str,
    pwd: Option<&str>,
) -> Result<SufficientCrypto, ErrorSigner> {
    match multisigner {
        MultiSigner::Ed25519(public) => {
            let ed25519_pair = match ed25519::Pair::from_string(full_address, pwd) {
                Ok(x) => x,
                Err(e) => return Err(ErrorSigner::AddressUse(e)),
            };
            if public != &ed25519_pair.public() {
                return Err(ErrorSigner::WrongPassword);
            }
            // secret zeroize on drop, https://docs.rs/ed25519-dalek/1.0.1/src/ed25519_dalek/secret.rs.html#43
            let signature = ed25519_pair.sign(to_sign);
            Ok(SufficientCrypto::Ed25519 {
                public: public.to_owned(),
                signature,
            })
        }
        MultiSigner::Sr25519(public) => {
            let sr25519_pair = match sr25519::Pair::from_string(full_address, pwd) {
                Ok(x) => x,
                Err(e) => return Err(ErrorSigner::AddressUse(e)),
            };
            if public != &sr25519_pair.public() {
                return Err(ErrorSigner::WrongPassword);
            }
            // pair zeroize on drop, https://docs.rs/schnorrkel/0.9.1/src/schnorrkel/keys.rs.html#680
            let signature = sr25519_pair.sign(to_sign);
            Ok(SufficientCrypto::Sr25519 {
                public: public.to_owned(),
                signature,
            })
        }
        MultiSigner::Ecdsa(public) => {
            let ecdsa_pair = match ecdsa::Pair::from_string(full_address, pwd) {
                Ok(x) => x,
                Err(e) => return Err(ErrorSigner::AddressUse(e)),
            };
            if public != &ecdsa_pair.public() {
                return Err(ErrorSigner::WrongPassword);
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
) -> Result<SufficientCrypto, ErrorSigner> {
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

/// Function to generate hex line of qr data corresponding to `sufficient_crypto` for load_types message
pub(crate) fn sufficient_crypto_load_types(
    multisigner: &MultiSigner,
    address_details: &AddressDetails,
    database_name: &str,
    seed_phrase: &str,
    pwd_entry: &str,
) -> Result<String, ErrorSigner> {
    let types_content = prep_types::<Signer>(database_name)?;
    let sufficient = match sufficient_crypto(
        multisigner,
        address_details,
        &types_content.to_sign(),
        seed_phrase,
        pwd_entry,
    ) {
        Ok(s) => {
            TrDbCold::new()
                .set_history(events_to_batch::<Signer>(
                    database_name,
                    vec![Event::TypesSigned(TypesExport::get(
                        &types_content,
                        &s.get_verifier_value(),
                    ))],
                )?)
                .apply::<Signer>(database_name)?;
            hex_qr_from_sufficient(s)?
        }
        Err(e) => {
            if let ErrorSigner::WrongPassword = e {
                TrDbCold::new()
                    .set_history(events_to_batch::<Signer>(
                        database_name,
                        vec![Event::WrongPassword],
                    )?)
                    .apply::<Signer>(database_name)?;
            }
            return Err(e);
        }
    };
    Ok(format!(
        "\"sufficient\":\"{}\",\"content\":{{\"type\":\"load_types\",{}}}",
        sufficient,
        types_content.show()
    ))
}

/// Function to generate hex line of qr data corresponding to `sufficient_crypto` for load_metadata message
pub(crate) fn sufficient_crypto_load_metadata(
    network_specs_key: &NetworkSpecsKey,
    network_version: u32,
    multisigner: &MultiSigner,
    address_details: &AddressDetails,
    database_name: &str,
    seed_phrase: &str,
    pwd_entry: &str,
) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let meta_values = get_meta_values_by_name_version::<Signer>(
        database_name,
        &network_specs.name,
        network_version,
    )?;
    let genesis_hash = get_genesis_hash(&network_specs.name, database_name)?;
    let load_meta_content = ContentLoadMeta::generate(&meta_values.meta, &genesis_hash);
    let sufficient = match sufficient_crypto(
        multisigner,
        address_details,
        &load_meta_content.to_sign(),
        seed_phrase,
        pwd_entry,
    ) {
        Ok(s) => {
            TrDbCold::new()
                .set_history(events_to_batch::<Signer>(
                    database_name,
                    vec![Event::MetadataSigned(MetaValuesExport::get(
                        &meta_values,
                        &s.get_verifier_value(),
                    ))],
                )?)
                .apply::<Signer>(database_name)?;
            hex_qr_from_sufficient(s)?
        }
        Err(e) => {
            if let ErrorSigner::WrongPassword = e {
                TrDbCold::new()
                    .set_history(events_to_batch::<Signer>(
                        database_name,
                        vec![Event::WrongPassword],
                    )?)
                    .apply::<Signer>(database_name)?;
            }
            return Err(e);
        }
    };
    Ok(format!(
        "\"sufficient\":\"{}\",\"content\":{{\"type\":\"load_metadata\",{}}}",
        sufficient,
        MetaValuesDisplay::get(&meta_values).show()
    ))
}

/// Function to generate hex line of qr data corresponding to `sufficient_crypto` for add_specs message
pub(crate) fn sufficient_crypto_add_specs(
    network_specs_key: &NetworkSpecsKey,
    multisigner: &MultiSigner,
    address_details: &AddressDetails,
    database_name: &str,
    seed_phrase: &str,
    pwd_entry: &str,
) -> Result<String, ErrorSigner> {
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
                .set_history(events_to_batch::<Signer>(
                    database_name,
                    vec![Event::NetworkSpecsSigned(NetworkSpecsExport::get(
                        &network_specs_to_send,
                        &s.get_verifier_value(),
                    ))],
                )?)
                .apply::<Signer>(database_name)?;
            hex_qr_from_sufficient(s)?
        }
        Err(e) => {
            if let ErrorSigner::WrongPassword = e {
                TrDbCold::new()
                    .set_history(events_to_batch::<Signer>(
                        database_name,
                        vec![Event::WrongPassword],
                    )?)
                    .apply::<Signer>(database_name)?;
            }
            return Err(e);
        }
    };
    Ok(format!("\"sufficient\":\"{}\",\"content\":{{\"type\":\"add_specs\",\"network_title\":\"{}\",\"network_logo\":\"{}\"}}", sufficient, network_specs_to_send.title, network_specs_to_send.logo))
}

fn hex_qr_from_sufficient(sufficient: SufficientCrypto) -> Result<String, ErrorSigner> {
    let qr_data = match png_qr(&sufficient.encode()) {
        Ok(a) => a,
        Err(e) => return Err(ErrorSigner::Qr(e.to_string())),
    };
    Ok(hex::encode(qr_data))
}
