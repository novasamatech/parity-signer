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

'use strict';
import '@polkadot/types/injector';

import {
	createType,
	GenericExtrinsicPayload,
	GenericCall,
	Metadata
} from '@polkadot/types';
import Call from '@polkadot/types/primitive/Generic/Call';
import { u8aConcat } from '@polkadot/util';
import { checkAddress, decodeAddress } from '@polkadot/util-crypto';

import {
	SUBSTRATE_NETWORK_LIST,
	SubstrateNetworkKeys
} from '../../src/constants';
import {
	constructDataFromBytes,
	rawDataToU8A,
	asciiToHex,
	hexToAscii,
	decodeToString,
	isJsonString
} from '../../src/util/decoders';
import { isAscii } from '../../src/util/strings';
import kusamaData from '../../src/util/static-kusama';

const SUBSTRATE_ID = new Uint8Array([0x53]);
const CRYPTO_SR25519 = new Uint8Array([0x01]);
const CMD_SIGN_MORTAL = new Uint8Array([0]);
const CMD_SIGN_MSG = new Uint8Array([3]);

const KUSAMA_ADDRESS = 'FF42iLDmp7JLeySMjwWWtYQqfycJvsJFBYrySoMvtGfvAGs';
const TEST_MESSAGE = 'THIS IS SPARTA!';

const metadata = new Metadata(kusamaData);
GenericCall.injectMetadata(metadata);

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

/* eslint-disable prettier/prettier */
const SIGN_MSG_TEST = new Uint8Array([
    0,  0,  1,  0,  0,
    83,   1,   3, 118,   2, 230, 253,  72, 157,  97,
    235,  53, 198,  82,  55, 106, 143, 113, 176, 252,
    203, 114,  24, 152, 116, 223,  74, 190, 250, 136,
    232, 158, 164,   7, 118,  84,  72,  73,  83,  32,
    73,  83,  32,  83,  80,  65,  82,  84,  65,  33
  ]);
/* eslint-enable prettier/prettier */

const SIGNER_PAYLOAD_TEST = {
	address: KUSAMA_ADDRESS,
	blockHash:
		'0xde8f69eeb5e065e18c6950ff708d7e551f68dc9bf59a07c52367c0280f805ec7',
	blockNumber: '0x231d30',
	era: createType('ExtrinsicEra', { current: 2301232, period: 200 }),
	genesisHash: SubstrateNetworkKeys.KUSAMA,
	method:
		'0x0600ffd7568e5f0a7eda67a82691ff379ac4bba4f9c9b859fe779b5d46363b61ad2db9e56c',
	nonce: 0x1234,
	specVersion: 123,
	tip: 0x5678
};

const SIGN_TX_TEST = u8aConcat(
	new Uint8Array([0, 0, 1, 0, 0]),
	SUBSTRATE_ID,
	CRYPTO_SR25519,
	CMD_SIGN_MORTAL,
	decodeAddress(KUSAMA_ADDRESS),
	new GenericExtrinsicPayload(SIGNER_PAYLOAD_TEST, { version: 4 }).toU8a()
);

describe.skip('sanity check', () => {
	it('sanity check address is kusama', () => {
		expect(checkAddress(KUSAMA_ADDRESS, 2)).toEqual([true, null]);
	});

	it('sanity check payload encodes as expected', () => {
		const payload = new GenericExtrinsicPayload(SIGNER_PAYLOAD_TEST, {
			version: 4
		});
		const fromBytes = new GenericExtrinsicPayload(payload.toU8a(), {
			version: 4
		});

		expect(payload).toMatchObject(fromBytes);
		expect(payload.genesisHash.toHex()).toEqual(SubstrateNetworkKeys.KUSAMA);
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
			const frameInfo = strippedU8a.slice(0, 5);
			const uos = strippedU8a.slice(5);

			expect(frameInfo).toEqual(new Uint8Array([0, 0, 1, 0, 0]));
			expect(uos[0]).toEqual(SUBSTRATE_ID[0]);
			expect(uos[1]).toEqual(CRYPTO_SR25519[0]);
			expect(uos[2]).toEqual(CMD_SIGN_MSG[0]);
		});
	});

	describe('UOS parsing', () => {
		// after stripping
		it('from Substrate UOS message', async () => {
			const unsignedData = await constructDataFromBytes(SIGN_MSG_TEST);

			expect(unsignedData).toBeDefined();
			expect(unsignedData.data.crypto).toEqual('sr25519');
			expect(unsignedData.data.data).toEqual('THIS IS SPARTA!');
			expect(unsignedData.data.account).toEqual(KUSAMA_ADDRESS);
		});

		it('from Substrate UOS Payload Mortal', async () => {
			const unsignedData = await constructDataFromBytes(SIGN_TX_TEST);

			expect(unsignedData.data.data.era.toHex()).toEqual(
				SIGNER_PAYLOAD_TEST.era.toHex()
			);
			expect(unsignedData.data.data.method.toHex()).toEqual(
				SIGNER_PAYLOAD_TEST.method
			);
			expect(unsignedData.data.data.blockHash.toHex()).toEqual(
				SIGNER_PAYLOAD_TEST.blockHash
			);
			expect(unsignedData.data.data.nonce.eq(SIGNER_PAYLOAD_TEST.nonce)).toBe(
				true
			);
			expect(
				unsignedData.data.data.specVersion.eq(SIGNER_PAYLOAD_TEST.specVersion)
			).toBe(true);
			expect(unsignedData.data.data.tip.eq(SIGNER_PAYLOAD_TEST.tip)).toBe(true);
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
			const payload = new GenericExtrinsicPayload(SIGNER_PAYLOAD_TEST, {
				version: 4
			});

			const call = new Call(payload.method);

			expect(call).toBeDefined();
			expect(call.args).toBeDefined();
			expect(call.meta).toBeDefined();
		});
	});
});
