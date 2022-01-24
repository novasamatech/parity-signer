use parity_scale_codec::{Decode, Encode};
use parity_scale_codec_derive;
use frame_metadata::{RuntimeMetadata, decode_different::DecodeDifferent};
use sled::IVec;
use sp_version::RuntimeVersion;

use crate::crypto::Encryption;
use crate::error::{Active, DatabaseActive, EntryDecodingActive, ErrorActive, ErrorSigner, ErrorSource, IncomingMetadataSourceActive, IncomingMetadataSourceActiveStr, MetadataError, MetadataSource, NotHexActive, Signer};
use crate::helpers::unhex;
use crate::keyring::{AddressBookKey, MetaKey};

/// Struct used to store the network metadata name and version in the database
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode, PartialEq)]
pub struct NameVersioned {
    pub name: String,
    pub version: u32,
}

/// Struct to store the metadata values (network name, network
/// version, full metadata as Vec<u8>)
#[derive(PartialEq, Clone)]
pub struct MetaValues {
    pub name: String,
    pub version: u32,
    pub meta: Vec<u8>,
}

impl MetaValues {
    pub fn from_entry_name_version_checked<T: ErrorSource>(name: &str, version: u32, meta_encoded: IVec) -> Result<Self, T::Error> {
        let meta = meta_encoded.to_vec();
        check_metadata::<T>(&meta, &name, version, MetadataSource::Database{name: name.to_string(), version})?;
        Ok(Self{
            name: name.to_string(),
            version,
            meta,
        })
    }
    pub fn from_entry_checked<T: ErrorSource>((meta_key_vec, meta_encoded): (IVec, IVec)) -> Result<Self, T::Error> {
        let (name, version) = MetaKey::from_ivec(&meta_key_vec).name_version::<T>()?;
        Self::from_entry_name_version_checked::<T>(&name, version, meta_encoded)
    }
    pub fn from_vec_metadata(meta_vec: &Vec<u8>) -> Result<Self, MetadataError> {
        let name_versioned = name_versioned_from_metadata(&runtime_metadata_from_vec(meta_vec)?)?;
        Ok(Self{
            name: name_versioned.name.to_string(),
            version: name_versioned.version,
            meta: meta_vec.to_vec(),
        })
    }
    pub fn from_runtime_metadata(runtime_metadata: &RuntimeMetadata, source: IncomingMetadataSourceActive) -> Result<Self, ErrorActive> {
        let name_versioned = match name_versioned_from_metadata(runtime_metadata) {
            Ok(a) => a,
            Err(e) => return Err(<Active>::faulty_metadata(e, MetadataSource::Incoming(source))),
        };
        Ok(Self{
            name: name_versioned.name.to_string(),
            version: name_versioned.version,
            meta: [vec![109, 101, 116, 97], runtime_metadata.encode()].concat(),
        })
    }
    /// Function to get MetaValues from metadata in format of hex string.
    /// Is used only on Active side, for:
    /// (a) default and test metadata loading;
    /// (b) decoding and evaluating fetched metadata;
    pub fn from_str_metadata (meta: &str, source: IncomingMetadataSourceActiveStr) -> Result<Self, ErrorActive> {
        let what = match &source {
            IncomingMetadataSourceActiveStr::Fetch{url} => NotHexActive::FetchedMetadata{url: url.to_string()},
            IncomingMetadataSourceActiveStr::Default{filename} => NotHexActive::DefaultMetadata{filename: filename.to_string()},
        };
        let meta_vec = unhex::<Active>(meta, what)?;
        match Self::from_vec_metadata(&meta_vec) {
            Ok(a) => Ok(a),
            Err(e) => return Err(<Active>::faulty_metadata(e, MetadataSource::Incoming(IncomingMetadataSourceActive::Str(source))))
        }
    }
}

