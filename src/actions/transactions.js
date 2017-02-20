'use strict'

import { NEW_SCANNED_TX, SIGNED_TX } from '../constants/TransactionActions'

export function newScannedTx(data) {
  return {
    type: NEW_SCANNED_TX,
    data: data,
  }
}

export function signedTx(data) {
  return {
    type: SIGNED_TX,
    data: data,
  }
}
