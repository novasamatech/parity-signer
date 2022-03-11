//! Network specs and verifier related types  
//!
//! Signer could be used only within networks introduced to the database.  
//! The network parameters are stored in the cold database as [`NetworkSpecs`].  
//! Hot database stores [`NetworkSpecsToSend`], that also is transferred
//! through `add_specs` QR codes into Signer when a new network is added.  
//! [`NetworkSpecs`] have all [`NetworkSpecsToSend`] information, and also `u8` 
//! order, for the order in which network appears on Signer device screen.  
//! [`ShortSpecs`] is the minimal [`NetworkSpecs`] subset needed to properly 
//! format decoded transactions.  
//! 
//! # `NetworkSpecs` in cold database
//!
//! [`NetworkSpecs`] are stored in `SPECSTREE` tree of the cold database,
//! with [`NetworkSpecsKey`] in key form as a key and SCALE-encoded [`NetworkSpecs`] 
//! as a value.  
//!
//! [`NetworkSpecs`] include both the `encryption` ([`Encryption`]) and network  
//! genesis hash (`[u8; 32]`), that are used for [`NetworkSpecsKey`] generation.  
//! [`NetworkSpecs`] retrieved for given [`NetworkSpecsKey`] always get checked 
//! for consistency.  
//!
//! If the network supports more than one encryption algorithm, each encryption
//! corresponds to different [`NetworkSpecsKey`], and any or all of them could be
//! coexisting in Signer simultaneously.  
//!
//! [`NetworkSpecs`] are generally expected to remain unchanged over time.  
//!
//! ## Adding new [`NetworkSpecs`]  
//!
//! New networks could be added to Signer through scanning `add_specs` QR code
//! for the network.
//!
//! ## [`NetworkSpecs`] updating (replacing old ones with new ones without deleting old ones)
//!
//! Signer will not allow to update network specs if critical parameters
//! have changed.  
//! These critical parameters are:  
//! - `base58prefix`, network-associated base58 prefix  
//! - `decimals`  
//! - `name`, network name, as it appears in the network metadata  
//! - `unit`  
//!
//! However, if non-critical parameters have changes, Signer will permit the
//! network specs updating.  
//! These non-critical parameters are:  
//! - `color`  
//! - `logo`, network-associated logo picture  
//! - `path_id`, default address derivation path for the network  
//! - `secondary_color`  
//! - `title`, network title, as it is displayed in Signer  
//!
//! Some quickly updating experimental networks are changing the genesis hash
//! often. Network genesis hash participates in [`NetworkSpecsKey`] 
//! generation. This way, if the genesis hash got updated, the network would 
//! appear "unknown" to Signer, and to use it, network specs with new genesis hash
//! would have to be added. Adding network specs with new genesis hash does not 
//! require deleting network specs with old genesis hash.  
//!
//! ## Balance representation: decimals and units  
//!
//! To represent the balance-related values properly, each network has associated
//! decimals and units. The balance-related values in, for example, transactions
//! are integer numbers, and are formatted properly only during the transactions
//! decoding. For this `decimals` and `unit` values from [`NetworkSpecs`] are used. 
//! `decimals` indicate the order of magnitude, by which the token `unit` 
//! exceeds the integer representing unit (see examples below).
//! Both `decimals` and `unit` values could be queried through rpc calls for each 
//! Signer-compatible network.  
//! Sometimes the networks have several available decimals and units, or none at all.  
//! This cases should be dealt with on case-by-case basis.  
//!
//! ## Examples: balance representation  
//!
//! Balance (`u64`) from transaction is decoded as `1`.
//! Network `decimals` value is `12`, network `unit` is `WND`.
//! The balance should be therefore represented as `1 pWND`.  
//!
//! Balance (`u64`) from transaction is decoded as `10000000`.
//! Network `decimals` value is `12`, network `unit` is `WND`.
//! The balance should be therefore represented as `10 uWND`.  

use parity_scale_codec::{Decode, Encode};
#[cfg(feature = "signer")]
use plot_icon::EMPTY_PNG;
use sled::IVec;
use sp_runtime::MultiSigner;

#[cfg(feature = "active")]
use crate::error_active::{
    Active, DatabaseActive, EntryDecodingActive, ErrorActive, MismatchActive,
};
use crate::{
    crypto::Encryption,
    error::{ErrorSource, SpecsKeySource},
    keyring::NetworkSpecsKey,
};
#[cfg(feature = "signer")]
use crate::{
    helpers::{make_identicon_from_multisigner, multisigner_to_encryption, multisigner_to_public},
    print::export_complex_single,
};

