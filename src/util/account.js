import { NetworkProtocols, NETWORK_LIST, SubstrateNetworkKeys } from '../constants';

export function accountId({
  address,
  networkKey
}) {

  const { ethereumChainId, protocol, genesisHash } = NETWORK_LIST[networkKey];

  if (typeof address !== 'string' || address.length === 0 || !networkKey) {
    throw new Error(`Couldn't create an accountId, address or networkKey missing`);
  }

  if (protocol === NetworkProtocols.SUBSTRATE){ 
    return `${protocol}:${address}:${genesisHash}`;
  } else {
    return `${protocol}:${address.toLowerCase()}@${ethereumChainId}`;
  }
}

export function empty(account = {}) {
  return {
    address: '',
    archived: false,
    createdAt: new Date().getTime(),
    derivationPassword: '', 
    derivationPath:'',
    encryptedSeed: null,
    name: '',
    networkKey: SubstrateNetworkKeys.KUSAMA,
    seed: '',
    seedPhrase: '',
    updatedAt: new Date().getTime(),
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
