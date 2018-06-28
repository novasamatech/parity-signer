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

import React from 'react';
import PropTypes from 'prop-types';
import {
  Alert,
  ScrollView,
  View,
  Text,
  TouchableOpacity,
  Share,
  StyleSheet
} from 'react-native';
import { Subscribe } from 'unstated';
import Icon from 'react-native-vector-icons/MaterialIcons';
import AccountsStore from '../stores/AccountsStore';
import Background from '../components/Background';
import AccountSeed from '../components/AccountSeed';
import AccountIconChooser from '../components/AccountIconChooser';
import TextInput from '../components/TextInput';
import TouchableItem from '../components/TouchableItem';
import colors from '../colors';

export default class AccountAdd extends React.PureComponent {
  static navigationOptions = {
    title: 'Add Account',
    headerBackTitle: 'Back'
  };

  render() {
    return (
      <Subscribe to={[AccountsStore]}>
        {accounts => <AccountAddView {...this.props} accounts={accounts} />}
      </Subscribe>
    );
  }
}

class AccountAddView extends React.PureComponent {
  componentWillMount() {
    this.props.navigation.addListener('willFocus', () => {
      this.props.accounts.resetNew();
    });
  }

  render() {
    const { navigation } = this.props;
    const isWelcome = navigation.getParam('isWelcome');
    return (
      <ScrollView style={styles.body} contentContainerStyle={{ padding: 20 }}>
        <Background />
        {isWelcome && <Text style={styles.titleTop}>GETTING STARTED</Text>}
        <TouchableItem
          style={styles.card}
          onPress={() => navigation.navigate('AccountNew', { isWelcome })}
        >
          <Icon
            style={{
              textAlign: 'center',
              color: colors.card_text,
              fontSize: 66
            }}
            name="layers"
          />
          <Text style={[styles.cardText, { marginTop: 20 }]}>
            Create New Account
          </Text>
        </TouchableItem>
        <TouchableItem
          style={[styles.card, { marginTop: 20 }]}
          onPress={() => navigation.navigate('AccountRecover', { isWelcome })}
        >
          <Icon
            style={{
              textAlign: 'center',
              color: colors.card_text,
              fontSize: 66
            }}
            name="graphic-eq"
          />
          <Text style={[styles.cardText, { marginTop: 20 }]}>
            Recover Account
          </Text>
        </TouchableItem>
        <TouchableItem style={[styles.card, { marginTop: 20 }]}>
          <Text style={styles.cardText}>About Accounts</Text>
        </TouchableItem>
      </ScrollView>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    flex: 1,
    flexDirection: 'column',
    overflow: 'hidden',
    backgroundColor: colors.bg
  },
  top: {
    flex: 1
  },
  bottom: {
    flexBasis: 50,
    paddingBottom: 15
  },
  titleTop: {
    color: colors.bg_text_sec,
    fontSize: 24,
    fontFamily: 'Manifold CF',
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
    fontFamily: 'Manifold CF',
    fontSize: 20,
    fontWeight: 'bold'
  }
});
