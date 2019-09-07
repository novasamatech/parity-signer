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
import AccountCard from '../components/AccountCard';
import PayloadDetailsCard from '../components/PayloadDetailsCard';
import TxDetailsCard from '../components/TxDetailsCard';
import QrView from '../components/QrView';
import { NETWORK_LIST, NetworkProtocols, SUBSTRATE_NETWORK_LIST, TX_DETAILS_MSG } from '../constants';
import fonts from '../fonts';
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';

export default class SignedTx extends React.PureComponent {
  render() {
    return (
      <Subscribe to={[ScannerStore, AccountsStore]}>
        {(scanner, accounts) => {
          return (
            <SignedTxView
              {...scanner.getTx()}
              data={scanner.getSignedTxData()}
              recipient={scanner.getRecipient()}
              sender={scanner.getSender()}
            />
          );
        }}
      </Subscribe>
    );
  }
}

export class SignedTxView extends React.PureComponent {
  static propTypes = {
    data: PropTypes.string.isRequired,
    gas: PropTypes.string,
    gasPrice: PropTypes.string,
    recipient: PropTypes.object,
    sender: PropTypes.object,
    value: PropTypes.string,
  };

  render() {
    const { data, gas, gasPrice, recipient, sender, value } = this.props;
    
    return (
      <ScrollView style={styles.body} contentContainerStyle={{ padding: 20 }}>
        <Text style={styles.topTitle}>SCAN SIGNATURE</Text>
        <View style={styles.qr}>
          <QrView data={data} />
        </View>
        <Text style={styles.title}>TRANSACTION DETAILS</Text>
        {
          NETWORK_LIST[sender.networkKey].protocol === NetworkProtocols.ETHEREUM
            ? (
              <React.Fragment>
                <TxDetailsCard
                  style={{ marginBottom: 20 }}
                  description={TX_DETAILS_MSG}
                  value={value}
                  gas={gas}
                  gasPrice={gasPrice}
                />
                <Text style={styles.title}>RECIPIENT</Text>
                <AccountCard
                  address={recipient.address}
                  networkKey={recipient.networkKey || ''}
                  title={recipient.name}
                />
              </React.Fragment>
            )
            : (
              <PayloadDetailsCard 
                style={{ marginBottom: 20 }}
                description={TX_DETAILS_MSG}
                protocol={SUBSTRATE_NETWORK_LIST[sender.networkKey].protocol}
                prefix={SUBSTRATE_NETWORK_LIST[sender.networkKey].prefix}
                signature={data}
              />
            )
        }   
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
  }
});
