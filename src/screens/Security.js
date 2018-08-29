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

import React from 'react';
import { Text, View, StyleSheet, ScrollView } from 'react-native';
import { Subscribe } from 'unstated';
import SecurityStore from '../stores/SecurityStore';
import Icon from 'react-native-vector-icons/MaterialIcons';
import colors from '../colors';

export default class Security extends React.PureComponent {
  render() {
    return (
      <Subscribe to={[SecurityStore]}>
        {securityStore => <SecurityView level={securityStore.getLevel()} />}
      </Subscribe>
    );
  }
}

class SecurityView extends React.PureComponent {
  render() {
    const { level } = this.props;
    const backgroundColor = {
      green: colors.bg_positive,
      red: colors.bg_alert
    }[level];

    const message = {
      green: 'Secure',
      red: 'Not Secure'
    }[level];

    return (
      <ScrollView style={styles.body} contentContainerStyle={{ padding: 20 }}>
        <Text style={styles.title}>YOUR DEVICE IS</Text>
        <View style={[styles.card, { backgroundColor, marginBottom: 20 }]}>
          <Icon
            style={[styles.cardText, { marginRight: 10, fontSize: 30 }]}
            name="security"
          />
          <Text style={styles.cardText}>{message}</Text>
        </View>
        <Text style={styles.title}>DEVICE SECURITY</Text>
        <View style={styles.headerContainer}>
          <Icon
            style={[styles.headerSecureIcon, { color: colors.bg_positive }]}
            name="security"
          />
          <Text style={[styles.headerTextRight, { color: colors.bg_positive }]}>
            Secure
          </Text>
        </View>
        <Text style={styles.text}>
          A device is considered secure if it does not have any internet access.
        </Text>
        <View style={styles.headerContainer}>
          <Icon
            style={[styles.headerSecureIcon, { color: colors.bg_alert }]}
            name="security"
          />
          <Text style={[styles.headerTextRight, { color: colors.bg_alert }]}>
            Not Secure
          </Text>
        </View>
        <Text style={styles.text}>
          A device is considered not secure if it has access to the internet. We recommend not keeping high balances
          on this device.
        </Text>
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
  title: {
    color: colors.bg_text_sec,
    fontSize: 18,
    fontFamily: 'Manifold CF',
    fontWeight: 'bold',
    paddingBottom: 20
  },
  card: {
    flex: 1,
    alignItems: 'center',
    flexDirection: 'row',
    backgroundColor: colors.card_bg,
    padding: 20
  },
  cardText: {
    color: colors.card_bg,
    fontFamily: 'Manifold CF',
    fontSize: 22,
    fontWeight: 'bold'
  },
  text: {
    marginBottom: 20,
    fontFamily: 'Roboto',
    fontSize: 14,
    color: colors.card_bg
  },
  headerContainer: {
    marginBottom: 15,
    flexDirection: 'row',
    alignItems: 'center'
  },
  headerSecureIcon: {
    marginLeft: 0,
    fontSize: 20,
    fontWeight: 'bold',
    paddingRight: 5,
    color: colors.bg_text_positive
  },
  headerTextRight: {
    marginLeft: 0,
    fontSize: 17,
    fontFamily: 'Roboto',
    fontWeight: 'bold',
    color: colors.bg_text_positive
  }
});
