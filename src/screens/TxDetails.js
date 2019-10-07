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
import { NETWORK_LIST } from '../constants';
import fonts from "../fonts";
import AccountCard from '../components/AccountCard';
import Background from '../components/Background';
import Button from '../components/Button';
import TxDetailsCard from '../components/TxDetailsCard';
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';
import { NetworkProtocols, SUBSTRATE_NETWORK_LIST } from '../constants';
import PayloadDetailsCard from '../components/PayloadDetailsCard';

export default class TxDetails extends React.PureComponent {
  static navigationOptions = {
    title: 'Transaction Details',
    headerBackTitle: 'Transaction details'
  };
  render() {
    return (
      <Subscribe to={[ScannerStore, AccountsStore]}>
        {(scannerStore) => {
          const txRequest = scannerStore.getTXRequest();

          if (txRequest) {
            const tx = scannerStore.getTx();

            return (
              <TxDetailsView
                {...{ ...this.props, ...tx }}
                scannerStore={scannerStore}
                sender={scannerStore.getSender()}
                recipient={scannerStore.getRecipient()}
                dataToSign={scannerStore.getDataToSign()}
                onNext={async () => {
                  try {
                    this.props.navigation.navigate('AccountUnlockAndSign');
                  } catch (e) {
                    scannerStore.setErrorMsg(e.message);
                  }
                }}
              />
            );
          } else {
            return null;
          }
        }}
      </Subscribe>
    );
  }
}

export class TxDetailsView extends React.PureComponent {
  static propTypes = {
    onNext: PropTypes.func.isRequired,
    dataToSign: PropTypes.oneOfType([PropTypes.string, PropTypes.object]).isRequired,
    sender: PropTypes.object.isRequired,
    recipient: PropTypes.object.isRequired,
    value: PropTypes.string,
    nonce: PropTypes.string,
    gas: PropTypes.string,
    gasPrice: PropTypes.string
  };

  render() {
    const { dataToSign, sender, recipient, value, nonce, gas, gasPrice, onNext } = this.props;

    const isEthereum = NETWORK_LIST[sender.networkKey].protocol === NetworkProtocols.ETHEREUM;
    const prefix = !isEthereum && SUBSTRATE_NETWORK_LIST[sender.networkKey].prefix;

    return (
      <ScrollView
        style={styles.b_flex}
      >
        <View style={styles.b_paddingH}>
          <Background />
          <Text style={[styles.header, styles.t_h1]}>Sign Transaction</Text>
          <Text style={styles.t_text}>From Account</Text>
        </View>
        <View style={styles.b_marginBottom}>
          <AccountCard
            title={sender.name}
            address={sender.address}
            networkKey={sender.networkKey}
          />
        </View>
        <Text style={[styles.b_paddingH, styles.t_text]}>Transaction Details</Text>
        {
          isEthereum
            ? (
              <React.Fragment>
                <TxDetailsCard
                  style={{ marginVertical: 16 }}
                  description="You are about to send the following amount"
                  value={value}
                  gas={gas}
                  gasPrice={gasPrice}
                />
                <Text style={[styles.b_paddingH, styles.t_text]}>Recipient</Text>
                <AccountCard
                  title={recipient.name}
                  address={recipient.address}
                  networkKey={recipient.networkKey || ''}
                />
              </React.Fragment>
            )
            : (
              <PayloadDetailsCard
                style={{ marginVertical: 16 }}
                description="You are about to confirm sending the following extrinsic"
                payload={dataToSign}
                prefix={prefix}
                />
            )
        }
        <View style={styles.b_paddingH}>
          <Button
            title="Sign Transaction"
            onPress={() => onNext()}
          />
        </View>
      </ScrollView>
    );
  }
}
