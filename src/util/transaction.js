import { rlpItem } from './native'

class Transaction {
  constructor(nonce, gasPrice, gas, action, value, data) {
    this.nonce = nonce
    this.gasPrice = gasPrice
    this.gas = gas
    this.action = action
    this.value = value
    this.data = data
  }
}

export default function transactionFromRlp(rlp, callback) {

}
