'use strict'

import React, { Component, PropTypes } from 'react'
import { StyleSheet, View, Text, TextInput, Button } from 'react-native'

export default class Send extends Component {
  render() {
    return (
      <View style={styles.view}>
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
          onPress={() => {}}
          title="Generate QR Code"
          color="green"
          accessibilityLabel="Press to generate QR Code"
        />
      </View>
    )
  }
}

const styles = StyleSheet.create({
  view: {
    flex: 1,
    marginTop: 60,
    marginBottom: 50,
    padding: 20,
  },
  hint: {
    marginBottom: 20,
  },
  input: {
    height: 20,
    marginBottom: 20,
  }
})

