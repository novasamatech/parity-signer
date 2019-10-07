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

// @flow

import PropTypes from 'prop-types';
import React from 'react';
import { StyleSheet, Text, View, ViewPropTypes } from 'react-native';

import AccountIcon from './AccountIcon';
import Address from './Address';
import styles from '../styles';
import { NETWORK_LIST, NetworkProtocols } from '../constants';
import TouchableItem from './TouchableItem';


export default class AccountCard extends React.PureComponent {
  static propTypes = {
    address: PropTypes.string.isRequired,
    networkKey: PropTypes.string,
    onPress: PropTypes.func,
    seedType: PropTypes.string,
    style: ViewPropTypes.style,
    title: PropTypes.string
  };

  static defaultProps = {
    title: 'no name',
    onPress: () => {}
  };

  render() {
    const { address, networkKey, onPress, seedType, style } = this.props;
    let { title } = this.props;
    title = title.length ? title : AccountCard.defaultProps.title;
    const seedTypeDisplay = seedType || '';
    const network = NETWORK_LIST[networkKey] || NETWORK_LIST[NetworkProtocols.UNKNOWN];

    return (
      <TouchableItem
        accessibilityComponentType="button"
        disabled={false}
        onPress={onPress}
      >
        <View style={[styles.b_row, {borderBottomWidth: 1, borderBottomColor: 'black'}]}>
          <View style={[{flex:1, alignItems:'center'}, styles.b_row]}>
            <View style={styles.b_paddingH}>
              <AccountIcon
                address={address}
                protocol={network.protocol}
                style={[styles.el_icon, styles.b_paddingH]}
              />
            </View>
            <View style={[styles.b_flex, {justifyContent:'space-between'}]}>
              <Address
                style={[styles.t_text, styles.t_color_sec]}
                address={address}
                protocol={network.protocol}
              />
              <View style={[styles.b_row, {alignItems:'center'}]}>
                <Text style={[styles.t_h2, styles.b_marginV_xs]}>{title}</Text>
                <Text style={[styles.t_text]}>{network.title}</Text>
              </View>
              <View style={[styles.b_row]}>
                <Text style={[styles.t_text, styles.t_color_sec]}>{seedTypeDisplay}</Text>
              </View>

              </View>
              <View
                style={[
                  {
                    width: 8,
                    height: 80,
                    marginLeft: 8,
                    backgroundColor: network.color
                  }
                ]}
                >
              </View>
            </View>
          </View>
      </TouchableItem>
    );
  }
}
