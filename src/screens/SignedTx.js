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

'use strict';

import React, { Component } from 'react';
import PropTypes from 'prop-types';
import { StyleSheet, View, Text } from 'react-native';
import Background from '../components/Background';
import QrView from '../components/QrView';
import { Subscribe } from 'unstated';
import ScannerStore from '../stores/ScannerStore';
import colors from '../colors';

export default class SignedTx extends Component {
  render() {
    return (
      <Subscribe to={[ScannerStore]}>
        {scanner => {
          return <SignedTxView data={scanner.getSignedTxData()} />;
        }}
      </Subscribe>
    );
  }
}

export class SignedTxView extends Component {
  static propTypes = {
    data: PropTypes.string.isRequired
  };

  render() {
    return (
      <View style={styles.body}>
        <Background />
        <Text style={styles.topTitle}>SIGNED TRANSACTION</Text>
        <View style={styles.qr}>
          <QrView text={this.props.data} />
        </View>
      </View>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    backgroundColor: colors.bg,
    flex: 1,
    flexDirection: 'column',
    padding: 20,
    overflow: 'hidden'
  },
  qr: {
    flex: 1,
    backgroundColor: colors.card_bg
  },
  topTitle: {
    textAlign: 'center',
    color: colors.bg_text_sec,
    fontSize: 24,
    fontFamily: 'Manifold CF',
    fontWeight: 'bold',
    paddingBottom: 20
  }
});
