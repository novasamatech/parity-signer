// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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

import {
	compactFromU8a,
	hexStripPrefix,
	hexToU8a,
	u8aToHex
} from '@polkadot/util';
import { encodeAddress } from '@polkadot/util-crypto';

import strings from 'modules/sign/strings';
import { SubstrateNetworkParams } from 'types/networkTypes';
import {
	EthereumParsedData,
	ParsedData,
	SubstrateCompletedParsedData,
	SubstrateMultiParsedData
} from 'types/scannerTypes';
import { blake2b } from 'utils/native';

// from https://github.com/maciejhirsz/uos#substrate-payload
const CRYPTO_ED25519 = '00';
const CRYPTO_SR25519 = '01';
const CMD_SIGN_MORTAL = '00';
const CMD_SIGN_HASH = '01';
const CMD_SIGN_IMMORTAL = '02';
const CMD_SIGN_MSG = '03';

/*
 * @return strippedData: the rawBytes from react-native-camera, stripped of the ec11 padding to fill the frame size. See: decoders.js
 * N.B. Substrate oversized/multipart payloads will already be hashed at this point.
 */
export function rawDataToU8A(rawData: string): Uint8Array | null {
	if (!rawData) {
		return null;
	}

	// Strip filler bytes padding at the end
	if (rawData.substr(-2) === 'ec') {
		rawData = rawData.substr(0, rawData.length - 2);
	}

	while (rawData.substr(-4) === 'ec11') {
		rawData = rawData.substr(0, rawData.length - 4);
	}

	// Verify that the QR encoding is binary and it's ending with a proper terminator
	if (rawData.substr(0, 1) !== '4' || rawData.substr(-1) !== '0') {
		return null;
	}

	// Strip the encoding indicator and terminator for ease of reading
	rawData = rawData.substr(1, rawData.length - 2);

	const length8 = parseInt(rawData.substr(0, 2), 16) || 0;
	const length16 = parseInt(rawData.substr(0, 4), 16) || 0;
	let length = 0;

	// Strip length prefix
	if (length8 * 2 + 2 === rawData.length) {
		rawData = rawData.substr(2);
		length = length8;
	} else if (length16 * 2 + 4 === rawData.length) {
		rawData = rawData.substr(4);
		length = length16;
	} else {
		return null;
	}

	const bytes = new Uint8Array(length);

	for (let i = 0; i < length; i++) {
		bytes[i] = parseInt(rawData.substr(i * 2, 2), 16);
	}

	return bytes;
}

/*
  Example Full Raw Data
  ---
  4 // indicates binary
  37 // indicates data length
  --- UOS Specific Data
  00 + // is it multipart?
  0001 + // how many parts in total?
  0000 +  // which frame are we on?
  53 // indicates payload is for Substrate
  01 // crypto: sr25519
  00 // indicates action: signData
  f4cd755672a8f9542ca9da4fbf2182e79135d94304002e6a09ffc96fef6e6c4c // public key
  544849532049532053504152544121 // actual payload to sign (should be SCALE or utf8)
  91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3 // genesis hash
  0 // terminator
  --- SQRC Filler Bytes
  ec11ec11ec11ec // SQRC filler bytes
  */

