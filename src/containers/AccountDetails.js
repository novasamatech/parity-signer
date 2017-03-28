'use strict'

import { Alert } from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import AccountDetails from '../components/AccountDetails'
import { deleteAccount, modifyAccount } from '../actions/accounts'

const mapDispatchToProps = (dispatch, ownProps) => ({
  onNameChange: (account, name) => {
    dispatch(modifyAccount(account, {
      name
    }))
  },
  onChangePin: (account) => {
    Actions.accountChangePin()
  },
  onDelete: (account) => {
    Alert.alert(
      'Delete the account?',
      `Account "${account.name}" will be unrecoverably wiped from your device.`,
      [
        {
          text: 'Yes',
          onPress: () => {
            dispatch(deleteAccount(account))
            Actions.pop()
          }
        },
        {
          text: 'No'
        }
      ]
    )
  }
})

const AccountDetailsContainer = connect(state => ({
  account: state.accounts.selected
}), mapDispatchToProps)(AccountDetails)

export default AccountDetailsContainer
