'use strict'

import React, { Component, PropTypes } from 'react'
import { StyleSheet, View, ScrollView, Text, Button, Image } from 'react-native'
import AppStyles from '../styles'
import { blockiesIcon } from '../util/native'

async function displayIcon(self, seed) {
  try {
    let icon = await blockiesIcon(seed)
    self.setState({
      icon: icon,
    })
  }
  catch (e) {
    console.log(e)
  }
}

export default class AccountDetails extends Component {
  static propTypes = {
    account: PropTypes.shape({
      address: PropTypes.string.isRequired,
    }).isRequired,
    onDisplayAddressPressed: PropTypes.func.isRequired,
    onDeleteAccountPressed: PropTypes.func.isRequired,
  }

  constructor(props) {
    super(props)
    this.state = {}
    displayIcon(this, '0x' + this.props.account.address)
  }

  render() {
    return (
      <ScrollView style={AppStyles.view}>
        <Image
          style={styles.icon}
          source={{uri: this.state.icon}}
        />
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
          <Button
            style={styles.button}
            onPress={() => this.props.onDeleteAccountPressed(this.props.account)}
            title='Delete'
            color='red'
            accessibilityLabel='Press to delete account'
          />
        </View>
      </ScrollView>
    )
  }
}

const styles = StyleSheet.create({
  button: {
    flex: 0.5,
  },
  icon: {
    height: 100,
    width: 100,
    resizeMode: 'contain',
    marginBottom: 20,
  }
})
