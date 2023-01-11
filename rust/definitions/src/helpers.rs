//! Common helper functions

#[cfg(feature = "signer")]
use constants::IDENTICON_IMG_SIZE;
use hex;
#[cfg(feature = "signer")]
use sp_core::{crypto::AccountId32, ecdsa, ed25519, sr25519};
use sp_core::{
    crypto::{Ss58AddressFormat, Ss58Codec},
    hexdisplay::HexDisplay,
    Hasher, KeccakHasher, H160, H256,
};
use sp_runtime::MultiSigner;
#[cfg(feature = "signer")]
use std::convert::TryInto;

#[cfg(feature = "signer")]
use plot_icon::{generate_png, EMPTY_PNG};

#[cfg(all(feature = "signer", target_os = "ios"))]
use plot_icon::generate_svg;

use crate::crypto::Encryption;
#[cfg(feature = "signer")]
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
}

#[cfg(feature = "signer")]
use crate::navigation::SignerImage;

/// Print identicon from
/// [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)  
#[cfg(feature = "signer")]
pub fn make_identicon_from_multisigner(
    multisigner: &MultiSigner,
    style: IdenticonStyle,
) -> SignerImage {
    match style {
        IdenticonStyle::Dots => make_identicon(&multisigner_to_public(multisigner)),
        IdenticonStyle::Blockies => {
            if let MultiSigner::Ecdsa(ref public) = multisigner {
                use eth_blockies::{eth_blockies_png_data, SeedString};
                let account = print_ethereum_address(public);
                let account = account.canonicalize_ethaddr();
                let dimension = (IDENTICON_IMG_SIZE, IDENTICON_IMG_SIZE);
                let compressed_output = false;
                SignerImage::Png {
                    image: eth_blockies_png_data(account, dimension, compressed_output),
                }
            } else {
                SignerImage::Png {
                    image: EMPTY_PNG.to_vec(),
                }
            }
        }
    }
}

#[cfg(feature = "signer")]
pub fn make_identicon_from_id20(id: &[u8; 20]) -> SignerImage {
    use eth_blockies::eth_blockies_png_data;

    let account = format!("0x{}", hex::encode(id));
    let dimension = (IDENTICON_IMG_SIZE, IDENTICON_IMG_SIZE);
    let compressed_output = false;
    let image = eth_blockies_png_data(account, dimension, compressed_output);

    SignerImage::Png { image }
}

#[cfg(feature = "signer")]
pub fn make_identicon_from_account(account: AccountId32) -> SignerImage {
    make_identicon(&<[u8; 32]>::from(account))
}

#[cfg(all(feature = "signer", target_os = "ios"))]
fn make_identicon(into_id: &[u8]) -> SignerImage {
    let image = generate_svg(into_id).to_string().into_bytes();

    SignerImage::Svg { image }
}

#[cfg(all(feature = "signer", not(target_os = "ios")))]
fn make_identicon(into_id: &[u8]) -> SignerImage {
    let image = match generate_png(into_id, IDENTICON_IMG_SIZE as u16) {
        Ok(a) => a,
        Err(_) => EMPTY_PNG.to_vec(),
    };

    SignerImage::Png { image }
}

/// Get [`MultiSigner`](https://docs.rs/sp-runtime/6.0.0/sp_runtime/enum.MultiSigner.html)
/// from public key and [`Encryption`](crate::crypto::Encryption)
#[cfg(feature = "signer")]
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
pub fn print_multisigner_as_base58_or_eth(
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
            MultiSigner::Ed25519(pubkey) => pubkey.to_ss58check(),
            MultiSigner::Sr25519(pubkey) => pubkey.to_ss58check(),
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

/// Turn a `ecdsa::Public` addr into an Ethereum address.
pub fn ecdsa_public_to_eth_address(public: &ecdsa::Public) -> Result<H160> {
    let decompressed = libsecp256k1::PublicKey::parse_compressed(&public.0)?.serialize();
    let mut m = [0u8; 64];
    m.copy_from_slice(&decompressed[1..65]);
    Ok(H160::from(H256::from_slice(
        KeccakHasher::hash(&m).as_bytes(),
    )))
}

/// Print a `ecdsa::Public` into `String`.
///
/// Panics if provided ecdsa public key is in wrong format.
fn print_ethereum_address(public: &ecdsa::Public) -> String {
    let account = ecdsa_public_to_eth_address(public).expect("Wrong ecdsa public key provided");

    format!("0x{:?}", HexDisplay::from(&account.as_bytes()))
}

pub fn base58_to_multisigner(base58: &str, encryption: &Encryption) -> Result<MultiSigner> {
    match encryption {
        Encryption::Ed25519 => {
            let pubkey = ed25519::Public::from_ss58check(base58)?;
            Ok(MultiSigner::Ed25519(pubkey))
        }
        Encryption::Sr25519 => {
            let pubkey = sr25519::Public::from_ss58check(base58)?;
            Ok(MultiSigner::Sr25519(pubkey))
        }
        Encryption::Ecdsa => {
            let pubkey = ecdsa::Public::from_ss58check(base58)?;
            Ok(MultiSigner::Ecdsa(pubkey))
        }
        Encryption::Ethereum => Err(Error::EthereumAddressNotSupported),
    }
}

/// Print id pic for metadata hash
///
/// Currently uses PNG identicon generator, could be changed later.
#[cfg(feature = "signer")]
pub fn pic_meta(meta_hash: &[u8]) -> SignerImage {
    make_identicon(meta_hash)
}

/// Print id pic for hash of SCALE-encoded types data
///
/// Currently uses PNG identicon generator, could be changed later.
#[cfg(feature = "signer")]
pub fn pic_types(types_hash: &[u8]) -> SignerImage {
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
        let encryption = Encryption::Sr25519;
        let public = sr25519::Pair::from_seed_slice(&secret_key)
            .unwrap()
            .public();
        let multisigner = Sr25519(public);

        let ss58 = print_multisigner_as_base58_or_eth(&multisigner, None, encryption);
        let result = base58_to_multisigner(&ss58, &Encryption::Sr25519).unwrap();
        assert_eq!(result, multisigner);
    }
}
