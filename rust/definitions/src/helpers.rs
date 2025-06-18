//! Common helper functions

use hex;
use sp_core::hexdisplay::AsBytesRef;
use sp_core::{crypto::AccountId32, ecdsa, ed25519, sr25519};
use sp_core::{
    crypto::{Ss58AddressFormat, Ss58Codec},
    hexdisplay::HexDisplay,
    Hasher, KeccakHasher, H160, H256,
};
use sp_runtime::MultiSigner;
use std::convert::TryInto;

use crate::crypto::Encryption;
use crate::error::Error;
use crate::error::Result;

/// Decode hexadecimal `&str` into `Vec<u8>`, with descriptive error  
///
/// Function could be used both on hot and cold side.  
///
/// In addition to encoded `&str` required is input of `T::NotHex`, to produce
/// error with details on what exactly turned out to be invalid hexadecimal
/// string.  
pub fn unhex(hex_entry: &str) -> Result<Vec<u8>> {
    let hex_entry = {
        if let Some(a) = hex_entry.strip_prefix("0x") {
            a
        } else {
            hex_entry
        }
    };
    Ok(hex::decode(hex_entry)?)
}

/// Get `Vec<u8>` public key from
/// [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)  
pub fn multisigner_to_public(m: &MultiSigner) -> Vec<u8> {
    match m {
        MultiSigner::Ed25519(a) => a.to_vec(),
        MultiSigner::Sr25519(a) => a.to_vec(),
        MultiSigner::Ecdsa(a) => a.0.to_vec(),
    }
}

/// Get [`Encryption`](crate::crypto::Encryption) from
/// [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)  
pub fn multisigner_to_encryption(m: &MultiSigner) -> Encryption {
    match m {
        MultiSigner::Ed25519(_) => Encryption::Ed25519,
        MultiSigner::Sr25519(_) => Encryption::Sr25519,
        MultiSigner::Ecdsa(_) => Encryption::Ecdsa,
    }
}

pub enum IdenticonStyle {
    /// Default style for substrate-based networks, dots in a circle.
    Dots,

    /// Blockies style used in Ethereum networks.
    Blockies,

    /// Jdenticon style used to identify key sets.
    Jdenticon,
}

use crate::navigation::Identicon;

/// Print identicon from
/// [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)  
pub fn make_identicon_from_multisigner(
    multisigner: &MultiSigner,
    style: IdenticonStyle,
) -> Identicon {
    match style {
        IdenticonStyle::Dots => Identicon::Dots {
            identity: multisigner_to_public(multisigner),
        },
        IdenticonStyle::Blockies => {
            if let MultiSigner::Ecdsa(ref public) = multisigner {
                Identicon::Blockies {
                    identity: print_ethereum_address(public),
                }
            } else {
                Identicon::Blockies {
                    identity: "".to_string(),
                }
            }
        }
        IdenticonStyle::Jdenticon => Identicon::Jdenticon {
            identity: print_multisigner_as_base58_or_eth_address(
                multisigner,
                None,
                multisigner_to_encryption(multisigner),
            ),
        },
    }
}

pub fn make_identicon_from_id20(id: &[u8; 20]) -> Identicon {
    let account = format!("0x{}", hex::encode(id));
    Identicon::Blockies { identity: account }
}

pub fn make_identicon_from_account(account: AccountId32) -> Identicon {
    make_identicon(&<[u8; 32]>::from(account))
}

fn make_identicon(into_id: &[u8]) -> Identicon {
    Identicon::Dots {
        identity: into_id.into(),
    }
}

/// Get [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)
/// from public key and [`Encryption`](crate::crypto::Encryption)
pub fn get_multisigner(public: &[u8], encryption: &Encryption) -> Result<MultiSigner> {
    match encryption {
        Encryption::Ed25519 => {
            let into_pubkey: [u8; 32] = public
                .to_vec()
                .try_into()
                .map_err(|_| Error::WrongPublicKeyLength)?;
            Ok(MultiSigner::Ed25519(ed25519::Public::from_raw(into_pubkey)))
        }
        Encryption::Sr25519 => {
            let into_pubkey: [u8; 32] = public
                .to_vec()
                .try_into()
                .map_err(|_| Error::WrongPublicKeyLength)?;
            Ok(MultiSigner::Sr25519(sr25519::Public::from_raw(into_pubkey)))
        }
        Encryption::Ecdsa | Encryption::Ethereum => {
            let into_pubkey: [u8; 33] = public
                .to_vec()
                .try_into()
                .map_err(|_| Error::WrongPublicKeyLength)?;
            Ok(MultiSigner::Ecdsa(ecdsa::Public::from_raw(into_pubkey)))
        }
    }
}

