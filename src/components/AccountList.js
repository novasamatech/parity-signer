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
import { Button, View, Text, ListView, StatusBar, StyleSheet } from 'react-native'
import AccountListRow from './AccountListRow'
import AppStyles from '../styles'

export default class AccountList extends Component {
  static propTypes = {
    accounts: PropTypes.arrayOf(PropTypes.shape({
      address: PropTypes.string.isRequired
    })).isRequired,
    onNewAccount: PropTypes.func.isRequired,
    onAccountSelected: PropTypes.func.isRequired
  }

  constructor (props) {
    super(props)
    const ds = new ListView.DataSource({rowHasChanged: (r1, r2) => r1 !== r2})
    this.state = {
      dataSource: ds.cloneWithRows(props.accounts)
    }
  }

  componentWillReceiveProps (nextProps) {
    this.setState({
      dataSource: this.state.dataSource.cloneWithRows(nextProps.accounts)
    })
  }

  render () {
    if (!this.props.accounts.length) {
      return (
        <View style={AppStyles.view}>
          <View style={styles.introContainer}>
            <Text style={styles.introText}>
              To sign transactions you need at least one account.
            </Text>
            <View style={AppStyles.buttonContainer}>
              <Button
                style={styles.introButton}
                onPress={this.props.onNewAccount}
                color='green'
                title='Create Account'
                accessibilityLabel='Create new account.'
              />
            </View>
          </View>
        </View>
      )
    }
    return (
      <ListView
        style={AppStyles.listView}
        dataSource={this.state.dataSource}
        renderRow={(rowData, sectionID: number, rowID: number, highlightRow) => {
          return (
            <AccountListRow
              upperText={rowData.name ? rowData.name : 'no name'}
              lowerText={'0x' + rowData.address}
              onPress={() => {
                highlightRow(sectionID, rowID)
                this.props.onAccountSelected(this.props.accounts[rowID])
              }}
            />
          )
        }}
        enableEmptySections
      >
        <StatusBar barStyle='light-content' />
      </ListView>
    )
  }
}

const styles = StyleSheet.create({
  introContainer: {
    padding: 30,
    flex: 1,
    flexDirection: 'column',
    justifyContent: 'center'
  },
  introText: {
    textAlign: 'center',
    fontSize: 16,
    marginBottom: 20
  }
})
