'use strict'

import React, { Component, PropTypes } from 'react'
import { TextInput, View, Button, StyleSheet } from 'react-native'
import AppStyles from '../styles'

export default class AccountPin extends Component {
  static propTypes = {
    onNextPressed: PropTypes.func.isRequired,
    account: PropTypes.object.isRequired,
    placeholder: PropTypes.string,
    extra: PropTypes.object
  }

  static defaultProps = {
    extra: {},
    placeholder: 'Enter PIN'
  }

  constructor (props) {
    super(props)
    this.state = {
      pin: ''
    }
  }

  onNext = () => {
    const {pin} = this.state
    const {account, extra} = this.props

    this.props.onNextPressed(pin, account, extra)
  }

  onChange = (pin) => {
    this.setState({
      pin
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