/// Print [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)
/// in base58 format
///
/// Could be done for both
/// [custom](https://docs.rs/sp-core/6.0.0/sp_core/crypto/trait.Ss58Codec.html#method.to_ss58check_with_version)
/// network-specific base58 prefix by providing `Some(value)` as `optional_prefix` or with
/// [default](https://docs.rs/sp-core/6.0.0/sp_core/crypto/trait.Ss58Codec.html#method.to_ss58check)
/// one by leaving it `None`.
pub fn print_multisigner_as_base58_or_eth_address(
    multi_signer: &MultiSigner,
    optional_prefix: Option<u16>,
    encryption: Encryption,
) -> String {
    match optional_prefix {
        Some(base58prefix) => {
            let version_for_base58 = Ss58AddressFormat::custom(base58prefix);
            match multi_signer {
                MultiSigner::Ed25519(pubkey) => {
                    pubkey.to_ss58check_with_version(version_for_base58)
                }
                MultiSigner::Sr25519(pubkey) => {
                    pubkey.to_ss58check_with_version(version_for_base58)
                }
                MultiSigner::Ecdsa(pubkey) => {
                    if encryption == Encryption::Ethereum {
                        print_ethereum_address(pubkey)
                    } else {
                        pubkey.to_ss58check_with_version(version_for_base58)
                    }
                }
            }
        }
        None => match multi_signer {
            MultiSigner::Ed25519(pubkey) => {
                let version = Ss58AddressFormat::try_from("BareEd25519")
                    .expect("unable to make Ss58AddressFormat from `BareEd25519`");
                pubkey.to_ss58check_with_version(version)
            }
            MultiSigner::Sr25519(pubkey) => {
                let version = Ss58AddressFormat::try_from("BareSr25519")
                    .expect("unable to make Ss58AddressFormat from `BareSr25519`");
                pubkey.to_ss58check_with_version(version)
            }
            MultiSigner::Ecdsa(pubkey) => {
                if encryption == Encryption::Ethereum {
                    print_ethereum_address(pubkey)
                } else {
                    pubkey.to_ss58check()
                }
            }
        },
    }
}

/// Print [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)
/// in base58 format or hex public key for ecdsa.
//
// This leaves an opportunity to restore the `MultiSigner` back if needed.
pub fn print_multisigner_as_base58_or_eth_public_key(
    multi_signer: &MultiSigner,
    optional_prefix: Option<u16>
) -> String {
    match optional_prefix {
        Some(base58prefix) => {
            let version_for_base58 = Ss58AddressFormat::custom(base58prefix);
            match multi_signer {
                MultiSigner::Ed25519(pubkey) => {
                    pubkey.to_ss58check_with_version(version_for_base58)
                }
                MultiSigner::Sr25519(pubkey) => {
                    pubkey.to_ss58check_with_version(version_for_base58)
                }
                MultiSigner::Ecdsa(pubkey) => {
                    print_ecdsa_public_key(pubkey)
                }
            }
        }
        None => match multi_signer {
            MultiSigner::Ed25519(pubkey) => {
                let version = Ss58AddressFormat::try_from("BareEd25519")
                    .expect("unable to make Ss58AddressFormat from `BareEd25519`");
                pubkey.to_ss58check_with_version(version)
            }
            MultiSigner::Sr25519(pubkey) => {
                let version = Ss58AddressFormat::try_from("BareSr25519")
                    .expect("unable to make Ss58AddressFormat from `BareSr25519`");
                pubkey.to_ss58check_with_version(version)
            }
            MultiSigner::Ecdsa(pubkey) => {
                print_ecdsa_public_key(pubkey)
            }
        },
    }
}

/// Turn a `ecdsa::Public` addr into an Ethereum address.
pub fn ecdsa_public_to_eth_address(public: &ecdsa::Public) -> Result<H160> {
    let decompressed = libsecp256k1::PublicKey::parse_compressed(&public.0)?.serialize();
    let mut m = [0u8; 64];
    m.copy_from_slice(&decompressed[1..65]);
    Ok(H160::from(H256::from_slice(
        KeccakHasher::hash(&m).as_bytes(),
    )))
}

/// Print a `ecdsa::Public` into Ethereum address `String`.
///
/// Panics if provided ecdsa public key is in wrong format.
fn print_ethereum_address(public: &ecdsa::Public) -> String {
    let account = ecdsa_public_to_eth_address(public).expect("Wrong ecdsa public key provided");

    format!("0x{:?}", HexDisplay::from(&account.as_bytes()))
}

/// Print a `ecdsa::Public` into Hex String `String`.
///
/// Panics if provided ecdsa public key is in wrong format.
fn print_ecdsa_public_key(key: &ecdsa::Public) -> String {
    format!("0x{:?}", HexDisplay::from(&key.0.as_bytes_ref()))
}

