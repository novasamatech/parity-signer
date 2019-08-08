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

// @flow

/*
  Example Full Raw Data
  ---
  4 // indicates binary
  37 // indicates data length
  0000
  0100
  00
  --- UOS Specific Data
  53 // indicates payload is for Substrate
  01 // crypto: sr25519
  00 // indicates action: signData
  f4cd755672a8f9542ca9da4fbf2182e79135d94304002e6a09ffc96fef6e6c4c // public key
  544849532049532053504152544121 // actual payload to sign (should be SCALE or utf8)
  0 // terminator
  --- SQRC Filler Bytes
  ec11ec11ec11ec // SQRC filler bytes
  */
export function rawDataToU8A(rawData) {
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

export function parseRawData(rawData) {
  const bytes = rawDataToU8A(rawData);
  const hex = bytes.map(byte => byte.toString(16));
  const uosAfterFrames = hex.slice(5); // FIXME handle multipart

  const zerothByte = uosAfterFrames[0];
  const firstByte = uosAfterFrames[1];
  const secondByte = uosAfterFrames[2];
  let action;
  let address;
  let data = {};
  data['data'] = {}; // for consistency with legacy data format.

  try {
    // decode payload appropriately via UOS
    switch (zerothByte) {
      case 45: // Ethereum UOS payload
        action = firstByte === 0 || firstByte === 2 ? 'signData' : firstByte === 1 ? 'signTransaction' : null;
        address = uosAfterFrames.slice(2, 22);

        data['action'] = action;
        data['data']['account'] = account;

        if (action === 'signData') {
          data['data']['rlp'] = uosAfterFrames[13];
        } else if (action === 'signTransaction') {
          data['data']['data'] = rawAfterFrames[13];
        } else {
          throw new Error('Could not determine action type.');
        }
        break;
      case 53: // Substrate UOS payload
        const crypto = firstByte === 0 ? 'ed25519' : firstByte === 1 ? 'sr25519' : null;
        const publicKeyAsBytes = uosAfterFrames.slice(3, 35);
        console.log('raw => ', rawData);
        console.log('uosafterframes => ', uosAfterFrames);

        const ss58Encoded = encodeAddress(publicKeyAsBytes, 2); // encode to kusama
        debugger;
        const hexEncodedData: Uint8Array = uosAfterFrames.slice(35);

        data['data']['crypto'] = crypto;
        data['data']['account'] = ss58Encoded;

        debugger;

        switch(secondByte) {
          case 0:
            data['action'] = 'signTransaction';
            if (encryptedData.length > 256) {
              data['oversized'] = true; // flag and warn that we are signing the hash because payload was too big.
              data['isHash'] = true; // flag and warn that signing a hash is inherently dangerous
              data['data']['data'] = blake2s(hexEncodedData);
            } else {
              data['isHash'] = false;
              data['data']['data'] = Payload(hexEncodedData);
            }
            break;
          case 1:
            data['action'] = 'signTransaction';
            data['isHash'] = true;
            data['data']['data'] = hexEncodedData; // data is a hash
            break;
          case 2:
            data['action'] = 'signTransaction';
            data['isHash'] = false;
            data['data']['data'] = Payload(hexEncodedData);
            break;
          case 3: // Cold Signer should attempt to decode message to utf8
            data['action'] = 'signData';
            data['isHash'] = false;
            data['data']['data'] = decodeToString(hexEncodedData.map(b => parseInt(b, 16)));
            break;
          default:
            break;
        }
        break;
      default:
        throw new Error('we cannot handle the payload: ', rawData);
    }

    return data;
  } catch (e) {
    throw new Error('we cannot handle the payload: ', rawData);
  }
}

export function decodeToString(message: Uint8Array): string {
  const decoder = new TextDecoder('utf8');

  return decoder.decode(message);
}

export function hexToAscii(hexBytes: Uint8Array): string {
	var hex  = hexBytes.toString();
	var str = '';
	for (var n = 0; n < hex.length; n += 2) {
		str += String.fromCharCode(parseInt(hex.substr(n, 2), 16));
	}

	return str;
 }