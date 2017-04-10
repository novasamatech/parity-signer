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

import React, { Component, PropTypes } from 'react'
import { ScrollView, View, Text, Button } from 'react-native'
import AppStyles from '../styles'
import AccountPrettyAddress from './AccountPrettyAddress'

const orUnknown = (value = 'Unknown') => value

export default class Send extends Component {
  static propTypes = {
    nextButtonTitle: PropTypes.string.isRequired,
    nextButtonDescription: PropTypes.string.isRequired,
    nextButtonAction: PropTypes.func.isRequired,
    txSenderAddress: PropTypes.string.isRequired,
    txRecipientAddress: PropTypes.string,
    txValue: PropTypes.string,
    txNonce: PropTypes.string,
    txGas: PropTypes.string,
    txGasPrice: PropTypes.string,
    txData: PropTypes.string,
    isSafe: PropTypes.bool.isRequired,
    fetchAccountName: PropTypes.func.isRequired
  }

  render () {
    return (
      <ScrollView style={AppStyles.view}>
        <Text style={AppStyles.hintText}>sender address</Text>
        <AccountPrettyAddress
          address={this.props.txSenderAddress}
          name={this.props.fetchAccountName(this.props.txSenderAddress)}
        />
        <Text style={AppStyles.hintText}>recipient address</Text>
        <AccountPrettyAddress
          address={this.props.txRecipientAddress}
          name={this.props.fetchAccountName(this.props.txRecipientAddress || '')}
        />
        <Text style={AppStyles.hintText}>amount to transfer (in ETH)</Text>
        <Text style={AppStyles.valueText}>{orUnknown(this.props.txValue)}</Text>
        <Text style={AppStyles.hintText}>nonce</Text>
        <Text style={AppStyles.valueText}>{orUnknown(this.props.txNonce)}</Text>
        <Text style={AppStyles.hintText}>gas</Text>
        <Text style={AppStyles.valueText}>{orUnknown(this.props.txGas)}</Text>
        <Text style={AppStyles.hintText}>gasPrice</Text>
        <Text style={AppStyles.valueText}>{orUnknown(this.props.txGasPrice)}</Text>
        <Text style={AppStyles.hintText}>data</Text>
        <Text style={AppStyles.valueText}>{orUnknown(this.props.txData)}</Text>
        {
          !this.props.isSafe
            ? <Text style={AppStyles.hintText}>
              Signing this transaction is unsafe. Proceed only if this transaction comes from trusted source.
            </Text>
            : <View />
        }
        <View style={AppStyles.buttonContainer}>
          <Button
            onPress={() => this.props.nextButtonAction()}
            title={this.props.nextButtonTitle}
            color='green'
            accessibilityLabel={this.props.nextButtonDescription}
        />
        </View>
      </ScrollView>
    )
  }
}
