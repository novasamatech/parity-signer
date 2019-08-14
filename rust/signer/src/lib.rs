// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

mod eth;
mod sr25519;
mod util;

use util::StringPtr;
use eth::{KeyPair, PhraseKind};

use ethsign::{Protected, keyfile::Crypto};
use rlp::decode_list;
use rustc_hex::{ToHex, FromHex};
use tiny_keccak::Keccak;
use tiny_keccak::keccak256 as keccak;
use blake2_rfc::blake2s::blake2s;
use std::num::NonZeroU32;

// 10240 is always non-zero, ergo this is safe
const CRYPTO_ITERATIONS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(10240) };

fn base64png(png: &[u8]) -> String {
    static HEADER: &str = "data:image/png;base64,";

    let mut out = String::with_capacity(png.len() + png.len() / 2 + HEADER.len());

    out.push_str(HEADER);

    base64::encode_config_buf(png, base64::STANDARD, &mut out);

    out
}

// string ffi
#[no_mangle]
pub unsafe extern fn rust_string_ptr(s: *mut String) -> *mut StringPtr {
    Box::into_raw(Box::new(StringPtr::from(&**s)))
}

#[no_mangle]
pub unsafe extern fn rust_string_destroy(s: *mut String) {
    let _ = Box::from_raw(s);
}

#[no_mangle]
pub unsafe extern fn rust_string_ptr_destroy(s: *mut StringPtr) {
    let _ = Box::from_raw(s);
}

// TODO: REMOVE, use ethkey_brainwallet_sign!
#[no_mangle]
pub unsafe extern fn ethkey_keypair_sign(keypair: *mut KeyPair, message: *mut StringPtr) -> *mut String {
    let keypair = &*keypair;
    let message: Vec<u8> = (*message).as_str().from_hex().unwrap();
    let signature = keypair.sign(&message).unwrap().to_hex();
    Box::into_raw(Box::new(signature))
}

fn qrcode_bytes(data: &[u8]) -> Option<String> {
    use qrcodegen::{QrCode, QrCodeEcc};
    use pixelate::{Image, Color, BLACK};

    let qr = QrCode::encode_binary(data, QrCodeEcc::Medium).ok()?;

    let palette = &[Color::Rgba(255,255,255,0), BLACK];
    let mut pixels = Vec::with_capacity((qr.size() * qr.size()) as usize);

    for y in 0..qr.size() {
        for x in 0..qr.size() {
            pixels.push(qr.get_module(x, y) as u8);
        }
    }

    let mut result = Vec::new();

    Image {
        palette,
        pixels: &pixels,
        width: qr.size() as usize,
        scale: 16,
    }.render(&mut result).ok()?;

    Some(base64png(&result))
}

