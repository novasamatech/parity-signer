//! Network specs, verifiers, and related types  
//!
//! Vault could be used only for networks introduced to the database.
//!
//! Each network has associated set of parameters called here network specs.
//! Cold database stores [`OrderedNetworkSpecs`].
//! Hot database stores [`NetworkSpecs`]. Air-gap transfers with
//! `add_specs` payload also contain [`NetworkSpecs`].
//! Network specs rarely change and are generally are introduced into
//! Vault only once for each network.
//!
//! Vault has verifier system storing public keys that Vault trusts are providing
//! correct updates to the information.  
//!
//! Cold database stores [`CurrentVerifier`] for each network and general verifier.  
//!
//! # Verifiers in cold database  
//!
//! Vault is expected to be safe to use as long as the information uploaded
//! into it through air-gap is the correct one.  
//!
//! Damaged network specs or network metadata could result in transactions
//! being parsed incorrectly.  
//!
//! Verifier system is introduced to have important transferable data signed
//! by a trusted party to exclude accidental or intentional errors. It is
//! especially important in keeping fast-changing network metadata updated.  
//!
//! ## Verifiers of data entering the Vault
//!
//! The data scanned into Vault has an associated [`Verifier`].
//!
//! If the information is verified, [`Verifier`] is `Some(VerifierValue)`.  
//!
//! If the information is not verified, [`Verifier`] is `None`.  
//!
//! Payloads with verified data contain trusted party public key and signature
//! for the data transferred, that are checked when the data is processed in
//! Vault. Unverified data gets processed as is, without any verification.
//! Unverified data should be used cautiously, and avoided unless absolutely
//! necessary.  
//!
//! [`VerifierValue`] is public key of the trusted party using a certain
//! encryption algorithm. At this moment, only `Standard` variant supporting
//! [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)
//! is available.  
//!
//! ## Verifiers of data already in the Vault
//!
//! There is [`CurrentVerifier`] for each network known to Vault, and
//! general verifier [`Verifier`] for some selected networks and for types.  
//!
//! ### Network verifiers  
//!
//! Each network in Vault has an associated [`CurrentVerifier`], having two
//! variants: `Valid` and `Dead`. Networks could be used by the Vault only if
//! their verifier is `CurrentVerifier::Valid` with associated
//! [`ValidCurrentVerifier`] value. Networks having `CurrentVerifier::Dead` no
//! longer could be used or updated, unless the Vault is completely wiped and
//! reset.  
//!
//! Network verifier information is stored in `VERIFIERS` tree of the cold
//! database, with [`VerifierKey`](crate::keyring::VerifierKey) in key form as
//! a key and encoded [`CurrentVerifier`] as a value. `VerifierKey` is derived
//! from network genesis hash. If the network supports more than one encryption
//! algorithm, network specs for all encryption algorithms and network metadata
//! would have the same [`CurrentVerifier`].  
//!
//! `ValidCurrentVerifier` has two variants: `General` and `Custom`.  
//!
//! `ValidCurrentVerifier::General` means that the network is verified by the
//! general verifier known to Vault. As installed or freshly wiped Vault
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
//! General verifier is the default [`Verifier`] known to the Vault. It is the
//! data source the Vault relies upon the most.
//!
//! General verifier is set up during Vault initialization, and is by
//! default Parity-associated key.  
//!
//! General verifier value is stored in `SETTREE` tree of the cold database
//! under key `GENERALVERIFIER`.  
//!
//! General verifier is used for verification of network specs and metadata
//! for some networks (ideally, the ones most used on particular Vault
//! device), and types information. Data previously verified by any
//! general verifier other than `None` could be updated only if the update
//! is signed by the same verifier. When general verifier is `None`, it gets
//! updated to [`Verifier`] of the first user-accepted verified updating payload
//! (i.e. with trust on first use).  
//!
//! Users can remove existing general verifier, this will re-initialize Vault
//! with general verifier set to `None` and reset the Vault data to defaults.
//!
//! It is not recommended to use Vault without any general verifier, so it is
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
//! never displayed in the user interface. Vault will recall them if the
//! network specs get re-loaded verified with correct general verifier.  
//!
//! Let's consider a case when user has removed the default general verifier.
//! After the reset, general verifier value becomes `None`, however, the Vault
//! still has the network specs and metadata for default networks, and types
//! information. User plans to operate the Vault with Parity-unrelated network,
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
//! unless the Vault is wiped and reset.
//!
//! At the moment there is no mechanism allowing users to set up the verifiers
//! except by loading and accepting the updates signed by these verifiers.  
//!
//! In some cases network verifiers and general verifier could be changed
//! by accepting updating payloads:  
//!
//! - `ValidCurrentVerifier::General` with general verifier `None` could be
//! upgraded to `ValidCurrentVerifier::General` with general verifier
//! `(Some(VerifierValue))`. Happens when Vault with no value set up for
//! the general verifier receives and accepts a verified update with any
//! network specs or types.  
//!
//! - Network `ValidCurrentVerifier::Custom(Some(VerifierValue))` could be
//! upgraded to `ValidCurrentVerifier::General` with general verifier set to
//! `(Some(VerifierValue))`.
//! Happens when Vault receives and accepts network specs update verified
//! by general verifier for this network. Note: metadata update with
//! verifier upgrade would be rejected.  
//!
//! - Network `ValidCurrentVerifier::Custom(None)` could be upgraded to
//! `ValidCurrentVerifier::General` with general verifier set to
//! `(Some(VerifierValue))`.
//! Happens when Vault receives and accepts network specs update verified
//! by general verifier for this network. Note: metadata update with
//! verifier upgrade would be rejected.  
//!
//! If [`CurrentVerifier`] upgrade occurs, all pre-upgrade network information
//! is wiped (Vault warns about that).
//!
//! If general verifier upgrade occurs, all pre-upgrade information verified
//! by the general verifier is wiped (Vault warns about that).
//!
//! ### Deleting network specs  
//!
//! User can remove any network from Vault. "Remove network" means removing:
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
//! would be impossible to use the network again unless the Vault is wiped.
//! This is a part of security policy.  
//!
//! # Network specs in cold database
//!
//! [`OrderedNetworkSpecs`] are stored in `SPECSTREE` tree of the cold database,
//! with [`NetworkSpecsKey`] in key form as a key and SCALE-encoded [`OrderedNetworkSpecs`]
//! as a value.  
//!
//! [`OrderedNetworkSpecs`] include both the `encryption` ([`Encryption`]) and network
//! genesis hash (`[u8; 32]`), that are used for [`NetworkSpecsKey`] generation.  
//! [`OrderedNetworkSpecs`] retrieved for given [`NetworkSpecsKey`] always get checked
//! for consistency.  
//!
//! If the network supports more than one encryption algorithm, each encryption
//! corresponds to different [`NetworkSpecsKey`], and any or all of them could be
//! coexisting in Vault simultaneously.
//!
//! [`OrderedNetworkSpecs`] are generally expected to remain unchanged over time.
//!
//! ## Adding new network specs  
//!
//! New networks could be added to Vault through scanning `add_specs` QR code
//! for the network.
//!
//! ## Updating network specs (replacing old ones with new ones without deleting
//! old ones)
//!
//! Vault will not allow to update network specs if critical parameters
//! have changed.  
//! These critical parameters are:  
//! - `base58prefix`, network-associated base58 prefix  
//! - `decimals`  
//! - `name`, network name, as it appears in the network metadata  
//! - `unit`  
//!
//! However, if non-critical parameters have changes, Vault will permit the
//! network specs updating.  
//! These non-critical parameters are:  
//! - `color`  
//! - `logo`, network-associated logo picture  
//! - `path_id`, default address derivation path for the network  
//! - `secondary_color`  
//! - `title`, network title, as it is displayed in Vault
//!
//! Some quickly updating experimental networks are changing the genesis hash
//! often. Network genesis hash participates in [`NetworkSpecsKey`]
//! generation. This way, if the genesis hash got updated, the network would
//! appear "unknown" to Vault, and to use it, network specs with new genesis hash
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
//! decoding. For this `decimals` and `unit` values from [`OrderedNetworkSpecs`] are used.
//! `decimals` indicate the order of magnitude, by which the token `unit`
//! exceeds the integer representing unit (see examples below).
//! Both `decimals` and `unit` values could be queried through RPC calls for each
//! Vault-compatible network.
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
use sled::IVec;
use sp_core::H256;
use sp_runtime::MultiSigner;

