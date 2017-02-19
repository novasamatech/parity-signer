import React from 'react'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import AccountsList from '../components/AccountsList'
import { selectAccount } from '../actions/accounts'

const mapDispatchToProps = (dispatch, ownProps) => ({
  onAccountSelected: (account) => {
    dispatch(selectAccount(account))
    Actions.details()
  }
})

const Accounts = connect(state => ({
  accounts: state.accounts.all
}), mapDispatchToProps)(AccountsList)

export default Accounts
