//! Network metadata and related types
//!
//! The main purpose of the Signer is to generate signatures for transactions.
//! Signer reads the transactions as QR codes with SCALE-encoded
//! information.
//! Any transaction before user is able to sign it must be decoded.
//!
//! Transaction decoding uses network metadata, and Signer needs latest
//! available network metadata to parse freshly generated transactions.  
//!
//! New network metadata could be added to Signer through scanning `load_metadata`
//! QR code for the network metadata. Signer allows loading new metadata only if
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
use sp_io::SubstrateHostFunctions;
use sp_version::RuntimeVersion;
#[cfg(feature = "active")]
use sp_wasm_interface::HostFunctions;
use std::collections::HashMap;

#[cfg(feature = "signer")]
use crate::error_signer::{ErrorSigner, Signer};
#[cfg(feature = "active")]
use crate::{
    crypto::Encryption,
    error_active::{
        Active, DatabaseActive, EntryDecodingActive, ErrorActive, IncomingMetadataSourceActive,
        IncomingMetadataSourceActiveStr, NotHexActive, Wasm,
    },
    helpers::unhex,
    keyring::AddressBookKey,
};
use crate::{
    error::{ErrorSource, MetadataError, MetadataSource},
    keyring::MetaKey,
};

/// Network information extracted from the metadata: name, version, optional
/// base58 prefix, warning about extensions incompatible with transaction
/// parsing for RuntimeMetadata with version 14 and above
#[derive(Decode, Encode, PartialEq)]
pub struct MetaInfo {
    /// Network name, from metadata `Version` constant  
    pub name: String,

    /// Network version, from in metadata `Version` constant  
    pub version: u32,

    /// Network base58 prefix, could be encountered in metadata `SS58Prefix`
    /// constant  
    ///
    /// If `SS58Prefix` constant is present in metadata, the prefix derived
    /// from it is expected to match `base58prefix` from `NetworkSpecs`.  
    pub optional_base58prefix: Option<u16>,

    /// Flag to indicate that extrinsic set from metadata with
    /// [`RuntimeMetadataV14`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/v14/struct.RuntimeMetadataV14.html)
    /// is insufficient for transaction decoding  
    pub warn_incomplete_extensions: bool,
}

/// Metadata values: name, version, optional base58 prefix, warning about
/// extensions incompatible with transaction parsing for RuntimeMetadata with
/// version 14 and above, and metadata itself as raw `Vec<u8>`
#[derive(PartialEq, Clone)]
pub struct MetaValues {
    /// Network name, from metadata `Version` constant  
    pub name: String,

    /// Network version, from in metadata `Version` constant  
    pub version: u32,

    /// Network base58 prefix, could be encountered in metadata `SS58Prefix`
    /// constant  
    ///
    /// If `SS58Prefix` constant is present in metadata, the prefix derived
    /// from it is expected to match `base58prefix` from `NetworkSpecs`.  
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
    pub fn from_entry_name_version_checked<T: ErrorSource>(
        name: &str,
        version: u32,
        meta_encoded: IVec,
    ) -> Result<Self, T::Error> {
        let meta_values = match Self::from_slice_metadata(&meta_encoded) {
            Ok(a) => a,
            Err(e) => {
                return Err(<T>::faulty_metadata(
                    e,
                    MetadataSource::Database {
                        name: name.to_string(),
                        version,
                    },
                ))
            }
        };
        if (meta_values.name != name) || (meta_values.version != version) {
            return Err(<T>::metadata_mismatch(
                name.to_string(),
                version,
                meta_values.name.to_string(),
                meta_values.version,
            ));
        }
        Ok(meta_values)
    }

    /// Gets [`MetaValues`] from either cold or hot database tree `METATREE`
    /// (key, value) entry  
    ///
    /// Checks that name and version from [`MetaKey`] match the ones in metadata
    /// `Version` constant.  
    pub fn from_entry_checked<T: ErrorSource>(
        (meta_key_vec, meta_encoded): (IVec, IVec),
    ) -> Result<Self, T::Error> {
        let (name, version) = MetaKey::from_ivec(&meta_key_vec).name_version::<T>()?;
        Self::from_entry_name_version_checked::<T>(&name, version, meta_encoded)
    }