/// Network parameters stored in the **cold** database 
/// `SPECSTREE` tree under [`NetworkSpecsKey`]
///
/// These network parameters must be in Signer database for the Signer to be 
/// able to operate with this network.
#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub struct NetworkSpecs {
    /// Network-specific prefix for address representation in 
    /// [base58 format](https://docs.rs/sp-core/6.0.0/sp_core/crypto/trait.Ss58Codec.html#method.to_ss58check_with_version)  
    pub base58prefix: u16,
    /// Network-associated color.  
    /// Historically is there, not doing much at the moment.  
    pub color: String,
    /// Order of magnitude, by which the token unit exceeds the balance integer unit.  
    /// Is used to display balance-related values properly.  
    pub decimals: u8,
    /// Encryption algorithm the network uses
    pub encryption: Encryption,
    /// Network genesis hash
    pub genesis_hash: [u8; 32],
    /// Network associated logo
    pub logo: String,
    /// Network name, as it appears in network metadata
    pub name: String,
    /// Order in which the network is displayed by Signer
    pub order: u8,
    /// Default derivation path for addresses in this network
    pub path_id: String,
    /// Network-associated secondary color.  
    /// Historically is there, not doing much at the moment.  
    pub secondary_color: String,
    /// Network title, as it appears in Signer menus.
    pub title: String,
    /// Token name, to display balance-related values properly.  
    pub unit: String,
}

/// Network parameters stored in the **hot** database 
/// `SPECSTREEPREP` tree under [`NetworkSpecsKey`] and sent as QR code 
/// in `add_specs` messages
///
/// These network parameters are sufficient to add network into Signer database.
#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub struct NetworkSpecsToSend {
    /// Network-specific prefix for address representation in 
    /// [base58 format](https://docs.rs/sp-core/6.0.0/sp_core/crypto/trait.Ss58Codec.html#method.to_ss58check_with_version)  
    pub base58prefix: u16,
    /// Network-associated color.  
    /// Historically is there, not doing much at the moment.  
    pub color: String,
    /// Order of magnitude, by which the token unit exceeds the balance integer unit.  
    /// Is used to display balance-related values properly.  
    pub decimals: u8,
    /// Encryption algorithm the network uses  
    pub encryption: Encryption,
    /// Network genesis hash  
    pub genesis_hash: [u8; 32],
    /// Network associated logo  
    pub logo: String,
    /// Network name, as it appears in network metadata  
    pub name: String,
    /// Default derivation path for addresses in this network  
    pub path_id: String,
    /// Network-associated secondary color.  
    /// Historically is there, not doing much at the moment.  
    pub secondary_color: String,
    /// Network title, as it appears in Signer menus.  
    pub title: String,
    /// Token name, to display balance-related values properly.  
    pub unit: String,
}

/// Network parameters needed to decode and display transaction
#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub struct ShortSpecs {
    /// Network-specific prefix for address representation in 
    /// [base58 format](https://docs.rs/sp-core/6.0.0/sp_core/crypto/trait.Ss58Codec.html#method.to_ss58check_with_version)  
    pub base58prefix: u16,
    /// Order of magnitude, by which the token unit exceeds the balance integer unit.  
    /// Is used to display balance-related values properly.  
    pub decimals: u8,
    /// Network genesis hash  
    pub genesis_hash: [u8; 32],
    /// Network name, as it appears in network metadata  
    pub name: String,
    /// Token name, to display balance-related values properly.  
    pub unit: String,
}

impl NetworkSpecs {
    #[cfg(feature = "signer")]
    pub fn show(
        &self,
        valid_current_verifier: &ValidCurrentVerifier,
        general_verifier: &Verifier,
    ) -> String {
        format!("\"base58prefix\":\"{}\",\"color\":\"{}\",\"decimals\":\"{}\",\"encryption\":\"{}\",\"genesis_hash\":\"{}\",\"logo\":\"{}\",\"name\":\"{}\",\"order\":\"{}\",\"path_id\":\"{}\",\"secondary_color\":\"{}\",\"title\":\"{}\",\"unit\":\"{}\",\"current_verifier\":{}", &self.base58prefix, &self.color, &self.decimals, &self.encryption.show(), hex::encode(&self.genesis_hash), &self.logo, &self.name, &self.order, &self.path_id, &self.secondary_color, &self.title, &self.unit, export_complex_single(valid_current_verifier, |a| a.show(general_verifier)))
    }
    #[cfg(feature = "signer")]
    pub fn print_single(&self) -> String {
        format!(
            "\"color\":\"{}\",\"logo\":\"{}\",\"secondary_color\":\"{}\",\"title\":\"{}\"",
            &self.color, &self.logo, &self.secondary_color, &self.title
        )
    }
    #[cfg(feature = "signer")]
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
    pub fn from_entry_with_key_checked<T: ErrorSource>(
        network_specs_key: &NetworkSpecsKey,
        network_specs_encoded: IVec,
    ) -> Result<Self, T::Error> {
        let (genesis_hash_vec, encryption) =
            network_specs_key.genesis_hash_encryption::<T>(SpecsKeySource::SpecsTree)?;
        let network_specs = match Self::decode(&mut &network_specs_encoded[..]) {
            Ok(a) => a,
            Err(_) => return Err(<T>::specs_decoding(network_specs_key.to_owned())),
        };
        if genesis_hash_vec[..] != network_specs.genesis_hash {
            return Err(<T>::specs_genesis_hash_mismatch(
                network_specs_key.to_owned(),
                network_specs.genesis_hash.to_vec(),
            ));
        }
        if encryption != network_specs.encryption {
            return Err(<T>::specs_encryption_mismatch(
                network_specs_key.to_owned(),
                network_specs.encryption,
            ));
        }
        Ok(network_specs)
    }
    pub fn from_entry_checked<T: ErrorSource>(
        (network_specs_key_vec, network_specs_encoded): (IVec, IVec),
    ) -> Result<Self, T::Error> {
        let network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
        Self::from_entry_with_key_checked::<T>(&network_specs_key, network_specs_encoded)
    }
}

