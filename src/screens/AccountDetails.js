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
import { StyleSheet, View, ScrollView, Text, TextInput, TouchableOpacity } from 'react-native'
import { Subscribe } from 'unstated'
import AccountsStore from '../stores/AccountsStore'
import AppStyles from '../styles'
import AccountIcon from '../components/AccountIcon'
import AccountDetailsCard from '../components/AccountDetailsCard'
import QrView from '../components/QrView'
import Button from '../components/Button'
import colors from '../colors';

export default class AccountDetails extends Component {
  static navigationOptions = {
    title: 'Account Details'
  }

  render () {
    const { account: { address, name } } = this.props;
    return (
      <Subscribe to={[AccountsStore]}>{
        (accounts) => {
          const account = accounts.getSelected()
          return (
            <View style={ styles.body }>
              <Text style={ styles.title }>ACCOUNT</Text>
              <AccountDetailsCard address={ account.address } title={ account.name } />
              <Button textStyles={{color: colors.card_bg_text}}
                      buttonStyles={{ backgroundColor: colors.card_bg }}
                      title="Show Account QR Code"
                      onPress={ () => {} }/>
            </View>
        )
        }
      }
      </Subscribe>
    )
  }
}

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
  wrapper: {
    borderRadius: 5
  },
  address: {
    flex: 1
  },
  qr: {
    flex: 1,
    padding: 10,
    marginTop: 50
  },
  deleteText: {
    textAlign: 'right'
  },
  changePinText: {
    textAlign: 'left',
    color: 'green'
  },
  actionsContainer: {
    flex: 1,
    flexDirection: 'row'
  },
  actionButtonContainer: {
    flex: 1
  }
})
