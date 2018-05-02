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

import React, { Component} from 'react'
import PropTypes from 'prop-types'
import { Text, View, StyleSheet } from 'react-native'
import AppStyles from '../styles'
import AccountIcon from './AccountIcon'
import AccountAddress from './AccountAddress'

export default class AccountPrettyAddress extends Component {
  static propTypes = {
    name: PropTypes.string.isRequired,
    address: PropTypes.string
  }

  render () {
    return (
      <View>
        <View style={styles.row}>
          <AccountIcon style={[AppStyles.icon, styles.icon]} seed={this.props.address ? '0x' + this.props.address : ''} />
          <Text style={{marginTop: 5, marginLeft: 5}}>{this.props.name}</Text>
        </View>
        <AccountAddress address={this.props.address} />
      </View>
    )
  }
}

const styles = StyleSheet.create({
  row: {
    flexDirection: 'row'
  },
  icon: {
    width: 30,
    height: 30,
    borderRadius: 15
  }
})
