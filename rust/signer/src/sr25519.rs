use codec::{Encode, Decode};
use lazy_static::lazy_static;
use regex::Regex;
use schnorrkel::{ExpansionMode, SecretKey};
use schnorrkel::derive::{ChainCode, Derivation};
use substrate_bip39::mini_secret_from_entropy;
use bip39::{Mnemonic, Language};
use base58::ToBase58;

pub struct KeyPair(schnorrkel::Keypair);

const SIGNING_CTX: &[u8] = b"substrate";
const JUNCTION_ID_LEN: usize = 32;
const CHAIN_CODE_LENGTH: usize = 32;

impl KeyPair {
	pub fn from_bip39_phrase(phrase: &str, password: Option<&str>) -> Option<KeyPair> {
		let mnemonic = Mnemonic::from_phrase(phrase, Language::English).ok()?;
        let mini_secret_key = mini_secret_from_entropy(mnemonic.entropy(), password.unwrap_or("")).ok()?;

		Some(KeyPair(mini_secret_key.expand_to_keypair(ExpansionMode::Ed25519)))
	}

    // Should match implementation at https://github.com/paritytech/substrate/blob/master/core/primitives/src/crypto.rs#L653-L682
    pub fn from_suri(suri: &str) -> Option<KeyPair> {
        lazy_static! {
            static ref RE_SURI: Regex = {
                Regex::new(r"^(?P<phrase>\w+( \w+)*)?(?P<path>(//?[^/]+)*)(///(?P<password>.*))?$")
                    .expect("constructed from known-good static value; qed")
            };

            static ref RE_JUNCTION: Regex = Regex::new(r"/(/?[^/]+)").expect("constructed from known-good static value; qed");
        }

        let cap = RE_SURI.captures(suri)?;
        let path = RE_JUNCTION.captures_iter(&cap["path"]).map(|j| DeriveJunction::from(&j[1]));

        let pair = Self::from_bip39_phrase(
            cap.name("phrase").map(|p| p.as_str())?,
            cap.name("password").map(|p| p.as_str())
        )?;

        Some(pair.derive(path))
    }

    fn derive(&self, path: impl Iterator<Item = DeriveJunction>) -> Self {
        let init = self.0.secret.clone();
        let result = path.fold(init, |acc, j| match j {
            DeriveJunction::Soft(cc) => acc.derived_key_simple(ChainCode(cc), &[]).0,
            DeriveJunction::Hard(cc) => derive_hard_junction(&acc, cc),
        });

        KeyPair(result.to_keypair())
    }

    pub fn ss58_address(&self, prefix: u8) -> String {
        let mut v = vec![prefix];
        v.extend_from_slice(&self.0.public.to_bytes());
        let r = ss58hash(&v);
        v.extend_from_slice(&r.as_bytes()[0..2]);
        v.to_base58()
    }

    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        let context = schnorrkel::signing_context(SIGNING_CTX);
        self.0.sign(context.bytes(message)).to_bytes()
    }
}

fn derive_hard_junction(secret: &SecretKey, cc: [u8; CHAIN_CODE_LENGTH]) -> SecretKey {
    secret.hard_derive_mini_secret_key(Some(ChainCode(cc)), b"").0.expand(ExpansionMode::Ed25519)
}

/// A since derivation junction description. It is the single parameter used when creating
/// a new secret key from an existing secret key and, in the case of `SoftRaw` and `SoftIndex`
/// a new public key from an existing public key.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Encode, Decode)]
enum DeriveJunction {
    /// Soft (vanilla) derivation. Public keys have a correspondent derivation.
    Soft([u8; JUNCTION_ID_LEN]),
    /// Hard ("hardened") derivation. Public keys do not have a correspondent derivation.
    Hard([u8; JUNCTION_ID_LEN]),
}

impl DeriveJunction {
    /// Consume self to return a hard derive junction with the same chain code.
    fn harden(self) -> Self { DeriveJunction::Hard(self.unwrap_inner()) }

    /// Create a new soft (vanilla) DeriveJunction from a given, encodable, value.
    ///
    /// If you need a hard junction, use `hard()`.
    fn soft<T: Encode>(index: T) -> Self {
        let mut cc: [u8; JUNCTION_ID_LEN] = Default::default();
        index.using_encoded(|data| if data.len() > JUNCTION_ID_LEN {
            let hash_result = blake2_rfc::blake2b::blake2b(JUNCTION_ID_LEN, &[], data);
            let hash = hash_result.as_bytes();
            cc.copy_from_slice(hash);
        } else {
            cc[0..data.len()].copy_from_slice(data);
        });
        DeriveJunction::Soft(cc)
    }

    /// Consume self to return the chain code.
    fn unwrap_inner(self) -> [u8; JUNCTION_ID_LEN] {
        match self {
            DeriveJunction::Hard(c) | DeriveJunction::Soft(c) => c,
        }
    }
}

impl<T: AsRef<str>> From<T> for DeriveJunction {
    fn from(j: T) -> DeriveJunction {
        let j = j.as_ref();
        let (code, hard) = if j.starts_with("/") {
            (&j[1..], true)
        } else {
            (j, false)
        };

        let res = if let Ok(n) = str::parse::<u64>(code) {
            // number
            DeriveJunction::soft(n)
        } else {
            // something else
            DeriveJunction::soft(code)
        };

        if hard {
            res.harden()
        } else {
            res
        }
    }
}

fn ss58hash(data: &[u8]) -> blake2_rfc::blake2b::Blake2bResult {
    const PREFIX: &[u8] = b"SS58PRE";

    let mut context = blake2_rfc::blake2b::Blake2b::new(64);
    context.update(PREFIX);
    context.update(data);
    context.finalize()
}
