use hex;
use codec::{Encode, Decode};
use frame_metadata::{RuntimeMetadataV12, DecodeDifferent};
use serde_json;
use blake2_rfc::blake2b::blake2b;

/// struct to decode the version metadata

#[derive(Debug, Encode, Decode)]
pub struct VersionDecoded {
    pub specname: String,
    implname: String,
    auth_version: u32,
    pub spec_version: u32,
    impl_version: u32,
    apis: Vec<(u8, u32)>,
    trans_version: u32,
}

/// function takes full metadata in format '0x******', decodes
/// using RuntimeMetadataV12, finds version in constants block
/// within system module, and outputs version as a decodeable 
/// vector; some checking done along the way

pub fn get_meta_const (meta: &str) -> Result<Vec<u8>, &str> {

    if !meta.starts_with("0x6d657461") {
        return Err("No 'meta' starting sequence in metadata");
    }
    
    let part1 = &meta[10..12];
    let part1_vec = hex::decode(part1).unwrap();
    let part1_decoded = u8::decode(&mut &part1_vec[..]).unwrap();

    //TODO: V12 and V13 are effectively identical currently; watch this place closely
    if (part1_decoded != 12) && (part1_decoded != 13) {
        return Err("RuntimeMetadata version incompatible");
    }
    
    let meta_str = &meta[12..];
    let meta_work = hex::decode(meta_str).unwrap();
    let meta_back = RuntimeMetadataV12::decode(&mut &meta_work[..]).unwrap();
    
    let mut out = Vec::new();
    let mut system_block = false;
    let mut constants_version = false;
    
    if let DecodeDifferent::Decoded(meta_vector) = meta_back.modules {
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

/// function takes version metadata vector Vec<u8> (such as one output by
/// get_meta_const) and derives chain name and version packaged
/// in struct

pub fn decode_version (version_meta: Vec<u8>) -> VersionDecoded {
    let out = VersionDecoded::decode(&mut &version_meta[..]).unwrap();
    out
}

/// function to calculate hash from metadata line

pub fn hash_from_meta (meta: &str) -> Option<String> {
    let hash = {
        if meta.starts_with("0x") {
            match hex::decode(&meta[2..]) {
                Ok(m) => Some(hex::encode(blake2b(32, &[], &m).as_bytes())),
                Err(_) => None,
            }
        }
        else {None}
    };
    hash
}

/// function to take metadata line as &str and produce json line containing
/// name, version, and hash;
/// calculates hash only if the input meta is valid (starts with 0x
/// and is legit hexadecimal), otherwise exports null instead of hash


pub fn meta_to_json (meta: &str) -> String {
    let hash = hash_from_meta(meta);
    let data = match get_meta_const(meta) {
        Ok(version_vector) => {
            let decoded = decode_version(version_vector);
            let name = decoded.specname;
            let version = decoded.spec_version;
            vec![Some(name), Some(version.to_string()), hash]
        },
        Err(_) => vec![None, None, hash],
    };
    if let Ok(out) = serde_json::to_string(&data) {
        out
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_json() {
        let a =  "0x20706f6c6b61646f743c7061726974792d706f6c6b61646f74000000001d0000000000000030df6acb689907609b0300000037e397fc7c91f5e40100000040fe3ad401f8959a04000000d2bc9897eed08f1502000000f78b278be53f454c02000000af2c0297a23e6d3d01000000ed99c5acb25eedf502000000cbca25e39f14238702000000687ad44ad37f03c201000000ab3c0572291feb8b01000000bc9d89904f5b923f0100000037c8bb1350a9a2a80100000006000000";
        let b = meta_to_json(a);
        assert_eq!(b, "[null,null,\"ac6c95f743a0c770d453855e5d95db6e4d6d8af91a05d6a97ce65bd72f24bd4d\"]");
    }    
}

