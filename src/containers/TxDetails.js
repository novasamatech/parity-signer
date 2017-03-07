'use strict'

import React from 'react'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import TxDetails from '../components/TxDetails'

const mapStateToProps = (state, ownProps) => ({
  nextButtonTitle: 'Next',
  nextButtonDescription: 'Choose account',
  txRecipientAddress: state.transactions.pendingTransaction.transaction.action,
  txValue: state.transactions.pendingTransaction.transaction.value,
  txNonce: state.transactions.pendingTransaction.transaction.nonce,
  txGas: state.transactions.pendingTransaction.transaction.gas,
  txGasPrice: state.transactions.pendingTransaction.transaction.gasPrice,
  txData: state.transactions.pendingTransaction.transaction.data,
})

const mapDispatchToProps = (dispatch, ownProps) => ({
  nextButtonAction: () => {
    Actions.accountEnterPin()
  }
})

const TxDetailsContainer = connect(mapStateToProps, mapDispatchToProps)(TxDetails)

export default TxDetailsContainer
