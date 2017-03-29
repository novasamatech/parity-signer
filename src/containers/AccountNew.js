'use strict'

import React, { Component, PropTypes } from 'react'
import {
  ScrollView, View, Text, TextInput, Button, TouchableOpacity, Share
} from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import debounce from 'debounce'
import AccountSeed from '../components/AccountSeed'
import { words } from '../util/random'
import { brainWalletAddress } from '../util/native'
import { selectAccount } from '../actions/accounts'
import AccountIcon from '../components/AccountIcon'
import AppStyles from '../styles'

const mapDispatchToProps = (dispatch) => {
  return {
    onAddAccount: (account) => {
      dispatch(selectAccount({
        seed: account.seed,
        address: account.address,
        name: account.name
      }))
      Actions.accountSetPin()
    },
    onBackupPhrase: (seed, address) => {
      Share.share({
        message: `Recovery phrase for ${address}: ${seed}`,
        ttile: `Recovery phrase for ${address}`
      })
    }
  }
}

export class AccountNew extends Component {
  static propTypes = {
    onBackupPhrase: PropTypes.func.isRequired,
    onAddAccount: PropTypes.func.isRequired
  }

  constructor (props) {
    super(props)

    const seed = words()

    this.state = {
      seed: seed,
      address: '',
      name: ''
    }

    this.updateAddress(this, seed)
  }

  async updateAddress (self, seed) {
    try {
      let address = await brainWalletAddress(seed)
      self.setState({
        seed: seed,
        address: address
      })
    } catch (e) {
      // this should never fail
      console.error(e)
    }
  }

  backupSeed = () => {
    const { address, seed } = this.state
    this.props.onBackupPhrase(seed, address)
  }

  render () {
    return (
      <ScrollView style={AppStyles.view}>
        <AccountIcon style={AppStyles.icon} seed={'0x' + this.state.address} />
        <Text style={AppStyles.hintText}>Account Name</Text>
        <TextInput
          placeholder='Name for this account'
          value={this.state.name}
          style={AppStyles.inputValue}
          editable
          multiline={false}
          returnKeyType='next'
          numberOfLines={1}
          fontSize={12}
          autoFocus
          onChangeText={(text) => { this.setState({name: text}) }}
        />
        <Text style={AppStyles.hintText}>Recovery Phrase</Text>
        <AccountSeed seed={this.state.seed} onChangeText={
          debounce((text) => { this.updateAddress(this, text) }, 100)
        } />
        <TouchableOpacity
          onPress={this.backupSeed}
          >
          <Text style={AppStyles.hintTextSmall}>Make sure to back up your recovery phrase! Click this text to share it.</Text>
        </TouchableOpacity>
        <Text style={AppStyles.hintText}>Address</Text>
        <TextInput
          editable={false}
          style={[AppStyles.inputValue, AppStyles.inputValueSmall]}
          value={`0x${this.state.address}`}
          />
        <View style={AppStyles.buttonContainer}>
          <Button
            onPress={() => this.props.onAddAccount({
              seed: this.state.seed,
              address: this.state.address,
              name: this.state.name
            })}
            title='Set up PIN'
            color='green'
            accessibilityLabel='Press to set up the PIN for the account'
          />
        </View>
      </ScrollView>
    )
  }
}

export default connect(
  undefined,
  mapDispatchToProps
)(AccountNew)
