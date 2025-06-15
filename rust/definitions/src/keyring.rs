//! Key types and key generation for hot and cold databases
//!
//! Cold database has following trees:  
//!
//! - `SPECSTREE`, for network specs `OrderedNetworkSpecs` entries, with keys
//!   [`NetworkSpecsKey`]  
//! - `VERIFIERS`, for network verifier [`CurrentVerifier`](crate::network_specs::CurrentVerifier)
//!   entries, with keys [`VerifierKey`]  
//! - `METATREE`, for `Vec<u8>` metadata entries, with keys [`MetaKey`] and
//!   prefix search with [`MetaKeyPrefix`]  
//! - `ADDRTREE`, for [`AddressDetails`](crate::users::AddressDetails) entries
//!   with public information associated with user addresses, with keys
//!   [`AddressKey`]  
//! - `SETTREE`, for types information, Vault danger status, and general
//!   verifier  
//! - `TRANSACTION`, to temporarily store transaction information while waiting
//!   for user approval  
//! - `HISTORY`, for [`Entry`](crate::history::Entry) log of all events
//!   happening in Vault, with keys [`Order`]
//!
//! Hot database has following trees:  
//!
//! - `SPECSTREEPREP`, for network specs [`NetworkSpecs`](crate::network_specs::NetworkSpecs)
//!   entries, with keys [`NetworkSpecsKey`]  
//! - `METATREE`, for `Vec<u8>` metadata entries, with keys [`MetaKey`] and
//!   prefix search with [`MetaKeyPrefix`]  
//! - `META_HISTORY`, for [`H256`] block hash entries, with keys [`MetaKey`] and
//!   prefix search with [`MetaKeyPrefix`]
//! - `SETTREE`, for types information  
//! - `ADDRESS_BOOK` for `AddressBookEntry` data needed to maintain hot database
//!   and send RPC calls to fetch network information, with keys `AddressBookKey`
//!
use parity_scale_codec::{Decode, Encode};
use sled::IVec;
use sp_core::{ecdsa, ed25519, sr25519, H256};
use sp_runtime::MultiSigner;

use crate::helpers::{get_multisigner, unhex};
use crate::{
    crypto::Encryption,
    error::{Error, Result},
};

/// Key in `SPECSTREE` tree (cold database) and in `SPECSPREPTREE` (hot database)  
///
/// [`NetworkSpecsKey`] is used to retrieve the
/// [`OrderedNetworkSpecs`](crate::network_specs::OrderedNetworkSpecs) in cold database and
/// [`NetworkSpecs`](crate::network_specs::NetworkSpecs) in hot
/// database.  
///
/// Key is derived from network genesis hash and encryption algorithm.  
///
/// Network could support more than one encryption algorithm. In this case
/// there would be more than one database entry with different
/// [`NetworkSpecsKey`] values. Such entries do not conflict.  
#[derive(Decode, Hash, Encode, PartialEq, Eq, Debug, Clone)]
pub struct NetworkSpecsKey(Vec<u8>);

/// Decoded `NetworkSpecsKey` content, encryption-based variants with vector
/// genesis hash inside
#[derive(Decode, Encode)]
enum NetworkSpecsKeyContent {
    Ed25519(H256),
    Sr25519(H256),
    Ecdsa(H256),
    Ethereum(H256),
}

impl NetworkSpecsKey {
    /// Generate [`NetworkSpecsKey`] from parts: network genesis hash and
    /// [`Encryption`]
    pub fn from_parts(genesis_hash: &H256, encryption: &Encryption) -> Self {
        let network_key_content = match encryption {
            Encryption::Ed25519 => NetworkSpecsKeyContent::Ed25519(*genesis_hash),
            Encryption::Sr25519 => NetworkSpecsKeyContent::Sr25519(*genesis_hash),
            Encryption::Ecdsa => NetworkSpecsKeyContent::Ecdsa(*genesis_hash),
            Encryption::Ethereum => NetworkSpecsKeyContent::Ethereum(*genesis_hash),
        };
        Self(network_key_content.encode())
    }

    /// Transform database `IVec` key into [`NetworkSpecsKey`] prior to processing  
    ///
    /// Infallible, no check of encryption validity is done here.
    pub fn from_ivec(ivec: &IVec) -> Self {
        Self(ivec.to_vec())
    }

    /// Transform hexadecimal `String` into [`NetworkSpecsKey`]  
    ///
    /// Vault receives hexadecimal strings from user interface.
    ///
    /// This function checks only that hexadecimal format is valid, no check
    /// of encryption validity is done here.  
    pub fn from_hex(hex_line: &str) -> Result<Self> {
        Ok(Self(unhex(hex_line)?))
    }

