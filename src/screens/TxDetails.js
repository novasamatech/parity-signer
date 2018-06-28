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
import { StyleSheet, ScrollView, View, Text } from 'react-native';
import { Subscribe } from 'unstated';
import ScannerStore from '../stores/ScannerStore';
import AccountsStore from '../stores/AccountsStore';
import Background from '../components/Background';
import Button from '../components/Button';
import AccountCard from '../components/AccountCard';
import TxDetailsCard from '../components/TxDetailsCard';
import colors from '../colors';

const orUnknown = (value = 'Unknown') => value;

export default class TxDetails extends React.PureComponent {
  static navigationOptions = {
    title: 'Transaction Details',
    headerBackTitle: 'Transaction details'
  };
  render() {
    return (
      <Subscribe to={[ScannerStore, AccountsStore]}>
        {(scannerStore, accounts) => {
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
                onPressAccount={async account => {
                  await accounts.select(account);
                  this.props.navigation.navigate('AccountDetails');
                }}
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
    dataToSign: PropTypes.string.isRequired,
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
          chainId={this.props.sender.chainId}
          onPress={() => {
            this.props.onPressAccount(this.props.sender);
          }}
        />
        <Text style={styles.title}>TRANSACTION DETAILS</Text>
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
          chainId={this.props.recipient.chainId || ''}
          onPress={() => {
            this.props.onPressAccount(this.props.recipient);
          }}
        />
        <Button
          buttonStyles={{ backgroundColor: colors.bg_positive, height: 60 }}
          title="Sign Transaction"
          textStyles={{ color: colors.card_text }}
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
    fontFamily: 'Manifold CF',
    fontWeight: 'bold',
    paddingBottom: 20
  },
  title: {
    color: colors.bg_text_sec,
    fontSize: 18,
    fontFamily: 'Manifold CF',
    fontWeight: 'bold',
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
