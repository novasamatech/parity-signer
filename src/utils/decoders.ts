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

import { TypeRegistry } from '@polkadot/types';
import {
	hexStripPrefix,
	hexToU8a,
	u8aToHex,
	u8aToString
} from '@polkadot/util';
import { encodeAddress } from '@polkadot/util-crypto';

import { blake2b } from './native';

import { ExtrinsicPayloadLatestVersion } from 'constants/chainData';
import {
	SUBSTRATE_NETWORK_LIST,
	SubstrateNetworkKeys
} from 'constants/networkSpecs';
import {
	EthereumParsedData,
	ParsedData,
	SubstrateCompletedParsedData,
	SubstrateMultiParsedData
} from 'types/scannerTypes';

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
  0 // terminator
  --- SQRC Filler Bytes
  ec11ec11ec11ec // SQRC filler bytes
  */

const registry = new TypeRegistry();

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

export async function constructDataFromBytes(
	bytes: Uint8Array,
	multipartComplete = false
): Promise<ParsedData> {
	const frameInfo = hexStripPrefix(u8aToHex(bytes.slice(0, 5)));
	const frameCount = parseInt(frameInfo.substr(2, 4), 16);
	const isMultipart = frameCount > 1; // for simplicity, even single frame payloads are marked as multipart.
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
					firstByte === '00' || firstByte === '01'
						? 'signData'
						: firstByte === '01'
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
						firstByte === '00'
							? 'ed25519'
							: firstByte === '01'
							? 'sr25519'
							: null;
					const pubKeyHex = uosAfterFrames.substr(6, 64);
					const publicKeyAsBytes = hexToU8a('0x' + pubKeyHex);
					const hexEncodedData = '0x' + uosAfterFrames.slice(70);
					const rawPayload = hexToU8a(hexEncodedData);

					const isOversized = rawPayload.length > 256;
					const defaultPrefix =
						SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.KUSAMA].prefix;
					let extrinsicPayload;
					let network;

					switch (secondByte) {
						case '00': // sign mortal extrinsic
						case '02': // sign immortal extrinsic
							extrinsicPayload = registry.createType(
								'ExtrinsicPayload',
								rawPayload,
								{
									version: ExtrinsicPayloadLatestVersion
								}
							);
							data.action = isOversized ? 'signData' : 'signTransaction';
							data.oversized = isOversized;
							data.isHash = isOversized;
							data.data.data = isOversized
								? await blake2b(
										u8aToHex(extrinsicPayload.toU8a(true), -1, false)
								  )
								: extrinsicPayload;

							// while we are signing a hash, we still have the ability to know what the signing payload is, so we should get that information into the store.
							data.preHash = extrinsicPayload;

							network =
								SUBSTRATE_NETWORK_LIST[extrinsicPayload.genesisHash.toHex()];

							if (!network) {
								throw new Error(
									`Signer does not currently support a chain with genesis hash: ${extrinsicPayload.genesisHash.toHex()}`
								);
							}

							data.data.account = encodeAddress(
								publicKeyAsBytes,
								network.prefix
							); // encode to the prefix;

							break;
						case '01': // data is a hash
							data.action = 'signData';
							data.oversized = false;
							data.isHash = true;
							data.data.data = rawPayload;
							data.data.account = encodeAddress(
								publicKeyAsBytes,
								defaultPrefix
							); // default to Kusama
							break;
						case '03': // Cold Signer should attempt to decode message to utf8
							data.action = 'signData';
							if (isOversized) {
								data.data.data = await blake2b(u8aToHex(rawPayload, -1, false));
								data.isHash = isOversized;
								data.oversized = isOversized;
							} else {
								data.data.data = u8aToString(rawPayload);
							}

							data.data.account = encodeAddress(
								publicKeyAsBytes,
								defaultPrefix
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
