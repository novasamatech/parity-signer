// @flow

import React from 'react'
import PropTypes from 'prop-types'
import { View, Text, Platform, StyleSheet,
  TouchableNativeFeedback, TouchableOpacity} from 'react-native'
import colors from '../colors'
import Card from './Card'
import AccountIcon from './AccountIcon'

export default class TxDetailsCard extends React.Component<{
  value: string,
  recipient: string,
  networkId: number
}> {
  static propTypes = {
    value: PropTypes.string.isRequired,
    recipient: PropTypes.string.isRequired,
    networkId: PropTypes.number
  };

  render() {
    const {
      value,
      recipient,
      networkId
    } = this.props;

    return (
      <View style={styles.body}>
        <Text style={styles.titleText}>You are about to send</Text>
        <Text style={styles.secondaryText}>{value}</Text>
        <Text style={styles.titleText}>To the following address</Text>
        <Text style={styles.secondaryText}>0x{recipient}</Text>
        <AccountIcon style={styles.icon} seed={recipient} />
      </View>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    padding: 20,
    flexDirection: 'column',
    backgroundColor: colors.card_bg,
  },
  content: {

  },
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
    color: colors.card_bg_text,
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
