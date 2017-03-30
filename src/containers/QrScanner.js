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
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import QrScanner from '../components/QrScanner'
import { selectAccount } from '../actions/accounts'
import { scannedTx } from '../actions/transactions'
import transaction from '../util/transaction'
import { keccak } from '../util/native'
import store from '../util/store'

var scanning = false

function displayAlert (text) {
  Alert.alert(text, undefined, [
    { text: 'OK', onPress: () => { scanning = false } }
  ])
}

function findAccountWithAddress (address) {
  return store.getState().accounts.all.find(account => {
    return account.address.toLowerCase() === address.toLowerCase()
  })
}

async function onScannedTransaction (data, dispatch) {
  try {
    if (scanning) {
      return
    }
    scanning = true
    let txRequest = JSON.parse(data)
    if (txRequest.action === 'signTransaction') {
      let account = findAccountWithAddress(txRequest.data.account)
      if (!account) {
        displayAlert('Invalid sender address ' + txRequest.data.account)
        return
      }
      let tx = await transaction(txRequest.data.rlp)
      let hash = await keccak(txRequest.data.rlp)
      dispatch(selectAccount(account))
      dispatch(scannedTx(hash, tx))
    } else if (txRequest.action === 'signTransactionHash') {
      let account = findAccountWithAddress(txRequest.data.account)
      if (!account) {
        displayAlert('Invalid sender address ' + txRequest.data.account)
        return
      }
      let details = txRequest.data.details
      let hash = txRequest.data.hash
      dispatch(selectAccount(account))
      dispatch(scannedTx(hash, details))
    } else {
      displayAlert('Invalid request')
      return
    }
    Actions.txDetails()
    scanning = false
  } catch (e) {
    console.log(e)
    displayAlert('Invalid transaction ' + e)
  }
}

const mapDispatchToProps = (dispatch, ownProps) => ({
  onBarCodeRead: (scanned) => {
    onScannedTransaction(scanned.data, dispatch)
  }
})

const QrScannerContainer = connect(
  undefined,
  mapDispatchToProps
)(QrScanner)

export default QrScannerContainer
