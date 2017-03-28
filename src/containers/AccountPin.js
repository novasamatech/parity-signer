'use strict'

import { Alert } from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import AccountPin from '../components/AccountPin'
import { addAccount, setNewPin, modifyAccount } from '../actions/accounts'
import { signTx } from '../actions/transactions'

export const AccountEnterPin = connect(
  (state) => ({
    account: state.accounts.selected,
    withConfirmation: false
  }),
  (dispatch) => ({
    onNextPressed: (pin, account) => {
      if (pin === account.pin) {
        dispatch(signTx(account))
      } else {
        Alert.alert('Invalid PIN')
      }
    }
  })
)(AccountPin)

export const AccountChangePin = connect(
  (state) => ({
    account: state.accounts.selected,
    placeholder: 'Current PIN',
    withConfirmation: false
  }),
  (dispatch) => ({
    onNextPressed: (pin, account, rlp) => {
      if (pin === account.pin) {
        Actions.accountSetPin()
      } else {
        Alert.alert('Invalid PIN')
      }
    }
  })
)(AccountPin)

export const AccountSetPin = connect(
  ({accounts}) => {
    const isNew = !accounts.all.find(acc => acc.address === accounts.selected.address)

    return {
      account: accounts.selected,
      placeholder: isNew ? 'Enter PIN' : 'Enter new PIN'
    }
  },
  (dispatch) => ({
    onNextPressed: (pin, account) => {
      dispatch(setNewPin(pin))
      Actions.accountConfirmPin()
    }
  })
)(AccountPin)

export const AccountConfirmPin = connect(
  ({accounts}) => {
    const isNew = !accounts.all.find(acc => acc.address === accounts.selected.address)

    return {
      account: accounts.selected,
      extra: { isNew },
      placeholder: 'Confirm PIN'
    }
  },
  (dispatch) => ({
    onNextPressed: (pin, account, {isNew}) => {
      if (pin !== account.newPin) {
        Alert.alert('PIN doesn\'t match')
        return
      }

      if (isNew) {
        dispatch(addAccount(account))
        Actions.popTo('accountList')
        Alert.alert('Account Created')
        return
      }

      dispatch(modifyAccount(account, {
        pin
      }))
      Actions.popTo('accountDetails')
      Alert.alert('PIN changed')
    }
  })
)(AccountPin)