export! {
    @Java_io_parity_signer_EthkeyBridge_ethkeyBrainwalletAddress
    fn ethkey_brainwallet_address(seed: &str) -> String {
        let (kind, keypair) = KeyPair::from_auto_phrase(seed);

        let mut out = String::with_capacity(47);

        out += match kind {
            PhraseKind::Bip39 => "bip39:",
            PhraseKind::Legacy => "legacy:",
        };
        out += &keypair.address().to_hex::<String>();
        out
    }

    @Java_io_parity_signer_EthkeyBridge_ethkeyBrainwalletBIP39Address
    fn ethkey_brainwallet_bip39_address(seed: &str) -> Option<String> {
        let keypair = KeyPair::from_bip39_phrase(seed)?;

        Some(keypair.address().to_hex())
    }

    @Java_io_parity_signer_EthkeyBridge_ethkeyBrainwalletSign
    fn ethkey_brainwallet_sign(seed: &str, message: &str) -> Option<String> {
        let (_, keypair) = KeyPair::from_auto_phrase(seed);
        let message: Vec<u8> = message.from_hex().ok()?;
        let signature = keypair.sign(&message).ok()?;

        Some(signature.to_hex())
    }

    @Java_io_parity_signer_EthkeyBridge_ethkeyRlpItem
    fn rlp_item(rlp: &str, position: u32) -> Option<String> {
        let hex: Vec<u8> = rlp.from_hex().ok()?;
        let rlp = decode_list::<Vec<u8>>(&hex);

        rlp.get(position as usize).map(|data| data.to_hex())
    }

    @Java_io_parity_signer_EthkeyBridge_ethkeyKeccak
    fn keccak256(data: &str) -> Option<String> {
        let data: Vec<u8> = data.from_hex().ok()?;

        Some(keccak(&data).to_hex())
    }

    @Java_io_parity_signer_EthkeyBridge_ethkeyBlake
    fn blake(data: &str) -> Option<String> {
        let data: Vec<u8> = data.from_hex().ok()?;

        Some(blake2s(32, &[], &data).as_bytes().to_hex())
    }

    @Java_io_parity_signer_EthkeyBridge_ethkeyBlockiesIcon
    fn blockies_icon(seed: String) -> Option<String> {
        use blockies::Ethereum;

        let mut result = Vec::new();
        let blockies = Ethereum::default();

        match blockies.create_icon(&mut result, seed.as_bytes()) {
            Ok(_) => Some(base64png(&result)),
            Err(_) => None,
        }
    }

    @Java_io_parity_signer_EthkeyBridge_ethkeyEthSign
    fn eth_sign(data: &str) -> Option<String> {
        let hex: Vec<u8> = data.from_hex().ok()?;
        let message = format!("\x19Ethereum Signed Message:\n{}", hex.len()).into_bytes();

        let mut res = [0u8; 32];
        let mut keccak = Keccak::new_keccak256();
        keccak.update(&message);
        keccak.update(&hex);
        keccak.finalize(&mut res);

        Some(res.to_hex())
    }

    @Java_io_parity_signer_EthkeyBridge_ethkeyRandomPhrase
    fn random_phrase() -> String {
        use bip39::{Mnemonic, MnemonicType, Language};
        let mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);

        mnemonic.into_phrase()
    }

    @Java_io_parity_signer_EthkeyBridge_ethkeyEncryptData
    fn encrypt_data(data: &str, password: String) -> Option<String> {
        let password = Protected::new(password.into_bytes());
        let crypto = Crypto::encrypt(data.as_bytes(), &password, CRYPTO_ITERATIONS).ok()?;

        serde_json::to_string(&crypto).ok()
    }

    @Java_io_parity_signer_EthkeyBridge_ethkeyDecryptData
    fn decrypt_data(data: &str, password: String) -> Option<String> {
        let password = Protected::new(password.into_bytes());

        let crypto: Crypto = serde_json::from_str(data).ok()?;
        let decrypted = crypto.decrypt(&password).ok()?;

        String::from_utf8(decrypted).ok()
    }

    @Java_io_parity_signer_EthkeyBridge_ethkeyQrCode
    fn qrcode(data: &str) -> Option<String> {
        qrcode_bytes(data.as_bytes())
    }

    @Java_io_parity_signer_EthkeyBridge_ethkeyQrCodeHex
    fn qrcode_hex(data: &str) -> Option<String> {
        qrcode_bytes(&data.from_hex::<Vec<u8>>().ok()?)
    }

    @Java_io_parity_signer_EthkeyBridge_substrateBrainwalletAddress
    fn substrate_brainwallet_address(suri: &str, prefix: u8) -> Option<String> {
        let keypair = sr25519::KeyPair::from_suri(suri)?;

        Some(keypair.ss58_address(prefix))
    }

    @Java_io_parity_signer_EthkeyBridge_substrateBrainwalletSign
    fn substrate_brainwallet_sign(suri: &str, message: &str) -> Option<String> {
        let keypair = sr25519::KeyPair::from_suri(suri)?;

        let message: Vec<u8> = message.from_hex().ok()?;
        let signature = keypair.sign(&message);

        Some(signature.to_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rlp_item() {
        let rlp = "f85f800182520894095e7baea6a6c7c4c2dfeb977efac326af552d870a801ba048b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353a0efffd310ac743f371de3b9f7f9cb56c0b28ad43601b4ab949f53faa07bd2c804";
        assert_eq!(rlp_item(rlp, 0), Some("".into()));
        assert_eq!(rlp_item(rlp, 1), Some("01".into()));
        assert_eq!(rlp_item(rlp, 2), Some("5208".into()));
        assert_eq!(rlp_item(rlp, 3), Some("095e7baea6a6c7c4c2dfeb977efac326af552d87".into()));
        assert_eq!(rlp_item(rlp, 4), Some("0a".into()));
        assert_eq!(rlp_item(rlp, 5), Some("".into()));
    }

    #[test]
    fn test_substrate_brainwallet_address() {
        let suri = "grant jaguar wish bench exact find voice habit tank pony state salmon";
        // Secret seed: 0xb139e4050f80172b44957ef9d1755ef5c96c296d63b8a2b50025bf477bd95224
        // Public key (hex): 0x944eeb240615f4a94f673f240a256584ba178e22dd7b67503a753968e2f95761
        let expected = "5FRAPSnpgmnXAnmPVv68fT6o7ntTvaZmkTED8jDttnXs9k4n";
        let generated = substrate_brainwallet_address(suri, 42).unwrap();

        assert_eq!(expected, generated);
    }

    #[test]
    fn test_substrate_brainwallet_address_suri() {
        let suri = "grant jaguar wish bench exact find voice habit tank pony state salmon//hard/soft/0";
        let expected = "5D4kaJXj5HVoBw2tFFsDj56BjZdPhXKxgGxZuKk4K3bKqHZ6";
        let generated = substrate_brainwallet_address(suri, 42).unwrap();

        assert_eq!(expected, generated);
    }
}