use crate::{
    error::{Error, Result},
    helpers::IdenticonStyle,
    navigation::Identicon,
};

use crate::helpers::{
    make_identicon_from_multisigner, multisigner_to_encryption, multisigner_to_public,
};
use crate::{crypto::Encryption, keyring::NetworkSpecsKey};

use crate::navigation::MVerifierDetails;

/// Network parameters stored SCALE-encoded in the **cold** database
/// `SPECSTREE` tree under [`NetworkSpecsKey`]
///
/// These network parameters must be in Vault database for the Vault to be
/// able to operate with this network.
#[derive(Decode, Encode, PartialEq, Eq, Debug, Clone)]
pub struct OrderedNetworkSpecs {
    pub specs: NetworkSpecs,

    /// Order in which the network is displayed by Vault
    pub order: u8,
}

/// Network parameters stored SCALE-encoded in the **hot** database
/// `SPECSTREEPREP` tree under [`NetworkSpecsKey`] and sent as QR code
/// in `add_specs` messages
///
/// These network parameters are sufficient to add network into Vault database.
#[derive(Decode, Encode, PartialEq, Eq, Debug, Clone)]
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
    pub genesis_hash: H256,

    /// Network associated logo  
    pub logo: String,

    /// Network name, as it appears in network metadata  
    pub name: String,

    /// Default derivation path for addresses in this network  
    pub path_id: String,

    /// Network-associated secondary color.  
    /// Historically is there, not doing much at the moment.  
    pub secondary_color: String,

    /// Network title, as it appears in Vault menus.
    pub title: String,

    /// Token name, to display balance-related values properly.  
    pub unit: String,
}