impl NetworkSpecsToSend {
    #[cfg(feature = "signer")]
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
    #[cfg(feature = "active")]
    pub fn from_entry_with_key_checked(
        network_specs_key: &NetworkSpecsKey,
        network_specs_to_send_encoded: IVec,
    ) -> Result<Self, ErrorActive> {
        let (genesis_hash_vec, encryption) =
            network_specs_key.genesis_hash_encryption::<Active>(SpecsKeySource::SpecsTree)?;
        let network_specs_to_send = match Self::decode(&mut &network_specs_to_send_encoded[..]) {
            Ok(a) => a,
            Err(_) => {
                return Err(ErrorActive::Database(DatabaseActive::EntryDecoding(
                    EntryDecodingActive::NetworkSpecsToSend(network_specs_key.to_owned()),
                )))
            }
        };
        if genesis_hash_vec[..] != network_specs_to_send.genesis_hash {
            return Err(ErrorActive::Database(DatabaseActive::Mismatch(
                MismatchActive::SpecsToSendGenesisHash {
                    key: network_specs_key.to_owned(),
                    genesis_hash: network_specs_to_send.genesis_hash.to_vec(),
                },
            )));
        }
        if encryption != network_specs_to_send.encryption {
            return Err(ErrorActive::Database(DatabaseActive::Mismatch(
                MismatchActive::SpecsToSendEncryption {
                    key: network_specs_key.to_owned(),
                    encryption: network_specs_to_send.encryption,
                },
            )));
        }
        Ok(network_specs_to_send)
    }
    #[cfg(feature = "active")]
    pub fn from_entry_checked(
        (network_specs_key_vec, network_specs_to_send_encoded): (IVec, IVec),
    ) -> Result<Self, ErrorActive> {
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
pub struct Verifier(pub Option<VerifierValue>);

#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub enum VerifierValue {
    Standard(MultiSigner),
}

#[cfg(feature = "signer")]
impl Verifier {
    pub fn show_card(&self) -> String {
        match &self.0 {
            Some(a) => a.show_card(),
            None => format!(
                "\"public_key\":\"\",\"identicon\":\"{}\",\"encryption\":\"none\"",
                hex::encode(EMPTY_PNG)
            ),
        }
    }
    pub fn show_error(&self) -> String {
        match &self.0 {
            Some(a) => a.show_error(),
            None => String::from("none"),
        }
    }
}

#[cfg(feature = "signer")]
impl VerifierValue {
    pub fn show_card(&self) -> String {
        match &self {
            VerifierValue::Standard(m) => {
                let hex_public = hex::encode(multisigner_to_public(m));
                let encryption = multisigner_to_encryption(m);
                let hex_identicon = hex::encode(make_identicon_from_multisigner(m));
                format!(
                    "\"public_key\":\"{}\",\"identicon\":\"{}\",\"encryption\":\"{}\"",
                    hex_public,
                    hex_identicon,
                    encryption.show()
                )
            }
        }
    }
    pub fn show_error(&self) -> String {
        match &self {
            VerifierValue::Standard(MultiSigner::Ed25519(x)) => {
                format!("public key: {}, encryption: ed25519", hex::encode(x.0))
            }
            VerifierValue::Standard(MultiSigner::Sr25519(x)) => {
                format!("public key: {}, encryption: sr25519", hex::encode(x.0))
            }
            VerifierValue::Standard(MultiSigner::Ecdsa(x)) => {
                format!("public key: {}, encryption: ecdsa", hex::encode(x.0))
            }
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
    Custom(Verifier),
}

#[cfg(feature = "signer")]
impl ValidCurrentVerifier {
    pub fn show(&self, general_verifier: &Verifier) -> String {
        match &self {
            ValidCurrentVerifier::General => format!(
                "\"type\":\"general\",\"details\":{}",
                export_complex_single(general_verifier, |a| a.show_card())
            ),
            ValidCurrentVerifier::Custom(a) => format!(
                "\"type\":\"custom\",\"details\":{}",
                export_complex_single(a, |a| a.show_card())
            ),
        }
    }
}
