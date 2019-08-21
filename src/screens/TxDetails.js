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
import { ScrollView, StyleSheet, Text } from 'react-native';
import { Subscribe } from 'unstated';
import colors from '../colors';
import fonts from "../fonts";
import AccountCard from '../components/AccountCard';
import Background from '../components/Background';
import Button from '../components/Button';
import TxDetailsCard from '../components/TxDetailsCard';
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';
import { NetworkProtocols } from '../constants';
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
    return (
      <ScrollView
        contentContainerStyle={styles.bodyContent}
        style={styles.body}
      >
        <Background />
        <Text style={styles.topTitle}>SIGN TRANSACTION</Text>
        <Text style={styles.title}>FROM ACCOUNT</Text>
        <AccountCard
          title={this.props.sender.name}
          address={this.props.sender.address}
          networkKey={this.props.sender.networkKey}
        />
        <Text style={styles.title}>TRANSACTION DETAILS</Text>

        {
          this.props.sender.protocol === NetworkProtocols.ETHEREUM
            ? (
              <React.Fragment>
                <TxDetailsCard
                  style={{ marginBottom: 20 }}
                  description="You are about to send the following amount"
                  value={this.props.value}
                  gas={this.props.gas}
                  gasPrice={this.props.gasPrice}
                />
                <Text style={styles.title}>RECIPIENT</Text>
                <AccountCard
                  title={this.props.recipient.name}
                  address={this.props.recipient.address}
                  networkKey={this.props.recipient.networkKey || ''}
                />
              </React.Fragment>
            )
            : (
              <PayloadDetailsCard
                style={{ marginBottom: 20 }}
                description="You are about to confirm sending the following extrinsic"
                payload={this.props.dataToSign}
                />
            )
        }

        <Button
          buttonStyles={{ height: 60 }}
          title="Sign Transaction"
          onPress={() => this.props.onNext()}
        />
      </ScrollView>
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
  bodyContent: {
    paddingBottom: 40
  },
  transactionDetails: {
    flex: 1,
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
  wrapper: {
    borderRadius: 5
  },
  address: {
    flex: 1
  },
  deleteText: {
    textAlign: 'right'
  },
  changePinText: {
    textAlign: 'left',
    color: 'green'
  },
  actionsContainer: {
    flex: 1,
    flexDirection: 'row'
  },
  actionButtonContainer: {
    flex: 1
  }
});
