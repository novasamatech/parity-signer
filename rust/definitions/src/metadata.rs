//! Network metadata and related types
//!
//! The main purpose of the Vault is to generate signatures for transactions.
//! Vault reads the transactions as QR codes with SCALE-encoded
//! information.
//! Any transaction before user is able to sign it must be decoded.
//!
//! Transaction decoding uses network metadata, and Vault needs latest
//! available network metadata to parse freshly generated transactions.  
//!
//! New network metadata could be added to Vault through scanning `load_metadata`
//! QR code for the network metadata. Vault allows loading new metadata only if
//! the network has network specs in the database and the incoming `load_metadata`
//! payload is signed by the verifier already associated with the network.  
//!
//! Metadata is stored both in cold and in hot databases tree `METATREE` as
//! SCALE-encoded [`MetaValues`] under key [`MetaKey`].  

use frame_metadata::{decode_different::DecodeDifferent, v14::RuntimeMetadataV14, RuntimeMetadata};
use parity_scale_codec::{Decode, Encode};
#[cfg(feature = "active")]
use sc_executor_common::{
    runtime_blob::RuntimeBlob,
    wasm_runtime::{InvokeMethod, WasmModule},
};
#[cfg(feature = "active")]
use sc_executor_wasmi::create_runtime;
use sled::IVec;
#[cfg(feature = "active")]
use sp_core::H256;
#[cfg(feature = "active")]
use sp_io::SubstrateHostFunctions;
use sp_version::RuntimeVersion;
#[cfg(feature = "active")]
use sp_wasm_interface::HostFunctions;
use std::collections::HashMap;

#[cfg(feature = "active")]
use crate::{crypto::Encryption, error_active::Wasm, helpers::unhex, keyring::AddressBookKey};
use crate::{
    error::{Error, MetadataError, Result},
    keyring::MetaKey,
};

/// Network information extracted from the metadata: name, version, optional
/// base58 prefix, warning about extensions incompatible with transaction
/// parsing for `RuntimeMetadata` with version 14 and above
#[derive(Decode, Encode, PartialEq, Eq)]
pub struct MetaInfo {
    /// Network name, from metadata `Version` constant  
    pub name: String,

    /// Network version, from in metadata `Version` constant  
    pub version: u32,

    /// Network base58 prefix, could be encountered in metadata `SS58Prefix`
    /// constant  
    ///
    /// If `SS58Prefix` constant is present in metadata, the prefix derived
    /// from it is expected to match `base58prefix` from `OrderedNetworkSpecs`.
    pub optional_base58prefix: Option<u16>,

    /// Flag to indicate that extrinsic set from metadata with
    /// [`RuntimeMetadataV14`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/v14/struct.RuntimeMetadataV14.html)
    /// is insufficient for transaction decoding  
    pub warn_incomplete_extensions: bool,
}

/// Metadata values: name, version, optional base58 prefix, warning about
/// extensions incompatible with transaction parsing for `RuntimeMetadata` with
/// version 14 and above, and metadata itself as raw `Vec<u8>`
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MetaValues {
    /// Network name, from metadata `Version` constant  
    pub name: String,

    /// Network version, from in metadata `Version` constant  
    pub version: u32,

    /// Network base58 prefix, could be encountered in metadata `SS58Prefix`
    /// constant  
    ///
    /// If `SS58Prefix` constant is present in metadata, the prefix derived
    /// from it is expected to match `base58prefix` from `OrderedNetworkSpecs`.
    pub optional_base58prefix: Option<u16>,

    /// Flag to indicate that extrinsic set from metadata with
    /// [`RuntimeMetadataV14`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/v14/struct.RuntimeMetadataV14.html)
    /// is insufficient for transaction decoding  
    pub warn_incomplete_extensions: bool,

    /// raw metadata
    pub meta: Vec<u8>,
}

