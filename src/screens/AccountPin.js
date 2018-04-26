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
import { TextInput, View, Button, StyleSheet, KeyboardAvoidingView } from 'react-native'
import AppStyles from '../styles'

export default class AccountPin extends Component {
  static propTypes = {
    onNextPressed: PropTypes.func.isRequired,
    placeholder: PropTypes.string,
    extra: PropTypes.object
  }

  static defaultProps = {
    placeholder: 'Enter PIN'
  }

  state = {
    pin: ''
  }

  onNext = () => {
    const {pin} = this.state
    const {extra} = this.props

    this.props.onNextPressed(pin, extra)
  }

  onChange = (pin) => {
    this.setState({
      pin
    })
  }

  render () {
    return (
      <View style={AppStyles.view}>
        <KeyboardAvoidingView style={AppStyles.center} behavior='padding'>
          <TextInput
            autoFocus
            clearTextOnFocus
            editable
            fontSize={24}
            keyboardType='numeric'
            multiline={false}
            autoCorrect={false}
            numberOfLines={1}
            onChangeText={this.onChange}
            placeholder={this.props.placeholder}
            returnKeyType='next'
            secureTextEntry
            style={AppStyles.pin}
            value={this.state.pin}
          />
          <View style={[AppStyles.buttonContainer, styles.button]}>
            <Button
              onPress={this.onNext}
              color='green'
              title='Next'
              accessibilityLabel={this.props.placeholder}
            />
          </View>
        </KeyboardAvoidingView>
      </View>
    )
  }
}

const styles = StyleSheet.create({
  button: {
    marginTop: 10,
    alignItems: 'center',
    justifyContent: 'center'
  }
})
