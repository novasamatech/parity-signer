import React from 'react'
import { connect } from 'react-redux'
import AccountDetails from '../components/AccountDetails'

const Account = connect(state => ({
  account: state.accounts.selected.address,
}))(AccountDetails)

export default Account
