use hex;
use sp_core::{Pair, ed25519, sr25519, ecdsa};
use sp_runtime::{MultiSigner, MultiSignature};
use definitions::{crypto::SufficientCrypto, error::{AddressKeySource, ErrorSigner, ExtraAddressKeySourceSigner, Signer}, history::{Event, MetaValuesExport, NetworkSpecsExport, TypesExport}, keyring::AddressKey, qr_transfers::{ContentAddSpecs, ContentLoadMeta}};
use parity_scale_codec::Encode;
use db_handling::{db_transactions::TrDbCold, helpers::{get_address_details, get_meta_values_by_name_version}, manage_history::{events_to_batch}, network_details::get_network_specs_by_hex_key, prep_messages::{prep_types, get_genesis_hash}};
use qrcode_static::png_qr;
use zeroize::Zeroize;


pub fn sign_as_address_key (to_sign: &Vec<u8>, address_key: AddressKey, full_address: &str, pwd: Option<&str>) -> Result<SufficientCrypto, ErrorSigner> {
    match address_key.multi_signer::<Signer>(AddressKeySource::Extra(ExtraAddressKeySourceSigner::Interface))? {
        MultiSigner::Ed25519(public) => {
            let ed25519_pair = match ed25519::Pair::from_string(full_address, pwd) {
                Ok(x) => x,
                Err(e) => return Err(ErrorSigner::AddressUse(e)),
            };
            if public != ed25519_pair.public() {return Err(ErrorSigner::WrongPassword)}
            // secret zeroize on drop, https://docs.rs/ed25519-dalek/1.0.1/src/ed25519_dalek/secret.rs.html#43
            let signature = ed25519_pair.sign(&to_sign[..]);
            Ok(SufficientCrypto::Ed25519{public, signature})
        },
        MultiSigner::Sr25519(public) => {
            let sr25519_pair = match sr25519::Pair::from_string(full_address, pwd) {
                Ok(x) => x,
                Err(e) => return Err(ErrorSigner::AddressUse(e)),
            };
            if public != sr25519_pair.public() {return Err(ErrorSigner::WrongPassword)}
            // pair zeroize on drop, https://docs.rs/schnorrkel/0.9.1/src/schnorrkel/keys.rs.html#680
            let signature = sr25519_pair.sign(&to_sign[..]);
            Ok(SufficientCrypto::Sr25519{public, signature})
        },
        MultiSigner::Ecdsa(public) => {
            let ecdsa_pair = match ecdsa::Pair::from_string(full_address, pwd) {
                Ok(x) => x,
                Err(e) => return Err(ErrorSigner::AddressUse(e)),
            };
            if public != ecdsa_pair.public() {return Err(ErrorSigner::WrongPassword)}
            let signature = ecdsa_pair.sign(&to_sign[..]);
            Ok(SufficientCrypto::Ecdsa{public, signature})
        },
    }
}

/// Function to generate `sufficient crypto line` for given public key
fn sufficient_crypto (address_key_hex: &str, to_sign: &Vec<u8>, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> Result<SufficientCrypto, ErrorSigner> {
    let address_key = AddressKey::from_hex(address_key_hex)?;
    let address_details = get_address_details(database_name, &address_key)?;
    let pwd = {
        if address_details.has_pwd {Some(pwd_entry)}
        else {None}
    };
    let mut full_address = seed_phrase.to_owned() + &address_details.path;
    match sign_as_address_key(to_sign, address_key, &full_address, pwd) {
        Ok(a) => {
            full_address.zeroize();
            Ok(a)
        },
        Err(e) => {
            full_address.zeroize();
            return Err(e)
        },
    }
}

/// Function to generate hex line of qr data corresponding to `sufficient_crypto` for load_types message
pub fn sufficient_crypto_load_types (address_key_hex: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> Result<String, ErrorSigner> {
    let types_content = prep_types::<Signer>(database_name)?;
    match sufficient_crypto (address_key_hex, &types_content.to_sign(), database_name, seed_phrase, pwd_entry) {
        Ok(s) => {
            TrDbCold::new()
                .set_history(events_to_batch::<Signer>(&database_name, vec![Event::TypesSigned(TypesExport::get(&types_content, &s.get_verifier_value()))])?)
                .apply::<Signer>(&database_name)?;
            hex_qr_from_signature(s.get_multi_signature())
        },
        Err(e) => {
            if let ErrorSigner::WrongPassword = e {
                TrDbCold::new()
                    .set_history(events_to_batch::<Signer>(&database_name, vec![Event::WrongPassword])?)
                    .apply::<Signer>(&database_name)?;
            }
            return Err(e)
        },
    }
}

/// Function to generate hex line of qr data corresponding to `sufficient_crypto` for load_metadata message
pub fn sufficient_crypto_load_metadata (network_name: &str, network_version: u32, address_key_hex: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> Result<String, ErrorSigner> {
    let meta_values = get_meta_values_by_name_version::<Signer>(database_name, network_name, network_version)?;
    let genesis_hash = get_genesis_hash(network_name, database_name)?;
    let load_meta_content = ContentLoadMeta::generate(&meta_values.meta, &genesis_hash);
    match sufficient_crypto (address_key_hex, &load_meta_content.to_sign(), database_name, seed_phrase, pwd_entry) {
        Ok(s) => {
            TrDbCold::new()
                .set_history(events_to_batch::<Signer>(&database_name, vec![Event::MetadataSigned(MetaValuesExport::get(&meta_values, &s.get_verifier_value()))])?)
                .apply::<Signer>(&database_name)?;
            hex_qr_from_signature(s.get_multi_signature())
        },
        Err(e) => {
            if let ErrorSigner::WrongPassword = e {
                TrDbCold::new()
                    .set_history(events_to_batch::<Signer>(&database_name, vec![Event::WrongPassword])?)
                    .apply::<Signer>(&database_name)?;
            }
            return Err(e)
        },
    }
}

/// Function to generate hex line of qr data corresponding to `sufficient_crypto` for add_specs message
pub fn sufficient_crypto_add_specs (network_specs_key_hex: &str, address_key_hex: &str, database_name: &str, seed_phrase: &str, pwd_entry: &str) -> Result<String, ErrorSigner> {
    let network_specs_to_send = get_network_specs_by_hex_key(database_name, network_specs_key_hex)?.to_send();
    let add_specs_content = ContentAddSpecs::generate(&network_specs_to_send);
    match sufficient_crypto (address_key_hex, &add_specs_content.to_sign(), database_name, seed_phrase, pwd_entry) {
        Ok(s) => {
            TrDbCold::new()
                .set_history(events_to_batch::<Signer>(&database_name, vec![Event::NetworkSpecsSigned(NetworkSpecsExport::get(&network_specs_to_send, &s.get_verifier_value()))])?)
                .apply::<Signer>(&database_name)?;
            hex_qr_from_signature(s.get_multi_signature())
        },
        Err(e) => {
            if let ErrorSigner::WrongPassword = e {
                TrDbCold::new()
                    .set_history(events_to_batch::<Signer>(&database_name, vec![Event::WrongPassword])?)
                    .apply::<Signer>(&database_name)?;
            }
            return Err(e)
        },
    }
}

fn hex_qr_from_signature(signature: MultiSignature) -> Result<String, ErrorSigner> {
    let qr_data = match png_qr(&signature.encode()) {
        Ok(a) => a,
        Err(e) => return Err(ErrorSigner::Qr(e.to_string())),
    };
    Ok(hex::encode(qr_data))
}
