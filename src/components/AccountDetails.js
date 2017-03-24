'use strict'

import React, { Component, PropTypes } from 'react'
import { StyleSheet, View, ScrollView, Text, Button } from 'react-native'
import AppStyles from '../styles'

export default class AccountDetails extends Component {
  static propTypes = {
    account: PropTypes.shape({
      address: PropTypes.string.isRequired
    }).isRequired,
    onDisplayAddressPressed: PropTypes.func.isRequired,
    onDeleteAccountPressed: PropTypes.func.isRequired
  }

  render () {
    return (
      <ScrollView style={AppStyles.view}>
        <Text style={AppStyles.hintText}>Name</Text>
        <Text style={AppStyles.valueText}>{this.props.account.name ? this.props.account.name : 'no name'}</Text>
        <Text style={AppStyles.hintText}>Address</Text>
        <Text style={AppStyles.valueText}>{this.props.account.address}</Text>
        <View style={AppStyles.buttonContainer}>
          <Button
            style={styles.button}
            onPress={this.props.onDisplayAddressPressed}
            title='Display Address QR Code'
            color='green'
            accessibilityLabel='Press to address QR Code'
          />
        </View>
        <View style={AppStyles.buttonContainer}>
          <Text
            style={styles.buttonText}
            onPress={() => this.props.onDeleteAccountPressed(this.props.account)}
          >
            Delete
          </Text>
        </View>
      </ScrollView>
    )
  }
}

const styles = StyleSheet.create({
  button: {
    flex: 0.5
  },
  buttonText: {
    textAlign: 'right'
  }
})
