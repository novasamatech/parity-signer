// Copyright 2015-2017 Parity Technologies (UK) Ltd.
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

const asString = x =>
  x
    .split('')
    .map(x => x.charCodeAt(0).toString(16))
    .join('');

export const brainWalletAddress = seed =>
  EthkeyBridge.brainWalletAddress(seed).then(address =>
    keccak(asString(address)).then(hash => checksummedAddress(address, hash))
  );
export const brainWalletSecret = seed => EthkeyBridge.brainWalletSecret(seed);
export const brainWalletSign = (seed, message) =>
  EthkeyBridge.brainWalletSign(seed, message);
export const rlpItem = (rlp, position) => EthkeyBridge.rlpItem(rlp, position);
export const keccak = data => EthkeyBridge.keccak(data);
export const ethSign = data => EthkeyBridge.ethSign(data);
export const blockiesIcon = seed =>
  EthkeyBridge.blockiesIcon(seed.toLowerCase()).then(
    icon => 'data:image/png;base64,' + icon
  );
export const words = () => EthkeyBridge.randomPhrase(11);
export const encryptData = (data, password) =>
  EthkeyBridge.encryptData(data, password);
export const decryptData = (data, password) =>
  EthkeyBridge.decryptData(data, password);
