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
