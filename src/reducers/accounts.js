'use strict'

import { ADD_ACCOUNT, SELECT_ACCOUNT, DELETE_ACCOUNT, SET_PIN } from '../constants/AccountActions'

const initialAccounts = {
  all: [{
    address: 'bF35fAA9C265bAf50C9CFF8c389C363B05753275',
    name: 'Test: Wallet',
    pin: '',
  }, {
    address: '4EECf99D543B278106ac0c0e8ffe616F2137f10a',
    name: 'Test: LockMyEther',
    pin: '',
  }],
  selected: {},
}

export default function accounts(state = initialAccounts, action) {
  switch (action.type) {
      case ADD_ACCOUNT:
        return Object.assign({}, state, {
          all: [
            ...state.all,
            action.account,
          ]
        })

    case SELECT_ACCOUNT:
      return Object.assign({}, state, {
        selected: action.account,
      })

    case DELETE_ACCOUNT:
      return Object.assign({}, state, {
        all: state.all.filter((account) => { return action.account != account })
      })

    case SET_PIN:
      return Object.assign({}, state, {
        selected: Object.assign({}, state.selected, {
          pin: action.pin
        })
      })

    default:
      return state
  }
}
