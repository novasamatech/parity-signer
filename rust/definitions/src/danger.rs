use parity_scale_codec::{Decode, Encode};
use parity_scale_codec_derive;

/// Struct to process the content of qr codes with load_metadata messages
pub struct DangerRecord (Vec<u8>);

#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
struct DecodedDangerRecord {
    device_was_online: bool,
}

impl DangerRecord {
    /// Function to set danger record to `safe`
    pub fn safe () -> Self {
        Self (
            DecodedDangerRecord {
                device_was_online: false,
            }.encode()
        )
    }
    /// Function to set danger record to `not_safe`
    pub fn not_safe () -> Self {
        Self (
            DecodedDangerRecord {
                device_was_online: true,
            }.encode()
        )
    }
    /// Function to get danger record from the corresponding database key
    pub fn from_vec (vec: &Vec<u8>) -> Self {
        Self(vec.to_vec())
    }
    /// Function to get `device_was_online` flag
    pub fn device_was_online (&self) -> Result<bool, &'static str>  {
        match <DecodedDangerRecord>::decode(&mut &self.0[..]) {
            Ok(a) => Ok(a.device_was_online),
            Err(_) => return Err("danger indicator content could not be decoded")
        }
    }
    /// Function to prepare the danger record information into storage as Vec<u8>
    pub fn store (&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

