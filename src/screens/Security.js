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

import React from 'react';
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';
import styles from '../styles';
import colors from '../colors';

export default class Security extends React.PureComponent {
  render() {
    return (
      <ScrollView style={styles.b_flex}>
        <View style={[styles.card, { backgroundColor: colors.bg_alert, marginBottom: 16}]}>
          <Icon
            style={[styles.t_h2, styles.t_bold, styles. t_center]}
            name="security"
          />
          <Text style={[styles.t_h2, styles.t_bold, styles. t_center]}>NOT SECURE</Text>
        </View>
        <View style={styles.b_paddingH}>
          <Text style={[styles.t_text, styles.t_light]}>
            A device is considered not secure if it has access to the internet or has any kind of connectivity enabled.
            Parity Signer is meant to be used on a device that will be kept offline at any time. Enabling any connectivity such as 
            wifi, cellular network, bluetooth, NFC, usb is a threat to the safety of the private keys stored on the device.
          </Text>
        </View>
      </ScrollView>
    );
  }
}
