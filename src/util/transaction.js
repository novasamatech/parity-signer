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

import { rlpItem } from './native'
import { fromWei } from './units'

class Transaction {
  constructor (nonce, gasPrice, gas, action, value, data) {
    this.nonce = nonce || '0'
    this.gasPrice = parseInt(gasPrice, 16).toString()
    this.gas = parseInt(gas, 16).toString()
    this.action = action
    this.value = fromWei(value)
    this.data = data || '-'
    this.isSafe = true
  }
}

async function asyncTransaction (rlp, resolve, reject) {
  try {
    let nonce = await rlpItem(rlp, 0)
    let gasPrice = await rlpItem(rlp, 1)
    let gas = await rlpItem(rlp, 2)
    let action = await rlpItem(rlp, 3)
    let value = await rlpItem(rlp, 4)
    let data = await rlpItem(rlp, 5)
    let tx = new Transaction(nonce, gasPrice, gas, action, value, data)
    resolve(tx)
  } catch (e) {
    reject(e)
  }
}

export default function transaction (rlp) {
  return new Promise((resolve, reject) => asyncTransaction(rlp, resolve, reject))
}
