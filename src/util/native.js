import { EthkeyBridge } from 'NativeModules'

export const brainWalletAddress = (seed) => EthkeyBridge.brainWalletAddress(seed)
export const brainWalletSecret = (seed) => EthkeyBridge.brainWalletSecret(seed)
export const brainWalletSign = (seed, message) => EthkeyBridge.brainWalletSign(seed, message)
export const rlpItem = (rlp, position) => EthkeyBridge.rlpItem(rlp, position)
export const keccak = (data) => EthkeyBridge.keccak(data)
export const blockiesIcon = (seed) => EthkeyBridge.blockiesIcon(seed)
