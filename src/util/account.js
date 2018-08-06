import { NETWORK_TYPE, NETWORK_ID } from '../constants';

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
