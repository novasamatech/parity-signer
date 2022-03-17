//! Network specs, verifiers, and related types  
//!
//! Signer could be used only for networks introduced to the database.  
//!
//! Each network has associated set of parameters called here network specs. 
//! Cold database stores [`NetworkSpecs`].  
//! Hot database stores [`NetworkSpecsToSend`]. Air-gap transfers with 
//! `add_specs` payload also contain [`NetworkSpecsToSend`].
//! Network specs rarely change and are generally are introduced into 
//! Signer only once for each network.
//!
//! Signer has verifier system storing public keys that Signer trusts are providing
//! correct updates to the information.  
//!
//! Cold database stores [`CurrentVerifier`] for each network and general verifier.  
//! 
//! # Verifiers in cold database  
//! 
//! Signer is expected to be safe to use as long as the information uploaded 
//! into it through air-gap is the correct one.  
//! 
//! Damaged network specs or network metadata could result in transactions 
//! being parsed incorrectly.  
//! 
//! Verifier system is introduced to have important transferrable data signed 
//! by a trusted party to exclude accidental or intentional errors. It is 
//! especially important in keeping fast-changing network metadata updated.  
//! 
//! ## Verifiers of data entering the Signer  
//! 
//! The data scanned into Signer has an associated [`Verifier`].  
//! 
//! If the information is verified, [`Verifier`] is `Some(VerifierValue)`.  
//! 
//! If the information is not verified, [`Verifier`] is `None`.  
//! 
//! Payloads with verified data contain trusted party public key and signature 
//! for the data transferred, that are checked when the data is processed in 
//! Signer. Unverified data gets processed as is, without any verification. 
//! Unverified data should be used cautiously, and avoided unless absolutely 
//! necessary.  
//! 
//! [`VerifierValue`] is public key of the trusted party using a certain 
//! encryption algorithm. At this moment, only `Standard` variant supporting 
//! [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html) 
//! is available.  
//! 
//! ## Verifiers of data already in the Signer  
//! 
//! There is [`CurrentVerifier`] for each network known to Signer, and 
//! general verifier [`Verifier`] for some selected networks and for types.  
//! 
//! ### Network verifiers  
//! 
//! Each network in Signer has an associated [`CurrentVerifier`], having two 
//! variants: `Valid` and `Dead`. Networks could be used by the Signer only if 
//! their verifier is `CurrentVerifier::Valid` with associated 
//! [`ValidCurrentVerifier`] value. Networks having `CurrentVerifier::Dead` no 
//! longer could be used or updated, unless the Signer is completely wiped and 
//! reset.  
//! 
//! Network verifier information is stored in `VERIFIERS` tree of the cold 
//! database, with `VerifierKey` in key form as a key and encoded 
//! [`CurrentVerifier`] as a value. `VerifierKey` is derived from network 
//! genesis hash. If the network supports more than one encryption algorithm, 
//! network specs for all encryption algorithms and network metadata would 
//! have the same [`CurrentVerifier`].  
//! 
//! `ValidCurrentVerifier` has two variants: `General` and `Custom`.  
//! 
//! `ValidCurrentVerifier::General` means that the network is verified by the 
//! general verifier known to Signer. As installed or freshly wiped Signer 
//! has default networks Polkadot, Kusama, and Westend having 
//! `ValidCurrentVerifier::General`.  
//! 
//! `ValidCurrentVerifier::Custom` means that the network is verified by 
//! some `Verifier` other than the general verifier.  
//! 
//! Updating network metadata could be done only if the metadata is signed by 
//! exactly same [`CurrentVerifier`] as already recorded in the database for 
//! this network.  
//! 
//! The messages for updating network specs or loading network specs for same 
//! network, but with different encryption algorithm, could be signed with 
//! same [`CurrentVerifier`] or an upgraded one.  
//! 
//! ### General verifier  
//! 
//! General verifier is the default [`Verifier`] known to the Signer. It is the 
//! data source the Signer relies upon the most.  
//! 
//! General verifier is set up during Signer initialization, and is by 
//! default Parity-associated key.  
//! 
//! General verifier value is stored in `SETTREE` tree of the cold database 
//! under key `GENERALVERIFIER`.  
//! 
//! General verifier is used for verification of network specs and metadata 
//! for some networks (ideally, the ones most used on particular Signer 
//! device), and types information. Data previously verified by any 
//! general verifier other than `None` could be updated only if the update 
//! is signed by the same verifier. When general verifier is `None`, it gets 
//! updated to [`Verifier`] of the first user-accepted verified updating payload 
//! (i.e. with trust on first use).  
//! 
//! Users can remove existing general verifier, this will re-initialize Signer 
//! with general verifier set to `None` and reset the Signer data to defaults.  
//! 
//! It is not recommended to use Signer without any general verifier, so it is 
//! best to set up the user-trusted party as the general verifier right after 
//! removing the old general verifier, by downloading and accepting some 
//! trusted data with network specs or types information.  
//! 
//! Note that setting general verifier from `None` to `Some(VerifierValue)` 
//! will remove all previously known data (i.e. network specs, network metadata, 
//! types information) associated with previous general verifier value `None`. 
//! This is done to avoid false trust to data that appeared before the 
//! verification began.  
//! 
//! Note that if the network specs get removed due to general verifier change, 
//! the addresses within this network are not deleted from the database, just 
//! never displayed in the user interface. Signer will recall them if the 
//! network specs get re-loaded verified with correct general verifier.  
//! 
//! Let's consider a case when user has removed the default general verifier. 
//! After the reset, general verifier value becomes `None`, however, the Signer 
//! still has the network specs and metadata for default networks, and types 
//! information. User plans to operate the Signer with Parity-unrelated network, 
//! and loads verified network specs for it. The received data verifier becomes 
//! the new general verifier, and the data for default networks and default 
//! types gets removed.  
//! 
//! ### Upgrading verifiers  
//! 
//! Once `ValidCurrentVerifier::Custom` is set to `Some(VerifierValue)`, it 
//! could not be changed to any other value except general verifier. Note that 
//! in this case general verifier is also already set to `Some(VerifierValue)`, 
//! since general verifier could not be `None` if there is not-`None` custom 
//! verifier.  
//! 
//! General verifier could not be changed once set up with `Some(VerifierValue)`, 
//! unless the Signer is wiped and reset.  
//!
//! At the moment there is no mechanism allowing users to set up the verifiers
//! except by loading and accepting the updates signed by these verifiers.  
//!
//! In some cases network verifiers and general verifier could be changed
//! by accepting updating payloads:  
//! 
//! - `ValidCurrentVerifier::General(None)` could be upgraded to 
//! `ValidCurrentVerifier::General(Some(VerifierValue))`. Happens when Signer
//! with no general verifier receives and accepts a verified update with any
//! network specs or types.  
//! 
//! - Network `ValidCurrentVerifier::Custom(Some(VerifierValue))` could be 
//! upgraded to `ValidCurrentVerifier::General(Some(VerifierValue))`. 
//! Happens when Signer receives and accepts network specs update verified 
//! by general verifier for this network. Note: metadata update with 
//! verifier upgrade would be rejected.  
//! 
//! - Network `ValidCurrentVerifier::Custom(None)` could be upgraded to 
//! `ValidCurrentVerifier::General(Some(VerifierValue))`. 
//! Happens when Signer receives and accepts network specs update verified 
//! by general verifier for this network. Note: metadata update with 
//! verifier upgrade would be rejected.  
//! 
//! If [`CurrentVerifier`] upgrade occurs, all pre-upgrade network information 
//! is wiped (Signer warns about that).  
//! 
//! If general verifier upgrade occurs, all pre-upgrade information verified 
//! by the general verifier is wiped (Signer warns about that).  
//!
//! ### Deleting network specs  
//! 
//! User can remove any network from Signer. "Remove network" means removing:
//! - all network specs associated with network genesis hash, 
//! - all metadata associated with network specs name, 
//! - all addresses within this network with any encryption,
//! - (possibly) modifying [`CurrentVerifier`].  
//! 
//! If the network was verified by `CurrentVerifier::Valid` with value 
//! `ValidCurrentVerifier::General`, verifier information in [`CurrentVerifier`] 
//! will not change. Restoring the network will require updates signed by the 
//! same general verifier as before.  
//! 
//! If the network was verified by `CurrentVerifier::Valid` with value 
//! `ValidCurrentVerifier::Custom` and the custom [`Verifier`] was `None`, no 
//! changes will happen, network verifier will stay 
//! `CurrentVerifier::Valid(ValidCurrentVerifier::Custom(None))`. Otherwise, 
//! the network verifier will be changed to `CurrentVerifier::Dead`, and it 
//! would be impossible to use the network again unless the Signer is wiped. 
//! This is a part of security policy.  
//! 
//! # Network specs in cold database
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
//! ## Adding new network specs  
//!
//! New networks could be added to Signer through scanning `add_specs` QR code
//! for the network.
//!
//! ## Updating network specs (replacing old ones with new ones without deleting 
//! old ones)
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
//! ## Deleting network specs
//! 
//! Any network could be removed, however sometimes there are associated changes
//! in corresponding verifier. See above.
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

