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

import { checkAddress, decodeAddress } from '@polkadot/util-crypto';
import {
	createType,
	GenericExtrinsicPayload,
	TypeRegistry
} from '@polkadot/types';
import { Metadata } from '@polkadot/metadata';
import { GenericCall as Call } from '@polkadot/types/generic';
import { hexToU8a, u8aConcat } from '@polkadot/util';
import { describe, expect, it } from '@jest/globals';

import { getOverrideTypes } from 'stores/NetworkContext';
import {
	SUBSTRATE_NETWORK_LIST,
	SubstrateNetworkKeys
} from 'constants/networkSpecs';
import { SubstrateCompletedParsedData } from 'types/scannerTypes';
import {
	constructDataFromBytes,
	rawDataToU8A,
	asciiToHex,
	hexToAscii,
	decodeToString,
	isJsonString
} from 'utils/decoders';
import { isAscii } from 'utils/strings';
import { defaultMetadata } from 'constants/networkMetadataList';

const SUBSTRATE_ID = new Uint8Array([0x53]);
const CRYPTO_SR25519 = new Uint8Array([0x01]);
const CMD_SIGN_MORTAL = new Uint8Array([0]);
const CMD_SIGN_MSG = new Uint8Array([3]);
const registry = new TypeRegistry();
registry.setMetadata(new Metadata(registry, defaultMetadata));

const KUSAMA_ADDRESS = 'FF42iLDmp7JLeySMjwWWtYQqfycJvsJFBYrySoMvtGfvAGs';
const TEST_MESSAGE = 'THIS IS SPARTA!';

const RN_TX_REQUEST_RAW_DATA =
	'4' + // indicates data is binary encoded
	'37' + // byte length of data
	'00' + // is it multipart?
	'0001' + // how many parts in total?
	'0000' + // which frame are we on?
	'53' + // S for Substrate
	'01' + // sr25519
	'03' + // sign message
	'7602e6fd489d61eb35c652376a8f71b0fccb72189874df4abefa88e89ea40776' + // key
	'5448495320495320535041525441210' + // THIS IS SPARTA!
	'ec11ec11ec11ec';

const SIGNER_PAYLOAD_TEST = {
	address: KUSAMA_ADDRESS,
	blockHash:
		'0xde8f69eeb5e065e18c6950ff708d7e551f68dc9bf59a07c52367c0280f805ec7',
	blockNumber: '0x231d30',
	era: createType(registry, 'ExtrinsicEra', { current: 2301232, period: 200 }),
	genesisHash: SubstrateNetworkKeys.KUSAMA,
	method:
		'0x0600008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a4813f6ffffffffff3f0102',
	nonce: 0x1234,
	specVersion: 123,
	tip: 0x5678,
	transactionVersion: 234
};

const SIGN_TX_TEST = u8aConcat(
	new Uint8Array([0, 0, 1, 0, 0]),
	SUBSTRATE_ID,
	CRYPTO_SR25519,
	CMD_SIGN_MORTAL,
	decodeAddress(KUSAMA_ADDRESS),
	new GenericExtrinsicPayload(registry, SIGNER_PAYLOAD_TEST, {
		version: 4
	}).toU8a(),
	new Uint8Array(hexToU8a(SubstrateNetworkKeys.KUSAMA))
);

describe('sanity check', () => {
	it('sanity check address is kusama', () => {
		expect(checkAddress(KUSAMA_ADDRESS, 2)).toEqual([true, null]);
	});

	it('sanity check payload encodes as expected', () => {
		const payload = new GenericExtrinsicPayload(registry, SIGNER_PAYLOAD_TEST, {
			version: 4
		});
		// const fromBytes = new GenericExtrinsicPayload(registry, payload.toU8a(), {
		// 	version: 4
		// });
		// expect(payload).toMatchObject(fromBytes);
		expect(payload.genesisHash.toHex()).toEqual(SubstrateNetworkKeys.KUSAMA);
	});
});

describe('type registry should get override types', () => {
	it('get network latest override types', () => {
		const testRegistry = new TypeRegistry();
		const westendOverrideTypes = getOverrideTypes(testRegistry, 'westend');
		expect(westendOverrideTypes).not.toEqual({});
	});
});

