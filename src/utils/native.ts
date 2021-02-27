// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Modifications Copyright (c) 2021 Thibaut Sardan

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

import { NativeModules } from 'react-native';
import { TryBrainWalletAddress } from 'utils/seedRefHooks';

import { checksummedAddress } from './checksum';

const { EthkeyBridge } = NativeModules || {};

interface AddressObject {
	address: string;
	bip39: boolean;
}

export function keccak(data: string): Promise<string> {
	return EthkeyBridge.keccak(data);
}

/**
 * Turn an address string tagged with either 'legacy:' or 'bip39:' prefix
 * to an object, marking if it was generated with BIP39.
 */
function untagAddress(address: string): AddressObject {
	let bip39 = false;

	const colonIdx = address.indexOf(':');

	if (colonIdx !== -1) {
		bip39 = address.substring(0, colonIdx) === 'bip39';
		address = address.substring(colonIdx + 1);
	}

	return {
		address,
		bip39
	};
}

function toHex(x: string): string {
	return x
		.split('')
		.map(c => c.charCodeAt(0).toString(16))
		.map(n => (n.length < 2 ? `0${n}` : n))
		.join('');
}

export async function brainWalletAddress(seed: string): Promise<AddressObject> {
	const taggedAddress = await EthkeyBridge.brainWalletAddress(seed);
	const { address, bip39 } = untagAddress(taggedAddress);
	const hash = await keccak(toHex(address));

	return {
		address: checksummedAddress(address, hash),
		bip39
	};
}

export async function brainWalletAddressWithRef(createBrainWalletFn: TryBrainWalletAddress): Promise<AddressObject> {
	const taggedAddress = await createBrainWalletFn();
	const { address, bip39 } = untagAddress(taggedAddress);
	const hash = await keccak(toHex(address));

	return {
		address: checksummedAddress(address, hash),
		bip39
	};
}

export async function brainWalletBIP39Address(seed: string): Promise<AddressObject | null> {
	try {
		const taggedAddress = await EthkeyBridge.brainWalletBIP(seed);
		const { address, bip39 } = untagAddress(taggedAddress);

		const hash = await keccak(toHex(address));

		return {
			address: checksummedAddress(address, hash),
			bip39
		};
	} catch (_) {
		return null;
	}
}

export function brainWalletSign(seed: string,
	message: string): Promise<string> {
	return EthkeyBridge.brainWalletSign(seed, message);
}

export function rlpItem(rlp: string, position: number): Promise<string> {
	return EthkeyBridge.rlpItem(rlp, position);
}

export function ethSign(data: string): Promise<string> {
	return EthkeyBridge.ethSign(data);
}

export function blockiesIcon(seed: string): Promise<string> {
	return EthkeyBridge.blockiesIcon(seed.toLowerCase());
}

export function words(wordsNumber: number): Promise<string> {
	return EthkeyBridge.randomPhrase(wordsNumber);
}

export function encryptData(data: string, password: string): Promise<string> {
	return EthkeyBridge.encryptData(data, password);
}

export function decryptData(data: string, password: string): Promise<string> {
	return EthkeyBridge.decryptData(data, password);
}

// Creates a QR code for the UTF-8 representation of a string
export function qrCode(data: string): Promise<string> {
	return EthkeyBridge.qrCode(data);
}

// Creates a QR code for binary data from a hex-encoded string
export function qrCodeHex(data: string): Promise<string> {
	return EthkeyBridge.qrCodeHex(data);
}

export function blake2b(data: string): Promise<string> {
	return EthkeyBridge.blake2b(data);
}

export function substrateSecret(suri: string): Promise<string> {
	return EthkeyBridge.substrateSecret(suri);
}

// Get an SS58 encoded address for a sr25519 account from a BIP39 phrase and a prefix.
// Prefix is a number used in the SS58 encoding:
//
//   Polkadot proper = 0
//   Kusama = 2
//   Default (testnets) = 42
export function substrateAddress(seed: string,
	prefix: number): Promise<string> {
	return EthkeyBridge.substrateAddress(seed, prefix);
}

// Sign data using sr25519 crypto for a BIP39 phrase. Message is hex-encoded byte array.
export function substrateSign(seed: string, message: string): Promise<string> {
	return EthkeyBridge.substrateSign(seed, message);
}

// Verify a sr25519 signature is valid
export function schnorrkelVerify(seed: string,
	message: string,
	signature: string): Promise<boolean> {
	return EthkeyBridge.schnorrkelVerify(seed, message, signature);
}

export class SeedRefClass {
	private dataRef: number;
	private valid: boolean;

	constructor() {
		this.dataRef = 0;
		this.valid = false;
	}

	isValid(): boolean {
		return this.valid;
	}

	// Decrypt a seed and store the reference. Must be called before signing.
	async tryCreate(encryptedSeed: string, password: string): Promise<number> {
		if (this.valid) {
			// Seed reference was already created.
			return this.dataRef;
		}

		const dataRef: number = await EthkeyBridge.decryptDataRef(encryptedSeed,
			password);

		this.dataRef = dataRef;
		this.valid = true;

		return this.dataRef;
	}

	trySubstrateAddress(suriSuffix: string, prefix: number): Promise<string> {
		if (!this.valid) {
			throw new Error('a seed reference has not been created');
		}

		return EthkeyBridge.substrateAddressWithRef(this.dataRef,
			suriSuffix,
			prefix);
	}

	tryBrainWalletAddress(): Promise<string> {
		if (!this.valid) {
			throw new Error('a seed reference has not been created');
		}

		return EthkeyBridge.brainWalletAddressWithRef(this.dataRef).then((address: string) => {
			return address;
		});
	}

	// Destroy the decrypted seed. Must be called before this leaves scope or
	// memory will leak.
	tryDestroy(): Promise<void> {
		if (!this.valid) {
			// Seed reference was never created or was already destroyed.
			throw new Error('cannot destroy an invalid seed reference');
		}

		return EthkeyBridge.destroyDataRef(this.dataRef).then(() => {
			this.valid = false;
		});
	}

	// Use the seed reference to sign a message. Will throw an error if
	// `tryDestroy` has already been called or if `tryCreate` failed.
	tryBrainWalletSign(message: string): Promise<string> {
		if (!this.valid) {
			// Seed reference was never created or was already destroyed.
			throw new Error('cannot sign with an invalid seed reference');
		}

		return EthkeyBridge.brainWalletSignWithRef(this.dataRef, message);
	}

	// Use a reference returned by decryptDataRef to sign a message
	trySubstrateSign(suriSuffix: string, message: string): Promise<string> {
		if (!this.valid) {
			// Seed reference was never created or was already destroyed.
			throw new Error('cannot sign with an invalid seed reference');
		}

		return EthkeyBridge.substrateSignWithRef(this.dataRef, suriSuffix, message);
	}

	trySubstrateSecret(suriSuffix: string): Promise<string> {
		if (!this.valid) {
			// Seed reference was never created or was already destroyed.
			throw new Error('cannot sign with an invalid seed reference');
		}

		return EthkeyBridge.substrateSecretWithRef(this.dataRef, suriSuffix);
	}
}
