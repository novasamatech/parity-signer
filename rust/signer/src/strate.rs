use schnorrkel::keys::ExpansionMode;
use substrate_bip39::mini_secret_from_entropy;
use bip39::{Mnemonic, Language};
use base58::ToBase58;

pub struct KeyPair(schnorrkel::Keypair);

impl KeyPair {
	pub fn from_bip39_phrase(phrase: &str) -> Option<KeyPair> {
		let mnemonic = Mnemonic::from_phrase(phrase, Language::English).ok()?;
        let mini_secret_key = mini_secret_from_entropy(mnemonic.entropy(), "").ok()?;

		Some(KeyPair(mini_secret_key.expand_to_keypair(ExpansionMode::Ed25519)))
	}

    pub fn ss58_address(&self, prefix: u8) -> String {
        let mut v = vec![prefix];
        v.extend_from_slice(&self.0.public.to_bytes());
        let r = ss58hash(&v);
        v.extend_from_slice(&r.as_bytes()[0..2]);
        v.to_base58()
    }
}

const PREFIX: &[u8] = b"SS58PRE";

fn ss58hash(data: &[u8]) -> blake2_rfc::blake2b::Blake2bResult {
    let mut context = blake2_rfc::blake2b::Blake2b::new(64);
    context.update(PREFIX);
    context.update(data);
    context.finalize()
}
