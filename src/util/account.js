import { SubstrateNetworkKeys, NetworkProtocols } from '../constants';

export function accountId({
  address,
  protocol = NetworkProtocols.SUBSTRATE,
  networkKey = SubstrateNetworkKeys.FRONTIER
}) {
  if (typeof address !== 'string' || address.length === 0) {
    throw new Error(`Couldn't create an accountId, address missing`);
  }
  return `${protocol}_${networkKey}_${address.toLowerCase()}`;
}

export function empty(account = {}) {
  return {
    name: '',
    protocol: NetworkProtocols.SUBSTRATE,
    networkKey: SubstrateNetworkKeys.KUSAMA,
    seed: '',
    // address for an empty seed phrase
    address: '',
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
