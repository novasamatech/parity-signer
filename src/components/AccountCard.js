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
import { StyleSheet, Text, View, ViewPropTypes } from 'react-native';
import colors from '../colors';
import { NETWORK_LIST } from '../constants';
import AccountIcon from './AccountIcon';
import TouchableItem from './TouchableItem';

export default class AccountCard extends React.PureComponent<{
  address: string,
  chainId: string,
  onPress: () => any,
  title?: string,
  seedType?: string
}> {
  static propTypes = {
    address: PropTypes.string.isRequired,
    chainId: PropTypes.string,
    onPress: PropTypes.func,
    seedType: PropTypes.string,
    style: ViewPropTypes.style,
    title: PropTypes.string
  };

  static defaultProps = {
    title: 'no name'
  };

  render() {
    const { address, chainId, onPress, seedType, style } = this.props;
    let { title } = this.props;
    title = title.length ? title : AccountCard.defaultProps.title;
    const seedTypeDisplay = seedType || '';
    const network = NETWORK_LIST[chainId];

    return (
      <TouchableItem
        accessibilityComponentType="button"
        disabled={false}
        onPress={onPress}
      >
        <View style={[styles.body, style]}>
          <View style={styles.content}>
            <AccountIcon style={styles.icon} seed={'0x' + address} />
            <View style={styles.desc}>
              <Text numberOfLines={1} style={styles.titleText}>
                {title}
              </Text>
              <Text
                numberOfLines={1}
                adjustsFontSizeToFit
                minimumFontScale={0.01}
                style={styles.secondaryText}
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
                styles.footerSeedType,
                {
                  color: network.secondaryColor
                }
              ]}
            >
              {seedTypeDisplay}
            </Text>
            <Text
              style={[
                styles.footerNetwork,
                {
                  color: network.secondaryColor
                }
              ]}
            >
              {network.title}
            </Text>
          </View>
        </View>
      </TouchableItem>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    paddingBottom: 20
  },
  content: {
    flexDirection: 'row',
    backgroundColor: colors.card_bg,
    padding: 10
  },
  icon: {
    width: 47,
    height: 47
  },
  desc: {
    flexDirection: 'column',
    justifyContent: 'space-between',
    paddingLeft: 10,
    flex: 1
  },
  footer: {
    backgroundColor: '#977CF6',
    flexDirection: 'row',
    justifyContent: 'space-between',
    padding: 5
  },
  titleText: {
    fontFamily: 'Roboto',
    fontSize: 20
  },
  secondaryText: {
    fontFamily: 'Roboto',
    color: colors.bg_text_sec,
    fontWeight: '500',
    fontSize: 14
  },
  footerSeedType: {
    fontFamily: 'Roboto',
    color: colors.card_bg
  },
  footerNetwork: {
    fontFamily: 'Roboto',
    color: colors.card_bg,
    fontWeight: 'bold'
  }
});
