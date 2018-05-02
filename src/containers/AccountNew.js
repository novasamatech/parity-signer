// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

'use strict'

import React, { Component } from 'react'
import PropTypes from 'prop-types'
import {
  ScrollView, View, Text, TextInput, Button, TouchableOpacity, Share
} from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import debounce from 'debounce'
import AccountSeed from '../components/AccountSeed'
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

    this.state = {
      seed: this.props.seed,
      address: '',
      name: ''
    }

    this.updateAddress(this, this.props.seed)
  }

  componentWillReceiveProps (newProps) {
    const { seed } = newProps
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
      <ScrollView style={AppStyles.fullscreenView}>
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
