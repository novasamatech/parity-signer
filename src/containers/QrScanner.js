'use strict'

import React, { Component } from 'react'
import { Vibration, Alert } from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import Scanner from '../components/Scanner'
import { scannedTx } from '../actions/transactions'
import transaction from '../util/transaction'

var scanning = false

async function onScannedTransaction(data, dispatch) {
  try {
    if (scanning) {
      return
    }
    scanning = true
    let tx = await transaction(data);
    dispatch(scannedTx(data, tx))
    Vibration.vibrate()
    Actions.confirm()
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
