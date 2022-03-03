use parity_scale_codec::{Decode, Encode};
use plot_icon::EMPTY_PNG;
use sled::IVec;
use sp_runtime::MultiSigner;

use crate::crypto::Encryption;
use crate::error::{Active, DatabaseActive, EntryDecodingActive, ErrorActive, ErrorSource, MismatchActive, SpecsKeySource};
use crate::helpers::{multisigner_to_public, multisigner_to_encryption, make_identicon_from_multisigner};
use crate::keyring::NetworkSpecsKey;
use crate::print::export_complex_single;

//TODO: rename fields to make them more clear
#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub struct NetworkSpecs {
    pub base58prefix: u16,
    pub color: String,
    pub decimals: u8,
    pub encryption: Encryption,
    pub genesis_hash: [u8; 32],
    pub logo: String,
    pub name: String,
    pub order: u8,
    pub path_id: String,
    pub secondary_color: String,
    pub title: String,
    pub unit: String,
}


#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub struct NetworkSpecsToSend {
    pub base58prefix: u16,
    pub color: String,
    pub decimals: u8,
    pub encryption: Encryption,
    pub genesis_hash: [u8; 32],
    pub logo: String,
    pub name: String,
    pub path_id: String,
    pub secondary_color: String,
    pub title: String,
    pub unit: String,
}

#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub struct ShortSpecs {
    pub base58prefix: u16,
    pub decimals: u8,
    pub genesis_hash: [u8; 32],
    pub name: String,
    pub unit: String,
}

impl NetworkSpecs {
    pub fn show(&self, valid_current_verifier: &ValidCurrentVerifier, general_verifier: &Verifier) -> String {
        format!("\"base58prefix\":\"{}\",\"color\":\"{}\",\"decimals\":\"{}\",\"encryption\":\"{}\",\"genesis_hash\":\"{}\",\"logo\":\"{}\",\"name\":\"{}\",\"order\":\"{}\",\"path_id\":\"{}\",\"secondary_color\":\"{}\",\"title\":\"{}\",\"unit\":\"{}\",\"current_verifier\":{}", &self.base58prefix, &self.color, &self.decimals, &self.encryption.show(), hex::encode(&self.genesis_hash), &self.logo, &self.name, &self.order, &self.path_id, &self.secondary_color, &self.title, &self.unit, export_complex_single(valid_current_verifier, |a| a.show(general_verifier)))
    }
    pub fn print_single(&self) -> String {
        format!("\"color\":\"{}\",\"logo\":\"{}\",\"secondary_color\":\"{}\",\"title\":\"{}\"", &self.color, &self.logo, &self.secondary_color, &self.title)
    }
    pub fn print_as_set_part(&self) -> String {
        format!("\"key\":\"{}\",\"color\":\"{}\",\"logo\":\"{}\",\"order\":\"{}\",\"secondary_color\":\"{}\",\"title\":\"{}\"", hex::encode(NetworkSpecsKey::from_parts(&self.genesis_hash, &self.encryption).key()), &self.color, &self.logo, &self.order, &self.secondary_color, &self.title)
    }
    pub fn to_send(&self) -> NetworkSpecsToSend {
        NetworkSpecsToSend {
            base58prefix: self.base58prefix,
            color: self.color.to_string(),
            decimals: self.decimals,
            encryption: self.encryption.to_owned(),
            genesis_hash: self.genesis_hash,
            logo: self.logo.to_string(),
            name: self.name.to_string(),
            path_id: self.path_id.to_string(),
            secondary_color: self.secondary_color.to_string(),
            title: self.title.to_string(),
            unit: self.unit.to_string(),
        }
    }
    pub fn short(&self) -> ShortSpecs {
        ShortSpecs {
            base58prefix: self.base58prefix,
            decimals: self.decimals,
            genesis_hash: self.genesis_hash,
            name: self.name.to_string(),
            unit: self.unit.to_string(),
        }
    }
    pub fn from_entry_with_key_checked<T: ErrorSource> (network_specs_key: &NetworkSpecsKey, network_specs_encoded: IVec) -> Result<Self, T::Error> {
        let (genesis_hash_vec, encryption) = network_specs_key.genesis_hash_encryption::<T>(SpecsKeySource::SpecsTree)?;
        let network_specs = match Self::decode(&mut &network_specs_encoded[..]) {
            Ok(a) => a,
            Err(_) => return Err(<T>::specs_decoding(network_specs_key.to_owned())),
        };
        if genesis_hash_vec[..] != network_specs.genesis_hash {return Err(<T>::specs_genesis_hash_mismatch(network_specs_key.to_owned(), network_specs.genesis_hash.to_vec()))}
        if encryption != network_specs.encryption {return Err(<T>::specs_encryption_mismatch(network_specs_key.to_owned(), network_specs.encryption))}
        Ok(network_specs)
    }
    pub fn from_entry_checked<T: ErrorSource> ((network_specs_key_vec, network_specs_encoded): (IVec, IVec)) -> Result<Self, T::Error> {
        let network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
        Self::from_entry_with_key_checked::<T>(&network_specs_key, network_specs_encoded)
    }
}

