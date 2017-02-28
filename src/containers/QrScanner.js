'use strict'

import React, { Component } from 'react'
import { Vibration, Alert } from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import Scanner from '../components/Scanner'
import { selectAccount } from '../actions/accounts'
import { scannedTx } from '../actions/transactions'
import transaction from '../util/transaction'
import store from '../util/store'

var scanning = false

async function onScannedTransaction(data, dispatch) {
  try {
    if (scanning) {
      return
    }
    scanning = true
    let tx_request = JSON.parse(data);
    let from = tx_request.from.toLowerCase()
    let account = store.getState().accounts.all.find(account => {
      return account.address.toLowerCase() == from
    })
    if (!account) {
      Alert.alert('Invalid sender address ' + tx_request.from, undefined, [
        { text: 'OK', onPress: () => { scanning = false }}
      ])
      return
    }
    let tx = await transaction(tx_request.rlp);
    dispatch(selectAccount(account))
    dispatch(scannedTx(data.rlp, tx))
    Vibration.vibrate()
    Actions.enterPin()
    scanning = false
  } catch (e) {
    console.log(e)
    Alert.alert('Invalid transaction', undefined, [
      { text: 'OK', onPress: () => { scanning = false }}
    ])
  }
}

const mapDispatchToProps = (dispatch, ownProps) => ({
  onBarCodeRead: (scanned) => {
    onScannedTransaction(scanned.data, dispatch)
  }
})

const QrScanner = connect(
  undefined,
  mapDispatchToProps
)(Scanner)

export default QrScanner
