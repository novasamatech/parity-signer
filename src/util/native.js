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

import { EthkeyBridge } from 'NativeModules';
import { checksummedAddress } from './checksum';

/**
 * Turn an address string tagged with either 'legacy:' or 'bip39:' prefix
 * to an object, marking if it was generated with BIP39.
 */
function untagAddress(address) {
  let bip39 = false;

  const colonIdx = address.indexOf(':');

  if (colonIdx !== -1) {
    bip39 = address.substring(0, colonIdx) === 'bip39';
    address = address.substring(colonIdx + 1);
  }

  return {
    bip39,
    address,
  };
}

function asString (x) {
  return x
    .split('')
    .map(x => x.charCodeAt(0).toString(16))
    .map(n => n.length < 2 ? `0${n}` : n)
    .join('');
}

export async function brainWalletAddress (seed) {
  const taggedAddress = await EthkeyBridge.brainWalletAddress(seed);
  const { bip39, address } = untagAddress(taggedAddress);
  const hash = await keccak(asString(address));

  return {
    bip39,
    address: checksummedAddress(address, hash),
  };
}

export function brainWalletBIP39Address (seed) {
  return EthkeyBridge
    .brainWalletBIP(seed)
    .then(async (taggedAddress) => {
      const { bip39, address } = untagAddress(taggedAddress);

      const hash = await keccak(asString(address));

      return {
        bip39,
        address: checksummedAddress(address, hash),
      };
    })
    .catch((_) => {
      return null;
    });
}

export function brainWalletSign (seed, message) {
  return EthkeyBridge.brainWalletSign(seed, message);
}

export function rlpItem (rlp, position) {
  return EthkeyBridge.rlpItem(rlp, position);
}

export function keccak (data) {
  return EthkeyBridge.keccak(asString(data));
}

export function ethSign (data) {
  return EthkeyBridge.ethSign(data);
}

export function blockiesIcon (seed) {
  return EthkeyBridge.blockiesIcon(seed.toLowerCase());
}

export function words () {
  return EthkeyBridge.randomPhrase();
}

export function encryptData (data, password) {
  return EthkeyBridge.encryptData(data, password);
}

export function decryptData (data, password) {
  return EthkeyBridge.decryptData(data, password);
}

// Creates a QR code for the UTF-8 representation of a string
export function qrCode (data) {
  return EthkeyBridge.qrCode(data);
}

// Creates a QR code for binary data from a hex-encoded string
export function qrCodeHex (data) {
  return EthkeyBridge.qrCodeHex(data);
}

export function blake2s (data) {
  return EthkeyBridge.blake2s(asString(data));
}

// Get an SS58 encoded address for a sr25519 account from a BIP39 phrase and a prefix.
// Prefix is a number used in the SS58 encoding:
//
//   Polkadot proper = 0
//   Kusama = 2
//   Default (testnets) = 42
export function substrateAddress (seed, prefix) {
  return EthkeyBridge.substrateAddress(seed, prefix);
}

// Sign data using sr25519 crypto for a BIP39 phrase. Message is hex-encoded byte array.
export function substrateSign (seed, message) {
  return EthkeyBridge.substrateSign(seed, message);
}