    /// Get genesis hash as `H256` and [`Encryption`] from [`NetworkSpecsKey`]
    pub fn genesis_hash_encryption(&self) -> Result<(H256, Encryption)> {
        match <NetworkSpecsKeyContent>::decode(&mut &self.0[..])? {
            NetworkSpecsKeyContent::Ed25519(b) => Ok((b, Encryption::Ed25519)),
            NetworkSpecsKeyContent::Sr25519(b) => Ok((b, Encryption::Sr25519)),
            NetworkSpecsKeyContent::Ecdsa(b) => Ok((b, Encryption::Ecdsa)),
            NetworkSpecsKeyContent::Ethereum(b) => Ok((b, Encryption::Ethereum)),
        }
    }

    /// Transform [`NetworkSpecsKey`] into `Vec<u8>` database key  
    pub fn key(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

/// Key in `VERIFIERS` tree (cold database)  
///
/// [`VerifierKey`] is used to retrieve network verifier information.  
///
/// Key is derived from network genesis hash.  
///
/// Same [`VerifierKey`] and same [`CurrentVerifier`](crate::network_specs::CurrentVerifier)
/// are corresponding to all network-associated information:
///
/// - network specs, for any encryption algorithm  
/// - network metadata
#[derive(Decode, Encode, Debug, Clone, PartialEq, Eq)]
pub struct VerifierKey(H256);

impl VerifierKey {
    /// Generate [`VerifierKey`] from network genesis hash
    pub fn from_parts(genesis_hash: H256) -> Self {
        Self(genesis_hash)
    }

    /// Transform database `IVec` key into [`VerifierKey`]  
    pub fn from_ivec(ivec: &IVec) -> Result<Self> {
        let bytes: [u8; 32] = ivec
            .to_vec()
            .try_into()
            .map_err(|_| Error::WrongPublicKeyLength)?;
        Ok(Self(bytes.into()))
    }

    /// Get genesis hash from the [`VerifierKey`]
    pub fn genesis_hash(&self) -> H256 {
        self.0
    }

    /// Transform [`VerifierKey`] into `Vec<u8>` database key  
    pub fn key(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}

// Used to export keyset
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Encode, Decode, Debug)]
pub struct RootKeyInfo {
    // used as a keyset identifier in transactions, e.g. dynamic derivation
    key_id: [u8; 32],

    // root public keys depending on derivation method, e.g. substrate and ethereum
    public_keys: Vec<RootPublicKey>,
}

impl RootKeyInfo {
    pub fn new(key_id: [u8; 32], public_keys: Vec<RootPublicKey>) -> Self {
        Self {
            key_id,
            public_keys,
        }
    }
}

/// Used when referring to a root key of the keyset
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Encode, Decode, Debug)]
pub enum RootPublicKey {
    Ed25519(ed25519::Public),
    Sr25519(sr25519::Public),
    Ecdsa(ecdsa::Public),
    Ethereum(ecdsa::Public),
}

/// Key in `ADDRTREE` tree (cold database)  
///
/// [`AddressKey`] is used to retrieve the address associated public information.  
///
/// Key is derived from public key and encryption algorithm.  
///
/// To create an address in Vault, sufficient information is:
///
/// - seed phrase  
/// - derivation: soft (`/`) and hard (`//`) junctions and password (`///`)  
/// - encryption algorithm  
/// - network for address to be used with (network must support the encryption
///   algorithm)  
///
/// The seed phrase and password are **not** stored in rust-managed database.
/// For storing seed phrases, Vault device's own key management system is used.
/// Passwords are never stored.  
/// Cold database stores only **non-secret** address associated information.  
///
/// Each address is defined by combination of public key, encryption algorithm,
/// and network.  
///
/// More than one address could be created for same seed phrase and derivation,
/// with same encryption algorithm, but for different networks.
///
/// For the user interface these addresses would appear as separate entities,
/// however, the database stores them under same [`AddressKey`], with a set of
/// allowed networks.  
#[derive(Decode, Encode, Debug, PartialEq, Eq, Clone)]
pub struct AddressKey {
    multisigner: MultiSigner,
    /// the root address is not used on any network and hence has no genesis hash.
    genesis_hash: Option<H256>,
}

impl AddressKey {
    /// Generate [`AddressKey`] from corresponding
    /// [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html) value  
    /// and a network prefix.
    pub fn new(multisigner: MultiSigner, genesis_hash: Option<H256>) -> Self {
        Self {
            multisigner,
            genesis_hash,
        }
    }

