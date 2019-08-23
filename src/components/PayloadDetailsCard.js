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

import colors from '../colors';
import fonts from '../fonts';

export default class PayloadDetailsCard extends React.PureComponent {
  static propTypes = {
    description: PropTypes.string.isRequired,
    payload: PropTypes.object,
    signature: PropTypes.string,
    style: ViewPropTypes.style
  };

  render() {
    const { description, payload, signature, style } = this.props;
    
    return (
      <View style={[styles.body, style]}>
        <Text style={styles.titleText}>{description}</Text>
        {
          payload && (
            <View style={{ padding: 5, paddingVertical: 2 }}>
              <ExtrinsicPart label='Block Hash' value={payload.blockHash.toString()} />
              <ExtrinsicPart label='Method' value={payload.method.toString()} />
              <ExtrinsicPart label='Era' value={payload.era.toString()} />
              <ExtrinsicPart label='Nonce' value={payload.nonce.toString()} />
              <ExtrinsicPart label='Tip' value={payload.tip.toString()} />
              <ExtrinsicPart label='Genesis Hash' value={payload.genesisHash.toString()} />
            </View>
          )
        }
        {
          signature && (
            <View style={{ padding: 5, paddingVertical: 2 }}>
              <Text style={styles.label}>Signature</Text>
              <Text style={styles.secondaryText}>{signature}</Text>
            </View>
          )
        }
      </View>
    );
  }
}

function ExtrinsicPart({ label, style, value }) {

  return (
    <View style={[{ justifyContent: 'center', alignItems: 'flex-start' }, style]}>
      <View
        style={{ padding: 5, paddingVertical: 2 }}
      >
        <Text style={styles.label}>
          {label}
        </Text>
        <Text style={styles.secondaryText}>{value}</Text>
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
  footer: {
    backgroundColor: '#977CF6',
    flexDirection: 'row-reverse',
    padding: 5
  },
  label: {
    backgroundColor: colors.bg,
    color: colors.card_bg,
    textAlign: 'left', 
    fontSize: 20, 
    fontFamily: fonts.bold,
  },
  icon: {
    width: 47,
    height: 47
  },
  titleText: {
    textAlign: 'center',
    fontFamily: fonts.bold,
    fontSize: 14,
    color: colors.card_bg_text
  },
  secondaryText: {
    textAlign: 'center',
    color: colors.card_bg_text,
    fontFamily: fonts.semiBold,
    fontSize: 12
  },
  footerText: {
    color: colors.card_bg,
    fontFamily: fonts.bold
  }
});
