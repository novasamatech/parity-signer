export function accountId ({ address, networkType, networkId }) {
  if (typeof address !== 'string' || address.length === 0) {
    throw new Error(`Couldn't create an accountId, missing address`);
  }
  if (typeof networkType !== 'string' || networkType.length === 0) {
    throw new Error(`Couldn't create an accountId, missing networkType`);
  }
  if (typeof networkId !== 'string' || networkId.length === 0) {
    throw new Error(`Couldn't create an accountId, missing networkId`);
  }
  return `${networkType}:0x${address}@${networkId}`
}
