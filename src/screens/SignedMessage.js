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

'use strict';

import PropTypes from 'prop-types';
import React from 'react';
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import { Subscribe } from 'unstated';
import colors from '../colors';
import fonts from "../fonts";
import QrView from '../components/QrView';
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';
import { isAscii } from '../util/message';

export default class SignedMessage extends React.PureComponent {
  render() {
    return (
      <Subscribe to={[ScannerStore, AccountsStore]}>
        {(scanner, accounts) => {
          return (
            <SignedMessageView
              data={scanner.getSignedTxData()}
              message={scanner.getMessage()}
            />
          );
        }}
      </Subscribe>
    );
  }
}

export class SignedMessageView extends React.PureComponent {
  static propTypes = {
    data: PropTypes.string.isRequired
  };

  render() {
    const { data, message } = this.props;

    return (
      <ScrollView style={styles.body} contentContainerStyle={{ padding: 20 }}>
        <Text style={styles.topTitle}>SCAN SIGNATURE</Text>
        <View style={styles.qr}>
          <QrView data={this.props.data} />
        </View>
        <Text style={styles.title}>MESSAGE</Text>
        <Text style={styles.message}>
          {isAscii(message)
            ? message
            : data}
        </Text>
      </ScrollView>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    backgroundColor: colors.bg,
    flex: 1,
    flexDirection: 'column',
    overflow: 'hidden'
  },
  qr: {
    marginBottom: 20,
    backgroundColor: colors.card_bg
  },
  topTitle: {
    textAlign: 'center',
    color: colors.bg_text_sec,
    fontSize: 24,
    fontFamily: fonts.bold,
    paddingBottom: 20
  },
  title: {
    color: colors.bg_text_sec,
    fontSize: 18,
    fontFamily: fonts.bold,
    paddingBottom: 20
  },
  message: {
    marginBottom: 20,
    padding: 10,
    height: 120,
    lineHeight: 26,
    fontFamily: fonts.regular,
    fontSize: 20,
    backgroundColor: colors.card_bg
  }
});
