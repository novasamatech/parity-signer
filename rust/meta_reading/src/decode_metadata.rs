use hex;
use parity_scale_codec::Decode;
use frame_metadata::{RuntimeMetadata, decode_different::DecodeDifferent};
use definitions::metadata::{MetaValues, VersionDecoded};


/// Function to search metadata as RuntimeMetadataV12 for system block,
/// and then find version entry within it

pub fn get_meta_const_light (meta_back: &RuntimeMetadata) -> Result<Vec<u8>, &'static str> {
    let mut out: Option<Vec<u8>> = None;
    let mut system_block = false;
    let mut constants_version = false;
    
    match meta_back {
        RuntimeMetadata::V12(metadata_v12) => {
            if let DecodeDifferent::Decoded(meta_vector) = &metadata_v12.modules {
                for x in meta_vector.iter() {
                    if x.name==DecodeDifferent::Encode("System") {
                        system_block = true;
                        if let DecodeDifferent::Decoded(constants_vector) = &x.constants {
                            for y in constants_vector.iter() {
                                if y.name==DecodeDifferent::Encode("Version") {
                                    constants_version = true;
                                    if let DecodeDifferent::Decoded(fin) = &y.value {out = Some(fin.to_vec());}
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
                                    if let DecodeDifferent::Decoded(fin) = &y.value {out = Some(fin.to_vec());}
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
                            out = Some(y.value.to_vec());
                            break;
                        }
                    }
                break;
                }
            }
        },
        _ => return Err("RuntimeMetadata version incompatible"),
    }
    
    if !system_block {
        return Err("No system block found");
    }
    if !constants_version {
        return Err("No version found in constants");
    }
    match out {
        Some(x) => Ok(x),
        None => return Err("No version retrieved from constants"),
    }
}


/// Function to decode metadata hex string into RuntimeMetadataV12,
/// then search for system block in it, and find version entry within it

pub fn get_meta_const (meta_unhex: &Vec<u8>) -> Result<Vec<u8>, &'static str> {

    if !meta_unhex.starts_with(&vec![109, 101, 116, 97]) {return Err("No 'meta' starting sequence in metadata")}
    if meta_unhex[4] < 12 {return Err("RuntimeMetadata version incompatible");}
    
    let meta_back = match RuntimeMetadata::decode(&mut &meta_unhex[4..]) {
        Ok(x) => x,
        Err(_) => return Err("Unable to decode runtime metadata"),
    };
    
    get_meta_const_light(&meta_back)
    
}


/// Function takes version metadata vector (such as one output by
/// get_meta_const) and derives chain name and version packaged
/// in struct

pub fn decode_version (meta: &str) -> Result<MetaValues, Box<dyn std::error::Error>> {
    
    let meta_hex = match &meta[..2] {
        "0x" => &meta[2..],
        _ => &meta[..],
    };
    
    let meta_unhex = hex::decode(meta_hex)?;
    
    let version_vector = get_meta_const(&meta_unhex)?;
    
    let version = VersionDecoded::decode(&mut &version_vector[..])?;
    
    Ok(MetaValues {
        name: version.specname.to_string(),
        version: version.spec_version,
        meta: meta_unhex,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    
    #[test]
    fn westend9070() {
        let meta = read_to_string("for_tests/westend9070").unwrap();
        let meta_values = decode_version(&meta.trim()).unwrap();
        assert!(meta_values.name == String::from("westend"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 9070, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn westend9033() {
        let meta = read_to_string("for_tests/westend9033").unwrap();
        let meta_values = decode_version(&meta.trim()).unwrap();
        assert!(meta_values.name == String::from("westend"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 9033, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn westend9030() {
        let meta = read_to_string("for_tests/westend9030").unwrap();
        let meta_values = decode_version(&meta.trim()).unwrap();
        assert!(meta_values.name == String::from("westend"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 9030, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn rococo9004() {
        let meta = read_to_string("for_tests/rococo9004").unwrap();
        let meta_values = decode_version(&meta.trim()).unwrap();
        assert!(meta_values.name == String::from("rococo"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 9004, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn rococo9002() {
        let meta = read_to_string("for_tests/rococo9002").unwrap();
        let meta_values = decode_version(&meta.trim()).unwrap();
        assert!(meta_values.name == String::from("rococo"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 9002, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn polkadot9080() {
        let meta = read_to_string("for_tests/polkadot9080").unwrap();
        let meta_values = decode_version(&meta.trim()).unwrap();
        assert!(meta_values.name == String::from("polkadot"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 9080, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn polkadot30() {
        let meta = read_to_string("for_tests/polkadot30").unwrap();
        let meta_values = decode_version(&meta.trim()).unwrap();
        assert!(meta_values.name == String::from("polkadot"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 30, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn polkadot29() {
        let meta = read_to_string("for_tests/polkadot29").unwrap();
        let meta_values = decode_version(&meta.trim()).unwrap();
        assert!(meta_values.name == String::from("polkadot"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 29, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn kusama9040() {
        let meta = read_to_string("for_tests/kusama9040").unwrap();
        let meta_values = decode_version(&meta.trim()).unwrap();
        assert!(meta_values.name == String::from("kusama"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 9040, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn kusama9010() {
        let meta = read_to_string("for_tests/kusama9010").unwrap();
        let meta_values = decode_version(&meta.trim()).unwrap();
        assert!(meta_values.name == String::from("kusama"), "Unexpected network name: {}", meta_values.name);
        assert!(meta_values.version == 9010, "Unexpected network name: {}", meta_values.version);
    }
    
    #[test]
    fn edgeware() {
        let meta = read_to_string("for_tests/edgeware").unwrap();
        let expected_error = String::from("No version found in constants");
        match decode_version(&meta.trim()) {
            Ok(x) => panic!("Unexpectedly decoded as {} version {}", x.name, x.version),
            Err(e) => {
                assert!(e.to_string() == expected_error, "Unexpected kind of error, {}", e);
            }
        }
    }
    
    #[test]
    fn centrifuge_amber() {
        let meta = read_to_string("for_tests/centrifugeAmber").unwrap();
        let expected_error = String::from("RuntimeMetadata version incompatible");
        match decode_version(&meta.trim()) {
            Ok(x) => panic!("Unexpectedly decoded as {} version {}", x.name, x.version),
            Err(e) => {
                assert!(e.to_string() == expected_error, "Unexpected kind of error, {}", e);
            }
        }
    }
    
}
