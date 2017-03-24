'use strict'

import { ADD_ACCOUNT, SELECT_ACCOUNT, DELETE_ACCOUNT, SET_PIN, SET_ACCOUNTS } from '../constants/AccountActions'

// format of the account
// {
// address: 'bF35fAA9C265bAf50C9CFF8c389C363B05753275',
// name: 'Test: Wallet',
// seed: '123',
// pin: ''
// }

// all accounts are loaded on init from db
const initialAccounts = {
  all: [],
  selected: {}
}

export default function accounts (state = initialAccounts, action) {
  switch (action.type) {
    case ADD_ACCOUNT:
      return Object.assign({}, state, {
        all: [
          ...state.all,
          action.account
        ]
      })

    case SELECT_ACCOUNT:
      return Object.assign({}, state, {
        selected: action.account
      })

    case DELETE_ACCOUNT:
      return Object.assign({}, state, {
        all: state.all.filter((account) => { return action.account !== account })
      })

    case SET_PIN:
      return Object.assign({}, state, {
        selected: Object.assign({}, state.selected, {
          pin: action.pin
        })
      })

    case SET_ACCOUNTS:
      return Object.assign({}, state, {
        all: action.accounts
      })

    default:
      return state
  }
}
