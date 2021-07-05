use thiserror::Error;

pub type Result<T> = anyhow::Result<T>;

pub fn return_json_array(mut string: String) -> std::result::Result<String, Box<dyn std::error::Error>> {
    match string.pop() {
            None | Some('[') => return Ok("[]".to_string()),
            Some(',') => {
                string.push_str("]");
                return Ok(string);
            }
            _ => return Err(Box::from("Database corrupted!"))
        }

}

#[derive(Error, Debug)]
pub enum Error {
	#[error("Could not derive key pair")]
	KeyPairIsNone,
	#[error("Error converting from hex: {0:?}")]
	FromHex(rustc_hex::FromHexError),
	#[error("Ethsign error: {0:?}")]
	Ethsign(ethsign::Error),
	#[error("Signature error: {0:?}")]
	Signature(schnorrkel::SignatureError),
	#[error("Error creating icon: {0:?}")]
	Blockies(blockies::Error),
	#[error("Error rendering QR code: {0:?}")]
	Pixelate(pixelate::Error),
}
