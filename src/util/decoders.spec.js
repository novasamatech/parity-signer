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

import { createType, GenericExtrinsicPayload } from '@polkadot/types';
import { hexToU8a, u8aConcat, u8aToU8a, u8aToHex, u8aToString } from '@polkadot/util';
import { checkAddress, decodeAddress, encodeAddress } from '@polkadot/util-crypto';
import { constructDataFromBytes, rawDataToU8A } from './decoders';

const SUBSTRATE_ID = new Uint8Array([0x53]);
const CRYPTO_SR25519 = new Uint8Array([0x01]);
const CMD_SIGN_TX = new Uint8Array([0]);
const CMD_SIGN_TX_HASH = new Uint8Array([1]);
const CMD_SIGN_IMMORTAL_TX = new Uint8Array([2]);
const CMD_SIGN_MSG = new Uint8Array([3]);

const KUSAMA_ADDRESS = 'FF42iLDmp7JLeySMjwWWtYQqfycJvsJFBYrySoMvtGfvAGs';
const TEST_MESSAGE = 'THIS IS SPARTA!';

const RN_TX_REQUEST_RAW_DATA = 
  '4' + // indicates data is binary encoded
  '37' +  // length of data
  '00' + // is it multipart?
  '0001' + // how many parts in total?
  '0000' +  // which frame are we on?
  '53' + // S for Substrate
  '01' + // sr25519
  '03' + // sign message
  '7602e6fd489d61eb35c652376a8f71b0fccb72189874df4abefa88e89ea40776' + // key
  '5448495320495320535041525441210' + // THIS IS SPARTA!
  'ec11ec11ec11ec';

const SIGN_MSG_TEST = new Uint8Array([
    0,  0,  1,  0,  0,
    83,   1,   3, 118,   2, 230, 253,  72, 157,  97,
    235,  53, 198,  82,  55, 106, 143, 113, 176, 252,
    203, 114,  24, 152, 116, 223,  74, 190, 250, 136,
    232, 158, 164,   7, 118,  84,  72,  73,  83,  32,
    73,  83,  32,  83,  80,  65,  82,  84,  65,  33
  ]);

const SIGNER_PAYLOAD_TEST = {
  method: '0x0500ffd7568e5f0a7eda67a82691ff379ac4bba4f9c9b859fe779b5d46363b61ad2db9e56c',
  era: '0x0703',
  nonce: '0x00001234',
  tip: '0x00000000000000000000000000005678',
  genesisHash: '0xdcd1346701ca8396496e52aa2785b1748deb6db09551b72159dcb3e08991025b',
  blockHash: '0xde8f69eeb5e065e18c6950ff708d7e551f68dc9bf59a07c52367c0280f805ec7'
};

const SIGN_TX_TEST = u8aConcat(
  new Uint8Array([0, 0, 1, 0, 0]),
  SUBSTRATE_ID,
  CRYPTO_SR25519,
  CMD_SIGN_TX,
  decodeAddress(KUSAMA_ADDRESS),
  createType('ExtrinsicPayload', SIGNER_PAYLOAD_TEST, { version: 3 }).toU8a()
);

// const RN_MULTIPART_TX_REQUEST_RAW_DATA_PT_1 = 
//   '4' + // indicates data is binary encoded
//   '37' +  // byte length of data
//   '01' + // is it multipart?
//   '0002' + // how many parts in total?
//   '0001' +  // which frame are we on?
//   '53' + // S for Substrate
//   '01' + // sr25519
//   '03' + // sign message
//   '7602e6fd489d61eb35c652376a8f71b0fccb72189874df4abefa88e89ea40776' + // key
//   '5448495320495320535041525441210' + // THIS IS SPARTA!
//   'ec11ec11ec11ec';

// const RN_MULTIPART_TX_REQUEST_RAW_DATA_PT_2 = 
//   '4' + // indicates data is binary encoded
//   '37' +  // byte length of data
//   '01' + // is it multipart?
//   '0002' + // how many parts in total?
//   '0002' +  // which frame are we on?
//   '53' + // S for Substrate
//   '01' + // sr25519
//   '03' + // sign message
//   '7602e6fd489d61eb35c652376a8f71b0fccb72189874df4abefa88e89ea407' + // key
//   '686520736169642c20746f206e6f206f6e6520696e20706172746963756c61722e' + // he said, to no one in particular.
//   'ec11ec11ec11ec';

// const RN_SIGN_HASH_RAW = 
//   '00' +
//   '0001' +
//   '0000' +
//   '53' +
//   '01' + // sr25519
//   '01' + // sign payload hash
//   '7602e6fd489d61eb35c652376a8f71b0fccb72189874df4abefa88e89ea407' + // key
//   ''

describe.skip('sanity check', () => {
  it('sanity check address is kusama', () => {
    expect(checkAddress(KUSAMA_ADDRESS, 2)).toEqual([true, null]);
  });
});

describe('decoders', () => {
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
  })

  describe('UOS parsing', () => {
    // after stripping
    it('from Substrate UOS message', () => {
      const unsignedData = constructDataFromBytes(SIGN_MSG_TEST);

      expect(unsignedData).toBeDefined();
      expect(unsignedData.data.crypto).toEqual('sr25519');
      expect(unsignedData.data.data).toEqual('THIS IS SPARTA!');
      expect(unsignedData.data.account).toEqual(KUSAMA_ADDRESS);
    });

    it.only('from Substrate UOS Payload', () => {
      const unsignedData = constructDataFromBytes(SIGN_TX_TEST);

      expect(unsignedData.data.data.era.toHex()).toEqual(SIGNER_PAYLOAD_TEST.era);
      expect(unsignedData.data.data.method.toHex()).toEqual(SIGNER_PAYLOAD_TEST.method);
      expect(unsignedData.data.data.blockHash.toHex()).toEqual(SIGNER_PAYLOAD_TEST.blockHash);
      expect(unsignedData.data.data.nonce.eq(SIGNER_PAYLOAD_TEST.nonce)).toBe(true);
      expect(unsignedData.data.data.tip.eq(SIGNER_PAYLOAD_TEST.tip)).toBe(true);
    });

    // it('from Substrate UOS Multipart Message', () => {
    //   const partData1 = parseRawData(RN_MULTIPART_TX_REQUEST_RAW_DATA_PT_1);
      
    //   expect(partData1).toBeDefined();
    //   expect(partData1.isMultipart).toEqual(true);
    //   expect(partData1.frameCount).toEqual(2);
    //   expect(partData1.currentFrame).toEqual(1);
    // });
  })
});
