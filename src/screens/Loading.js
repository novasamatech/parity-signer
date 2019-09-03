// Copyright 2015-2019 Parity Technologies (UK) Ltd.
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
import { StyleSheet, View } from 'react-native';
import { NavigationActions, StackActions } from 'react-navigation';

import colors from '../colors';
import { empty } from '../util/account';
import { loadAccounts, loadToCAndPPConfirmation, saveAccount } from '../util/db';

export default class Loading extends React.PureComponent {
  static navigationOptions = {
    title: 'Add Account',
    headerBackTitle: 'Back'
  };

  async componentDidMount() {
    const tocPP = await loadToCAndPPConfirmation();
    const firstScreen = 'Welcome';
    const firstScreenActions = StackActions.reset({
      index: 0,
      actions: [NavigationActions.navigate({ routeName: firstScreen })],
      key: null
    });
    let tocActions;

    if (!tocPP) {
      this.migrateAccount_v1();
      this.migrateAccount_v2();

      tocActions = StackActions.reset({
        index: 0,
        actions: [
          NavigationActions.navigate({
            routeName: 'TocAndPrivacyPolicy',
            params: {
              firstScreenActions
            }
          })
        ]
      });
    } else {
      tocActions = firstScreenActions;
    }
    
    await loadAccounts()
    this.props.navigation.dispatch(tocActions);
  }

  async migrateAccount_v1 () {
    const oldAccounts_v1 = await loadAccounts(1);
    const accounts = oldAccounts_v1.map(empty).map(a => ({ ...a, v1recov: true }));
    accounts.forEach(saveAccount);
  }

  // only ethereum account with chainId and networkType properties
  async migrateAccount_v2 () {
    const oldAccounts_v2 = await loadAccounts(2);
    const accounts = oldAccounts_v2.map(empty).map(a => {
      let result = {}
      if (a.chainId) {
        result = { ...a, networkKey: a.chainId, v2recov: true };
        delete result.chainId;
        delete result.networkType;
      }
      return result
    })
    accounts.forEach(saveAccount);
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
  }
});
