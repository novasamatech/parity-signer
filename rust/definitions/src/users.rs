use parity_scale_codec_derive::{Decode, Encode};

#[derive(Decode, Encode)]
pub struct AddressDetails {
    pub name_for_seed: String,
    pub path: String,
    pub has_pwd: bool,
    pub name: String,
    pub network_path_id: String,
}
