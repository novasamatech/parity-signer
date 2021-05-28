use std::collections::HashMap;
use parity_scale_codec::{Decode};
use frame_metadata::{RuntimeMetadataV12, DecodeDifferent};

use super::utils_chainspecs::collect_meta;

/// function to make a hashmap of all types encountered in chain methods

pub fn map_types (meta: &RuntimeMetadataV12) -> HashMap<String, u32> {
    
    let mut types_map = HashMap::new();
    
    if let DecodeDifferent::Decoded(meta_vector) = &meta.modules {
        for y in meta_vector.iter() {
            if let Some(DecodeDifferent::Decoded(calls)) = &y.calls {
                for z in calls {
                    if let DecodeDifferent::Decoded(args) = &z.arguments {
                        for a in args.iter() {
                            if let DecodeDifferent::Decoded(c) = &a.ty {
                                let count = types_map.entry(c.to_string()).or_insert(0);
                                *count +=1;
                            }
                        }
                    }
                }
            }
        }
    }
    types_map
}

/// function to make a hashmap of all types in all chains for file

pub fn map_types_all (metadata_contents: &str) -> HashMap<String, u32> {
    
    let metadata = collect_meta(metadata_contents);
    
    let mut types_map = HashMap::new();
    
    for x in metadata.iter() {
        let meta_unhex = hex::decode(&x.meta[2..]).unwrap();
        if let Ok(data_back) = RuntimeMetadataV12::decode(&mut &meta_unhex[..]) {
            if let DecodeDifferent::Decoded(meta_vector) = data_back.modules {
                for y in meta_vector.iter() {
                    if let Some(DecodeDifferent::Decoded(calls)) = &y.calls {
                        for z in calls {
                            if let DecodeDifferent::Decoded(args) = &z.arguments {
                                for a in args.iter() {
                                    if let DecodeDifferent::Decoded(c) = &a.ty {
                                        let count = types_map.entry(c.to_string()).or_insert(0);
                                        *count +=1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    types_map
}

