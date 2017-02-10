'use strict'

import React, { Component, PropTypes } from 'react'
import { Text, View, StyleSheet } from 'react-native'

export default class Scanned extends Component {
  static propTypes = {
    transactions: PropTypes.shape({
      pendingTx: PropTypes.string.isRequired,
    }).isRequired
  }

  render() {
    return (
      <View style={styles.view}>
        <Text>You scanned {this.props.transactions.pendingTx}</Text>
      </View>
    )
  }
}

const styles = StyleSheet.create({
  view: {
    flex: 1,
    marginTop: 60,
    marginBottom: 50,
  },
})
