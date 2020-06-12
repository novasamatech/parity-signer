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

use bip39::{Language, Mnemonic, MnemonicType};
use blake2_rfc::blake2b::blake2b;
use blockies::Ethereum;
use ethsign::{keyfile::Crypto, Protected};
use pixelate::{BLACK, Color, Image};
use qrcodegen::{QrCode, QrCodeEcc};
use rlp::decode_list;
use rustc_hex::{FromHex, ToHex};
use tiny_keccak::Keccak;
use tiny_keccak::keccak256 as keccak;

use eth::{KeyPair, PhraseKind};
use result::{Error, Result};

mod eth;
mod export;
mod result;
mod sr25519;

const CRYPTO_ITERATIONS: u32 = 10240;

fn base64png(png: &[u8]) -> String {
	static HEADER: &str = "data:image/png;base64,";
	let mut out = String::with_capacity(png.len() + png.len() / 2 + HEADER.len());
	out.push_str(HEADER);
	base64::encode_config_buf(png, base64::STANDARD, &mut out);
	out
}

fn qrcode_bytes(data: &[u8]) -> crate::Result<String> {
	let qr = QrCode::encode_binary(data, QrCodeEcc::Medium)?;
	let palette = &[Color::Rgba(255, 255, 255, 0), BLACK];
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
	}
		.render(&mut result)
		.map_err(|e| crate::Error::Pixelate(e))?;
	Ok(base64png(&result))
}

