'use strict'

import { Alert } from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import AccountPin from '../components/AccountPin'
import { addAccount, modifyAccount, setNewPin } from '../actions/accounts'
import { signedTx } from '../actions/transactions'
import { keccak, brainWalletSign } from '../util/native'

async function signTransaction (dispatch, account, rlp) {
  try {
    let hash = await keccak(rlp)
    let signature = await brainWalletSign(account.seed, hash)
    dispatch(signedTx(signature))
    Actions.qrViewTx()
  } catch (e) {
    console.log(e)
  }
}

export const AccountEnterPin = connect(
  (state) => ({
    account: state.accounts.selected,
    extra: {
      rlp: state.transactions.pendingTransaction.rlp
    },
    withConfirmation: false
  }),
  (dispatch) => ({
    onNextPressed: (pin, account, {rlp}) => {
      if (pin === account.pin) {
        signTransaction(dispatch, account, rlp)
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
