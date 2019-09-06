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

import { NetworkProtocols, NETWORK_LIST, SubstrateNetworkKeys } from '../constants';

export function accountId({
  address,
  networkKey
}) {
  if (typeof address !== 'string' || address.length === 0 || !networkKey || !NETWORK_LIST[networkKey]) {
    throw new Error(`Couldn't create an accountId. Address or networkKey missing, or network key was invalid.`);
  }

  const { ethereumChainId='', protocol, genesisHash='' } = NETWORK_LIST[networkKey];

  if (protocol === NetworkProtocols.SUBSTRATE) {
    return `${protocol}:${address}:${genesisHash}`;
  } else {
    return `${protocol}:0x${address.toLowerCase()}@${ethereumChainId}`;
  }
}

export function empty(address = '', networkKey = SubstrateNetworkKeys.KUSAMA) {
  return {
    address: address,
    createdAt: new Date().getTime(),
    derivationPassword: '',
    derivationPath:'',
    encryptedSeed: null,
    name: '',
    networkKey: networkKey,
    seed: '',
    seedPhrase: '',
    updatedAt: new Date().getTime(),
    validBip39Seed: false,
  };
}

export function validateSeed(seed, validBip39Seed) {
  if (seed.length === 0) {
    return {
      accountRecoveryAllowed: false,
      reason: `A seed phrase is required.`,
      valid: false
    };
  }

  const words = seed.split(' ');

  for (let word of words) {
    if (word === '') {
      return {
        accountRecoveryAllowed: true,
        reason: `Extra whitespace found.`,
        valid: false
      };
    }
  }

  if (!validBip39Seed) {
    return {
      accountRecoveryAllowed: true,
      reason: `This recovery phrase will be treated as a legacy Parity brain wallet.`,
      valid: false
    };
  }

  return {
    reason: null,
    valid: true
  };
}
