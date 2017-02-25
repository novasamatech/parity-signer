'use strict'

import { NEW_SCANNED_TX, SIGNED_TX } from '../constants/TransactionActions'

const initialState = {
  pendingTransaction: {
    transaction: {},
    rlp: '',
  },
  signedTransaction: {
    signature: '',
  },
}

export default function transactions(state = initialState, action) {
  switch (action.type) {
      case NEW_SCANNED_TX:
        return Object.assign({}, state, {
          pendingTransaction: {
            rlp: action.rlp,
            transaction: action.transaction,
          }
        })
      case SIGNED_TX:
        return Object.assign({}, state, {
          signedTransaction: action.signature,
        })
      default:
        return state
  }
}
