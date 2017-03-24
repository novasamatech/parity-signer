'use strict'

import { Alert } from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import AccountPin from '../components/AccountPin'
import { addAccount, setPin } from '../actions/accounts'
import { signedTx } from '../actions/transactions'
import { keccak, brainWalletSign } from '../util/native'
import { saveAccount } from '../util/db'

const mapStateToPropsEnterPin = (state, ownProps) => ({
  account: state.accounts.selected,
  extra: state.transactions.pendingTransaction.rlp
})

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

const mapDispatchToPropsEnterPin = (dispatch, ownProps) => ({
  onNextPressed: (pin, account, rlp) => {
    if (pin === account.pin) {
      signTransaction(dispatch, account, rlp)
    } else {
      Alert.alert('Invalid pin')
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
    } else {
      Alert.alert('Invalid pin')
    }
  }
})

export const AccountEnterPin = connect(mapStateToPropsEnterPin, mapDispatchToPropsEnterPin)(AccountPin)

export const AccountSetPin = connect(undefined, mapDispatchToPropsSetPin)(AccountPin)

export const AccountConfirmPin = connect(mapStateToPropsConfirmPin, mapDispatchToPropsConfirmPin)(AccountPin)
