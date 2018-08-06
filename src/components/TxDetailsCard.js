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
import {
  View,
  Text,
  Platform,
  StyleSheet,
  TouchableNativeFeedback,
  TouchableOpacity,
  ViewPropTypes
} from 'react-native';
import colors from '../colors';
import Card from './Card';
import AccountIcon from './AccountIcon';

const WEI_IN_ETH = 1000000000000000000;

export default class TxDetailsCard extends React.PureComponent<{
  value: string,
  description: string,
  gas: string,
  gasPrice: string,
  style: Object
}> {
  static propTypes = {
    value: PropTypes.string.isRequired,
    description: PropTypes.string.isRequired,
    gas: PropTypes.string.isRequired,
    gasPrice: PropTypes.string.isRequired,
    style: ViewPropTypes.style
  };

  render() {
    const { value, description, gas, gasPrice, style } = this.props;

    return (
      <View style={[styles.body, style]}>
        <Text style={styles.titleText}>{description}</Text>
        <Amount
          style={{ marginTop: 10 }}
          value={value}
          gas={gas}
          gasPrice={gasPrice}
        />
      </View>
    );
  }
}

function Amount({ style, value, gas, gasPrice }) {
  const fee = parseInt(gas) * parseInt(gasPrice) / WEI_IN_ETH;
  return (
    <View style={[{ justifyContent: 'center', alignItems: 'center' }, style]}>
      <View>
        <View
          style={{ padding: 5, paddingVertical: 2, backgroundColor: colors.bg }}
        >
          <Text
            style={{ textAlign: 'center', fontSize: 20, fontWeight: '800' }}
          >
            <Text style={{ color: colors.card_bg }}>{value}</Text>
            <Text style={{ color: colors.bg_text_sec }}> ETH</Text>
          </Text>
        </View>
        <View style={{ padding: 5, backgroundColor: colors.bg_text_sec }}>
          <Text
            style={{
              textAlign: 'center',
              fontSize: 12,
              fontWeight: '800',
              color: colors.card_bg
            }}
          >
            fee: {fee} ETH
          </Text>
        </View>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  body: {
    padding: 20,
    paddingTop: 10,
    flexDirection: 'column',
    backgroundColor: colors.card_bg
  },
  content: {},
  icon: {
    width: 47,
    height: 47
  },
  footer: {
    backgroundColor: '#977CF6',
    flexDirection: 'row-reverse',
    padding: 5
  },
  titleText: {
    textAlign: 'center',
    fontFamily: 'Manifold CF',
    fontSize: 14,
    fontWeight: '600',
    color: colors.card_bg_text
  },
  secondaryText: {
    textAlign: 'center',
    color: colors.card_bg_text,
    fontFamily: 'Manifold CF',
    fontWeight: '500',
    fontSize: 12
  },
  footerText: {
    color: colors.card_bg,
    fontFamily: 'Manifold CF',
    fontWeight: 'bold'
  }
});
