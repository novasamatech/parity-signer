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

import { EthkeyBridge } from 'NativeModules'

const asString = (x) => x.split('').map(x => x.charCodeAt(0).toString(16)).join('')

export const brainWalletAddress = (seed) => EthkeyBridge.brainWalletAddress(seed)
  .then(address => keccak(asString(address)).then(hash => ({address, hash: hash})))
  .then(acc => {
    let result = ''
    for (let n = 0; n < 40; n++) {
      result = `${result}${parseInt(acc.hash[n], 16) > 7 ? acc.address[n].toUpperCase() : acc.address[n]}`
    }
    return result
  })
export const brainWalletSecret = (seed) => EthkeyBridge.brainWalletSecret(seed)
export const brainWalletSign = (seed, message) => EthkeyBridge.brainWalletSign(seed, message)
export const rlpItem = (rlp, position) => EthkeyBridge.rlpItem(rlp, position)
export const keccak = (data) => EthkeyBridge.keccak(data)
export const blockiesIcon = (seed) => EthkeyBridge.blockiesIcon(seed.toLowerCase()).then(icon => 'data:image/png;base64,' + icon)
