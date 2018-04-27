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
  ScrollView, View, Text, TextInput, Button, TouchableOpacity, Share, StyleSheet
} from 'react-native'
import { connect } from 'react-redux'
import { Actions } from 'react-native-router-flux'
import debounce from 'debounce'
import AccountSeed from '../components/AccountSeed'
import { brainWalletAddress } from '../util/native'
import { selectAccount } from '../actions/accounts'
import AccountIcon from '../components/AccountIcon'
import AppStyles from '../styles'

import colors from '../colors'

const mapDispatchToProps = (dispatch, props) => {
  return {
    onAddAccount: (account) => {
      dispatch(selectAccount({
        seed: account.seed,
        address: account.address,
        name: account.name
      }))
      props.navigation.navigate('AccountSetPin')
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
  static navigationOptions = {
    title: 'New Account'
  }
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
      <ScrollView style={AppStyles.view}>
        <Text style={ styles.title_top }>CREATE ACCOUNT</Text>
        <Text style={ styles.title }>CHOOSE AN IDENTICON</Text>
      </ScrollView>
    )
  }
}

function IdenticonChooser() {
  const style = StyleSheet.create({
    body: {
      backgroundColor: colors.card_bg
    },
    addressText: {

    }
  });
  return (
    <View>

    </View>
  )
}

export default connect(
  undefined,
  mapDispatchToProps
)(AccountNew)

const styles = StyleSheet.create({
  body: {
    flex: 1,
    flexDirection: 'column',
    padding: 20,
    backgroundColor: colors.bg
  },
  title: {
    color: colors.bg_text_sec,
    fontSize: 18,
    fontWeight: 'bold',
    paddingBottom: 20
  },
  title_top: {
    color: colors.bg_text_sec,
    fontSize: 24,
    fontWeight: 'bold',
    paddingBottom: 20,
    textAlign: 'center'
  },
});
