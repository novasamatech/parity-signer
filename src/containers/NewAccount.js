import React, { Component } from 'react'
import { View, Text, StyleSheet } from 'react-native'
import NewAccountInput from '../components/NewAccountInput'
import { words } from '../actions/random'

export default class NewAccount extends Component {
  constructor(props) {
    super(props)
  }

  render() {
    return (
      <View style={styles.view}>
        <NewAccountInput seed={words()} onChangeText={() => {}}/>
      </View>
    )
  }
}

const styles = StyleSheet.create({
  view: {
    flex: 1,
    marginTop: 60,
    marginBottom: 50,
    padding: 10
  },
})
