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

import { hexToU8a, u8aConcat, u8aToU8a, u8aToHex, u8aToString } from '@polkadot/util';
import { checkAddress, decodeAddress, encodeAddress } from '@polkadot/util-crypto';
import { parseRawData, rawDataToU8A } from './decoders';

const SUBSTRATE_ID = new Uint8Array([0x53]);
const CRYPTO_SR25519 = new Uint8Array([0x01]);
const CMD_SIGN_TX = new Uint8Array([0x00]);
const CMD_SIGN_TX_HASH = new Uint8Array([0x01]);
const CMD_SIGN_IMMORTAL_TX = new Uint8Array([0x02]);
const CMD_SIGN_MSG = new Uint8Array([0x03]);

const RN_TX_REQUEST_RAW_DATA = 
  '4' + // indicates data is binary encoded
  '37' +  // byte length of data
  '00' + // is it multipart?
  '0001' + // how many parts in total?
  '0000' +  // which frame are we on?
  '53' + // S for Substrate
  '01' + // sr25519
  '03' + // sign message
  '7602e6fd489d61eb35c652376a8f71b0fccb72189874df4abefa88e89ea40776' + // key
  '5448495320495320535041525441210' + // message
  'ec11ec11ec11ec';

const RN_MULTIPART_TX_REQUEST_RAW_DATA_PT_1 = 
  '4' + // indicates data is binary encoded
  '37' +  // byte length of data
  '01' + // is it multipart?
  '0002' + // how many parts in total?
  '0001' +  // which frame are we on?
  '53' + // S for Substrate
  '01' + // sr25519
  '03' + // sign message
  '7602e6fd489d61eb35c652376a8f71b0fccb72189874df4abefa88e89ea40776' + // key
  '5448495320495320535041525441210' + // THIS IS SPARTA!
  'ec11ec11ec11ec';

const RN_MULTIPART_TX_REQUEST_RAW_DATA_PT_2 = 
  '4' + // indicates data is binary encoded
  '37' +  // byte length of data
  '01' + // is it multipart?
  '0002' + // how many parts in total?
  '0002' +  // which frame are we on?
  '53' + // S for Substrate
  '01' + // sr25519
  '03' + // sign message
  '7602e6fd489d61eb35c652376a8f71b0fccb72189874df4abefa88e89ea407' + // key
  '686520736169642c20746f206e6f206f6e6520696e20706172746963756c61722e' + // he said, to no one in particular.
  'ec11ec11ec11ec';

const RN_SIGN_TX_RAW = 
  '00' +
  '0001' +
  '0000' +
  '53' +
  '01' + // sr25519
  '00' + // sign payload
  '7602e6fd489d61eb35c652376a8f71b0fccb72189874df4abefa88e89ea407' + // key
  '25511822302537215797235531988255106143113176252203114241521162237419025013623215816471182221431052381812241012251401058025511214112685311042201552451547197351031924015128941994829350000073220209521031202131150731108217039133177116141235109176149811833389220179224137145291502552158614295101262181031683814525555154196187164249201184892541191559370545997173451852291082097222689102' + // Signer SCALE Payload
  'ec11ec11ec'

const RN_OVERSIZED_TX_RAW = 

const RN_SIGN_HASH_RAW = 
  '00' +
  '0001' +
  '0000' +
  '53' +
  '01' + // sr25519
  '01' + // sign payload hash
  '7602e6fd489d61eb35c652376a8f71b0fccb72189874df4abefa88e89ea407' + // key
  ''

 
const KUSAMA_ADDRESS = 'FF42iLDmp7JLeySMjwWWtYQqfycJvsJFBYrySoMvtGfvAGs';
const TEST_MESSAGE = 'THIS IS SPARTA!';

const TEST_SUBSTRATE_MSG = u8aToHex(u8aConcat(
    SUBSTRATE_ID,
    CRYPTO_SR25519,
    CMD_SIGN_MSG,
    decodeAddress(KUSAMA_ADDRESS),
    u8aToU8a(TEST_MESSAGE)
  ));

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
    it('from Substrate UOS message', () => {
      const unsignedData = parseRawData(RN_TX_REQUEST_RAW_DATA);

      expect(unsignedData).toBeDefined();
      expect(unsignedData.data.crypto).toEqual('sr25519');
      expect(unsignedData.data.data).toEqual('THIS IS SPARTA!');
      expect(unsignedData.data.account).toEqual(KUSAMA_ADDRESS);
    });

    it.only('from Substrate UOS Payload', () => {
      const unsignedData = parseRawData(RN_SIGN_TX_RAW);

      expect(unsignedData).toBeDefined();
    });

    it('from oversized Substrate UOS Message', () = {
      const unsignedData = parseRawData(RN_OVERSIZED_TX_RAW);


    })

    it('from Substrate UOS Multipart Message', () => {
      const partData1 = parseRawData(RN_MULTIPART_TX_REQUEST_RAW_DATA_PT_1);
      
      expect(partData1).toBeDefined();
      expect(partData1.isMultipart).toEqual(true);
      expect(partData1.frameCount).toEqual(2);
      expect(partData1.currentFrame).toEqual(1);
    });
  })
});
