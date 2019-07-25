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
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';
import colors from '../colors';

export default class Security extends React.PureComponent {
  render() {
    return (
      <ScrollView style={styles.body} contentContainerStyle={{ padding: 20 }}>
        <View style={[styles.card, { backgroundColor: colors.bg_alert, marginBottom: 20 }]}>
          <Icon
            style={[styles.cardText, { marginRight: 10, fontSize: 30 }]}
            name="security"
          />
          <Text style={styles.cardText}>NOT SECURE</Text>
        </View>
        <Text style={styles.text}>
          A device is considered not secure if it has access to the internet or has any king of connectivity enabled.
          Parity Signer is meant to be used on a device that will be kept offline any time. Enabling any kind of connectivity, such as 
          Wifi, Cellular, Bluetooth, NFC, USB is a threat to the safety of the private keys stored on the device.
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
});
