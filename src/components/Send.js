'use strict'

import React, { Component, PropTypes } from 'react'
import { StyleSheet, View, Text, TextInput, Button } from 'react-native'
import AppStyles from '../styles'

export default class Send extends Component {
  static propTypes = {
    nextButtonTitle: PropTypes.string.isRequired,
    nextButtonDescription: PropTypes.string.isRequired,
    nextButtonAction: PropTypes.func.isRequired,
  }

  render() {
    return (
      <View style={AppStyles.view}>
        <Text style={styles.hint}>recipient address</Text>
        <TextInput
          placeholder='the recipient address'
          style={styles.input}
          editable={true}
          multiline={false}
          autoFocus={true}
          returnKeyType='next'
          numberOfLines={1}
          fontSize={16}
        />
        <Text style={styles.hint}>amount to transfer (in ETH)</Text>
        <TextInput
          placeholder=''
          value='0.0'
          style={styles.input}
          editable={true}
          multiline={false}
          returnKeyType='next'
          numberOfLines={1}
          fontSize={16}
        />
        <Text style={styles.hint}>total transaction amount</Text>
        <TextInput
          placeholder=''
          value='0.0'
          style={styles.input}
          editable={true}
          multiline={false}
          returnKeyType='next'
          numberOfLines={1}
          fontSize={16}
        />
        <Button
          onPress={() => this.props.nextButtonAction()}
          title={this.props.nextButtonTitle}
          color="green"
          accessibilityLabel={this.props.nextButtonDescription}
        />
      </View>
    )
  }
}

const styles = StyleSheet.create({
  hint: {
    marginBottom: 20,
  },
  input: {
    height: 20,
    marginBottom: 20,
  }
})