    /// Generate [`AddressKey`] from parts: raw public key and [`Encryption`]  
    ///
    /// Could result in error if public key length does not match the
    /// expected length for chosen encryption algorithm.  
    pub fn from_parts(
        public: &[u8],
        encryption: &Encryption,
        genesis_hash: Option<H256>,
    ) -> Result<Self> {
        let multisigner = get_multisigner(public, encryption)?;
        Ok(Self::new(multisigner, genesis_hash))
    }

    /// Transform database `IVec` key into [`AddressKey`] prior to processing  
    ///
    /// Infallible, the validity of resulting `AddressKey` is not checked.
    pub fn from_ivec(ivec: &IVec) -> Result<Self> {
        let vec = ivec.to_vec();
        Ok(Self::decode(&mut &vec[..])?)
    }

    /// Transform hexadecimal `String` into [`AddressKey`]  
    ///
    /// Vault receives hexadecimal strings from user interface.
    ///
    /// This function checks only that hexadecimal format is valid, no length
    /// check happens here.  
    pub fn from_hex(hex_address_key: &str) -> Result<Self> {
        Ok(Self::decode(&mut &unhex(hex_address_key)?[..])?)
    }

    /// Get public key and [`Encryption`] from the [`AddressKey`]  
    pub fn public_key_encryption(&self) -> Result<(Vec<u8>, Encryption)> {
        match &self.multisigner {
            MultiSigner::Ed25519(b) => Ok((b.to_vec(), Encryption::Ed25519)),
            MultiSigner::Sr25519(b) => Ok((b.to_vec(), Encryption::Sr25519)),
            MultiSigner::Ecdsa(b) => Ok((b.0.to_vec(), Encryption::Ecdsa)),
        }
    }

    /// Get [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)
    /// from the [`AddressKey`]  
    pub fn multi_signer(&self) -> &MultiSigner {
        &self.multisigner
    }

    /// Transform [`AddressKey`] into `Vec<u8>` database key  
    pub fn key(&self) -> Vec<u8> {
        self.encode()
    }
}

/// Key in `METATREE` (cold and hot database) and in `META_HISTORY` tree (hot
/// database)
///
/// [`MetaKey`] is used to retrieve raw `Vec<u8>` metadata from `METATREE` and
/// relevant block hash entries from `META_HISTORY`.  
///
/// Key is derived from network name as it appears in the metadata and network
/// version.  
///
/// Each [`MetaKey`] corresponds to single metadata entry and each metadata
/// entry has unique [`MetaKey`]. This is so because:
///
/// - Metadata that could be used in Vault must contain `Version` constant in
///   pallet `System`, and only such metadata can be added in the databases.  
///
/// - Two raw metadata entries corresponding to same network name and network
///   version must be identical. If the metadata changes without bumping the
///   network version, both Vault and hot database client would produce an error.
///   It is not possible to switch the metadata in cold or hot database to the
///   changed one without removing the old entry first.  
#[derive(Debug, Clone)]
pub struct MetaKey(Vec<u8>);

/// Decoded `MetaKey` content, struct with network name and network version inside  
#[derive(Decode, Encode)]
struct MetaKeyContent {
    name: String,
    version: u32,
}

impl MetaKey {
    /// Generate [`MetaKey`] from parts: network name and network version
    pub fn from_parts(name: &str, version: u32) -> Self {
        let meta_key_content = MetaKeyContent {
            name: name.to_string(),
            version,
        };
        Self(meta_key_content.encode())
    }

    /// Transform database `IVec` key into [`MetaKey`] prior to processing  
    ///
    /// Infallible, the validity of resulting `MetaKey` is not checked.
    pub fn from_ivec(ivec: &IVec) -> Self {
        Self(ivec.to_vec())
    }

    /// Get network name and network version from the [`MetaKey`]
    ///
    /// Could result in error if key is corrupted.  
    pub fn name_version(&self) -> Result<(String, u32)> {
        let res = <MetaKeyContent>::decode(&mut &self.0[..])?;
        Ok((res.name, res.version))
    }

