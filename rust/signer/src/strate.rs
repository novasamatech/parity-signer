use schnorrkel::MiniSecretKey;
use bip39::{Mnemonic, Seed, Language};

pub struct KeyPair(schnorrkel::Keypair);

impl KeyPair {
	pub fn from_bip39_phrase(phrase: &str) -> Option<KeyPair> {
		let mnemonic = Mnemonic::from_phrase(phrase, Language::English).ok()?;
		let seed = Seed::new(&mnemonic, "");
		let mini_secret_key = MiniSecretKey::from_bytes(&seed.as_bytes()[..32]).ok()?;

		Some(KeyPair(mini_secret_key.expand_to_keypair()))
	}
}
