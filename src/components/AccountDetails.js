'use strict'

import React, { Component, PropTypes } from 'react'
import { StyleSheet, View, Text, Button } from 'react-native'
import AppStyles from '../styles'

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
      <View style={AppStyles.view}>
        <Text style={AppStyles.hintText}>Name</Text>
        <Text style={AppStyles.valueText}>{this.props.account.name ? this.props.account.name : 'no name'}</Text>
        <Text style={AppStyles.hintText}>Address</Text>
        <Text style={AppStyles.valueText}>{this.props.account.address}</Text>
        <View style={styles.buttonContainer}>
          {/*<Button
            style={styles.button}
            onPress={this.props.onSendTransactionPressed}
            title='Send Transaction'
            color='green'
            accessibilityLabel='Press to send new transaction'
          />*/}
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
  buttonContainer: {
    //flexDirection: 'row',
    //justifyContent: 'space-between'
  },
  button: {
    flex: 0.5,
  }
})
