'use strict'

import React from 'react'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import AccountDetails from '../components/AccountDetails'
import { deleteAccount } from '../actions/accounts'

const mapDispatchToProps = (dispatch, ownProps) => ({
  onSendTransactionPressed: () => {
    Actions.send()
  },
  onDeleteAccountPressed: (account) => {
    dispatch(deleteAccount(account))
    Actions.pop()
  },
})

const Account = connect(state => ({
  account: state.accounts.selected,
}), mapDispatchToProps)(AccountDetails)

export default Account
