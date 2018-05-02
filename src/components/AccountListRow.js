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
import { TouchableHighlight, StyleSheet, View, Text } from 'react-native'
import AppStyles from '../styles'
import AccountIcon from './AccountIcon'

export default class AccountListRow extends Component {
  static propTypes = {
    upperText: PropTypes.string.isRequired,
    lowerText: PropTypes.string.isRequired,
    onPress: PropTypes.func.isRequired
  }

  render () {
    return (
      <TouchableHighlight style={styles.row} onPress={this.props.onPress} underlayColor='#0004'>
        <View style={{flexDirection: 'column'}}>
          <View style={styles.innerRow}>
            <AccountIcon style={[AppStyles.icon, styles.icon]} seed={this.props.lowerText} />
            <View style={styles.accountDetails}>
              <Text style={styles.upperText} ellipsizeMode='middle' numberOfLines={1}>{this.props.upperText}</Text>
              <Text style={styles.lowerText} ellipsizeMode='middle' numberOfLines={1}>{this.props.lowerText}</Text>
            </View>
          </View>
          <View style={{height: 1, backgroundColor: '#ccc'}} />
          <View style={{height: 1, backgroundColor: '#ddd'}} />
        </View>
      </TouchableHighlight>
    )
  }
}

const styles = StyleSheet.create({
  row: {
    backgroundColor: '#F8F8F8'
  },
  innerRow: {
    padding: 5,
    flexDirection: 'row'
  },
  accountDetails: {
    flexDirection: 'column',
    justifyContent: 'center'
  },
  icon: {
    height: 60,
    width: 60,
    borderRadius: 30,
    marginRight: 10,
    marginBottom: 0
  },
  upperText: {
    fontSize: 16,
    color: '#888'
  },
  lowerText: {
    marginTop: 5,
    color: '#aaa',
    fontSize: 10
  }
})
