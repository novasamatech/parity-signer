import React, { Component } from 'react'
import { Vibration } from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import Scanner from '../components/Scanner'
import { newScannedTx } from '../actions/transactions'

const mapDispatchToProps = (dispatch, ownProps) => ({
  onBarCodeRead: (scanned) => {
    dispatch(newScannedTx(scanned.data))
    Vibration.vibrate()
    Actions.signer()
  }
})

const QrScanner = connect(
  undefined,
  mapDispatchToProps
)(Scanner)

export default QrScanner
