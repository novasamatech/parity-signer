'use strict'

import React, { Component, PropTypes } from 'react'
import { TextInput, View, Button, StyleSheet } from 'react-native'
import AppStyles from '../styles'

export default class AccountPin extends Component {
  static propTypes = {
    onNextPressed: PropTypes.func.isRequired
  }

  constructor (props) {
    super(props)
    this.state = {
      text: ''
    }
  }

  onNext = () => {
    const {text} = this.state
    const {account, extra} = this.props

    this.props.onNextPressed(text, account, extra)
  }

  onChange = (text) => {
    this.setState({
      text
    })
  }

  render () {
    return (
      <View style={AppStyles.view}>
        <View style={AppStyles.center}>
          <TextInput
            autoFocus
            clearTextOnFocus
            editable
            fontSize={24}
            keyboardType='numeric'
            multiline={false}
            numberOfLines={1}
            onChangeText={this.onChange}
            placeholder='enter pin here'
            returnKeyType='next'
            secureTextEntry
            style={AppStyles.pin}
            value={this.state.text}
          />
          <View style={[AppStyles.buttonContainer, styles.button]}>
            <Button
              onPress={this.onNext}
              color='green'
              title='Next'
              accessibilityLabel='Confrim PIN'
            />
          </View>
        </View>
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
