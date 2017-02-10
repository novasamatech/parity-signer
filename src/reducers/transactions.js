import { NEW_SCANNED_TX } from '../constants/TransactionActions'

const initialState = {}

export default function transactions(state = initialState, { type, data }) {
  switch (type) {
      case NEW_SCANNED_TX:
        return Object.assign({}, state, {
          pendingTx: data,
        })
      default:
        return state
  }
}