    /// Transform [`MetaKey`] into `Vec<u8>` database key  
    pub fn key(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

/// Prefix for searching in `METATREE` (cold and hot database) and in
/// `META_HISTORY` tree (hot database)
///
/// [`MetaKeyPrefix`] is used to retrieve all available `Vec<u8>` metadata
/// for a given network name from `METATREE` and all relevant block hash entries
/// from `META_HISTORY`.  
///
/// Prefix is derived from network name as it appears in the metadata.  
///
/// [`MetaKey`] consists of concatenated encoded network name and encoded
/// network version.
/// [`MetaKeyPrefix`] consists only of encoded network name, and is therefore
/// a common prefix for all [`MetaKey`] corresponding to the given network
/// name and all available network versions.  
#[derive(Debug)]
pub struct MetaKeyPrefix(Vec<u8>);

/// Decoded `MetaKeyPrefix` content, struct with network name inside  
#[derive(Decode, Encode)]
struct MetaKeyPrefixContent(String);

impl MetaKeyPrefix {
    /// Generate [`MetaKeyPrefix`] from network name
    pub fn from_name(name: &str) -> Self {
        let meta_key_prefix_content = MetaKeyPrefixContent(name.to_string());
        Self(meta_key_prefix_content.encode())
    }

    /// Transform [`MetaKeyPrefix`] into `Vec<u8>` database key prefix  
    pub fn prefix(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

/// Key in `HISTORY` tree (cold database)  
///
/// [`Order`] is used to retrieve history log entry.  
///
/// History log [`Entry`](crate::history::Entry) contains timestamp and a set
/// of simultaneously occurred events.
///
/// Order is generated from the number of the history entry in the database
/// `HISTORY` tree.  
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Order(u32);

impl Order {
    /// Transform database `IVec` key into [`Order`].  
    ///
    /// If `Order` could not be decoded, i.e. entry is corrupted, produces an
    /// error.  
    pub fn from_ivec(ivec: &IVec) -> Result<Self> {
        Ok(Self(<u32>::decode(&mut &ivec[..])?))
    }

    /// Generate [`Order`] from `u32` number
    pub fn from_number(n: u32) -> Self {
        Self(n)
    }

    /// Produce `u32` number from the [`Order`].
    ///
    /// Number here is the number of history entry in the `HISTORY` database.
    pub fn stamp(&self) -> u32 {
        self.0
    }

    /// Transform [`Order`] into `Vec<u8>` database key
    pub fn store(&self) -> Vec<u8> {
        self.0.encode()
    }
}

/// Key in `ADDRESS_BOOK` tree (hot database)  
///
/// Key is used to retrieve the `AddressBookEntry` for network.
///
/// Key is a SCALE-encoded address book title, which is either a network name
/// as it is stated in the metadata for default networks, or the name with
/// `-<encryption>` added for non-default networks.
///
/// Database could have a few entries for related networks, for example,
/// entry "westend" for default Westend, and entry "westend-ed25519" for
/// Westend with `Ed25519` encryption. Such entries would not conflict.
#[derive(Debug, Clone)]
#[cfg(feature = "active")]
pub struct AddressBookKey(Vec<u8>);

/// Decoded `AddressBookKey` content, struct with network address book title inside  
#[derive(Decode, Encode)]
#[cfg(feature = "active")]
struct AddressBookKeyContent(String);

#[cfg(feature = "active")]
impl AddressBookKey {
    /// Generate [`AddressBookKey`] from network address book title
    pub fn from_title(title: &str) -> Self {
        let address_book_key_content = AddressBookKeyContent(title.to_string());
        Self(address_book_key_content.encode())
    }

    /// Transform database `IVec` key into [`AddressBookKey`] prior to processing  
    ///
    /// Infallible, the validity of resulting `AddressBookKey` is not checked.
    pub fn from_ivec(ivec: &IVec) -> Self {
        Self(ivec.to_vec())
    }

    /// Get the network address book title from the [`AddressBookKey`]
    ///
    /// Could result in error if key is corrupted.
    pub fn title(&self) -> Result<String> {
        Ok(<AddressBookKeyContent>::decode(&mut &self.0[..])?.0)
    }

    /// Transform [`AddressBookKey`] into `Vec<u8>` database key  
    pub fn key(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_in_network_specs_key_signer() {
        let network_specs_key_hex =
            "0450e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let network_specs_key = NetworkSpecsKey::from_hex(network_specs_key_hex).unwrap();
        let error = network_specs_key.genesis_hash_encryption().unwrap_err();
        if let Error::CodecError(_) = error {
        } else {
            panic!("Expected codec error, received {error:?}");
        }
    }

    #[test]
    fn error_in_network_specs_key_active() {
        let network_specs_key_hex =
            "0450e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let network_specs_key =
            NetworkSpecsKey::from_ivec(&IVec::from(hex::decode(network_specs_key_hex).unwrap()));
        let error = network_specs_key.genesis_hash_encryption().unwrap_err();
        if let Error::CodecError(_) = error {
        } else {
            panic!("Expected codec error, received {error:?}");
        }
    }
}
