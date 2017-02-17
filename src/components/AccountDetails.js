'use strict'

import React, { Component, PropTypes } from 'react'
import { StyleSheet, View, Text, Button } from 'react-native'

export default class AccountDetails extends Component {
  static propTypes = {
    account: PropTypes.shape({
      address: PropTypes.string.isRequired,
    }).isRequired,
    onSendTransactionPressed: PropTypes.func.isRequired,
    onDeleteAccountPressed: PropTypes.func.isRequired,
  }

  render() {
    return (
      <View style={styles.view}>
        <Text style={styles.text}>Name</Text>
        <Text style={styles.props}>{this.props.account.name ? this.props.account.name : 'no name'}</Text>
        <Text style={styles.text}>Address</Text>
        <Text style={styles.props}>{this.props.account.address}</Text>
        <View style={styles.buttonContainer}>
          <Button
            style={styles.button}
            onPress={this.props.onSendTransactionPressed}
            title='Send Transaction'
            color='green'
            accessibilityLabel='Press to send new transaction'
          />
          <Button
            style={styles.button}
            onPress={() => this.props.onDeleteAccountPressed(this.props.account)}
            title='Delete'
            color='red'
            accessibilityLabel='Press to delete account'
          />
        </View>
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
  text: {
    marginBottom: 20,
  },
  props: {
    marginBottom: 20,
    fontSize: 16,
  },
  buttonContainer: {
    flexDirection: 'row',
    justifyContent: 'space-between'
  },
  button: {
    flex: 0.5,
  }
})
