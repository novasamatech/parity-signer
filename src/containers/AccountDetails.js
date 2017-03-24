'use strict'

import { Alert } from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import AccountDetails from '../components/AccountDetails'
import { deleteAccount } from '../actions/accounts'
import { deleteAccount as dbDeleteAccount } from '../util/db'

const mapDispatchToProps = (dispatch, ownProps) => ({
  onDisplayAddressPressed: () => {
    Actions.qrViewAddress()
  },
  onDeleteAccountPressed: (account) => {
    Alert.alert('Do you want to delete the account?', undefined, [
      { text: 'Yes',
        onPress: () => {
          dispatch(deleteAccount(account))
          dbDeleteAccount(account)
          Actions.pop()
        }},
      { text: 'No' }
    ])
  }
})

const AccountDetailsContainer = connect(state => ({
  account: state.accounts.selected
}), mapDispatchToProps)(AccountDetails)

export default AccountDetailsContainer
