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

'use strict';

import React, { Component } from 'react';
import PropTypes from 'prop-types';
import { Alert, ScrollView, View, Text, TouchableOpacity, Share, StyleSheet } from 'react-native';
import { Subscribe } from 'unstated';
import Icon from 'react-native-vector-icons/MaterialIcons';
import AccountsStore from '../stores/AccountsStore';
import AccountSeed from '../components/AccountSeed';
import AccountIconChooser from '../components/AccountIconChooser';
import TextInput from '../components/TextInput';
import TouchableItem from '../components/TouchableItem';
import colors from '../colors';

export default class AccountAdd extends Component {
  static navigationOptions = {
    title: 'Add Account',
    headerBackTitle: 'Back'
  };
  render() {
    return (
      <Subscribe to={[AccountsStore]}>{accounts => <AccountAddView {...this.props} accounts={accounts} />}</Subscribe>
    );
  }
}

class AccountAddView extends Component {
  render() {
    return (
      <View style={styles.body}>
        <TouchableItem style={styles.card} onPress={() => this.props.navigation.navigate('AccountNew')}>
          <Icon style={{ textAlign: 'center', color: colors.card_text, fontSize: 66 }} name="layers" />
          <Text style={[styles.cardText, { marginTop: 20 }]}>Create New Account</Text>
        </TouchableItem>
        <TouchableItem
          style={[styles.card, { marginTop: 20 }]}
          onPress={() => this.props.navigation.navigate('AccountRecover')}
        >
          <Icon style={{ textAlign: 'center', color: colors.card_text, fontSize: 66 }} name="graphic-eq" />
          <Text style={[styles.cardText, { marginTop: 20 }]}>Recover Account</Text>
        </TouchableItem>
        <TouchableItem style={[styles.card, { marginTop: 20 }]}>
          <Text style={styles.cardText}>About Accounts</Text>
        </TouchableItem>
      </View>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    backgroundColor: colors.bg,
    padding: 20,
    flex: 1,
    flexDirection: 'column'
  },
  top: {
    flex: 1
  },
  bottom: {
    flexBasis: 50,
    paddingBottom: 15
  },
  titleTop: {
    fontFamily: 'Roboto',
    color: colors.bg_text_sec,
    fontSize: 24,
    fontWeight: 'bold',
    paddingBottom: 20,
    textAlign: 'center'
  },
  card: {
    backgroundColor: colors.card_bg,
    padding: 20
  },
  cardText: {
    textAlign: 'center',
    color: colors.card_text,
    fontFamily: 'Roboto',
    fontSize: 20,
    fontWeight: 'bold'
  }
});
