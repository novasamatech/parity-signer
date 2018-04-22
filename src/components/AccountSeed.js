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
import { TextInput, StyleSheet } from 'react-native'

export default class AccountSeed extends Component {
  static propTypes = {
    seed: PropTypes.string.isRequired,
    onChangeText: PropTypes.func.isRequired
  }

  state = {
    text: this.props.seed,
    height: 0
  }

  onChange = (text) => {
    this.setState({
      text: text
    })
    this.props.onChangeText(text)
  }

  onContentSizeChange = (event) => {
    this.setState({
      height: event.nativeEvent.contentSize.height
    })
  }

  render () {
    return (
      <TextInput
        editable
        fontSize={12}
        maxLength={240}
        multiline
        onChangeText={this.onChange}
        onContentSizeChange={this.onContentSizeChange}
        placeholder='Parity account recovery phrase'
        returnKeyType='default'
        selectTextOnFocus
        spellCheck={false}
        style={[styles.input, {height: Math.max(35, this.state.height)}]}
        value={this.state.text}
      />
    )
  }
}

const styles = StyleSheet.create({
  input: {
    height: 120,
    fontWeight: 'bold',
    textAlignVertical: 'top'
  }
})
