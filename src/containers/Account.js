'use strict'

import React from 'react'
import { Alert } from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import AccountDetails from '../components/AccountDetails'
import { deleteAccount } from '../actions/accounts'

const mapDispatchToProps = (dispatch, ownProps) => ({
  onDisplayAddressPressed: () => {
    Actions.displayAddress()
  },
  onDeleteAccountPressed: (account) => {
    Alert.alert('Do you want to delete the account?', undefined, [
      { text: 'Yes', onPress: () => {
        dispatch(deleteAccount(account))
        Actions.pop()
      }},
      { text: 'No' }
    ])
  },
})

const Account = connect(state => ({
  account: state.accounts.selected,
}), mapDispatchToProps)(AccountDetails)

export default Account
