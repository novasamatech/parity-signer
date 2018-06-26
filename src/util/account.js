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

export function phraseSplit(phrase) {
  return Array.isArray(phrase) ? phrase : phrase.split(' ');
}

export function phraseJoin(phrase) {
  return Array.isArray(phrase) ? phrase.join(' ') : phrase;
}
