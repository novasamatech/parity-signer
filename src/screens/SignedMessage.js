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
import styles from '../styles';
import QrView from '../components/QrView';
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';
import { hexToAscii, isAscii } from '../util/message';

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
      <ScrollView style={styles.b_flex}>
        <View style={styles.b_paddingH}>
          <Text style={[styles.header, styles.t_h1]}>Scan Signature</Text>
        </View>
        <QrView data={this.props.data} />
        <View style={[styles.b_paddingH, styles.b_marginBottom]}>
          <Text style={[styles.t_text, styles.b_marginV_xs]}>Message</Text>
          <Text style={[styles.t_parityS, styles.seedText]}>
            {isAscii(message)
              ? hexToAscii(message)
              : data}
          </Text>
        </View>
      </ScrollView>
    );
  }
}