impl MetaValues {
    /// Generates [`MetaValues`] from value extracted from database tree
    /// `METATREE` either in cold or in hot database using known associated
    /// network name and network version  
    ///
    /// Checks that input name and version match the ones in metadata `Version`
    /// constant.  
    pub fn from_entry_name_version_checked(
        name: &str,
        version: u32,
        meta_encoded: IVec,
    ) -> Result<Self> {
        let meta_values = Self::from_slice_metadata(&meta_encoded)?;
        if (meta_values.name != name) || (meta_values.version != version) {
            return Err(Error::MetadataMismatch {
                this_name: name.to_string(),
                this_version: version,
                that_name: meta_values.name.to_string(),
                that_version: meta_values.version,
            });
        }
        Ok(meta_values)
    }

    /// Gets [`MetaValues`] from either cold or hot database tree `METATREE`
    /// (key, value) entry  
    ///
    /// Checks that name and version from [`MetaKey`] match the ones in metadata
    /// `Version` constant.  
    pub fn from_entry_checked((meta_key_vec, meta_encoded): (IVec, IVec)) -> Result<Self> {
        let (name, version) = MetaKey::from_ivec(&meta_key_vec).name_version()?;
        Self::from_entry_name_version_checked(&name, version, meta_encoded)
    }

    /// Gets [`MetaValues`] from raw metadata in `Vec<u8>` format
    ///
    /// Produces [`MetadataError`] if the metadata is somehow not suitable for
    /// use in Vault.
    pub fn from_slice_metadata(meta_slice: &[u8]) -> Result<Self> {
        let meta_info = info_from_metadata(&runtime_metadata_from_slice(meta_slice)?)?;
        Ok(Self {
            name: meta_info.name.to_string(),
            version: meta_info.version,
            optional_base58prefix: meta_info.optional_base58prefix,
            warn_incomplete_extensions: meta_info.warn_incomplete_extensions,
            meta: meta_slice.to_vec(),
        })
    }

    /// Gets [`MetaValues`] from `wasm` file
    ///
    /// Could be used to generate metadata updates before metadata release.
    #[cfg(feature = "active")]
    pub fn from_wasm_file(filename: &str) -> Result<Self> {
        let metadata = convert_wasm_into_metadata(filename)?;
        Self::from_slice_metadata(&metadata)
    }

    /// Gets [`MetaValues`] from raw hexadecimal metadata
    ///
    /// Is used only on Active side, for:
    /// (a) default and test metadata loading;
    /// (b) decoding and evaluating fetched metadata;
    #[cfg(feature = "active")]
    pub fn from_str_metadata(meta: &str) -> Result<Self> {
        let meta_vec = unhex(meta)?;
        Self::from_slice_metadata(&meta_vec)
    }
}

/// Extracts raw metadata in `Vec<u8>` format from `wasm` file.
///
/// Is used only on Active side, to generate metadata updates before metadata
/// release.
#[cfg(feature = "active")]
pub fn convert_wasm_into_metadata(filename: &str) -> Result<Vec<u8>> {
    let buffer = std::fs::read(filename).map_err(Wasm::File)?;
    let runtime_blob = RuntimeBlob::uncompress_if_needed(&buffer).map_err(Wasm::WasmError)?;
    let wasmi_runtime = create_runtime(
        runtime_blob,
        sc_executor_common::wasm_runtime::HeapAllocStrategy::Static { extra_pages: 64 },
        SubstrateHostFunctions::host_functions(),
        false,
    )
    .map_err(Wasm::WasmError)?;
    let mut wasmi_instance = wasmi_runtime.new_instance().map_err(Wasm::Executor)?;
    let data = wasmi_instance
        .call(InvokeMethod::Export("Metadata_metadata"), &[])
        .map_err(Wasm::Executor)?;
    Ok(<Vec<u8>>::decode(&mut &data[..]).map_err(|_| Wasm::DecodingMetadata)?)
}

