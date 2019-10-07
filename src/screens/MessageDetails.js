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

import { isU8a, u8aToHex } from '@polkadot/util';
import PropTypes from 'prop-types';
import React from 'react';
import { Alert, ScrollView, StyleSheet, Text, View } from 'react-native';
import { Subscribe } from 'unstated';
import styles from '../styles';
import AccountCard from '../components/AccountCard';
import Background from '../components/Background';
import Button from '../components/Button';
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';
import { hexToAscii, isAscii } from '../util/message';

export default class MessageDetails extends React.PureComponent {
  static navigationOptions = {
    title: 'Transaction Details',
    headerBackTitle: 'Transaction details'
  };
  render() {
    return (
      <Subscribe to={[ScannerStore, AccountsStore]}>
        {(scannerStore, accounts) => {
          const dataToSign = scannerStore.getDataToSign();
          const message = scannerStore.getMessage();

          if (dataToSign) {
            return (
              <MessageDetailsView
                {...this.props}
                scannerStore={scannerStore}
                sender={scannerStore.getSender()}
                message={isU8a(message) ? u8aToHex(message) : message}
                dataToSign={isU8a(dataToSign) ? u8aToHex(dataToSign) : dataToSign}
                isHash={scannerStore.getIsHash()}
                onNext={async () => {
                  try {
                    this.props.navigation.navigate('AccountUnlockAndSign', {
                      next: 'SignedMessage'
                    });
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

export class MessageDetailsView extends React.PureComponent {
  static propTypes = {
    onNext: PropTypes.func.isRequired,
    dataToSign: PropTypes.string.isRequired,
    isHash: PropTypes.bool,
    sender: PropTypes.object.isRequired,
    message: PropTypes.string.isRequired
  };

  render() {
    const { dataToSign, isHash, message, onNext, sender} = this.props;

    return (
      <ScrollView
        style={styles.b_flex}
      >
        <View style={styles.b_paddingH} >
          <Background />
          <Text style={[styles.header, styles.t_h1]}>Sign Message</Text>
          <Text style={styles.t_text}>From Account</Text>
        </View>
        <View style = {styles.b_marginBottom}>
          <AccountCard
            title={sender.name}
            address={sender.address}
            networkKey={sender.networkKey}
          />
        </View>
          <View style={styles.b_paddingH} >
          <Text style={[styles.t_text, styles.b_marginV_xs]}>Message</Text>
          <Text style={[styles.t_parityS, styles.seedText]}>
            {isAscii(message)
              ? hexToAscii(message)
              : dataToSign}
          </Text>
          <Button
            title="Sign Message"
            onPress={() => {
              isHash
                ? Alert.alert(
                    "Warning",
                    "The payload of the transaction you are signing is too big to be decoded. Not seeing what you are signing is inherently unsafe. If possible, contact the developer of the application generating the transaction to ask for multipart support.",
                    [
                      {
                        text: 'I take the risk',
                        onPress: () => onNext()
                      },
                      {
                        text: 'Cancel',
                        style: 'cancel'
                      }
                    ]
                  )
                : onNext()
            }}
          />
        </View>
      </ScrollView>
    );
  }
}