/// Network parameters stored SCALE-encoded in the **cold** database 
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

/// Network parameters stored SCALE-encoded in the **hot** database 
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
    /// Prints network specs in json format
    #[cfg(feature = "signer")]
    pub fn show(
        &self,
        valid_current_verifier: &ValidCurrentVerifier,
        general_verifier: &Verifier,
    ) -> String {
        format!("\"base58prefix\":\"{}\",\"color\":\"{}\",\"decimals\":\"{}\",\"encryption\":\"{}\",\"genesis_hash\":\"{}\",\"logo\":\"{}\",\"name\":\"{}\",\"order\":\"{}\",\"path_id\":\"{}\",\"secondary_color\":\"{}\",\"title\":\"{}\",\"unit\":\"{}\",\"current_verifier\":{}", &self.base58prefix, &self.color, &self.decimals, &self.encryption.show(), hex::encode(&self.genesis_hash), &self.logo, &self.name, &self.order, &self.path_id, &self.secondary_color, &self.title, &self.unit, export_complex_single(valid_current_verifier, |a| a.show(general_verifier)))
    }

    /// Makes [`NetworkSpecsToSend`] from [`NetworkSpecs`]
    #[cfg(feature = "signer")]
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

    /// Makes [`ShortSpecs`] from [`NetworkSpecs`]
    #[cfg(feature = "signer")]
    pub fn short(&self) -> ShortSpecs {
        ShortSpecs {
            base58prefix: self.base58prefix,
            decimals: self.decimals,
            genesis_hash: self.genesis_hash,
            name: self.name.to_string(),
            unit: self.unit.to_string(),
        }
    }

    /// Gets [`NetworkSpecs`] from [`NetworkSpecsKey`] and associated value 
    /// from cold database tree `SPECSTREE`  
    ///
    /// Checks that there is no genesis hash or encryption mismatch between 
    /// key and specs content.  
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

    /// Gets [`NetworkSpecs`] from cold database tree `SPECSTREE` (key, value) 
    /// entry  
    ///
    /// Checks that there is no genesis hash or encryption mismatch between 
    /// key and specs content.  
    pub fn from_entry_checked<T: ErrorSource>(
        (network_specs_key_vec, network_specs_encoded): (IVec, IVec),
    ) -> Result<Self, T::Error> {
        let network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
        Self::from_entry_with_key_checked::<T>(&network_specs_key, network_specs_encoded)
    }
}

