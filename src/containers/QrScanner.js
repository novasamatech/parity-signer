'use strict'

import { Alert } from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import QrScanner from '../components/QrScanner'
import { selectAccount } from '../actions/accounts'
import { scannedTx } from '../actions/transactions'
import transaction from '../util/transaction'
import store from '../util/store'

var scanning = false

async function onScannedTransaction (data, dispatch) {
  try {
    if (scanning) {
      return
    }
    scanning = true
    let txRequest = JSON.parse(data)
    let from = txRequest.from.toLowerCase()
    let account = store.getState().accounts.all.find(account => {
      return account.address.toLowerCase() === from
    })
    if (!account) {
      Alert.alert('Invalid sender address ' + txRequest.from, undefined, [
        { text: 'OK', onPress: () => { scanning = false } }
      ])
      return
    }
    let tx = await transaction(txRequest.rlp)
    dispatch(selectAccount(account))
    dispatch(scannedTx(txRequest.rlp, tx))
    // Vibration.vibrate()
    Actions.txDetails()
    scanning = false
  } catch (e) {
    console.log(e)
    Alert.alert('Invalid transaction', undefined, [
      { text: 'OK', onPress: () => { scanning = false } }
    ])
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
