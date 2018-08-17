import { NETWORK_TYPE, NETWORK_ID } from '../constants';
import WORDS from '../../res/wordlist.json';

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
  const set = new Set();
  for (let word of words) {
    if (typeof WORDS_INDEX[word] === 'undefined') {
      if (set.has(word)) {
        return {
          valid: false,
          reason: `A duplicated word "${word}" found. Words in seed phrase must be unique.`
        };
      }
      if (word === '') {
        return {
          valid: false,
          reason: `Extra whitespace found`
        };
      }
      return {
        valid: false,
        reason: `The word "${word}" is not from the word list`
      }
    }
    set.add(word);
  }
  if (set.size < 11) {
    return {
      valid: false,
      reason: `Add ${11 - set.size} more unique word(s) to compose a secure seed phrase`
    }
  }
  return {
    valid: true,
    reason: null
  };
}
