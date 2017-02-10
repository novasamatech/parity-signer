import { NEW_SCANNED_TX } from '../constants/TransactionActions'

export function newScannedTx(data) {
  return {
    type: NEW_SCANNED_TX,
    data: data,
  }
}
