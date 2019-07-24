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

import PropTypes from 'prop-types';
import React from 'react';
import { StyleSheet, Text, View } from 'react-native';
import colors from '../colors';
import { NETWORK_LIST } from '../constants';
import AccountIcon from './AccountIcon';
import TouchableItem from './TouchableItem';

export default class AccountDetailsCard extends React.PureComponent<{
  title: string,
  address: string,
  networkKey: string,
  onPress: () => any
}> {
  static propTypes = {
    title: PropTypes.string.isRequired,
    address: PropTypes.string.isRequired,
    networkKey: PropTypes.string,
    onPress: PropTypes.func
  };

  render() {
    const { title, address, networkKey, onPress } = this.props;
    const network = NETWORK_LIST[networkKey];

    return (
      <TouchableItem
        accessibilityComponentType="button"
        disabled={false}
        onPress={onPress}
      >
        <View style={styles.body}>
          <View style={styles.content}>
            <AccountIcon style={styles.icon} seed={'0x' + address} />
            <View style={styles.desc}>
              <Text numberOfLines={1} style={styles.titleText}>
                {title}
              </Text>
              <Text style={styles.editText}>Tap to edit account</Text>
            </View>
          </View>
          <View>
            <Text
              numberOfLines={1}
              adjustsFontSizeToFit
              minimumFontScale={0.01}
              style={styles.addressText}
            >
              0x{address}
            </Text>
          </View>
        </View>
        <View
          style={[
            styles.footer,
            {
              backgroundColor: network.color
            }
          ]}
        >
          <Text
            style={[
              styles.footerText,
              {
                color: network.secondaryColor
              }
            ]}
          >
            {network.title}
          </Text>
        </View>
      </TouchableItem>
    );
  }
}

const styles = StyleSheet.create({
  body: {
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
    fontSize: 16
  },
  footerText: {
    color: colors.card_bg,
    fontWeight: 'bold'
  }
});
