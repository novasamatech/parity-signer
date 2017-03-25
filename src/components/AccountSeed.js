'use strict'

import React, { Component, PropTypes } from 'react'
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
