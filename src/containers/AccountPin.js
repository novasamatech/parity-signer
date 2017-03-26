'use strict'

import { Alert } from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import AccountPin from '../components/AccountPin'
import { addAccount, setPin } from '../actions/accounts'
import { signedTx } from '../actions/transactions'
import { brainWalletSign } from '../util/native'
import { saveAccount } from '../util/db'
import store from '../util/store'

const mapStateToPropsEnterPin = (state, ownProps) => ({
  account: state.accounts.selected
})

async function signTransaction (dispatch, account) {
  try {
    let hash = store.getState().transactions.pendingTransaction.rlpHash
    let signature = await brainWalletSign(account.seed, hash)
    dispatch(signedTx(signature))
    Actions.qrViewTx()
  } catch (e) {
    console.log(e)
  }
}

const mapDispatchToPropsEnterPin = (dispatch, ownProps) => ({
  onNextPressed: (pin, account) => {
    if (pin === account.pin) {
      signTransaction(dispatch, account)
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
