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
import filter from 'lodash/filter';
import { ScrollView, StyleSheet, Text } from 'react-native';
import { Subscribe } from 'unstated';
import colors from '../colors';
import TouchableItem from '../components/TouchableItem';
import { ETHEREUM_NETWORK_LIST } from '../constants';
import AccountsStore from '../stores/AccountsStore';

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
    const availableEthereumNetworkList = filter(
      ETHEREUM_NETWORK_LIST,
      'available'
    );
    return (
      <ScrollView style={styles.body} contentContainerStyle={{ padding: 20 }}>
        <Text style={styles.title}>CHOOSE NETWORK</Text>
        { availableEthereumNetworkList.map(network => (
          <TouchableItem
            key={network.ethereumChainId}
            style={[
              styles.card,
              {
                marginTop: 20,
                backgroundColor: network.color
              }
            ]}
            onPress={() => {
              accounts.updateNew({ networkKey: network.ethereumChainId });
              navigation.goBack();
            }}
          >
            <Text
              style={[
                styles.cardText,
                {
                  color: network.secondaryColor
                }
              ]}
            >
              {network.title}
            </Text>
          </TouchableItem>
        ))}
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
  title: {
    fontFamily: 'Manifold CF',
    color: colors.bg_text_sec,
    fontSize: 18,
    fontWeight: 'bold',
    paddingBottom: 20
  },
  card: {
    backgroundColor: colors.card_bg,
    padding: 20
  },
  cardText: {
    color: colors.card_text,
    fontFamily: 'Manifold CF',
    fontSize: 20,
    fontWeight: 'bold'
  }
});