export! {
	@Java_io_parity_signer_EthkeyBridge_ethkeyBrainwalletAddress
	fn ethkey_brainwallet_address(
		seed: &str
	) -> String {
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
	fn ethkey_brainwallet_bip39_address(
		seed: &str
	) -> crate::Result<String> {
		let keypair = KeyPair::from_bip39_phrase(seed)
			.ok_or(crate::Error::KeyPairIsNone)?;
		Ok(keypair.address().to_hex())
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeyBrainwalletSign
	fn ethkey_brainwallet_sign(
		seed: &str,
		message: &str
	) -> crate::Result<String> {
		let (_, keypair) = KeyPair::from_auto_phrase(seed);
		let message: Vec<u8> = message.from_hex()
			.map_err(|e| crate::Error::FromHex(e))?;
		let signature = keypair.sign(&message)
			.map_err(|e| crate::Error::Ethsign(e))?;
		Ok(signature.to_hex())
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeyRlpItem
	fn rlp_item(
		rlp: &str,
		position: u32
	) -> crate::Result<String> {
		let hex: Vec<u8> = rlp.from_hex()
			.map_err(|e| crate::Error::FromHex(e))?;
		let rlp = decode_list::<Vec<u8>>(&hex);
		rlp.get(position as usize).map(|data| data.to_hex())
			.ok_or(anyhow::anyhow!("index out of bounds"))
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeyKeccak
	fn keccak256(
		data: &str
	) -> crate::Result<String> {
		let data: Vec<u8> = data.from_hex()
			.map_err(|e| crate::Error::FromHex(e))?;
		Ok(keccak(&data).to_hex())
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeyBlake
	fn blake(
		data: &str
	) -> crate::Result<String> {
		let data: Vec<u8> = data.from_hex()
			.map_err(|e| crate::Error::FromHex(e))?;
		Ok(blake2b(32, &[], &data).as_bytes().to_hex())
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeyBlockiesIcon
	fn blockies_icon(
		seed: String
	) -> crate::Result<String> {
		let mut result = Vec::new();
		let blockies = Ethereum::default();
		match blockies.create_icon(&mut result, seed.as_bytes()) {
			Ok(_) => Ok(base64png(&result)),
			Err(e) => Err(crate::Error::Blockies(e).into()),
		}
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeyEthSign
	fn eth_sign(
		data: &str
	) -> crate::Result<String> {
		let hex: Vec<u8> = data.from_hex()
			.map_err(|e| crate::Error::FromHex(e))?;
		let message = format!("\x19Ethereum Signed Message:\n{}", hex.len()).into_bytes();
		let mut res = [0u8; 32];
		let mut keccak = Keccak::new_keccak256();
		keccak.update(&message);
		keccak.update(&hex);
		keccak.finalize(&mut res);
		Ok(res.to_hex())
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeyRandomPhrase
	fn random_phrase(
		words_number:u32
	) -> String {
		let mnemonic_type = match MnemonicType::for_word_count(words_number as usize) {
			Ok(t) => t,
			Err(_e) => MnemonicType::Words24,
		};
		let mnemonic = Mnemonic::new(mnemonic_type, Language::English);
		mnemonic.into_phrase()
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeyEncryptData
	fn encrypt_data(
		data: &str,
		password: String
	) -> crate::Result<String> {
		let password = Protected::new(password.into_bytes());
		let crypto = Crypto::encrypt(data.as_bytes(), &password, CRYPTO_ITERATIONS)
			.map_err(|e| crate::Error::Ethsign(e))?;
		Ok(serde_json::to_string(&crypto)?)
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeyDecryptData
	fn decrypt_data(
		data: &str,
		password: String
	) -> crate::Result<String> {
		let password = Protected::new(password.into_bytes());
		let crypto: Crypto = serde_json::from_str(data)?;
		let decrypted = crypto.decrypt(&password)
			.map_err(|e| crate::Error::Ethsign(e))?;
		Ok(String::from_utf8(decrypted)?)
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeyQrCode
	fn qrcode(
		data: &str
	) -> crate::Result<String> {
		qrcode_bytes(data.as_bytes())
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeyQrCodeHex
	fn qrcode_hex(
		data: &str
	) -> crate::Result<String> {
		let bytes = &data.from_hex::<Vec<u8>>()
			.map_err(|e| crate::Error::FromHex(e))?;
		qrcode_bytes(&bytes)
	}

	@Java_io_parity_signer_EthkeyBridge_substrateBrainwalletAddress
	fn substrate_brainwallet_address(
		suri: &str,
		prefix: u8
	) -> crate::Result<String> {
		let keypair = sr25519::KeyPair::from_suri(suri)
			.ok_or(crate::Error::KeyPairIsNone)?;
		Ok(keypair.ss58_address(prefix))
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeySubstrateMiniSecretKey
	fn substrate_mini_secret_key(
		suri: &str
	) -> crate::Result<String> {
		let bytes = sr25519::KeyPair::get_derived_secret(&suri)
			.ok_or(crate::Error::KeyPairIsNone)?;
		Ok(bytes.to_hex())
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeySubstrateMiniSecretKeyWithRef
	fn substrate_mini_secret_key_with_ref (
		seed_ref: i64,
		suri_suffix: &str
	) -> crate::Result<String> {
		let seed = unsafe { Box::from_raw(seed_ref as *mut String) };
		let suri = format!("{}{}", &seed, suri_suffix);
		let bytes = sr25519::KeyPair::get_derived_secret(&suri)
			.ok_or(crate::Error::KeyPairIsNone)?;
		let _ = Box::into_raw(seed) as i64;
		Ok(bytes.to_hex())
	}

	@Java_io_parity_signer_EthkeyBridge_substrateBrainwalletSign
	fn substrate_brainwallet_sign(
		suri: &str,
		message: &str
	) -> crate::Result<String> {
		let keypair = sr25519::KeyPair::from_suri(suri)
			.ok_or(crate::Error::KeyPairIsNone)?;
		let message: Vec<u8> = message.from_hex()
			.map_err(|e| crate::Error::FromHex(e))?;
		let signature = keypair.sign(&message);
		Ok(signature.to_hex())
	}

	@Java_io_parity_signer_EthkeyBridge_schnorrkelVerify
	fn schnorrkel_verify(
		suri: &str,
		msg: &str,
		signature: &str
	) -> crate::Result<bool> {
		let keypair = sr25519::KeyPair::from_suri(suri)
			.ok_or(crate::Error::KeyPairIsNone)?;
		let message: Vec<u8> = msg.from_hex()
			.map_err(|e| crate::Error::FromHex(e))?;
		let signature: Vec<u8> = signature.from_hex()
			.map_err(|e| crate::Error::FromHex(e))?;
		keypair.verify_signature(&message, &signature)
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeyDecryptDataRef
	fn decrypt_data_ref(
		data: &str,
		password: String
	) -> crate::Result<i64> {
		let password = Protected::new(password.into_bytes());
		let crypto: Crypto = serde_json::from_str(data)?;
		let decrypted = crypto.decrypt(&password)
			.map_err(|e| crate::Error::Ethsign(e))?;
		Ok(Box::into_raw(Box::new(String::from_utf8(decrypted).ok())) as i64)
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeyDestroyDataRef
	fn destroy_data_ref(data_ref: i64) -> () {
		unsafe { Box::from_raw(data_ref as *mut String) };
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeyBrainwalletSignWithRef
	fn ethkey_brainwallet_sign_with_ref(
		seed_ref: i64,
		message: &str
	) -> crate::Result<String> {
		let seed = unsafe { Box::from_raw(seed_ref as *mut String) };
		let (_, keypair) = KeyPair::from_auto_phrase(&seed);
		let message: Vec<u8> = message.from_hex()
			.map_err(|e| crate::Error::FromHex(e))?;
		let signature = keypair.sign(&message)
			.map_err(|e| crate::Error::Ethsign(e))?;
		// so that the reference remains valid
		let _ = Box::into_raw(seed) as i64;
		Ok(signature.to_hex())
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeySubstrateBrainwalletSignWithRef
	fn substrate_brainwallet_sign_with_ref(
		seed_ref: i64,
		suri_suffix: &str,
		message: &str
	) -> crate::Result<String> {
		let seed = unsafe { Box::from_raw(seed_ref  as *mut String) };
		let suri = format!("{}{}", &seed, suri_suffix);
		let keypair = sr25519::KeyPair::from_suri(&suri)
			.ok_or(crate::Error::KeyPairIsNone)?;
		let message: Vec<u8> = message.from_hex()
			.map_err(|e| crate::Error::FromHex(e))?;
		let signature = keypair.sign(&message);
		// so that the reference remains valid
		let _ = Box::into_raw(seed) as i64;
		Ok(signature.to_hex())
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeySubstrateWalletAddressWithRef
	fn substrate_address_with_ref(
		seed_ref: i64,
		suri_suffix: &str,
		prefix: u8
	) -> crate::Result<String> {
		let seed = unsafe { Box::from_raw(seed_ref  as *mut String) };
		let suri = format!("{}{}", &seed, suri_suffix);
		let keypair = sr25519::KeyPair::from_suri(&suri)
			.ok_or(crate::Error::KeyPairIsNone)?;
		// so that the reference remains valid
		let _ = Box::into_raw(seed) as i64;
		Ok(keypair.ss58_address(prefix))
	}

	@Java_io_parity_signer_EthkeyBridge_ethkeyBrainWalletAddressWithRef
	fn brain_wallet_address_with_ref(
		seed_ref: i64
	) -> crate::Result<String> {
		let seed = unsafe { Box::from_raw(seed_ref  as *mut String) };
		let address = ethkey_brainwallet_address(&seed);
		// so that the reference remains valid
		let _ = Box::into_raw(seed) as i64;
		Ok(address)
	}
}

ffi_support::define_string_destructor!(signer_destroy_string);

#[cfg(test)]
mod tests {
	use super::*;

	static SEED_PHRASE: &str =
		"grant jaguar wish bench exact find voice habit tank pony state salmon";
	static SURI_SUFFIX: &str = "//hard/soft/0";
	static SURI_SUFFIX_HARD: &str = "//hard";
	static ENCRYPTED_SEED: &str = "{\"cipher\":\"aes-128-ctr\",\"cipherparams\":{\"iv\":\"47b4b75d13045ff7569da858e234f7ea\"},\"ciphertext\":\"ca1cf5387822b70392c4aeec729676f91ab00a795d7593fb7e52ecc333dbc4a1acbedc744b5d8d519c714e194bd741995244c8128bfdce6c184d6bda4ca136ed265eedcee9\",\"kdf\":\"pbkdf2\",\"kdfparams\":{\"c\":10240,\"dklen\":32,\"prf\":\"hmac-sha256\",\"salt\":\"b4a2d1edd1a70fe2eb48d7aff15c19e234f6aa211f5142dddb05a59af12b3381\"},\"mac\":\"b38a54eb382f2aa1a8be2f7b86fe040fe112d0f42fea03fac186dccdd7ae3eb9\"}";
	static PIN: &str = "000000";
	static SUBSTRATE_ADDRESS: &str = "5D4kaJXj5HVoBw2tFFsDj56BjZdPhXKxgGxZuKk4K3bKqHZ6";
	static ETHEREUM_ADDRESS: &str = "bip39:f85f35e47e976650641ecd1a644e33edccc9cab1";

	#[test]
	fn test_random_phrase() {
		let result_12 = random_phrase(12);
		assert_eq!(12, result_12.split_whitespace().count());
		let result_24 = random_phrase(24);
		assert_eq!(24, result_24.split_whitespace().count());
		let result_17 = random_phrase(17);
		assert_eq!(24, result_17.split_whitespace().count());
	}

	#[test]
	fn test_blake() {
		let data = "454545454545454545454545454545454545454545454545454545454545454501\
                    000000000000002481853da20b9f4322f34650fea5f240dcbfb266d02db94bfa01\
                    53c31f4a29dbdbf025dd4a69a6f4ee6e1577b251b655097e298b692cb34c18d318\
                    2cac3de0dc00000000";
		let expected = "1025e5db74fdaf4d2818822dccf0e1604ae9ccc62f26cecfde23448ff0248abf";
		let result = blake(data);

		assert_eq!(expected.to_string(), result.unwrap());
	}

	#[test]
	fn test_rlp_item() {
		let rlp = "f85f800182520894095e7baea6a6c7c4c2dfeb977efac326af552d870a801ba048b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353a0efffd310ac743f371de3b9f7f9cb56c0b28ad43601b4ab949f53faa07bd2c804";
		assert_eq!(rlp_item(rlp, 0).unwrap(), "".to_owned());
		assert_eq!(rlp_item(rlp, 1).unwrap(), "01".to_owned());
		assert_eq!(rlp_item(rlp, 2).unwrap(), "5208".to_owned());
		assert_eq!(
			rlp_item(rlp, 3).unwrap(),
			"095e7baea6a6c7c4c2dfeb977efac326af552d87".to_owned()
		);
		assert_eq!(rlp_item(rlp, 4).unwrap(), "0a".to_owned());
		assert_eq!(rlp_item(rlp, 5).unwrap(), "".to_owned());
	}

	#[test]
	fn test_substrate_brainwallet_address() {
		// Secret seed: 0xb139e4050f80172b44957ef9d1755ef5c96c296d63b8a2b50025bf477bd95224
		// Public key (hex): 0x944eeb240615f4a94f673f240a256584ba178e22dd7b67503a753968e2f95761
		let expected = "5FRAPSnpgmnXAnmPVv68fT6o7ntTvaZmkTED8jDttnXs9k4n";
		let generated = substrate_brainwallet_address(SEED_PHRASE, 42).unwrap();

		assert_eq!(expected, generated);
	}

	#[test]
	fn test_substrate_secret() {
		let data_pointer = decrypt_data_ref(ENCRYPTED_SEED, String::from(PIN)).unwrap();
		let suri = format!("{}{}", SEED_PHRASE, SURI_SUFFIX_HARD);
		let expected = "0c4a1f0e772497883ba79c484dfed441008c38572769ab40260a959127949665";
		let generated = substrate_mini_secret_key(&suri).unwrap();
		assert_eq!(expected, generated);
		let passworded_suri = format!("{}///password", SURI_SUFFIX_HARD);
		let generated_passworded_secret= substrate_mini_secret_key_with_ref(data_pointer, &passworded_suri).unwrap();
		let expected_passworded_secret = "057687d479e550b1c0caca121db7e7519c573ebb6a7ce6f771213e41900181f6";
		assert_eq!(expected_passworded_secret, generated_passworded_secret);
	}

	#[test]
	fn test_substrate_secret_with_ref() {
		let data_pointer = decrypt_data_ref(ENCRYPTED_SEED, String::from(PIN)).unwrap();
		let expected = "0c4a1f0e772497883ba79c484dfed441008c38572769ab40260a959127949665";
		let generated = substrate_mini_secret_key_with_ref(data_pointer, SURI_SUFFIX_HARD).unwrap();
		assert_eq!(expected, generated);
	}

	#[test]
	fn test_substrate_brainwallet_address_suri() {
		let suri = format!("{}{}", SEED_PHRASE, SURI_SUFFIX);
		let generated = substrate_brainwallet_address(&suri, 42).unwrap();

		assert_eq!(SUBSTRATE_ADDRESS, generated);
	}

	#[test]
	fn test_substrate_sign() {
		let msg: String = b"Build The Future".to_hex();
		let signature = substrate_brainwallet_sign(SEED_PHRASE, &msg).unwrap();

		let is_valid = schnorrkel_verify(SEED_PHRASE, &msg, &signature).unwrap();

		assert!(is_valid);
	}

	#[test]
	fn test_substrate_sign_with_ref() {
		let msg: String = b"Build The Future".to_hex();
		let data_pointer = decrypt_data_ref(ENCRYPTED_SEED, String::from(PIN)).unwrap();
		let signature_by_ref =
			substrate_brainwallet_sign_with_ref(data_pointer, SURI_SUFFIX, &msg).unwrap();
		let suri = format!("{}{}", SEED_PHRASE, SURI_SUFFIX);
		let is_valid = schnorrkel_verify(&suri, &msg, &signature_by_ref).unwrap();
		destroy_data_ref(data_pointer);
		assert!(is_valid);
	}

	#[test]
	fn decrypt_with_ref() {
		let decrypted_result = decrypt_data(ENCRYPTED_SEED, String::from(PIN)).unwrap();
		assert_eq!(SEED_PHRASE, decrypted_result);
	}

	#[test]
	fn test_generate_substrate_address() {
		let data_pointer = decrypt_data_ref(ENCRYPTED_SEED, String::from(PIN)).unwrap();
		let address = substrate_address_with_ref(data_pointer, SURI_SUFFIX, 42).unwrap();
		destroy_data_ref(data_pointer);
		assert_eq!(address, SUBSTRATE_ADDRESS);
	}

	#[test]
	fn test_generate_ethereum_address() {
		let data_pointer = decrypt_data_ref(ENCRYPTED_SEED, String::from(PIN)).unwrap();
		let address = brain_wallet_address_with_ref(data_pointer).unwrap();
		destroy_data_ref(data_pointer);
		assert_eq!(address, ETHEREUM_ADDRESS);
	}
}
