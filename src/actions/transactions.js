'use strict'

import { NEW_SCANNED_TX, SIGNED_TX } from '../constants/TransactionActions'

export function scannedTx (rlp, transaction) {
  return {
    type: NEW_SCANNED_TX,
    rlp,
    transaction
  }
}

export function signedTx (signature) {
  return {
    type: SIGNED_TX,
    signature: signature
  }
}
