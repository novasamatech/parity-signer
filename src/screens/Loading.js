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
import { View, Text, StyleSheet } from 'react-native';
import { StackActions, NavigationActions } from 'react-navigation';
import { Subscribe } from 'unstated';
import { loadAccounts, loadAccounts_v1, saveAccount } from '../util/db';
import AccountsStore from '../stores/AccountsStore';
import colors from '../colors';
import { empty } from '../util/account';

export default class Loading extends React.PureComponent {
  static navigationOptions = {
    title: 'Add Account',
    headerBackTitle: 'Back'
  };

  async componentDidMount() {
    let accounts = await loadAccounts();
    if (0 === accounts.length) {
      // Try to migrate v1 accounts
      const oldAccounts = await loadAccounts_v1();

      accounts = oldAccounts.map(empty).map(a => ({ ...a, v1recov: true }));
      accounts.forEach(saveAccount);
    }

    if (accounts.filter(a => !a.archived).length) {
      const resetAction = StackActions.reset({
        index: 0,
        actions: [NavigationActions.navigate({ routeName: 'Tabs' })]
      });
      this.props.navigation.dispatch(resetAction);
    } else {
      const resetAction = StackActions.reset({
        index: 0,
        actions: [NavigationActions.navigate({ routeName: 'Welcome' })]
      });
      this.props.navigation.dispatch(resetAction);
    }
  }

  render() {
    return <View style={styles.body} />;
  }
}

const styles = StyleSheet.create({
  body: {
    backgroundColor: colors.bg,
    padding: 20,
    flex: 1,
    flexDirection: 'column'
  },
  titleTop: {
    fontFamily: 'Manifold CF',
    color: colors.bg_text_sec,
    fontSize: 24,
    fontWeight: 'bold',
    paddingBottom: 20,
    textAlign: 'center'
  }
});
