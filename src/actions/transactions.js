'use strict'

import { Actions } from 'react-native-router-flux'
import { NEW_SCANNED_TX, SIGN_TX } from '../constants/TransactionActions'
import { brainWalletSign } from '../util/native'

export function scannedTx (rlpHash, transaction) {
  return {
    type: NEW_SCANNED_TX,
    rlpHash,
    transaction
  }
}

export function signTx (account) {
  return function (dispatch, getState) {
    let hash = getState().transactions.pendingTransaction.rlpHash
    return brainWalletSign(account.seed, hash).then(
      signature => {
        dispatch({
          type: SIGN_TX,
          signature: signature
        })
        Actions.qrViewTx()
      },
      error => console.log(error)
    )
  }
}