/// Network parameters needed to decode and display transaction
#[derive(Decode, Encode, PartialEq, Eq, Debug, Clone)]
pub struct ShortSpecs {
    /// Network-specific prefix for address representation in
    /// [base58 format](https://docs.rs/sp-core/6.0.0/sp_core/crypto/trait.Ss58Codec.html#method.to_ss58check_with_version)  
    pub base58prefix: u16,

    /// Order of magnitude, by which the token unit exceeds the balance integer unit.  
    /// Is used to display balance-related values properly.  
    pub decimals: u8,

    /// Network genesis hash  
    pub genesis_hash: H256,

    /// Network name, as it appears in network metadata  
    pub name: String,

    /// Token name, to display balance-related values properly.  
    pub unit: String,
}

impl OrderedNetworkSpecs {
    /// Gets [`OrderedNetworkSpecs`] from [`NetworkSpecsKey`] and associated value
    /// from cold database tree `SPECSTREE`  
    ///
    /// Checks that there is no genesis hash or encryption mismatch between
    /// key and specs content.  
    pub fn from_entry_with_key_checked(
        network_specs_key: &NetworkSpecsKey,
        network_specs_encoded: IVec,
    ) -> Result<Self> {
        let (genesis_hash_vec, encryption) = network_specs_key.genesis_hash_encryption()?;
        let ordered_specs = Self::decode(&mut &network_specs_encoded[..])?;
        let network_specs = &ordered_specs.specs;
        if &genesis_hash_vec[..] != network_specs.genesis_hash.as_bytes() {
            return Err(Error::SpecsGenesisHashMismatch {
                network_specs_key: network_specs_key.to_owned(),
                genesis_hash: network_specs.genesis_hash,
            });
        }
        if encryption != network_specs.encryption {
            return Err(Error::SpecsToSendEncryptionMismatch {
                network_specs_key: network_specs_key.to_owned(),
                encryption: network_specs.encryption,
            });
        }
        Ok(ordered_specs)
    }

    /// Gets [`OrderedNetworkSpecs`] from cold database tree `SPECSTREE` (key, value)
    /// entry  
    ///
    /// Checks that there is no genesis hash or encryption mismatch between
    /// key and specs content.  
    pub fn from_entry_checked(
        (network_specs_key_vec, network_specs_encoded): (IVec, IVec),
    ) -> Result<Self> {
        let network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
        Self::from_entry_with_key_checked(&network_specs_key, network_specs_encoded)
    }
}

impl NetworkSpecs {
    /// Makes [`OrderedNetworkSpecs`] from [`NetworkSpecs`],
    /// needs `order` input
    ///
    /// `order` is network number on the list of networks in Vault.
    ///
    /// This happens when Vault receives new network specs through QR update.
    pub fn to_store(&self, order: u8) -> OrderedNetworkSpecs {
        OrderedNetworkSpecs {
            specs: self.to_owned(),
            order,
        }
    }

    /// Makes [`ShortSpecs`] from [`NetworkSpecs`]
    pub fn short(&self) -> ShortSpecs {
        ShortSpecs {
            base58prefix: self.base58prefix,
            decimals: self.decimals,
            genesis_hash: self.genesis_hash,
            name: self.name.to_string(),
            unit: self.unit.to_string(),
        }
    }

