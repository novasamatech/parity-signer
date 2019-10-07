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
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import { Subscribe } from 'unstated';
import styles from '../styles';
import TouchableItem from '../components/TouchableItem';
import { NETWORK_LIST, UnknownNetworkKeys } from '../constants';
import AccountsStore from '../stores/AccountsStore';
import { empty } from '../util/account';

export default class AccountNetworkChooser extends React.PureComponent {
  static navigationOptions = {
    title: 'Choose a network',
    headerBackTitle: 'Back'
  };
  render() {
    return (
      <Subscribe to={[AccountsStore]}>
        {accounts => (
          <AccountNetworkChooserView {...this.props} accounts={accounts} />
        )}
      </Subscribe>
    );
  }
}

class AccountNetworkChooserView extends React.PureComponent {
  render() {
    const { navigation } = this.props;
    const { accounts } = this.props;

    return (
      <ScrollView style={styles.b_flex}>
        <View style={styles.b_paddingH}>
          <Text style={[styles.t_h1, styles.header]}></Text>
          <Text style={styles.t_text}>Choose network</Text>
        </View>
        { Object.entries(NETWORK_LIST)
          .filter(([networkKey]) => networkKey !== UnKnownNetworkKeys.UNKNOWN )
          .map(([networkKey, networkParams]) => (
            <TouchableItem
              key={networkKey}
              style={[
                styles.card,
                {
                  backgroundColor: networkParams.color
                }
              ]}
              onPress={() => {
                accounts.updateNew(empty('', networkKey));
                navigation.goBack();
              }}
            >
              <Text
                style={[
                  styles.t_h2,
                  styles.t_bold,
                  styles.t_center,
                  {
                    color: networkParams.secondaryColor
                  }
                ]}
              >
                {networkParams.title}
              </Text>
            </TouchableItem>
        ))}
      </ScrollView>
    );
  }
}
