import WORDS from '../../res/wordlist.json';
import { NETWORK_ID, NETWORK_TYPE } from '../constants';

export { WORDS };
export const WORDS_INDEX = WORDS.reduce(
  (res, w) => Object.assign(res, { [w]: 1 }),
  {}
);

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
    networkType: NETWORK_TYPE.ethereum,
    chainId: NETWORK_ID.frontier,
    seed: '',
    // address for an empty seed phrase
    address: '00a329c0648769A73afAc7F9381E08FB43dBEA72',
    createdAt: new Date().getTime(),
    updatedAt: new Date().getTime(),
    archived: false,
    encryptedSeed: null,
    ...account
  };
}

export function validateSeed(seed) {
  if (seed.length === 0) {
    return {
      valid: false,
      reason: `You're trying to recover from an empty seed phrase`
    };
  }
  const words = seed.split(' ');

  if (words.length < 11) {
    return {
      valid: false,
      reason: `Add ${11 - words.length} more unique word(s) to compose a secure seed phrase`
    }
  }
  return {
    valid: true,
    reason: null
  };
}
