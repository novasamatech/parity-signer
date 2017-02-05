import React from 'react'
import { connect } from 'react-redux'
import AccountsList from '../components/AccountsList'

const Accounts = connect(state => ({
  accounts: state.accounts
}))(AccountsList)

export default Accounts
