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
import { Alert, ScrollView, StyleSheet, Text } from 'react-native';
import { Subscribe } from 'unstated';
import colors from '../colors';
import fonts from "../fonts";
import AccountCard from '../components/AccountCard';
import Background from '../components/Background';
import Button from '../components/Button';
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';
import { isAscii } from '../util/message';

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
                onPressAccount={async account => {
                  await accounts.select(account);
                  this.props.navigation.navigate('AccountDetails');
                }}
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
    isHash: PropTypes.bool.isRequired,
    sender: PropTypes.object.isRequired,
    message: PropTypes.string.isRequired
  };

  render() {
    const {dataToSign, isHash, message, onNext, onPressAccount, sender} = this.props;

    return (
      <ScrollView
        contentContainerStyle={styles.bodyContent}
        style={styles.body}
      >
        <Background />
        <Text style={styles.topTitle}>SIGN MESSAGE</Text>
        <Text style={styles.title}>FROM ACCOUNT</Text>
        <AccountCard
          title={sender.name}
          address={sender.address}
          networkKey={sender.networkKey}
          onPress={() => {
            onPressAccount(sender);
          }}
        />
        <Text style={styles.title}>MESSAGE</Text>
        <Text style={styles.message}>
          {isAscii(message)
            ? message
            : dataToSign}
        </Text>
        <Button
          buttonStyles={{ height: 60 }}
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
  message: {
    marginBottom: 20,
    padding: 10,
    height: 120,
    lineHeight: 26,
    fontSize: 20,
    fontFamily: fonts.regular,
    backgroundColor: colors.card_bg
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
