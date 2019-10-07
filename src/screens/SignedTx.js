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
import AccountCard from '../components/AccountCard';
import PayloadDetailsCard from '../components/PayloadDetailsCard';
import TxDetailsCard from '../components/TxDetailsCard';
import QrView from '../components/QrView';
import { NETWORK_LIST, NetworkProtocols, SUBSTRATE_NETWORK_LIST, TX_DETAILS_MSG } from '../constants';
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
      <ScrollView style={styles.b_flex}>
        <Text style={[styles.b_paddingH, styles.header, styles.t_h1]}>Scan Signature</Text>
        <QrView data={data} />
        <Text style={[styles.b_paddingH, styles.b_marginBottom, styles.t_text]}>Transaction Details</Text>
        {
          NETWORK_LIST[sender.networkKey].protocol === NetworkProtocols.ETHEREUM
            ? (
              <React.Fragment>
                <TxDetailsCard
                  style={{ marginBottom: 16 }}
                  description={TX_DETAILS_MSG}
                  value={value}
                  gas={gas}
                  gasPrice={gasPrice}
                />
                <Text style={[styles.b_paddingH, styles.t_text]}>Recipient</Text>
                <AccountCard
                  address={recipient.address}
                  networkKey={recipient.networkKey || ''}
                  title={recipient.name}
                />
              </React.Fragment>
            )
            : (
              <PayloadDetailsCard 
                style={{ marginBottom: 16 }}
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