/// Function to search metadata as RuntimeMetadata for system block,
/// decode RuntimeVersion constant,
/// output NameVersioned
pub fn name_versioned_from_metadata (runtime_metadata: &RuntimeMetadata) -> Result<NameVersioned, MetadataError> {
    let mut runtime_version_encoded: Option<Vec<u8>> = None;
    let mut system_block = false;
    let mut constants_version = false;
    match runtime_metadata {
        RuntimeMetadata::V12(metadata_v12) => {
            if let DecodeDifferent::Decoded(meta_vector) = &metadata_v12.modules {
                for x in meta_vector.iter() {
                    if x.name==DecodeDifferent::Encode("System") {
                        system_block = true;
                        if let DecodeDifferent::Decoded(constants_vector) = &x.constants {
                            for y in constants_vector.iter() {
                                if y.name==DecodeDifferent::Encode("Version") {
                                    constants_version = true;
                                    if let DecodeDifferent::Decoded(fin) = &y.value {runtime_version_encoded = Some(fin.to_vec());}
                                    break;
                                }
                            }
                        }
                        break;
                    }
                }
            }
        },
        RuntimeMetadata::V13(metadata_v13) => {
            if let DecodeDifferent::Decoded(meta_vector) = &metadata_v13.modules {
                for x in meta_vector.iter() {
                    if x.name==DecodeDifferent::Encode("System") {
                        system_block = true;
                        if let DecodeDifferent::Decoded(constants_vector) = &x.constants {
                            for y in constants_vector.iter() {
                                if y.name==DecodeDifferent::Encode("Version") {
                                    constants_version = true;
                                    if let DecodeDifferent::Decoded(fin) = &y.value {runtime_version_encoded = Some(fin.to_vec());}
                                    break;
                                }
                            }
                        }
                        break;
                    }
                }
            }
        },
        RuntimeMetadata::V14(metadata_v14) => {
            for x in metadata_v14.pallets.iter() {
                if x.name == "System" {
                    system_block = true;
                    for y in x.constants.iter() {
                        if y.name == "Version" {
                            constants_version = true;
                            runtime_version_encoded = Some(y.value.to_vec());
                            break;
                        }
                    }
                break;
                }
            }
        },
        _ => return Err(MetadataError::VersionIncompatible),
    }
    if !system_block {return Err(MetadataError::NoSystemPallet)}
    if !constants_version {return Err(MetadataError::NoVersionInConstants)}
    let runtime_version_encoded = match runtime_version_encoded {
        Some(x) => x,
        None => return Err(MetadataError::NoVersionInConstants),
    };
    let runtime_version = match RuntimeVersion::decode(&mut &runtime_version_encoded[..]) {
        Ok(a) => a,
        Err(_) => return Err(MetadataError::RuntimeVersionNotDecodeable),
    };
    Ok(NameVersioned {
        name: runtime_version.spec_name.to_string(),
        version: runtime_version.spec_version,
    })
}

/// Function to check the integrity of the metadata versus given network name and version
pub fn check_metadata<T: ErrorSource>(meta: &Vec<u8>, network_name: &str, network_version: u32, source: MetadataSource<T>) -> Result<(), T::Error> {
    let meta_values = match MetaValues::from_vec_metadata(meta) {
        Ok(a) => a,
        Err(e) => return Err(<T>::faulty_metadata(e, source))
    };
    if (meta_values.name != network_name)||(meta_values.version != network_version) {return Err(<T>::metadata_mismatch(network_name.to_string(), network_version, meta_values.name.to_string(), meta_values.version))}
    Ok(())
}

pub fn runtime_metadata_from_vec(meta_vec: &Vec<u8>) -> Result<RuntimeMetadata, MetadataError> {
    if !meta_vec.starts_with(&vec![109, 101, 116, 97]) {return Err(MetadataError::NotMeta)}
    if meta_vec[4] < 12 {return Err(MetadataError::VersionIncompatible)}
    match RuntimeMetadata::decode(&mut &meta_vec[4..]) {
        Ok(x) => Ok(x),
        Err(_) => return Err(MetadataError::UnableToDecode),
    }
}

/// Struct for processing 
pub struct MetaSetElement {
    pub name: String,
    pub version: u32,
    pub runtime_metadata: RuntimeMetadata,
}

impl MetaSetElement {
    pub fn from_entry ((meta_key_vec, meta_encoded): (IVec, IVec)) -> Result<Self, ErrorSigner> {
        let (network_name, network_version) = MetaKey::from_ivec(&meta_key_vec).name_version::<Signer>()?;
        let runtime_metadata = match runtime_metadata_from_vec(&meta_encoded.to_vec()) {
            Ok(a) => a,
            Err(e) => return Err(<Signer>::faulty_metadata(e, MetadataSource::Database{name: network_name, version: network_version})),
        };
        let (name, version) = match name_versioned_from_metadata(&runtime_metadata) {
            Ok(a) => {
                if (a.version != network_version)||(a.name != network_name) {return Err(<Signer>::metadata_mismatch(network_name.to_string(), network_version, a.name.to_string(), a.version))}
                (a.name, a.version)
            },
            Err(e) => return Err(<Signer>::faulty_metadata(e, MetadataSource::Database{name: network_name, version: network_version})),
        };
        Ok(Self{
            name,
            version,
            runtime_metadata,
        })
    }
}

/// Struct to store network information needed for metadata and network specs fetching
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub struct AddressBookEntry {
    pub name: String,
    pub genesis_hash: [u8; 32],
    pub address: String,
    pub encryption: Encryption,
    pub def: bool,
}