describe('decoders', () => {
	describe('strings', () => {
		it('check if string is ascii', () => {
			const message = 'hello world';
			const numbers = 123;

			expect(isAscii(message)).toBe(true);
			expect(isAscii(numbers)).toBe(true);
		});

		it('converts ascii to hex', () => {
			const message = 'hello world';
			const messageHex = asciiToHex(message);

			expect(hexToAscii(messageHex)).toBe(message);
		});

		it('converts bytes to ascii', () => {
			/* eslint-disable-next-line prettier/prettier */
      const messageBytes = new Uint8Array([84,  72,  73,  83,  32, 73,  83,  32,  83,  80,  65,  82,  84,  65,  33]);
			const message = decodeToString(messageBytes);

			expect(message).toBeDefined();
			expect(message).toBe(TEST_MESSAGE);
		});

		it('checks if string is JSON parseable', () => {
			const jsonString = JSON.stringify({ a: 1, b: 2 });
			const notJsonString = '0x90u23jaiof';
			const validExample = isJsonString(jsonString);
			const inValidExample = isJsonString(notJsonString);

			expect(validExample).toBe(true);
			expect(inValidExample).toBe(false);
		});
	});

	describe('rawDataToU8a', () => {
		it('should properly extract only UOS relevant data from RNCamera txRequest.rawData', () => {
			const strippedU8a = rawDataToU8A(RN_TX_REQUEST_RAW_DATA);
			expect(strippedU8a).not.toBeNull();
			const frameInfo = strippedU8a!.slice(0, 5);
			const uos = strippedU8a!.slice(5);

			expect(frameInfo).toEqual(new Uint8Array([0, 0, 1, 0, 0]));
			expect(uos[0]).toEqual(SUBSTRATE_ID[0]);
			expect(uos[1]).toEqual(CRYPTO_SR25519[0]);
			expect(uos[2]).toEqual(CMD_SIGN_MSG[0]);
		});

		it('works for extrinsic of kusama transferring', () => {
			// prettier-ignore
			const receiveSigner = [0, 0, 1, 0, 0, 83, 1, 2, 90, 74, 3, 248, 74, 25, 207, 142, 189, 164, 14, 98, 53, 140, 89, 40, 112, 105, 26, 156, 244, 86, 19, 139, 180, 130, 153, 105, 209, 15, 233, 105, 160, 4, 0, 34, 89, 2, 152, 77, 89, 94, 72, 235, 188, 163, 222, 48, 73, 75, 190, 61, 85, 240, 76, 223, 82, 83, 185, 206, 135, 220, 108, 253, 109, 101, 100, 7, 0, 228, 11, 84, 2, 117, 3, 4, 0, 31, 4, 0, 0, 176, 168, 212, 147, 40, 92, 45, 247, 50, 144, 223, 183, 230, 31, 135, 15, 23, 180, 24, 1, 25, 122, 20, 156, 169, 54, 84, 73, 158, 163, 218, 254, 249, 14, 158, 218, 236, 196, 15, 137, 75, 114, 19, 61, 247, 7, 46, 106, 185, 128, 128, 172, 127, 21, 50, 149, 7, 47, 66, 149, 129, 126, 115, 107];
			const rawData =
				'49900000100005301025a4a03f84a19cf8ebda40e62358c592870691a9cf456138bb4829969d10fe969a00400225902984d595e48ebbca3de30494bbe3d55f04cdf5253b9ce87dc6cfd6d65640700e40b5402750304001f040000b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafef90e9edaecc40f894b72133df7072e6ab98080ac7f153295072f4295817e736b0ec11ec11ec11ec11ec11ec11ec11ec11ec11ec11ec11ec11ec11ec';
			const strippedU8a = rawDataToU8A(rawData);
			expect([].slice.call(strippedU8a)).toEqual(receiveSigner);
		});
	});

	describe('UOS parsing', () => {
		it('from Substrate UOS Payload Mortal', async () => {
			const networks = new Map(Object.entries(SUBSTRATE_NETWORK_LIST));
			const unsignedData = await constructDataFromBytes(
				SIGN_TX_TEST,
				false,
				networks
			);
			const rawPayload = (unsignedData as SubstrateCompletedParsedData).data
				.data;

			const payload = registry.createType('ExtrinsicPayload', rawPayload, {
				version: 4
			});

			expect(payload.era.toHex()).toEqual(SIGNER_PAYLOAD_TEST.era.toHex());
			expect(payload.method.toHex()).toEqual(SIGNER_PAYLOAD_TEST.method);
			expect(payload.blockHash.toHex()).toEqual(SIGNER_PAYLOAD_TEST.blockHash);
			expect(payload.nonce.eq(SIGNER_PAYLOAD_TEST.nonce)).toBe(true);
			expect(payload.specVersion.eq(SIGNER_PAYLOAD_TEST.specVersion)).toBe(
				true
			);
			expect(payload.tip.eq(SIGNER_PAYLOAD_TEST.tip)).toBe(true);
		});
	});

	describe('Type injection from metadata', () => {
		it('can fetch the prefix matching to a hash', () => {
			const kusamaPrefix =
				SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.KUSAMA].prefix;
			// const substratePrefix = SUBSTRATE_NETWORK_LIST[SubstrateNetworkKeys.SUBSTRATE_DEV].prefix;

			expect(kusamaPrefix).toBe(2);
			// expect(substrate).toBe(42);
		});

		it('decodes Payload Method to something human readable with Kusama metadata', () => {
			const payload = new GenericExtrinsicPayload(
				registry,
				SIGNER_PAYLOAD_TEST,
				{
					version: 4
				}
			);

			const call = new Call(registry, payload.method);

			expect(call).toBeDefined();
			expect(call.args).toBeDefined();
			expect(call.meta).toBeDefined();
		});
	});
});
