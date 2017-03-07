'use strict'

import React, { Component, PropTypes } from 'react'
import { Text, TextInput, View } from 'react-native'
import AppStyles from '../styles'

export default class AccountPin extends Component {
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
      <View style={AppStyles.view}>
        <View style={AppStyles.center}>
        <TextInput
          style={AppStyles.pin}
          placeholder='enter pin here'
          editable={true}
          multiline={false}
          autoFocus={true}
          returnKeyType='next'
          numberOfLines={1}
          fontSize={24}
          onChangeText={(text) => {this.setState({text: text})}}
          value={this.state.text}
          onEndEditing={() => { this.props.onNextPressed(this.state.text, this.props.account, this.props.extra) }}
        />
      </View>
      </View>
    )
  }
}