    /// Gets [`NetworkSpecs`] from [`NetworkSpecsKey`] and associated
    /// value from hot database tree `SPECSTREEPREP`  
    ///
    /// Checks that there is no genesis hash or encryption mismatch between
    /// key and specs content.  
    #[cfg(feature = "active")]
    pub fn from_entry_with_key_checked(
        network_specs_key: &NetworkSpecsKey,
        network_specs_to_send_encoded: IVec,
    ) -> Result<Self> {
        let (genesis_hash_vec, encryption) = network_specs_key.genesis_hash_encryption()?;
        let network_specs_to_send = Self::decode(&mut &network_specs_to_send_encoded[..])?;
        if &genesis_hash_vec[..] != network_specs_to_send.genesis_hash.as_bytes() {
            return Err(Error::SpecsToSendGenesisHash {
                network_specs_key: network_specs_key.to_owned(),
                genesis_hash: network_specs_to_send.genesis_hash,
            });
        }
        if encryption != network_specs_to_send.encryption {
            return Err(Error::SpecsToSendEncryptionMismatch {
                network_specs_key: network_specs_key.to_owned(),
                encryption: network_specs_to_send.encryption,
            });
        }
        Ok(network_specs_to_send)
    }

    /// Gets [`NetworkSpecs`] from hot database tree `SPECSTREEPREP`
    /// (key, value) entry  
    ///
    /// Checks that there is no genesis hash or encryption mismatch between
    /// key and specs content.  
    #[cfg(feature = "active")]
    pub fn from_entry_checked(
        (network_specs_key_vec, network_specs_to_send_encoded): (IVec, IVec),
    ) -> Result<Self> {
        let network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
        Self::from_entry_with_key_checked(&network_specs_key, network_specs_to_send_encoded)
    }
}

/// Network properties that must be fetched with RPC call for properties
/// in each compatible network
#[derive(Decode, Encode, PartialEq, Eq, Debug)]
#[cfg(feature = "active")]
pub struct NetworkProperties {
    pub base58prefix: u16,
    pub decimals: u8,
    pub unit: String,
}

/// Verifier information
///
/// Either real verifier or information that there is no verifier.
#[derive(Decode, Encode, PartialEq, Eq, Debug, Clone)]
pub struct Verifier {
    pub v: Option<VerifierValue>,
}

/// Information on known and existing verifier  
///
/// Verifier public key with associated encryption algorithm.
#[derive(Decode, Encode, PartialEq, Eq, Debug, Clone)]
pub enum VerifierValue {
    /// public key for standard substrate-compatible encryption algorithms  
    Standard { m: MultiSigner },
}

impl Verifier {
    /// Get the [`MVerifierDetails`] for UI to show.
    pub fn show_card(&self) -> MVerifierDetails {
        match &self.v {
            Some(a) => a.show_card(),
            None => MVerifierDetails {
                public_key: String::new(),
                identicon: Identicon::default(),
                encryption: String::new(),
            },
        }
    }

    /// Display [`Verifier`] in human-readable format, for errors  
    pub fn show_error(&self) -> String {
        match &self.v {
            Some(a) => a.show_error(),
            None => String::from("none"),
        }
    }
}

impl VerifierValue {
    /// Get the [`MVerifierDetails`] for UI to show.
    pub fn show_card(&self) -> MVerifierDetails {
        match &self {
            VerifierValue::Standard { m } => {
                let public_key = hex::encode(multisigner_to_public(m));
                let encryption = multisigner_to_encryption(m).show();
                let identicon = make_identicon_from_multisigner(m, IdenticonStyle::Dots);

                MVerifierDetails {
                    public_key,
                    identicon,
                    encryption,
                }
            }
        }
    }

    /// Display [`VerifierValue`] in human-readable format, for errors  
    pub fn show_error(&self) -> String {
        match &self {
            VerifierValue::Standard {
                m: MultiSigner::Ed25519(x),
            } => {
                format!("public key: {}, encryption: ed25519", hex::encode(x.0))
            }
            VerifierValue::Standard {
                m: MultiSigner::Sr25519(x),
            } => {
                format!("public key: {}, encryption: sr25519", hex::encode(x.0))
            }
            VerifierValue::Standard {
                m: MultiSigner::Ecdsa(x),
            } => {
                format!("public key: {}, encryption: ecdsa", hex::encode(x.0))
            }
        }
    }
}

/// Current network verifier
#[derive(Decode, Encode, PartialEq, Eq, Debug, Clone)]
pub enum CurrentVerifier {
    /// Verifier is valid, Vault can use the network
    Valid(ValidCurrentVerifier),
}

/// Possible variants of valid current network verifier
///
/// Could be general verifier (by default for networks Polkadot, Kusama, Westend),
/// or custom verifier.
#[derive(Decode, Encode, PartialEq, Eq, Debug, Clone)]
pub enum ValidCurrentVerifier {
    /// Network has general verifier
    General,

    /// Network has some other verifier, different from the general one
    Custom { v: Verifier },
}
