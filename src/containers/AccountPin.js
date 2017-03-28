'use strict'

import { Alert } from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import AccountPin from '../components/AccountPin'
import { addAccount, setPin } from '../actions/accounts'
import { signTx } from '../actions/transactions'
import { saveAccount } from '../util/db'

const mapStateToPropsEnterPin = (state, ownProps) => ({
  account: state.accounts.selected
})

const mapDispatchToPropsEnterPin = (dispatch, ownProps) => ({
  onNextPressed: (pin, account) => {
    if (pin === account.pin) {
      dispatch(signTx(account))
    } else {
      Alert.alert('Invalid PIN')
    }
  }
})

const mapDispatchToPropsSetPin = (dispatch, ownProps) => ({
  onNextPressed: (pin) => {
    dispatch(setPin(pin))
    Actions.accountConfirmPin()
  }
})

const mapStateToPropsConfirmPin = (state, ownProps) => ({
  account: state.accounts.selected
})

const mapDispatchToPropsConfirmPin = (dispatch, ownProps) => ({
  onNextPressed: (pin, account) => {
    if (pin === account.pin) {
      dispatch(addAccount(account))
      saveAccount(account)
      Actions.popTo('accountList')
      Alert.alert('Account created')
    } else {
      Alert.alert('Invalid PIN')
    }
  }
})

export const AccountEnterPin = connect(mapStateToPropsEnterPin, mapDispatchToPropsEnterPin)(AccountPin)

export const AccountSetPin = connect(undefined, mapDispatchToPropsSetPin)(AccountPin)

export const AccountConfirmPin = connect(mapStateToPropsConfirmPin, mapDispatchToPropsConfirmPin)(AccountPin)
