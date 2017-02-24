'use strict'

import React, { Component, PropTypes } from 'react'
import { Text, TextInput, View, StyleSheet } from 'react-native'

export default class Pin extends Component {
  static propTypes = {
    onNextPressed: PropTypes.func.isRequired,
  }

  constructor(props) {
    super(props)
    this.state = {
      text: '',
    }
  }

  render() {
    return (
      <View style={styles.view}>
        <TextInput
          style={styles.input}
          placeholder='enter pin here'
          editable={true}
          multiline={false}
          autoFocus={true}
          returnKeyType='next'
          keyboardType='numeric'
          numberOfLines={1}
          fontSize={24}
          onChangeText={(text) => {this.setState({text: text})}}
          value={this.state.text}
          onEndEditing={() => { this.props.onNextPressed(this.state.text, this.props.account) }}
        />
      </View>
    )
  }
}

const styles = StyleSheet.create({
  view: {
    flex: 1,
    marginTop: 60,
    marginBottom: 300,
    alignItems: 'center',
    justifyContent: 'center',
  },
  input: {
    height: 20,
    textAlign: 'center'
  }
})
