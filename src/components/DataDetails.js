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
import { View, Button, Text } from 'react-native'
import AppStyles from '../styles'

function isAscii (data) {
  for (var i = 2; i < data.length; i += 2) {
    let n = parseInt(data.substr(i, 2), 16)

    if (n < 32 || n >= 128) {
      return false
    }
  }
  return true
}

function hexToAscii (hexx) {
  var hex = hexx.toString()
  var str = ''
  for (var i = 0; i < hex.length; i += 2) {
    str += String.fromCharCode(parseInt(hex.substr(i, 2), 16))
  }
  return str
}

export default class DataDetails extends Component {
  static propTypes = {
    data: PropTypes.string.isRequired,
    onNextButtonPressed: PropTypes.func.isRequired
  }

  render () {
    return (
      <View style={AppStyles.view}>
        <Text style={AppStyles.hintText}>Data to sign</Text>
        <Text style={AppStyles.valueText}>{ isAscii(this.props.data) ? hexToAscii(this.props.data) : this.props.data }</Text>
        <Button
          onPress={this.props.onNextButtonPressed}
          title='Next'
          color='gree'
          accessibilityLabel='Enter Pin'
        />
      </View>
    )
  }
}
