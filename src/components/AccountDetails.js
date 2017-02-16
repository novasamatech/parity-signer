'use strict'

import React, { Component, PropTypes } from 'react'
import { StyleSheet, View, Text, Button } from 'react-native'

export default class AccountDetails extends Component {
  static propTypes = {
    account: PropTypes.shape({
      address: PropTypes.string.isRequired,
    }).isRequired
  }

  render() {
    return (
      <View style={styles.view}>
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
  }
})
