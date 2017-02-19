import React from 'react'
import { Alert } from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import Pin from '../components/Pin'
import { addAccount, setPin } from '../actions/accounts'
import { signedTx } from '../actions/transactions'

const mapStateToPropsEnterPin = (state, ownProps) => ({
  account: state.accounts.selected,
})

const mapDispatchToPropsEnterPin = (dispatch, ownProps) => ({
  onNextPressed: (pin, account) => {
    if (pin === account.pin) {
      dispatch(signedTx('my super awesome data'))
      Actions.display()
    } else {
      Alert.alert('Invalid pin')
    }
  },
})

const mapDispatchToPropsSetPin = (dispatch, ownProps) => ({
  onNextPressed: (pin) => {
    dispatch(setPin(pin))
    Actions.confirmPin()
  }
})

const mapStateToPropsConfirmPin = (state, ownProps) => ({
  account: state.accounts.selected,
})

const mapDispatchToPropsConfirmPin = (dispatch, ownProps) => ({
  onNextPressed: (pin, account) => {
    if (pin === account.pin) {
      dispatch(addAccount(account))
      Actions.popTo('accounts')
    } else {
      Alert.alert('Invalid pin')
    }
  }
})

export const EnterPin = connect(mapStateToPropsEnterPin, mapDispatchToPropsEnterPin)(Pin)

export const SetPin = connect(undefined, mapDispatchToPropsSetPin)(Pin)

export const ConfirmPin = connect(mapStateToPropsConfirmPin, mapDispatchToPropsConfirmPin)(Pin)