impl AddressBookEntry {
    pub fn from_entry((address_book_key_encoded, address_book_entry_encoded): (IVec, IVec)) -> Result<AddressBookEntry, ErrorActive> {
        let title = AddressBookKey::from_ivec(&address_book_key_encoded).title()?;
        AddressBookEntry::from_entry_with_title(&title, &address_book_entry_encoded)
    }
    pub fn process_entry((address_book_key_encoded, address_book_entry_encoded): (IVec, IVec)) -> Result<(String, AddressBookEntry), ErrorActive> {
        let title = AddressBookKey::from_ivec(&address_book_key_encoded).title()?;
        let address_book_entry = AddressBookEntry::from_entry_with_title(&title, &address_book_entry_encoded)?;
        Ok((title, address_book_entry))
    }
    pub fn from_entry_with_title(title: &str, address_book_entry_encoded: &IVec) -> Result<AddressBookEntry, ErrorActive> {
        match <AddressBookEntry>::decode(&mut &address_book_entry_encoded[..]) {
            Ok(a) => Ok(a),
            Err(_) => return Err(ErrorActive::Database(DatabaseActive::EntryDecoding(EntryDecodingActive::AddressBookEntryTitle{title: title.to_string()}))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::DefaultLoading;
    use std::fs::read_to_string;
    
    #[test]
    fn westend9070() {
        let filename = String::from("for_tests/westend9070");
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(&meta.trim(), IncomingMetadataSourceActiveStr::Default{filename}).unwrap();
        assert!(meta_values.name == String::from("westend"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 9070, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn westend9033() {
        let filename = String::from("for_tests/westend9033");
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(&meta.trim(), IncomingMetadataSourceActiveStr::Default{filename}).unwrap();
        assert!(meta_values.name == String::from("westend"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 9033, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn westend9030() {
        let filename = String::from("for_tests/westend9030");
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(&meta.trim(), IncomingMetadataSourceActiveStr::Default{filename}).unwrap();
        assert!(meta_values.name == String::from("westend"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 9030, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn rococo9004() {
        let filename = String::from("for_tests/rococo9004");
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(&meta.trim(), IncomingMetadataSourceActiveStr::Default{filename}).unwrap();
        assert!(meta_values.name == String::from("rococo"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 9004, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn rococo9002() {
        let filename = String::from("for_tests/rococo9002");
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(&meta.trim(), IncomingMetadataSourceActiveStr::Default{filename}).unwrap();
        assert!(meta_values.name == String::from("rococo"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 9002, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn polkadot9080() {
        let filename = String::from("for_tests/polkadot9080");
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(&meta.trim(), IncomingMetadataSourceActiveStr::Default{filename}).unwrap();
        assert!(meta_values.name == String::from("polkadot"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 9080, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn polkadot30() {
        let filename = String::from("for_tests/polkadot30");
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(&meta.trim(), IncomingMetadataSourceActiveStr::Default{filename}).unwrap();
        assert!(meta_values.name == String::from("polkadot"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 30, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn polkadot29() {
        let filename = String::from("for_tests/polkadot29");
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(&meta.trim(), IncomingMetadataSourceActiveStr::Default{filename}).unwrap();
        assert!(meta_values.name == String::from("polkadot"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 29, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn kusama9040() {
        let filename = String::from("for_tests/kusama9040");
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(&meta.trim(), IncomingMetadataSourceActiveStr::Default{filename}).unwrap();
        assert!(meta_values.name == String::from("kusama"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 9040, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn kusama9010() {
        let filename = String::from("for_tests/kusama9010");
        let meta = read_to_string(&filename).unwrap();
        let meta_values = MetaValues::from_str_metadata(&meta.trim(), IncomingMetadataSourceActiveStr::Default{filename}).unwrap();
        assert!(meta_values.name == String::from("kusama"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 9010, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn edgeware() {
        let filename = String::from("for_tests/edgeware");
        let meta = read_to_string(&filename).unwrap();
        let expected_error = <Active>::show(&ErrorActive::DefaultLoading(DefaultLoading::FaultyMetadata{filename: filename.to_string(), error: MetadataError::NoVersionInConstants}));
        match MetaValues::from_str_metadata(&meta.trim(), IncomingMetadataSourceActiveStr::Default{filename}) {
            Ok(x) => panic!("Unexpectedly decoded as {} version {}", x.name, x.version),
            Err(e) => {
                assert!(<Active>::show(&e) == expected_error, "Unexpected kind of error, {}", <Active>::show(&e));
            }
        }
    }
    
    #[test]
    fn centrifuge_amber() {
        let filename = String::from("for_tests/centrifugeAmber");
        let meta = read_to_string(&filename).unwrap();
        let expected_error = <Active>::show(&ErrorActive::DefaultLoading(DefaultLoading::FaultyMetadata{filename: filename.to_string(), error: MetadataError::VersionIncompatible}));
        match MetaValues::from_str_metadata(&meta.trim(), IncomingMetadataSourceActiveStr::Default{filename}) {
            Ok(x) => panic!("Unexpectedly decoded as {} version {}", x.name, x.version),
            Err(e) => {
                assert!(<Active>::show(&e) == expected_error, "Unexpected kind of error, {}", <Active>::show(&e));
            }
        }
    }
}

