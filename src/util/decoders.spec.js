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
  '43' + 
  '7' + 
  '0000010000' + 
  '53' + 
  '01' +
  '03' +
  '7602e6fd489d61eb35c652376a8f71b0fccb72189874df4abefa88e89ea407' +
  '765448495320495320535041525441210' +
  'ec11ec11ec11ec';
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
  it('should properly extract only UOS relevant data from RNCamera txRequest.rawData', () => {
    const strippedU8a = rawDataToU8A(RN_TX_REQUEST_RAW_DATA);
    const frameInfo = strippedU8a.slice(0, 5);
    const uos = strippedU8a.slice(5);

    // console.log('frame info ', frameInfo);
    // console.log('uos => ', uos);

    expect(frameInfo).toEqual(new Uint8Array([0, 0, 1, 0, 0]));
    expect(uos[0]).toEqual(SUBSTRATE_ID[0]);
    expect(uos[1]).toEqual(CRYPTO_SR25519[0]);
    expect(uos[2]).toEqual(CMD_SIGN_MSG[0]);
  });

  it('should properly construct data from Substrate UOS message', () => {
    const unsignedData = parseRawData(RN_TX_REQUEST_RAW_DATA);

    expect(unsignedData).toBeDefined();
    expect(unsignedData.data.crypto).toEqual('sr25519');
    expect(unsignedData.data.data).toEqual('THIS IS SPARTA!');
    expect(unsignedData.data.account).toEqual(KUSAMA_ADDRESS);
  });
});
