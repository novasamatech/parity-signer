use thiserror::Error;

pub type Result<T> = anyhow::Result<T>;

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
