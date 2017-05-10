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
import { Actions } from 'react-native-router-flux'
import {
  ADD_ACCOUNT, SELECT_ACCOUNT, DELETE_ACCOUNT, MODIFY_ACCOUNT, SET_NEW_PIN, SET_OLD_PIN, SET_ACCOUNTS
} from '../constants/AccountActions'
import { encryptData, decryptData } from '../util/native'

export function addAccount (pin) {
  return async function (dispatch, getState) {
    try {
      let account = getState().accounts.selected
      if (!account) {
        return
      }

      if (account.newPin !== pin) {
        Alert.alert('PIN must be the same')
        return
      }

      let seed = await encryptData(account.seed, pin)
      dispatch({
        type: ADD_ACCOUNT,
        account: {
          encryptedSeed: seed,
          address: account.address,
          name: account.name
        }
      })
      Actions.popTo('accountList')
      Alert.alert('Account Created')
    } catch (e) {
      console.error(e)
    }
  }
}

export function selectAccount (account) {
  return {
    type: SELECT_ACCOUNT,
    account
  }
}

export function deleteAccount (account) {
  return {
    type: DELETE_ACCOUNT,
    account
  }
}

export function modifyAccount (account, modifications) {
  return {
    type: MODIFY_ACCOUNT,
    account,
    modifications
  }
}

export function setOldPin (pin) {
  return async function (dispatch, getState) {
    let account = getState().accounts.selected
    if (!account) {
      return
    }

    try {
      await decryptData(account.encryptedSeed, pin)
      dispatch({
        type: SET_OLD_PIN,
        pin
      })
      Actions.accountSetPin()
    } catch (e) {
      Alert.alert('Invalid PIN')
    }
  }
}

export function setNewPin (pin) {
  return function (dispatch) {
    dispatch({
      type: SET_NEW_PIN,
      pin
    })
    Actions.accountConfirmPin()
  }
}

export function setAccounts (accounts) {
  return {
    type: SET_ACCOUNTS,
    accounts
  }
}

export function changePin (newPin) {
  return async function (dispatch, getState) {
    let account = getState().accounts.selected
    if (!account) {
      return
    }

    if (account.newPin !== newPin) {
      Alert.alert('New PIN must be the same')
      return
    }

    Keyboard.dismiss()

    try {
      let seed = await decryptData(account.encryptedSeed, account.oldPin)
      let encryptedSeed = await encryptData(seed, newPin)

      dispatch(modifyAccount(account, {
        encryptedSeed: encryptedSeed
      }))

      Actions.popTo('accountDetails')
      Alert.alert('PIN changed')
    } catch (e) {
      // this is unreachanle if setOldPin was called before
      console.error(e)
    }
  }
}