pub fn base58_or_eth_pubkey_to_multisigner(
    base58_or_eth: &str,
    encryption: &Encryption,
) -> Result<MultiSigner> {
    match encryption {
        Encryption::Ed25519 => {
            let pubkey = ed25519::Public::from_ss58check(base58_or_eth)?;
            Ok(MultiSigner::Ed25519(pubkey))
        }
        Encryption::Sr25519 => {
            let pubkey = sr25519::Public::from_ss58check(base58_or_eth)?;
            Ok(MultiSigner::Sr25519(pubkey))
        }
        Encryption::Ethereum | Encryption::Ecdsa => {
            let raw_key = unhex(base58_or_eth)?
                .try_into()
                .map_err(|_| Error::WrongPublicKeyLength)?;

            let pubkey = ecdsa::Public::from_raw(raw_key);
            Ok(MultiSigner::Ecdsa(pubkey))
        }
    }
}

/// Print id pic for metadata hash
///
/// Currently uses PNG identicon generator, could be changed later.
pub fn pic_meta(meta_hash: &[u8]) -> Identicon {
    make_identicon(meta_hash)
}

/// Print id pic for hash of SCALE-encoded types data
///
/// Currently uses PNG identicon generator, could be changed later.
pub fn pic_types(types_hash: &[u8]) -> Identicon {
    make_identicon(types_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use sp_core::Pair;
    use sp_runtime::MultiSigner::Sr25519;

    #[test]
    fn test_eth_account_1() {
        let secret_key =
            hex::decode("502f97299c472b88754accd412b7c9a6062ef3186fba0c0388365e1edec24875")
                .unwrap();

        let public_key = ecdsa::Pair::from_seed_slice(&secret_key).unwrap().public();

        assert_eq!(
            print_ethereum_address(&public_key),
            "0x976f8456e4e2034179b284a23c0e0c8f6d3da50c"
        )
    }

    #[test]
    fn test_eth_account_2() {
        let secret_key =
            hex::decode("0f02ba4d7f83e59eaa32eae9c3c4d99b68ce76decade21cdab7ecce8f4aef81a")
                .unwrap();

        let public_key = ecdsa::Pair::from_seed_slice(&secret_key).unwrap().public();

        assert_eq!(
            print_ethereum_address(&public_key),
            "0x420e9f260b40af7e49440cead3069f8e82a5230f",
        )
    }

    #[test]
    fn test_ss85_to_multisigner() {
        let secret_key =
            hex::decode("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a")
                .unwrap();
        let public = sr25519::Pair::from_seed_slice(&secret_key)
            .unwrap()
            .public();
        let multisigner = Sr25519(public);

        let ss58 = print_multisigner_as_base58_or_eth_public_key(&multisigner, None);
        let result = base58_or_eth_pubkey_to_multisigner(&ss58, &Encryption::Sr25519).unwrap();
        assert_eq!(result, multisigner);
    }

    #[test]
    fn test_ethereum_to_multisigner() {
        let secret_key =
            hex::decode("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a")
                .unwrap();
        let public = ecdsa::Pair::from_seed_slice(&secret_key)
            .unwrap()
            .public();
        let multisigner = MultiSigner::Ecdsa(public);

        let hexpubkey = print_multisigner_as_base58_or_eth_public_key(&multisigner, None);
        let result = base58_or_eth_pubkey_to_multisigner(&hexpubkey, &Encryption::Ethereum).unwrap();
        assert_eq!(result, multisigner);
    }

    #[test]
    fn test_print_multisigner_address_polkadot() {
        let multisigner = Sr25519(sr25519::Public(
            hex::decode("4a755d99a3cbafc1918769c292848bc87bc2e3cb3e09c17856a1c7d0c784b41c")
                .unwrap()
                .try_into()
                .unwrap(),
        ));
        assert_eq!(
            print_multisigner_as_base58_or_eth_address(&multisigner, Some(0), Encryption::Sr25519),
            "12gdQgfKFbiuba7hHS81MMr1rQH2amezrCbWixXZoUKzAm3q"
        );
    }

    #[test]
    fn test_print_multisigner_address_no_network() {
        let multisigner = Sr25519(sr25519::Public(
            hex::decode("4a755d99a3cbafc1918769c292848bc87bc2e3cb3e09c17856a1c7d0c784b41c")
                .unwrap()
                .try_into()
                .unwrap(),
        ));
        assert_eq!(
            print_multisigner_as_base58_or_eth_address(&multisigner, None, Encryption::Sr25519),
            "8UHfgCidtbdkdXABy12jG7SVtRKdxHX399eLeAsGKvUT2U6"
        );
    }
}
