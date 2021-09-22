use hex;
use parity_scale_codec::Decode;
use frame_metadata::{RuntimeMetadataV12, DecodeDifferent};
use definitions::metadata::{MetaValues, VersionDecoded};


/// Function to search metadata as RuntimeMetadataV12 for system block,
/// and then find version entry within it

pub fn get_meta_const_light (meta_back: &RuntimeMetadataV12) -> Result<Vec<u8>, &'static str> {
    let mut out = Vec::new();
    let mut system_block = false;
    let mut constants_version = false;
    
    if let DecodeDifferent::Decoded(meta_vector) = &meta_back.modules {
        for x in meta_vector.iter() {
            if x.name==DecodeDifferent::Encode("System") {
                system_block = true;
                if let DecodeDifferent::Decoded(constants_vector) = &x.constants {
                    for y in constants_vector.iter() {
                        if y.name==DecodeDifferent::Encode("Version") {
                            constants_version = true;
                            if let DecodeDifferent::Decoded(fin) = &y.value {out = fin.to_vec();}
                            break;
                        }
                    }
                }
                break;
            }
        }
    }
    if !system_block {
        return Err("No system block found");
    }
    if !constants_version {
        return Err("No version found in constants");
    }
    if out.len()==0 {
        return Err("No version retrieved from constants");
    }
    Ok(out)
}


/// Function to decode metadata hex string into RuntimeMetadataV12,
/// then search for system block in it, and find version entry within it

pub fn get_meta_const (meta_unhex: &Vec<u8>) -> Result<Vec<u8>, &'static str> {

    if !meta_unhex.starts_with(&vec![109, 101, 116, 97]) {return Err("No 'meta' starting sequence in metadata")}
    if meta_unhex[4] < 12 {return Err("RuntimeMetadata version incompatible");}
    
    let meta_back = match RuntimeMetadataV12::decode(&mut &meta_unhex[5..]) {
        Ok(x) => x,
        Err(_) => return Err("Unable to decode metadata into V12"),
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