impl NetworkSpecsToSend {
    /// Prints network specs in json format
    #[cfg(feature = "signer")]
    pub fn show(&self) -> String {
        format!("\"base58prefix\":\"{}\",\"color\":\"{}\",\"decimals\":\"{}\",\"encryption\":\"{}\",\"genesis_hash\":\"{}\",\"logo\":\"{}\",\"name\":\"{}\",\"path_id\":\"{}\",\"secondary_color\":\"{}\",\"title\":\"{}\",\"unit\":\"{}\"", &self.base58prefix, &self.color, &self.decimals, &self.encryption.show(), hex::encode(&self.genesis_hash), &self.logo, &self.name, &self.path_id, &self.secondary_color, &self.title, &self.unit)
    }

    /// Makes [`NetworkSpecs`] from [`NetworkSpecsToSend`], 
    /// needs `order` input
    ///
    /// `order` is network number on the list of networks in Signer.
    ///
    /// This happens when Signer receives new network specs through QR update.
    #[cfg(feature = "signer")]
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

    /// Gets [`NetworkSpecsToSend`] from [`NetworkSpecsKey`] and associated 
    /// value from hot database tree `SPECSTREEPREP`  
    ///
    /// Checks that there is no genesis hash or encryption mismatch between 
    /// key and specs content.  
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

