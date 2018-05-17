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

import React, { Component } from 'react'
import PropTypes from 'prop-types'
import { Alert, View, Text, StyleSheet, KeyboardAvoidingView } from 'react-native'
import { Subscribe } from 'unstated'
import AccountsStore from '../stores/AccountsStore'
import TextInput from '../components/TextInput'
import Button from '../components/Button'
import AppStyles from '../styles'
import colors from '../colors'

export default class AccountUnlock extends Component {
  render() {
    return (
      <Subscribe to={[AccountsStore]}>{
        (accounts) => <AccountUnlockView { ...this.props } accounts={ accounts } />}
      </Subscribe>
    )
  }
}

class AccountUnlockView extends Component {
  state = {
    pin: '',
  }

  render () {
    const { accounts } = this.props
    return (
      <View style={styles.body}>
        <Text style={ styles.titleTop }>UNLOCK ACCOUNT</Text>
        <Text style={ styles.title }>PIN</Text>
        <PinInput
          onChangeText={(pin) => this.setState({pin})}
          value={this.state.pin}
        />
        <Button
          onPress={async () => {
            try {
              let res = await accounts.checkPinForSelected(this.state.pin)
              console.log(res)
              Alert.alert('PIN OK')
              this.props.navigation.goBack()
            } catch (e) {
              Alert.alert('INCORRECT PIN')
            }
          }}
          color='green'
          title='CHECK PIN'
          accessibilityLabel={'CHECK PIN'}
        />
      </View>
    )
  }
}

function PinInput (props) {
  return (
    <TextInput
      autoFocus
      clearTextOnFocus
      editable
      fontSize={24}
      keyboardType='numeric'
      multiline={false}
      autoCorrect={false}
      numberOfLines={1}
      returnKeyType='next'
      secureTextEntry
      style={ styles.pinInput }
      { ...props }
    />
  )
}

const styles = StyleSheet.create({
  body: {
    backgroundColor: colors.bg,
    padding: 20,
     flex: 1,
  },
  bodyContainer: {
    flex: 1,
    flexDirection: 'column',
    justifyContent: 'space-between',
  },
  top: {
    flex: 1
  },
  bottom: {
    flexBasis: 50,
    paddingBottom: 15
  },
  title: {
    fontFamily: 'Roboto',
    color: colors.bg_text_sec,
    fontSize: 18,
    fontWeight: 'bold',
    paddingBottom: 10
  },
  titleTop: {
    color: colors.bg_text_sec,
    fontSize: 24,
    fontWeight: 'bold',
    paddingBottom: 20,
    textAlign: 'center'
  },
  hintText: {
    fontFamily: 'Roboto',
    textAlign: 'center',
    color: colors.bg_text_sec,
    fontWeight: '700',
    fontSize: 12,
    paddingBottom: 20
  },
  pinInput: {
    marginBottom: 20
  },
  nextStep: {
    marginTop: 20,
  }
});