export async function constructDataFromBytes(
	bytes: Uint8Array,
	multipartComplete = false,
	networks: Map<string, SubstrateNetworkParams>
): Promise<ParsedData> {
	const frameInfo = hexStripPrefix(u8aToHex(bytes.slice(0, 5)));
	const frameCount = parseInt(frameInfo.substr(2, 4), 16);
	const isMultipart = frameCount > 1; // for simplicity, even single frame payloads are marked as multipart.
	if (frameCount > 50) throw new Error(strings.ERROR_WRONG_RAW);
	const currentFrame = parseInt(frameInfo.substr(6, 4), 16);
	const uosAfterFrames = hexStripPrefix(u8aToHex(bytes.slice(5)));

	// UOS after frames can be metadata json
	if (isMultipart && !multipartComplete) {
		const partData: SubstrateMultiParsedData = {
			currentFrame,
			frameCount,
			isMultipart,
			partData: uosAfterFrames
		};
		return partData;
	}

	const zerothByte = uosAfterFrames.substr(0, 2);
	const firstByte = uosAfterFrames.substr(2, 2);
	const secondByte = uosAfterFrames.substr(4, 2);

	let action;

	try {
		// decode payload appropriately via UOS
		switch (zerothByte) {
			case '45': {
				// Ethereum UOS payload
				const data = {
					data: {} // for consistency with legacy data format.
				} as EthereumParsedData;
				action =
					firstByte === CRYPTO_ED25519 || firstByte === CRYPTO_SR25519
						? 'signData'
						: firstByte === CRYPTO_SR25519
						? 'signTransaction'
						: null;
				const address = uosAfterFrames.substr(4, 44);

				data.action = action;
				data.data.account = address;
				if (action === 'signData') {
					data.data.rlp = uosAfterFrames[13];
				} else if (action === 'signTransaction') {
					data.data.data = uosAfterFrames[13];
				} else {
					throw new Error('Could not determine action type.');
				}
				return data;
			}
			case '53': {
				// Substrate UOS payload
				const data = {
					data: {} // for consistency with legacy data format.
				} as SubstrateCompletedParsedData;
				try {
					data.data.crypto =
						firstByte === CRYPTO_ED25519
							? 'ed25519'
							: firstByte === CRYPTO_SR25519
							? 'sr25519'
							: null;
					const pubKeyHex = uosAfterFrames.substr(6, 64);
					const publicKeyAsBytes = hexToU8a('0x' + pubKeyHex);
					const hexEncodedData = '0x' + uosAfterFrames.slice(70);
					const hexPayload = hexEncodedData.slice(0, -64);
					const genesisHash = `0x${hexEncodedData.substr(-64)}`;
					const rawPayload = hexToU8a(hexPayload);
					data.data.genesisHash = genesisHash;
					const isOversized = rawPayload.length > 256;
					const network = networks.get(genesisHash);
					if (!network) {
						throw new Error(strings.ERROR_NO_NETWORK);
					}

					switch (secondByte) {
						case CMD_SIGN_MORTAL:
						case CMD_SIGN_IMMORTAL:
							data.action = isOversized ? 'signData' : 'signTransaction';
							data.oversized = isOversized;
							data.isHash = isOversized;
							const [offset] = compactFromU8a(rawPayload);
							const payload = rawPayload.subarray(offset);
							data.data.data = isOversized
								? await blake2b(u8aToHex(payload, -1, false))
								: rawPayload;
							data.data.account = encodeAddress(
								publicKeyAsBytes,
								network.prefix
							); // encode to the prefix;

							break;
						case CMD_SIGN_HASH:
						case CMD_SIGN_MSG:
							data.action = 'signData';
							data.oversized = false;
							data.isHash = secondByte === CMD_SIGN_HASH;
							data.data.data = hexPayload;
							data.data.account = encodeAddress(
								publicKeyAsBytes,
								network.prefix
							); // default to Kusama
							break;
						default:
							break;
					}
				} catch (e) {
					throw new Error(
						'Error: something went wrong decoding the Substrate UOS payload: ' +
							uosAfterFrames
					);
				}
				return data;
			}
			default:
				throw new Error('Error: Payload is not formatted correctly: ' + bytes);
		}
	} catch (e) {
		throw new Error('we cannot handle the payload: ' + bytes);
	}
}

export function decodeToString(message: Uint8Array): string {
	const encodedString = String.fromCharCode.apply(null, Array.from(message));
	return decodeURIComponent(escape(encodedString));
}

export function asciiToHex(message: string): string {
	const result = [];
	for (let i = 0; i < message.length; i++) {
		const hex = Number(message.charCodeAt(i)).toString(16);
		result.push(hex);
	}
	return result.join('');
}

export function hexToAscii(hexBytes: string): string {
	const hex = hexBytes.toString();
	let str = '';
	for (let n = 0; n < hex.length; n += 2) {
		str += String.fromCharCode(parseInt(hex.substr(n, 2), 16));
	}

	return str;
}

export function isJsonString(str: any): boolean {
	if (!str) {
		return false;
	}

	try {
		JSON.parse(str);
	} catch (e) {
		return false;
	}
	return true;
}

export function isAddressString(str: string): boolean {
	if (!str) {
		return false;
	}

	return (
		str.substr(0, 2) === '0x' ||
		str.substr(0, 9) === 'ethereum:' ||
		str.substr(0, 10) === 'substrate:'
	);
}

export function encodeNumber(value: number): Uint8Array {
	return new Uint8Array([value >> 8, value & 0xff]);
}
