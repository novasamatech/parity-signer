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

/// Struct for the network information extracted from the metadata:
/// name, version, optional base58 prefix
#[derive(Decode, Encode, PartialEq)]
pub struct MetaInfo {
    pub name: String,
    pub version: u32,
    pub optional_base58prefix: Option<u16>,
    pub warn_incomplete_extensions: bool,
}

/// Struct to store the metadata values (network name, network
/// version, optional base58 prefix from metadata, full metadata as Vec<u8>)
#[derive(PartialEq, Clone)]
pub struct MetaValues {
    pub name: String,
    pub version: u32,
    pub optional_base58prefix: Option<u16>,
    pub warn_incomplete_extensions: bool,
    pub meta: Vec<u8>,
}

impl MetaValues {
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
    pub fn from_entry_checked<T: ErrorSource>(
        (meta_key_vec, meta_encoded): (IVec, IVec),
    ) -> Result<Self, T::Error> {
        let (name, version) = MetaKey::from_ivec(&meta_key_vec).name_version::<T>()?;
        Self::from_entry_name_version_checked::<T>(&name, version, meta_encoded)
    }
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
    /// Function to get MetaValues from metadata in format of hex string.
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

/// Function to search metadata as RuntimeMetadata for system block,
/// decode RuntimeVersion constant,
/// output MetaInfo
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

pub fn runtime_metadata_from_slice(meta_vec: &[u8]) -> Result<RuntimeMetadata, MetadataError> {
    if !meta_vec.starts_with(&[109, 101, 116, 97]) {
        return Err(MetadataError::NotMeta);
    }
    if meta_vec[4] < 12 {
        return Err(MetadataError::VersionIncompatible);
    }
    match RuntimeMetadata::decode(&mut &meta_vec[4..]) {
        Ok(x) => Ok(x),
        Err(_) => Err(MetadataError::UnableToDecode),
    }
}

/// Function to check if the v14 metadata has all signed extensions required for transaction decoding.
/// True if extensions are incomplete.
/// Currently, the decoding of the transaction demands that metadata version, network genesis hash,
/// and era are among signed extensions. Otherwise, a ParserMetadataError would occur on decoding.
/// However, we can not simply forbid the loading of the metadata without required set of
/// signed extensions into Signer.
/// This function should be used for warnings only on generate_message side and during metadata
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

/// Struct to keep metadata and its info for transaction decoding
#[cfg(feature = "signer")]
pub struct MetaSetElement {
    pub name: String,
    pub version: u32,
    pub optional_base58prefix: Option<u16>,
    pub runtime_metadata: RuntimeMetadata,
}

#[cfg(feature = "signer")]
impl MetaSetElement {
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
}

/// Struct to store network information needed for metadata and network specs fetching
#[derive(Decode, Encode, PartialEq)]
#[cfg(feature = "active")]
pub struct AddressBookEntry {
    pub name: String,
    pub genesis_hash: [u8; 32],
    pub address: String,
    pub encryption: Encryption,
    pub def: bool,
}

#[cfg(feature = "active")]
impl AddressBookEntry {
    pub fn from_entry(
        (address_book_key_encoded, address_book_entry_encoded): (IVec, IVec),
    ) -> Result<AddressBookEntry, ErrorActive> {
        let title = AddressBookKey::from_ivec(&address_book_key_encoded).title()?;
        AddressBookEntry::from_entry_with_title(&title, &address_book_entry_encoded)
    }
    pub fn process_entry(
        (address_book_key_encoded, address_book_entry_encoded): (IVec, IVec),
    ) -> Result<(String, AddressBookEntry), ErrorActive> {
        let title = AddressBookKey::from_ivec(&address_book_key_encoded).title()?;
        let address_book_entry =
            AddressBookEntry::from_entry_with_title(&title, &address_book_entry_encoded)?;
        Ok((title, address_book_entry))
    }
    pub fn from_entry_with_title(
        title: &str,
        address_book_entry_encoded: &IVec,
    ) -> Result<AddressBookEntry, ErrorActive> {
        match <AddressBookEntry>::decode(&mut &address_book_entry_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => Err(ErrorActive::Database(DatabaseActive::EntryDecoding(
                EntryDecodingActive::AddressBookEntryTitle {
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
            &meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
        assert!(
            meta_values.name == String::from("westend"),
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
            &meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
        assert!(
            meta_values.name == String::from("westend"),
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
            &meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
        assert!(
            meta_values.name == String::from("westend"),
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
            &meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
        assert!(
            meta_values.name == String::from("rococo"),
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
            &meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
        assert!(
            meta_values.name == String::from("rococo"),
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
            &meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
        assert!(
            meta_values.name == String::from("polkadot"),
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
            &meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
        assert!(
            meta_values.name == String::from("polkadot"),
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
            &meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
        assert!(
            meta_values.name == String::from("polkadot"),
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
            &meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
        assert!(
            meta_values.name == String::from("kusama"),
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
            &meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
        assert!(
            meta_values.name == String::from("kusama"),
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
            &meta.trim(),
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
            &meta.trim(),
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
            &meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
        assert!(
            meta_values.name == String::from("westend"),
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
            &meta.trim(),
            IncomingMetadataSourceActiveStr::Default { filename },
        )
        .unwrap();
        assert!(
            meta_values.name == String::from("shell"),
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
            meta_values.name == String::from("westend"),
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
