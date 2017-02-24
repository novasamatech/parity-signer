'use strict'

import React from 'react'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import Send from '../components/Send'

const mapStateToProps = (state, ownProps) => ({
  nextButtonTitle: 'Next',
  nextButtonDescription: 'Choose account',
  txRecipientAddress: '0xbF35fAA9C265bAf50C9CFF8c389C363B05753275',
  txValue: '20 eth',
  txNonce: '100',
  txGas: '200',
  txGasPrice: '',
  txData: '',
})

const mapDispatchToProps = (dispatch, ownProps) => ({
  nextButtonAction: () => {
    Actions.select()
  }
})

const ConfirmTransaction = connect(mapStateToProps, mapDispatchToProps)(Send)

export default ConfirmTransaction