/// Get [`MetaInfo`] from
/// [`RuntimeMetadata`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/enum.RuntimeMetadata.html)
///
/// Searches `System` pallet within the metadata, gets from it `Version` and
/// optionally `SS58Prefix` constants.
///
/// Produces [`MetaInfo`] if the metadata is suitable for the Vault, and
/// [`MetadataError`] if not.
///
/// `RuntimeMetadata` suitable for use in Vault:
///
/// - must be of runtime version `V12` or above
/// - must have 'System' pallet  
/// - must have `Version` constant in `System` pallet, SCALE-decodeable  
/// - can have `SS58Prefix` constant in `System` pallet, and if it does, the
///   constant must be SCALE-decodeable  
///
/// Additionally, for [`RuntimeMetadataV14`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/v14/struct.RuntimeMetadataV14.html)
/// the extensions set must be decoding-compatible for any signable transaction.  
pub fn info_from_metadata(runtime_metadata: &RuntimeMetadata) -> Result<MetaInfo> {
    let mut runtime_version_encoded: Option<&[u8]> = None;
    let mut base58_prefix_encoded: Option<&[u8]> = None;
    let mut warn_incomplete_extensions = false;
    let mut system_block = false;
    match runtime_metadata {
        RuntimeMetadata::V12(metadata_v12) => {
            if let DecodeDifferent::Decoded(meta_vector) = &metadata_v12.modules {
                for x in meta_vector.iter() {
                    if x.name == DecodeDifferent::Encode("System") {
                        system_block = true;
                        if let DecodeDifferent::Decoded(constants_vector) = &x.constants {
                            for y in constants_vector.iter() {
                                if y.name == DecodeDifferent::Encode("Version") {
                                    if let DecodeDifferent::Decoded(fin) = &y.value {
                                        runtime_version_encoded = Some(fin);
                                    }
                                }
                                if y.name == DecodeDifferent::Encode("SS58Prefix") {
                                    if let DecodeDifferent::Decoded(fin) = &y.value {
                                        base58_prefix_encoded = Some(fin);
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
        RuntimeMetadata::V13(metadata_v13) => {
            if let DecodeDifferent::Decoded(meta_vector) = &metadata_v13.modules {
                for x in meta_vector.iter() {
                    if x.name == DecodeDifferent::Encode("System") {
                        system_block = true;
                        if let DecodeDifferent::Decoded(constants_vector) = &x.constants {
                            for y in constants_vector.iter() {
                                if y.name == DecodeDifferent::Encode("Version") {
                                    if let DecodeDifferent::Decoded(fin) = &y.value {
                                        runtime_version_encoded = Some(fin);
                                    }
                                }
                                if y.name == DecodeDifferent::Encode("SS58Prefix") {
                                    if let DecodeDifferent::Decoded(fin) = &y.value {
                                        base58_prefix_encoded = Some(fin);
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
        RuntimeMetadata::V14(metadata_v14) => {
            for x in metadata_v14.pallets.iter() {
                if x.name == "System" {
                    system_block = true;
                    for y in x.constants.iter() {
                        if y.name == "Version" {
                            runtime_version_encoded = Some(&y.value)
                        }
                        if y.name == "SS58Prefix" {
                            base58_prefix_encoded = Some(&y.value)
                        }
                    }
                    break;
                }
            }
            warn_incomplete_extensions = need_v14_warning(metadata_v14);
        }
        _ => return Err(MetadataError::VersionIncompatible.into()),
    }
    if !system_block {
        return Err(MetadataError::NoSystemPallet.into());
    }
    let runtime_version = match runtime_version_encoded {
        Some(mut x) => match RuntimeVersion::decode(&mut x) {
            Ok(a) => a,
            Err(_) => return Err(MetadataError::RuntimeVersionNotDecodeable.into()),
        },
        None => return Err(MetadataError::NoVersionInConstants.into()),
    };
    let optional_base58prefix = match base58_prefix_encoded {
        Some(mut x) => match <u16>::decode(&mut x) {
            Ok(a) => Some(a),
            Err(_) => match <u8>::decode(&mut x) {
                // in some older metadata `u8` is used for base58 prefix, likely a legacy thing
                Ok(a) => Some(a as u16),
                Err(_) => return Err(MetadataError::Base58PrefixNotDecodeable.into()),
            },
        },
        None => None,
    };
    Ok(MetaInfo {
        name: runtime_version.spec_name.to_string(),
        version: runtime_version.spec_version,
        optional_base58prefix,
        warn_incomplete_extensions,
    })
}

/// Get [`RuntimeMetadata`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/enum.RuntimeMetadata.html)
/// from slice of raw `Vec<u8>` metadata
///
/// Raw `Vec<u8>` metadata suitable for use in Vault:
///
/// - must begin with b"meta"  
/// - after that must be SCALE-encoded `RuntimeMetadata` with runtime version `V12` or above
pub fn runtime_metadata_from_slice(meta: &[u8]) -> Result<RuntimeMetadata> {
    if !meta.starts_with(&[109, 101, 116, 97]) {
        return Err(MetadataError::NotMeta.into());
    }
    if meta[4] < 12 {
        return Err(MetadataError::VersionIncompatible.into());
    }
    Ok(RuntimeMetadata::decode(&mut &meta[4..]).map_err(|_| MetadataError::UnableToDecode)?)
}

/// Checks if the `v14` metadata has all signed extensions required for transaction decoding.
/// True if extensions are incomplete.
///
/// Currently, the decoding of the transaction demands that metadata version, network genesis hash,
/// and era are among signed extensions. Otherwise, a `ParserMetadataError` would occur on decoding.
/// However, we can not simply forbid the loading of the metadata without required set of
/// signed extensions into Vault.
///
/// This function should be used for warnings only on `generate_message` side and during metadata
/// loading into Vault.
fn need_v14_warning(metadata_v14: &RuntimeMetadataV14) -> bool {
    let mut signed_extensions = HashMap::new();
    for x in metadata_v14.extrinsic.signed_extensions.iter() {
        let count = signed_extensions
            .entry(x.identifier.to_string())
            .or_insert(0);
        *count += 1;
    }
    !(signed_extensions.get("CheckSpecVersion") == Some(&1)
        && signed_extensions.get("CheckGenesis") == Some(&1)
        && signed_extensions.get("CheckMortality") == Some(&1)) // no warning needed if each one encountered, and only once
}

/// Metadata as checked [`RuntimeMetadata`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/enum.RuntimeMetadata.html)
/// with network info extracted from it, for transaction decoding
pub struct MetaSetElement {
    /// Network name, from metadata `Version` constant
    name: String,

    /// Network version, from in metadata `Version` constant  
    version: u32,

    /// Network base58 prefix, could be encountered in metadata `SS58Prefix`
    /// constant  
    ///
    /// If `SS58Prefix` constant is present in metadata, the prefix derived
    /// from it is expected to match `base58prefix` from `OrderedNetworkSpecs`.
    optional_base58prefix: Option<u16>,

    /// [`RuntimeMetadata`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/enum.RuntimeMetadata.html)
    /// [`MetaSetElement`] is successfully generated only if metadata is a
    /// suitable one
    runtime_metadata: RuntimeMetadata,
}

impl MetaSetElement {
    /// Generates `MetaSetElement` from Vault database tree `METATREE` (key, value)
    /// entry  
    ///
    /// Checks that name and version from [`MetaKey`] match the ones in metadata
    /// `Version` constant.  
    ///
    /// Also checks that the metadata is suitable for use in Vault. Since the
    /// metadata already was accepted in the database at some point, errors here
    /// are very unlikely to happen and would indicate the database corruption
    pub fn from_entry((meta_key_vec, meta_encoded): (IVec, IVec)) -> Result<Self> {
        let (network_name, network_version) = MetaKey::from_ivec(&meta_key_vec).name_version()?;
        let runtime_metadata = runtime_metadata_from_slice(&meta_encoded)?;
        let (name, version, optional_base58prefix) = {
            let a = info_from_metadata(&runtime_metadata)?;
            if (a.version != network_version) || (a.name != network_name) {
                return Err(Error::MetadataMismatch {
                    this_name: network_name,
                    this_version: network_version,
                    that_name: a.name.to_string(),
                    that_version: a.version,
                });
            }
            (a.name, a.version, a.optional_base58prefix)
        };
        Ok(Self {
            name,
            version,
            optional_base58prefix,
            runtime_metadata,
        })
    }

    /// Gets network name
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    /// Gets network version
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Gets optional base58 prefix, if there is one in the metadata
    pub fn optional_base58prefix(&self) -> Option<u16> {
        self.optional_base58prefix
    }

    /// Gets runtime metadata, to be used in transcation decoding
    pub fn runtime_metadata(&self) -> &RuntimeMetadata {
        &self.runtime_metadata
    }
}

/// Network information needed for RPC calls in the network and for managing the
/// hot database  
///
/// Hot database contains tree `ADDRESS_BOOK` with information needed to perform
/// RPC calls in networks and generate `load_metadata` and `add_specs` payloads.
///
/// `ADDRESS_BOOK` tree stores SCALE-encoded [`AddressBookEntry`] entries under
/// keys [`AddressBookKey`]
#[derive(Debug, Decode, Encode, PartialEq, Eq)]
#[cfg(feature = "active")]
pub struct AddressBookEntry {
    /// Network name, as it appears in `Version` constant in metadata  
    ///
    /// If network data is queried through RPC call, retrieved metadata must
    /// have exactly same network name in `Version` constant  
    pub name: String,

    /// Network genesis hash  
    ///
    /// If network data is queried through RPC call, retrieved version must
    /// be same as the one in address book  
    pub genesis_hash: H256,

    /// URL address for RPC calls, with or without preferred port
    pub address: String,

    /// [`Encryption`] that is supported by the network  
    pub encryption: Encryption,

    /// Address book entry is the default one  
    ///
    /// Default networks currently are Polkadot, Kusama, Westend with `Sr25519`
    /// encryption  
    pub def: bool,
}

#[cfg(feature = "active")]
impl AddressBookEntry {
    /// Gets [`AddressBookEntry`] from hot database tree `ADDRESS_BOOK`
    /// (key, value) entry.  
    pub fn from_entry(
        (address_book_key_encoded, address_book_entry_encoded): (IVec, IVec),
    ) -> Result<AddressBookEntry> {
        let title = AddressBookKey::from_ivec(&address_book_key_encoded).title()?;
        AddressBookEntry::from_entry_with_title(&title, &address_book_entry_encoded)
    }

    /// Gets network address book title and [`AddressBookEntry`] as a tuple from
    /// hot database tree `ADDRESS_BOOK` (key, value) entry.
    ///
    /// Network address book title **differs** from `title` in network specs.
    /// This is just a key in hot database `ADDRESS_BOOK`, and is not displayed
    /// anywhere else.  
    pub fn process_entry(
        (address_book_key_encoded, address_book_entry_encoded): (IVec, IVec),
    ) -> Result<(String, AddressBookEntry)> {
        let title = AddressBookKey::from_ivec(&address_book_key_encoded).title()?;
        let address_book_entry =
            AddressBookEntry::from_entry_with_title(&title, &address_book_entry_encoded)?;
        Ok((title, address_book_entry))
    }

    /// Gets [`AddressBookEntry`] from network address book title and associated
    /// value from hot database tree `ADDRESS_BOOK`.  
    pub fn from_entry_with_title(
        _title: &str,
        address_book_entry_encoded: &IVec,
    ) -> Result<AddressBookEntry> {
        Ok(<AddressBookEntry>::decode(
            &mut &address_book_entry_encoded[..],
        )?)
    }
}

/// `META_HISTORY` tree entry
#[derive(Debug, PartialEq, Eq)]
#[cfg(feature = "active")]
pub struct MetaHistoryEntry {
    /// network name, from key
    pub name: String,

    /// network metadata version, from key
    pub version: u32,

    /// block hash at which the metadata was fetched, from value
    pub block_hash: H256,
}

#[cfg(feature = "active")]
impl MetaHistoryEntry {
    /// From the whole entry
    pub fn from_entry((meta_key_vec, hash_encoded): (IVec, IVec)) -> Result<Self> {
        let (name, version) = MetaKey::from_ivec(&meta_key_vec).name_version()?;
        Self::from_entry_with_key_parts(&name, version, &hash_encoded)
    }

    /// From the entry with already known key parts
    pub fn from_entry_with_key_parts(
        name: &str,
        version: u32,
        hash_encoded: &IVec,
    ) -> Result<Self> {
        let block_hash = H256::decode(&mut &hash_encoded[..])?;
        Ok(MetaHistoryEntry {
            name: name.to_string(),
            version,
            block_hash,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn westend9070() {
        let filename = String::from("for_tests/westend9070");
        let meta = read_to_string(filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(meta.trim()).unwrap();
        assert!(
            meta_values.name == *"westend",
            "Unexpected network name: {}",
            meta_values.name
        );
        assert!(
            meta_values.version == 9070,
            "Unexpected network name: {}",
            meta_values.version
        );
    }

    #[test]
    fn westend9033() {
        let filename = String::from("for_tests/westend9033");
        let meta = read_to_string(filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(meta.trim()).unwrap();
        assert!(
            meta_values.name == *"westend",
            "Unexpected network name: {}",
            meta_values.name
        );
        assert!(
            meta_values.version == 9033,
            "Unexpected network name: {}",
            meta_values.version
        );
    }

    #[test]
    fn westend9030() {
        let filename = String::from("for_tests/westend9030");
        let meta = read_to_string(filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(meta.trim()).unwrap();
        assert!(
            meta_values.name == *"westend",
            "Unexpected network name: {}",
            meta_values.name
        );
        assert!(
            meta_values.version == 9030,
            "Unexpected network name: {}",
            meta_values.version
        );
    }

    #[test]
    fn rococo9004() {
        let filename = String::from("for_tests/rococo9004");
        let meta = read_to_string(filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(meta.trim()).unwrap();
        assert!(
            meta_values.name == *"rococo",
            "Unexpected network name: {}",
            meta_values.name
        );
        assert!(
            meta_values.version == 9004,
            "Unexpected network name: {}",
            meta_values.version
        );
    }

    #[test]
    fn rococo9002() {
        let filename = String::from("for_tests/rococo9002");
        let meta = read_to_string(filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(meta.trim()).unwrap();
        assert!(
            meta_values.name == *"rococo",
            "Unexpected network name: {}",
            meta_values.name
        );
        assert!(
            meta_values.version == 9002,
            "Unexpected network name: {}",
            meta_values.version
        );
    }

    #[test]
    fn polkadot9080() {
        let filename = String::from("for_tests/polkadot9080");
        let meta = read_to_string(filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(meta.trim()).unwrap();
        assert!(
            meta_values.name == *"polkadot",
            "Unexpected network name: {}",
            meta_values.name
        );
        assert!(
            meta_values.version == 9080,
            "Unexpected network name: {}",
            meta_values.version
        );
    }

    #[test]
    fn polkadot30() {
        let filename = String::from("for_tests/polkadot30");
        let meta = read_to_string(filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(meta.trim()).unwrap();
        assert!(
            meta_values.name == *"polkadot",
            "Unexpected network name: {}",
            meta_values.name
        );
        assert!(
            meta_values.version == 30,
            "Unexpected network name: {}",
            meta_values.version
        );
    }

    #[test]
    fn polkadot29() {
        let filename = String::from("for_tests/polkadot29");
        let meta = read_to_string(filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(meta.trim()).unwrap();
        assert!(
            meta_values.name == *"polkadot",
            "Unexpected network name: {}",
            meta_values.name
        );
        assert!(
            meta_values.version == 29,
            "Unexpected network name: {}",
            meta_values.version
        );
    }

    #[test]
    fn kusama9040() {
        let filename = String::from("for_tests/kusama9040");
        let meta = read_to_string(filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(meta.trim()).unwrap();
        assert!(
            meta_values.name == *"kusama",
            "Unexpected network name: {}",
            meta_values.name
        );
        assert!(
            meta_values.version == 9040,
            "Unexpected network name: {}",
            meta_values.version
        );
    }

    #[test]
    fn kusama9010() {
        let filename = String::from("for_tests/kusama9010");
        let meta = read_to_string(filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(meta.trim()).unwrap();
        assert!(
            meta_values.name == *"kusama",
            "Unexpected network name: {}",
            meta_values.name
        );
        assert!(
            meta_values.version == 9010,
            "Unexpected network name: {}",
            meta_values.version
        );
    }

    #[test]
    fn edgeware() {
        let filename = String::from("for_tests/edgeware");
        let meta = read_to_string(filename).unwrap();

        match MetaValues::from_str_metadata(meta.trim()) {
            Ok(x) => panic!("Unexpectedly decoded as {} version {}", x.name, x.version),
            Err(e) => {
                if let Error::MetadataError(MetadataError::NoVersionInConstants) = e {
                } else {
                    panic!("expected Error::WrongPublicKeyLength, got {e:?}");
                }
            }
        }
    }

    #[test]
    fn centrifuge_amber() {
        let filename = String::from("for_tests/centrifugeAmber");
        let meta = read_to_string(filename).unwrap();

        match MetaValues::from_str_metadata(meta.trim()) {
            Ok(x) => panic!("Unexpectedly decoded as {} version {}", x.name, x.version),
            Err(e) => {
                if let Error::MetadataError(MetadataError::VersionIncompatible) = e {
                } else {
                    panic!("expected Error::WrongPublicKeyLength, got {e:?}");
                }
            }
        }
    }

    #[test]
    fn westend9150() {
        let filename = String::from("for_tests/westend9150");
        let meta = read_to_string(filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(meta.trim()).unwrap();
        assert!(
            meta_values.name == *"westend",
            "Unexpected network name: {}",
            meta_values.name
        );
        assert!(
            meta_values.version == 9150,
            "Unexpected network name: {}",
            meta_values.version
        );
        assert!(
            !meta_values.warn_incomplete_extensions,
            "Expected complete extensions in westend9150."
        )
    }

    #[test]
    fn shell200() {
        let filename = String::from("for_tests/shell200");
        let meta = read_to_string(filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(meta.trim()).unwrap();
        assert!(
            meta_values.name == *"shell",
            "Unexpected network name: {}",
            meta_values.name
        );
        assert!(
            meta_values.version == 200,
            "Unexpected network name: {}",
            meta_values.version
        );
        assert!(
            meta_values.warn_incomplete_extensions,
            "Expected incomplete extensions warning in shell200."
        )
    }

    #[test]
    fn unwasm_westend9150() {
        let filename = "for_tests/westend_runtime-v9150.compact.compressed.wasm";
        let meta_values = MetaValues::from_wasm_file(filename).unwrap();
        assert!(
            meta_values.name == *"westend",
            "Unexpected network name: {}",
            meta_values.name
        );
        assert!(
            meta_values.version == 9150,
            "Unexpected network name: {}",
            meta_values.version
        );
        assert!(
            !meta_values.warn_incomplete_extensions,
            "Expected complete extensions in westend9150."
        )
    }
}
