// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

'use strict'

import {
  ADD_ACCOUNT, SELECT_ACCOUNT, DELETE_ACCOUNT, MODIFY_ACCOUNT, SET_NEW_PIN, SET_OLD_PIN, SET_ACCOUNTS
} from '../constants/AccountActions'
// TODO [ToDr] Move side-effects to middleware
import { saveAccount, deleteAccount } from '../util/db'

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
      let account = action.account
      delete account.seed
      delete account.newPin

      saveAccount(account)

      return {
        ...state,
        all: [
          ...state.all,
          account
        ]
      }

    case SELECT_ACCOUNT:
      return {
        ...state,
        selected: action.account
      }

    case SET_OLD_PIN:
      return {
        ...state,
        selected: {
          ...state.selected,
          oldPin: action.pin
        }
      }

    case SET_NEW_PIN:
      return {
        ...state,
        selected: {
          ...state.selected,
          newPin: action.pin
        }
      }

    case DELETE_ACCOUNT:
      deleteAccount(action.account)

      return {
        ...state,
        all: state.all.filter(account => action.account !== account)
      }

    case MODIFY_ACCOUNT:
      if (state.selected === action.account) {
        Object.assign(state.selected, action.modifications)
        delete state.selected.newPin
      }

      return {
        ...state,
        all: state.all.map(account => {
          if (account.address === action.account.address) {
            account = Object.assign({}, account, action.modifications)
            saveAccount(account)
            return account
          }

          return account
        })
      }

    case SET_ACCOUNTS:
      return {
        ...state,
        all: action.accounts
      }

    default:
      return state
  }
}
