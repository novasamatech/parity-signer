'use strict'

import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import AccountList from '../components/AccountList'
import { selectAccount } from '../actions/accounts'

const mapDispatchToProps = (dispatch, ownProps) => ({
  onAccountSelected: (account) => {
    dispatch(selectAccount(account))
    Actions.accountDetails()
  }
})

const AccountListContainer = connect(state => ({
  accounts: state.accounts.all
}), mapDispatchToProps)(AccountList)

export default AccountListContainer
