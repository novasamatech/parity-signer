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

import { Keyboard } from 'react-native'
import { connect } from 'react-redux'
import AccountPin from '../components/AccountPin'
import { addAccount, setNewPin, setOldPin, changePin } from '../actions/accounts'
import { signHash } from '../actions/signer'

export const AccountEnterPin = connect(
  (state) => ({
    withConfirmation: false
  }),
  (dispatch) => ({
    onNextPressed: (pin) => {
      Keyboard.dismiss()
      dispatch(signHash(pin))
    }
  })
)(AccountPin)

export const AccountChangePin = connect(
  (state) => ({
    placeholder: 'Current PIN',
    withConfirmation: false
  }),
  (dispatch) => ({
    onNextPressed: (pin) => {
      dispatch(setOldPin(pin))
    }
  })
)(AccountPin)

export const AccountSetPin = connect(
  ({accounts}) => {
    const isNew = !accounts.all.find(acc => acc.address === accounts.selected.address)

    return {
      placeholder: isNew ? 'Enter PIN' : 'Enter new PIN'
    }
  },
  (dispatch) => ({
    onNextPressed: (pin) => {
      dispatch(setNewPin(pin))
    }
  })
)(AccountPin)

export const AccountConfirmPin = connect(
  ({accounts}) => {
    const isNew = !accounts.all.find(acc => acc.address === accounts.selected.address)

    return {
      extra: { isNew },
      placeholder: 'Confirm PIN'
    }
  },
  (dispatch) => ({
    onNextPressed: async (pin, {isNew}) => {
      if (isNew) {
        dispatch(addAccount(pin))
        return
      }
      dispatch(changePin(pin))
    }
  })
)(AccountPin)