    /// Gets [`NetworkSpecsToSend`] from hot database tree `SPECSTREEPREP` 
    /// (key, value) entry  
    ///
    /// Checks that there is no genesis hash or encryption mismatch between 
    /// key and specs content.  
    #[cfg(feature = "active")]
    pub fn from_entry_checked(
        (network_specs_key_vec, network_specs_to_send_encoded): (IVec, IVec),
    ) -> Result<Self, ErrorActive> {
        let network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
        Self::from_entry_with_key_checked(&network_specs_key, network_specs_to_send_encoded)
    }
}

/// Network properties that must be fetched with rpc call for properties
/// in each compatible network
#[derive(Decode, Encode, PartialEq, Debug)]
#[cfg(feature = "active")]
pub struct NetworkProperties {
    pub base58prefix: u16,
    pub decimals: u8,
    pub unit: String,
}

/// Verifier information
///
/// Either real verifier or information that there is no verifier.
#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub struct Verifier(pub Option<VerifierValue>);

/// Information on known and existing verifier  
///
/// Verifier public key with associated encryption algorithm.
#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub enum VerifierValue {
    /// public key for standard substrate-compatible encryption algorithms  
    Standard(MultiSigner),
}

#[cfg(feature = "signer")]
impl Verifier {
    /// Display [`Verifier`] in json-like format, for json exports  
    pub fn show_card(&self) -> String {
        match &self.0 {
            Some(a) => a.show_card(),
            None => format!(
                "\"public_key\":\"\",\"identicon\":\"{}\",\"encryption\":\"none\"",
                hex::encode(EMPTY_PNG)
            ),
        }
    }

    /// Display [`Verifier`] in human-readable format, for errors  
    pub fn show_error(&self) -> String {
        match &self.0 {
            Some(a) => a.show_error(),
            None => String::from("none"),
        }
    }
}

#[cfg(feature = "signer")]
impl VerifierValue {
    /// Display [`VerifierValue`] in json-like format, for json exports  
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

    /// Display [`VerifierValue`] in human-readable format, for errors  
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

/// Current network verifier
#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub enum CurrentVerifier {
    /// Verifier is valid, Signer can use the network
    Valid(ValidCurrentVerifier),
    /// Verifier is invalid, Signer would not be able to use the network 
    /// without wipe and reset
    Dead,
}

/// Possible variants of valid current network verifier
///
/// Could be general verifier (by default for networks Polkadot, Kusama, Westend),
/// or custom verifier.
#[derive(Decode, Encode, PartialEq, Debug, Clone)]
pub enum ValidCurrentVerifier {
    /// Network has general verifier
    General,
    /// Network has some other verifier, different from the general one
    Custom(Verifier),
}

#[cfg(feature = "signer")]
impl ValidCurrentVerifier {
    /// Display [`ValidCurrentVerifier`] in json-like format, for json exports  
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
