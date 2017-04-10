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
import { ENABLE_SCANNER, DISABLE_SCANNER, DISABLE_SCANNER_WARNINGS, RESET_SCANNER } from '../actions/ScannerActions'
import { selectAccount } from './accounts'
import { scannedTx } from './transactions'
import transaction from '../util/transaction'
import { keccak } from '../util/native'

export function enableScanner () {
  return {
    type: ENABLE_SCANNER
  }
}

export function disableScanner () {
  return {
    type: DISABLE_SCANNER
  }
}

export function disableScannerWarnings () {
  return {
    type: DISABLE_SCANNER_WARNINGS
  }
}

export function resetScanner () {
  return {
    type: RESET_SCANNER
  }
}

export function displayScannerWarning (warning) {
  return function (dispatch, getState) {
    if (getState().scanner.shouldDisplayWarning) {
      dispatch(disableScannerWarnings())
      Alert.alert(warning, undefined, [{
        text: 'OK',
        onPress: () => {
          dispatch(enableScanner())
        }
      }])
    } else {
      dispatch(enableScanner())
    }
  }
}

function findAccountWithAddress (getState, address) {
  return getState().accounts.all.find(account => {
    return account.address.toLowerCase() === address.toLowerCase()
  })
}

export function scannerDispatch (data) {
  return async function (dispatch, getState) {
    if (!getState().scanner.scannerEnabled) {
      return
    }

    dispatch(disableScanner())
    try {
      let txRequest = JSON.parse(data)
      let account = findAccountWithAddress(getState, txRequest.data.account)
      if (!account) {
        dispatch(displayScannerWarning('Invalid sender address ' + txRequest.data.account))
        return
      }

      if (txRequest.action == 'signTransaction') {
        let tx = await transaction(txRequest.data.rlp)
        let hash = await keccak(txRequest.data.rlp)
        dispatch(selectAccount(account))
        dispatch(scannedTx(hash, tx))
      } else if (txRequest.action == 'signTransactionHash') {
        let details = txRequest.data.details
        let hash = txRequest.data.hash
        dispatch(selectAccount(account))
        dispatch(scannedTx(hash, details))
      } else {
        dispatch(displayScannerWarning('Invalid request'))
        return
      }
      Actions.txDetails()
      dispatch(enableScanner())
    } catch (e) {
      console.error(e)
      dispatch(displayScannerWarning('Invalid transaction'))
    }
  }
}