impl NetworkSpecsToSend {
    pub fn show(&self) -> String {
        format!("\"base58prefix\":\"{}\",\"color\":\"{}\",\"decimals\":\"{}\",\"encryption\":\"{}\",\"genesis_hash\":\"{}\",\"logo\":\"{}\",\"name\":\"{}\",\"path_id\":\"{}\",\"secondary_color\":\"{}\",\"title\":\"{}\",\"unit\":\"{}\"", &self.base58prefix, &self.color, &self.decimals, &self.encryption.show(), hex::encode(&self.genesis_hash), &self.logo, &self.name, &self.path_id, &self.secondary_color, &self.title, &self.unit)
    }
    pub fn to_store(&self, order: u8) -> NetworkSpecs {
        NetworkSpecs {
            base58prefix: self.base58prefix,
            color: self.color.to_string(),
            decimals: self.decimals,
            encryption: self.encryption.to_owned(),
            genesis_hash: self.genesis_hash,
            logo: self.logo.to_string(),
            name: self.name.to_string(),
            order,
            path_id: self.path_id.to_string(),
            secondary_color: self.secondary_color.to_string(),
            title: self.title.to_string(),
            unit: self.unit.to_string(),
        }
    }
    pub fn from_entry_with_key_checked (network_specs_key: &NetworkSpecsKey, network_specs_to_send_encoded: IVec) -> Result<Self, ErrorActive> {
        let (genesis_hash_vec, encryption) = network_specs_key.genesis_hash_encryption::<Active>(SpecsKeySource::SpecsTree)?;
        let network_specs_to_send = match Self::decode(&mut &network_specs_to_send_encoded[..]) {
            Ok(a) => a,
            Err(_) => return Err(ErrorActive::Database(DatabaseActive::EntryDecoding(EntryDecodingActive::NetworkSpecsToSend(network_specs_key.to_owned())))),
        };
        if genesis_hash_vec[..] != network_specs_to_send.genesis_hash {return Err(ErrorActive::Database(DatabaseActive::Mismatch(MismatchActive::SpecsToSendGenesisHash{key: network_specs_key.to_owned(), genesis_hash: network_specs_to_send.genesis_hash.to_vec()})))}
        if encryption != network_specs_to_send.encryption {return Err(ErrorActive::Database(DatabaseActive::Mismatch(MismatchActive::SpecsToSendEncryption{key: network_specs_key.to_owned(), encryption: network_specs_to_send.encryption})))}
        Ok(network_specs_to_send)
    }
    pub fn from_entry_checked ((network_specs_key_vec, network_specs_to_send_encoded): (IVec, IVec)) -> Result<Self, ErrorActive> {
        let network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
        Self::from_entry_with_key_checked(&network_specs_key, network_specs_to_send_encoded)
    }
}

#[derive(Decode, Encode, PartialEq, Debug)]
pub struct NetworkProperties {
    pub base58prefix: u16,
    pub decimals: u8,
    pub unit: String,
}

/// Verifier for both network metadata and for types information
#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub struct Verifier (pub Option<VerifierValue>);

#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub enum VerifierValue {
    Standard (MultiSigner),
}

impl Verifier {
    pub fn show_card(&self) -> String {
        match &self.0 {
            Some(a) => a.show_card(),
            None => format!("\"public_key\":\"\",\"identicon\":\"{}\",\"encryption\":\"none\"", hex::encode(EMPTY_PNG)),
        }
    }
    pub fn show_error(&self) -> String {
        match &self.0 {
            Some(a) => a.show_error(),
            None => String::from("none"),
        }
    }
}

impl VerifierValue {
    pub fn show_card(&self) -> String {
        match &self {
            VerifierValue::Standard(m) => {
                let hex_public = hex::encode(multisigner_to_public(m));
                let encryption = multisigner_to_encryption(m);
                let hex_identicon = hex::encode(make_identicon_from_multisigner(m));
                format!("\"public_key\":\"{}\",\"identicon\":\"{}\",\"encryption\":\"{}\"", hex_public, hex_identicon, encryption.show())
            },
        }
    }
    pub fn show_error(&self) -> String {
        match &self {
            VerifierValue::Standard(MultiSigner::Ed25519(x)) => format!("public key: {}, encryption: ed25519", hex::encode(x.0)),
            VerifierValue::Standard(MultiSigner::Sr25519(x)) => format!("public key: {}, encryption: sr25519", hex::encode(x.0)),
            VerifierValue::Standard(MultiSigner::Ecdsa(x)) => format!("public key: {}, encryption: ecdsa", hex::encode(x.0)),
        }
    }
}

/// Current network verifier.
/// Could be general verifier (by default, for networks: kusama, polkadot, westend, rococo),
/// or could be custom verifier.
#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub enum CurrentVerifier {
    Valid(ValidCurrentVerifier),
    Dead,
}

/// Possible variants of valid current network verifier.
#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub enum ValidCurrentVerifier {
    General,
    Custom (Verifier),
}

impl ValidCurrentVerifier {
    pub fn show(&self, general_verifier: &Verifier) -> String {
        match &self {
            ValidCurrentVerifier::General => format!("\"type\":\"general\",\"details\":{}", export_complex_single(general_verifier, |a| a.show_card())),
            ValidCurrentVerifier::Custom(a) => format!("\"type\":\"custom\",\"details\":{}", export_complex_single(a, |a| a.show_card())),
        }
    }
}
