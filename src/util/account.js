import { EthereumNetworkIds, NetworkTypes } from '../constants';

export function accountId({
  address,
  networkType = 'ethereum',
  chainId = '1'
}) {
  if (typeof address !== 'string' || address.length === 0) {
    throw new Error(`Couldn't create an accountId, missing address`);
  }
  return `${networkType}:0x${address.toLowerCase()}@${chainId}`;
}

export function empty(account = {}) {
  return {
    name: '',
    networkType: NetworkTypes.ETHEREUM,
    chainId: EthereumNetworkIds.FRONTIER,
    seed: '',
    // address for an empty seed phrase
    address: '00a329c0648769A73afAc7F9381E08FB43dBEA72',
    createdAt: new Date().getTime(),
    updatedAt: new Date().getTime(),
    archived: false,
    encryptedSeed: null,
    validBip39Seed: false,
    ...account
  };
}

export function validateSeed(seed, validBip39Seed) {
  if (seed.length === 0) {
    return {
      valid: false,
      reason: `You're trying to recover from an empty seed phrase`
    };
  }
  const words = seed.split(' ');

  for (let word of words) {
    if (word === '') {
      return {
        valid: false,
        reason: `Extra whitespace found`
      };
    }
  }

  if (!validBip39Seed) {
    return {
      valid: false,
      reason: `This recovery phrase will be treated as a legacy Parity brain wallet`
    };
  }

  return {
    valid: true,
    reason: null
  };
}
