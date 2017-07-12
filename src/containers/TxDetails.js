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
  txRlpHash: state.signer.hashToSign,
  txSenderAddress: state.accounts.selected.address,
  txRecipientAddress: state.signer.transactionDetails.action,
  txValue: state.signer.transactionDetails.value,
  txNonce: state.signer.transactionDetails.nonce,
  txGas: state.signer.transactionDetails.gas,
  txGasPrice: state.signer.transactionDetails.gasPrice,
  txData: state.signer.transactionDetails.data,
  isSafe: state.signer.transactionDetails.isSafe,
  txSenderName: fetchAccountName(state, state.accounts.selected.address),
  txRecipientName: fetchAccountName(state, state.signer.transactionDetails.action)
})

const mapDispatchToProps = (dispatch, ownProps) => ({
  nextButtonAction: () => {
    Actions.accountEnterPin()
  }
})

const TxDetailsContainer = connect(mapStateToProps, mapDispatchToProps)(TxDetails)

export default TxDetailsContainer
