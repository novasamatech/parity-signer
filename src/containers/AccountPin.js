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

import { Alert, Keyboard } from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import AccountPin from '../components/AccountPin'
import { addAccount, setNewPin, modifyAccount } from '../actions/accounts'
import { signTx } from '../actions/transactions'

export const AccountEnterPin = connect(
  (state) => ({
    account: state.accounts.selected,
    withConfirmation: false
  }),
  (dispatch) => ({
    onNextPressed: (pin, account) => {
      if (pin === account.pin) {
        Keyboard.dismiss()
        dispatch(signTx(account))
      } else {
        Alert.alert('Invalid PIN')
      }
    }
  })
)(AccountPin)

export const AccountChangePin = connect(
  (state) => ({
    account: state.accounts.selected,
    placeholder: 'Current PIN',
    withConfirmation: false
  }),
  (dispatch) => ({
    onNextPressed: (pin, account, rlp) => {
      if (pin === account.pin) {
        Actions.accountSetPin()
      } else {
        Alert.alert('Invalid PIN')
      }
    }
  })
)(AccountPin)

export const AccountSetPin = connect(
  ({accounts}) => {
    const isNew = !accounts.all.find(acc => acc.address === accounts.selected.address)

    return {
      account: accounts.selected,
      placeholder: isNew ? 'Enter PIN' : 'Enter new PIN'
    }
  },
  (dispatch) => ({
    onNextPressed: (pin, account) => {
      dispatch(setNewPin(pin))
      Actions.accountConfirmPin()
    }
  })
)(AccountPin)

export const AccountConfirmPin = connect(
  ({accounts}) => {
    const isNew = !accounts.all.find(acc => acc.address === accounts.selected.address)

    return {
      account: accounts.selected,
      extra: { isNew },
      placeholder: 'Confirm PIN'
    }
  },
  (dispatch) => ({
    onNextPressed: (pin, account, {isNew}) => {
      if (pin !== account.newPin) {
        Alert.alert('PIN doesn\'t match')
        return
      }

      Keyboard.dismiss()

      if (isNew) {
        dispatch(addAccount(account))
        Actions.popTo('accountList')
        Alert.alert('Account Created')
        return
      }

      dispatch(modifyAccount(account, {
        pin
      }))
      Actions.popTo('accountDetails')
      Alert.alert('PIN changed')
    }
  })
)(AccountPin)
