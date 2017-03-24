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