    /// Gets [`MetaValues`] from raw metadata in `Vec<u8>` format
    ///
    /// Produces [`MetadataError`] if the metadata is somehow not suitable for
    /// use in Signer.
    pub fn from_slice_metadata(meta_slice: &[u8]) -> Result<Self, MetadataError> {
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
    pub fn from_wasm_file(filename: &str) -> Result<Self, ErrorActive> {
        let metadata = convert_wasm_into_metadata(filename).map_err(|e| ErrorActive::Wasm {
            filename: filename.to_string(),
            wasm: e,
        })?;
        Self::from_slice_metadata(&metadata).map_err(|e| ErrorActive::Wasm {
            filename: filename.to_string(),
            wasm: Wasm::FaultyMetadata(e),
        })
    }

    /// Gets [`MetaValues`] from raw hexadecimal metadata
    ///
    /// Is used only on Active side, for:
    /// (a) default and test metadata loading;
    /// (b) decoding and evaluating fetched metadata;
    #[cfg(feature = "active")]
    pub fn from_str_metadata(
        meta: &str,
        source: IncomingMetadataSourceActiveStr,
    ) -> Result<Self, ErrorActive> {
        let what = match &source {
            IncomingMetadataSourceActiveStr::Fetch { url } => NotHexActive::FetchedMetadata {
                url: url.to_string(),
            },
            IncomingMetadataSourceActiveStr::Default { filename } => {
                NotHexActive::DefaultMetadata {
                    filename: filename.to_string(),
                }
            }
            IncomingMetadataSourceActiveStr::Check { filename } => NotHexActive::CheckedMetadata {
                filename: filename.to_string(),
            },
        };
        let meta_vec = unhex::<Active>(meta, what)?;
        match Self::from_slice_metadata(&meta_vec) {
            Ok(a) => Ok(a),
            Err(e) => Err(<Active>::faulty_metadata(
                e,
                MetadataSource::Incoming(IncomingMetadataSourceActive::Str(source)),
            )),
        }
    }
}

/// Extracts raw metadata in `Vec<u8>` format from `wasm` file.
///
/// Is used only on Active side, to generate metadata updates before metadata
/// release.
#[cfg(feature = "active")]
pub fn convert_wasm_into_metadata(filename: &str) -> Result<Vec<u8>, Wasm> {
    let buffer = std::fs::read(filename).map_err(Wasm::File)?;
    let runtime_blob = RuntimeBlob::uncompress_if_needed(&buffer).map_err(Wasm::RuntimeBlob)?;
    let wasmi_runtime = create_runtime(
        runtime_blob,
        64,
        SubstrateHostFunctions::host_functions(),
        false,
    )
    .map_err(Wasm::WasmiRuntime)?;
    let mut wasmi_instance = wasmi_runtime.new_instance().map_err(Wasm::WasmiInstance)?;
    let data = wasmi_instance
        .call(InvokeMethod::Export("Metadata_metadata"), &[])
        .map_err(Wasm::Call)?;
    <Vec<u8>>::decode(&mut &data[..]).map_err(|_| Wasm::DecodingMetadata)
}

/// Get [`MetaInfo`] from
/// [`RuntimeMetadata`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/enum.RuntimeMetadata.html)
///
/// Searches `System` pallet within the metadata, gets from it `Version` and
/// optionally `SS58Prefix` constants.
///
/// Produces [`MetaInfo`] if the metadata is suitable for the Signer, and
/// [`MetadataError`] if not.
///
/// `RuntimeMetadata` suitable for use in Signer:  
///
/// - must be of runtime version V12 or above  
/// - must have 'System' pallet  
/// - must have `Version` constant in `System` pallet, SCALE-decodeable  
/// - can have `SS58Prefix` constant in `System` pallet, and if it does, the
/// constant must be SCALE-decodeable  
///
/// Additionally, for [`RuntimeMetadataV14`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/v14/struct.RuntimeMetadataV14.html)
/// the extensions set must be decoding-compatible for any signable transaction.  
pub fn info_from_metadata(runtime_metadata: &RuntimeMetadata) -> Result<MetaInfo, MetadataError> {
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
        _ => return Err(MetadataError::VersionIncompatible),
    }
    if !system_block {
        return Err(MetadataError::NoSystemPallet);
    }
    let runtime_version = match runtime_version_encoded {
        Some(mut x) => match RuntimeVersion::decode(&mut x) {
            Ok(a) => a,
            Err(_) => return Err(MetadataError::RuntimeVersionNotDecodeable),
        },
        None => return Err(MetadataError::NoVersionInConstants),
    };
    let optional_base58prefix = match base58_prefix_encoded {
        Some(mut x) => match <u16>::decode(&mut x) {
            Ok(a) => Some(a),
            Err(_) => match <u8>::decode(&mut x) {
                // in some older metadata u8 is used for base58 prefix, likely a legacy thing
                Ok(a) => Some(a as u16),
                Err(_) => return Err(MetadataError::Base58PrefixNotDecodeable),
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
/// Raw `Vec<u8>` metadata suitable for use in Signer:  
///
/// - must begin with b"meta"  
/// - after that must be SCALE-encoded `RuntimeMetadata` with runtime version V12 or above
pub fn runtime_metadata_from_slice(meta: &[u8]) -> Result<RuntimeMetadata, MetadataError> {
    if !meta.starts_with(&[109, 101, 116, 97]) {
        return Err(MetadataError::NotMeta);
    }
    if meta[4] < 12 {
        return Err(MetadataError::VersionIncompatible);
    }
    match RuntimeMetadata::decode(&mut &meta[4..]) {
        Ok(x) => Ok(x),
        Err(_) => Err(MetadataError::UnableToDecode),
    }
}

/// Checks if the v14 metadata has all signed extensions required for transaction decoding.
/// True if extensions are incomplete.
///
/// Currently, the decoding of the transaction demands that metadata version, network genesis hash,
/// and era are among signed extensions. Otherwise, a ParserMetadataError would occur on decoding.
/// However, we can not simply forbid the loading of the metadata without required set of
/// signed extensions into Signer.
///
/// This function should be used for warnings only on `generate_message` side and during metadata
/// loading into Signer.
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
#[cfg(feature = "signer")]
pub struct MetaSetElement {
    /// Network name, from metadata `Version` constant
    name: String,

    /// Network version, from in metadata `Version` constant  
    version: u32,

    /// Network base58 prefix, could be encountered in metadata `SS58Prefix`
    /// constant  
    ///
    /// If `SS58Prefix` constant is present in metadata, the prefix derived
    /// from it is expected to match `base58prefix` from `NetworkSpecs`.  
    optional_base58prefix: Option<u16>,

    /// [`RuntimeMetadata`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/enum.RuntimeMetadata.html)
    /// [`MetaSetElement`] is successfully generated only if metadata is a
    /// suitable one
    runtime_metadata: RuntimeMetadata,
}

#[cfg(feature = "signer")]
impl MetaSetElement {
    /// Generates `MetaSetElement` from Signer database tree `METATREE` (key, value)
    /// entry  
    ///
    /// Checks that name and version from [`MetaKey`] match the ones in metadata
    /// `Version` constant.  
    ///
    /// Also checks that the metadata is suitable for use in Signer. Since the
    /// metadata already was accepted in the database at some point, errors here
    /// are very unlikely to happen and would indicate the database corruption
    pub fn from_entry((meta_key_vec, meta_encoded): (IVec, IVec)) -> Result<Self, ErrorSigner> {
        let (network_name, network_version) =
            MetaKey::from_ivec(&meta_key_vec).name_version::<Signer>()?;
        let runtime_metadata = match runtime_metadata_from_slice(&meta_encoded) {
            Ok(a) => a,
            Err(e) => {
                return Err(<Signer>::faulty_metadata(
                    e,
                    MetadataSource::Database {
                        name: network_name,
                        version: network_version,
                    },
                ))
            }
        };
        let (name, version, optional_base58prefix) = match info_from_metadata(&runtime_metadata) {
            Ok(a) => {
                if (a.version != network_version) || (a.name != network_name) {
                    return Err(<Signer>::metadata_mismatch(
                        network_name,
                        network_version,
                        a.name.to_string(),
                        a.version,
                    ));
                }
                (a.name, a.version, a.optional_base58prefix)
            }
            Err(e) => {
                return Err(<Signer>::faulty_metadata(
                    e,
                    MetadataSource::Database {
                        name: network_name,
                        version: network_version,
                    },
                ))
            }
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

/// Network information needed for rpc calls in the network and for managing the
/// hot database  
///
/// Hot database contains tree `ADDRESS_BOOK` with information needed to perform
/// rpc calls in networks and generate `load_metadata` and `add_specs` payloads.
///
/// `ADDRESS_BOOK` tree stores SCALE-encoded [`AddressBookEntry`] entries under
/// keys [`AddressBookKey`]
#[derive(Decode, Encode, PartialEq)]
#[cfg(feature = "active")]
pub struct AddressBookEntry {
    /// Network name, as it appears in `Version` constant in metadata  
    ///
    /// If network data is queired through rpc call, retrieved metadata must
    /// have exactly same network name in `Version` constant  
    pub name: String,

    /// Network genesis hash  
    ///
    /// If network data is queried through rpc call, retrieved version must
    /// be same as the one in address book  
    pub genesis_hash: [u8; 32],

    /// Url address for rpc calls, with or without preferred port  
    pub address: String,

    /// [`Encryption`] that is supported by the network  
    pub encryption: Encryption,

    /// Address book entry is the default one  
    ///
    /// Default networks currently are Polkadot, Kusama, Westend with Sr25519
    /// encryption  
    pub def: bool,
}

#[cfg(feature = "active")]
impl AddressBookEntry {
    /// Gets [`AddressBookEntry`] from from hot database tree `ADDRESS_BOOK`
    /// (key, value) entry.  
    pub fn from_entry(
        (address_book_key_encoded, address_book_entry_encoded): (IVec, IVec),
    ) -> Result<AddressBookEntry, ErrorActive> {
        let title = AddressBookKey::from_ivec(&address_book_key_encoded).title()?;
        AddressBookEntry::from_entry_with_title(&title, &address_book_entry_encoded)
    }

    /// Gets network address book title and [`AddressBookEntry`] as a tuple from
    /// from hot database tree `ADDRESS_BOOK` (key, value) entry.  
    ///
    /// Network address book title **differs** from `title` in network specs.
    /// This is just a key in hot database `ADDRESS_BOOK`, and is not displayed
    /// anywhere else.  
    pub fn process_entry(
        (address_book_key_encoded, address_book_entry_encoded): (IVec, IVec),
    ) -> Result<(String, AddressBookEntry), ErrorActive> {
        let title = AddressBookKey::from_ivec(&address_book_key_encoded).title()?;
        let address_book_entry =
            AddressBookEntry::from_entry_with_title(&title, &address_book_entry_encoded)?;
        Ok((title, address_book_entry))
    }

    /// Gets [`AddressBookEntry`] from network address book title and associated
    /// value from hot database tree `ADDRESS_BOOK`.  
    pub fn from_entry_with_title(
        title: &str,
        address_book_entry_encoded: &IVec,
    ) -> Result<AddressBookEntry, ErrorActive> {
        match <AddressBookEntry>::decode(&mut &address_book_entry_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => Err(ErrorActive::Database(DatabaseActive::EntryDecoding(
                EntryDecodingActive::AddressBookEntry {
                    title: title.to_string(),
                },
            ))),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "test")]
mod tests {
    use super::*;
    use crate::error_active::DefaultLoading;
    use std::fs::read_to_string;

    #[test]
    fn westend9070() {
        let filename = String::from("for_tests/westend9070");
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(
            meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
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
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(
            meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
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
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(
            meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
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
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(
            meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
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
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(
            meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
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
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(
            meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
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
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(
            meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
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
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(
            meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
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
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(
            meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
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
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(
            meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
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
        let meta = read_to_string(&filename).unwrap();
        let expected_error = <Active>::show(&ErrorActive::DefaultLoading(
            DefaultLoading::FaultyMetadata {
                filename: filename.to_string(),
                error: MetadataError::NoVersionInConstants,
            },
        ));
        match MetaValues::from_str_metadata(
            meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        ) {
            Ok(x) => panic!("Unexpectedly decoded as {} version {}", x.name, x.version),
            Err(e) => {
                assert!(
                    <Active>::show(&e) == expected_error,
                    "Unexpected kind of error, {}",
                    <Active>::show(&e)
                );
            }
        }
    }

    #[test]
    fn centrifuge_amber() {
        let filename = String::from("for_tests/centrifugeAmber");
        let meta = read_to_string(&filename).unwrap();
        let expected_error = <Active>::show(&ErrorActive::DefaultLoading(
            DefaultLoading::FaultyMetadata {
                filename: filename.to_string(),
                error: MetadataError::VersionIncompatible,
            },
        ));
        match MetaValues::from_str_metadata(
            meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        ) {
            Ok(x) => panic!("Unexpectedly decoded as {} version {}", x.name, x.version),
            Err(e) => {
                assert!(
                    <Active>::show(&e) == expected_error,
                    "Unexpected kind of error, {}",
                    <Active>::show(&e)
                );
            }
        }
    }

    #[test]
    fn westend9150() {
        let filename = String::from("for_tests/westend9150");
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(
            meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
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
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(
            meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
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
