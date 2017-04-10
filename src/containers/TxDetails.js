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

import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import TxDetails from '../components/TxDetails'

const fetchAccountName = (state, address = '') => {
  let account = state.accounts.all.find(account => {
    return account.address.toLowerCase() === address.toLowerCase()
  })
  return account ? account.name : 'Unknown'
}

const mapStateToProps = (state, ownProps) => ({
  nextButtonTitle: 'Next',
  nextButtonDescription: 'Enter Pin',
  txRlpHash: state.transactions.pendingTransaction.rlpHash,
  txSenderAddress: state.accounts.selected.address,
  txRecipientAddress: state.transactions.pendingTransaction.transaction.action,
  txValue: state.transactions.pendingTransaction.transaction.value,
  txNonce: state.transactions.pendingTransaction.transaction.nonce,
  txGas: state.transactions.pendingTransaction.transaction.gas,
  txGasPrice: state.transactions.pendingTransaction.transaction.gasPrice,
  txData: state.transactions.pendingTransaction.transaction.data,
  isSafe: state.transactions.pendingTransaction.transaction.isSafe,
  txSenderName: fetchAccountName(state, state.accounts.selected.address),
  txRecipientName: fetchAccountName(state, state.transactions.pendingTransaction.transaction.action)
})

const mapDispatchToProps = (dispatch, ownProps) => ({
  nextButtonAction: () => {
    Actions.accountEnterPin()
  }
})

const TxDetailsContainer = connect(mapStateToProps, mapDispatchToProps)(TxDetails)

export default TxDetailsContainer
