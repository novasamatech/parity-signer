'use strict'

import React, { Component } from 'react'
import { View, Text, TextInput, Button, StyleSheet } from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import debounce from 'debounce'
import NewAccountInput from '../components/NewAccountInput'
import { words } from '../util/random'
import { brainWalletAddress } from '../util/crypto'
import { selectAccount }  from '../actions/accounts'
import AppStyles from '../styles'

const mapDispatchToProps = (dispatch) => {
  return {
    addAccount: (account) => {
      dispatch(selectAccount({
        seed: account.seed,
        address: account.address,
        name: account.name,
      }))
      Actions.setPin()
    }
  }
}

export class NewAccount extends Component {
  constructor(props) {
    super(props)

    const seed = words()

    this.state = {
      seed: seed,
      address: '',
      name: '',
    }

    brainWalletAddress(seed, (address) => {
      this.setState({
        address: address,
      })
    })
  }

  render() {
    var self = this;
    return (
      <View style={AppStyles.view}>
        <Text style={styles.hint}>name</Text>
        <TextInput
          placeholder='My Account'
          value={this.state.name}
          style={styles.input}
          editable={true}
          multiline={false}
          returnKeyType='next'
          numberOfLines={1}
          fontSize={16}
          autoFocus={true}
          onChangeText={(text) => {this.setState({name: text})}}
        />
        <Text style={styles.hint}>brain wallet seed</Text>
        <NewAccountInput seed={this.state.seed} onChangeText={
          debounce((text) => {
            brainWalletAddress(text, (address) => {
              self.setState({
                seed: text,
                address: address,
              })
            })
          }, 100)}
        />
        <Text style={styles.address}>0x{this.state.address}</Text>
        <Button
          onPress={() => this.props.addAccount({
            seed: this.state.seed,
            address: this.state.address,
            name: this.state.name,
          })}
          title="Add Account"
          color="green"
          accessibilityLabel="Press to add new account"
        />
      </View>
    )
  }
}

const styles = StyleSheet.create({
  hint: {
    marginBottom: 20,
  },
  address: {
    marginBottom: 20,
    fontSize: 12,
  },
  input: {
    height: 20,
    marginBottom: 20,
  }
})

export default connect(
  undefined,
  mapDispatchToProps
)(NewAccount)

