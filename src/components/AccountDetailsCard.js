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

// @flow

import React from 'react';
import PropTypes from 'prop-types';
import { View, Text, Platform, StyleSheet, TouchableNativeFeedback, TouchableOpacity } from 'react-native';
import colors from '../colors';
import Card from './Card';
import AccountIcon from './AccountIcon';

export default class AccountDetailsCard extends React.Component<{
  title: string,
  address: string,
  networkId: number,
  onPress: () => any
}> {
  static propTypes = {
    title: PropTypes.string.isRequired,
    address: PropTypes.string.isRequired,
    networkId: PropTypes.number,
    onPress: PropTypes.func
  };

  render() {
    const { title, address, networkId, onPress } = this.props;

    const Touchable = Platform.OS === 'android' ? TouchableNativeFeedback : TouchableOpacity;

    return (
      <Touchable accessibilityComponentType="button" disabled={false} onPress={onPress}>
        <View style={styles.body}>
          <View style={styles.content}>
            <AccountIcon style={styles.icon} seed={'0x' + address} />
            <View style={styles.desc}>
              <Text style={styles.titleText}>{title}</Text>
              <Text style={styles.editText}>Tap to edit account</Text>
            </View>
          </View>
          <View>
            <Text style={styles.addressText}>0x{address}</Text>
          </View>
        </View>
      </Touchable>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    marginBottom: 20,
    padding: 20,
    backgroundColor: colors.card_bg
  },
  content: {
    flexDirection: 'row',
    backgroundColor: colors.card_bg
  },
  icon: {
    width: 70,
    height: 70
  },
  desc: {
    flexDirection: 'column',
    paddingLeft: 10,
    flex: 1
  },
  footer: {
    backgroundColor: '#977CF6',
    flexDirection: 'row-reverse',
    padding: 5
  },
  titleText: {
    fontSize: 20
  },
  editText: {
    paddingTop: 12,
    color: colors.bg_text_sec,
    fontWeight: '500',
    fontSize: 15
  },
  addressText: {
    paddingTop: 20,
    color: colors.bg,
    fontWeight: '700',
    fontSize: 11
  },
  footerText: {
    color: colors.card_bg,
    fontWeight: 'bold'
  }
});
