// @flow

import React from 'react';
import PropTypes from 'prop-types';
import { View, Text, Platform, StyleSheet, TouchableNativeFeedback, TouchableOpacity } from 'react-native';
import colors from '../colors';
import Card from './Card';
import AccountIcon from './AccountIcon';

const WEI_IN_ETH = 1000000000000000000;

export default class TxDetailsCard extends React.Component<{
  value: string,
  recipient: string,
  networkId: number
}> {
  static propTypes = {
    value: PropTypes.string.isRequired,
    gas: PropTypes.string.isRequired,
    gasPrice: PropTypes.string.isRequired,
    style: View.propTypes.style
  };

  render() {
    const { value, gas, gasPrice, recipient, networkId, style } = this.props;

    return (
      <View style={[styles.body, style]}>
        <Text style={styles.titleText}>You are about to send</Text>
        <Amount style={{ marginTop: 10 }} value={value} gas={gas} gasPrice={gasPrice} />
      </View>
    );
  }
}

function Amount({ style, value, gas, gasPrice }) {
  const fee = parseInt(gas) * parseInt(gasPrice) / WEI_IN_ETH;
  return (
    <View style={[{ justifyContent: 'center', alignItems: 'center' }, style]}>
      <View>
        <View style={{ padding: 5, paddingVertical: 2, backgroundColor: colors.bg }}>
          <Text style={{ textAlign: 'center', fontSize: 20, fontWeight: '800' }}>
            <Text style={{ color: colors.card_bg }}>{value}</Text>
            <Text style={{ color: colors.bg_text_sec }}> ETH</Text>
          </Text>
        </View>
        <View style={{ padding: 5, backgroundColor: colors.bg_text_sec }}>
          <Text style={{ textAlign: 'center', fontSize: 10, fontWeight: '800', color: colors.card_bg }}>
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
    fontSize: 20,
    fontWeight: '700',
    color: colors.card_bg_text
  },
  secondaryText: {
    textAlign: 'center',
    color: colors.card_bg_text,
    fontWeight: '500',
    fontSize: 10
  },
  footerText: {
    color: colors.card_bg,
    fontWeight: 'bold'
  }
});
