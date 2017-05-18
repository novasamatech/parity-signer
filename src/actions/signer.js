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

import { Alert } from 'react-native'
import { Actions } from 'react-native-router-flux'
import { NEW_SCANNED_TX, NEW_SCANNED_DATA, SIGN_HASH } from '../constants/SignerActions'
import { brainWalletSign, decryptData } from '../util/native'

export function scannedTx (hash, transaction) {
  return {
    type: NEW_SCANNED_TX,
    hash,
    transaction
  }
}

export function scannedData (hash, data) {
  return {
    type: NEW_SCANNED_DATA,
    hash,
    data
  }
}

export function signHash (pin) {
  return async function (dispatch, getState) {
    try {
      let account = getState().accounts.selected
      let hash = getState().signer.hashToSign
      let seed = await decryptData(account.encryptedSeed, pin)
      let signature = await brainWalletSign(seed, hash)
      dispatch({
        type: SIGN_HASH,
        signature: signature
      })
      Actions.qrViewTx()
    } catch (e) {
      Alert.alert('Invalid PIN')
    }
  }
}
