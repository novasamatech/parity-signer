import { EthkeyBridge } from 'NativeModules'

export const brainWalletAddress = (seed) => EthkeyBridge.brainWalletAddress(seed)
  .then(address => keccak(address).then(hash => ({address, hash: hash.slice(-40)})))
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
