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
import { Alert, StyleSheet, Text, View } from 'react-native';
import { RNCamera } from 'react-native-camera';
import { Subscribe } from 'unstated';

import colors from '../colors';
import fonts from '../fonts';
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';
import {isAddressString, isJsonString, rawDataToU8A} from '../util/decoders';

export default class Scanner extends React.PureComponent {
  static navigationOptions = {
    title: 'Transaction Details',
    headerBackTitle: 'Scanner'
  };

  constructor(props) {
    super(props);
    this.state = { enableScan: true };
  }

  showErrorMessage(scannerStore, title, message) {
    this.setState({ enableScan: false });
    Alert.alert(title, message, [
      {
        text: 'Try again',
        onPress: () => {
          scannerStore.cleanup();
          this.setState({ enableScan: true });
        }
      }
    ]);
  }

  render() {
    return (
      <Subscribe to={[ScannerStore, AccountsStore]}>
        {(scannerStore, accountsStore) => {
          return (
            <QrScannerView
              navigation={this.props.navigation}
              scannerStore={scannerStore}
              onBarCodeRead={async txRequestData => {
                if (scannerStore.isBusy() || !this.state.enableScan) {
                  return;
                }

                try {
                  if(isAddressString(txRequestData.data)) {
                    return this.showErrorMessage(scannerStore, text.ADDRESS_ERROR_TITLE, text.ADDRESS_ERROR_MESSAGE);
                  } else if (isJsonString(txRequestData.data)) {
                    // Ethereum Legacy
                    await scannerStore.setUnsigned(txRequestData.data);
                  } else {
                    const strippedData = rawDataToU8A(txRequestData.rawData);
                    await scannerStore.setParsedData(
                      strippedData,
                      accountsStore
                    );
                  }
  
                  if (scannerStore.getUnsigned()) {
                    await scannerStore.setData(accountsStore);
                    if (scannerStore.getType() === 'transaction') {
                      this.props.navigation.navigate('TxDetails');
                    } else {
                      this.props.navigation.navigate('MessageDetails');
                    }
                  } else {
                    return;
                  }
                } catch (e) {
                  return this.showErrorMessage(scannerStore, text.PARSE_ERROR_TITLE, e.message);
                }
              }}
            />
          );
        }}
      </Subscribe>
    );
  }
}

export class QrScannerView extends React.PureComponent {
  constructor(props) {
    super(props);
    this.setBusySubscription = null;
    this.setReadySubscription = null;
  }

  static propTypes = {
    onBarCodeRead: PropTypes.func.isRequired
  };

  componentDidMount() {
    this.setBusySubscription = this.props.navigation.addListener(
      'willFocus',
      () => {
        this.props.scannerStore.setReady();
      }
    );
    this.setReadySubscription = this.props.navigation.addListener(
      'didBlur',
      () => {
        this.props.scannerStore.setBusy();
      }
    );
  }

  componentWillUnmount() {
    this.setBusySubscription.remove();
    this.setReadySubscription.remove();
  }

  render() {
    if (this.props.scannerStore.isBusy()) {
      return <View style={styles.inactive} />;
    }
    return (
      <RNCamera
        captureAudio={false}
        onBarCodeRead={this.props.onBarCodeRead}
        style={styles.view}
      >
        <View style={styles.body}>
          <View style={styles.top}>
            <Text style={styles.titleTop}>SCANNER</Text>
          </View>
          <View style={styles.middle}>
            <View style={styles.middleLeft} />
            <View style={styles.middleCenter} />
            <View style={styles.middleRight} />
          </View>
          <View style={styles.bottom}>
            <Text style={styles.descTitle}>Scan QR Code</Text>
            <Text style={styles.descSecondary}>To Sign a New Transaction</Text>
          </View>
        </View>
      </RNCamera>
    );
  }
}

const text = {
  ADDRESS_ERROR_TITLE: 'Address detected',
  ADDRESS_ERROR_MESSAGE: 'Please create a transaction using a software such as MyCrypto or Fether so that Parity Signer can sign it.',
  PARSE_ERROR_TITLE: 'Unable to parse transaction'
};

const styles = StyleSheet.create({
  inactive: {
    backgroundColor: colors.bg,
    padding: 20,
    flex: 1,
    flexDirection: 'column'
  },
  view: {
    flex: 1,
    backgroundColor: 'black'
  },
  body: {
    flex: 1,
    flexDirection: 'column',
    backgroundColor: 'transparent'
  },
  top: {
    flexBasis: 80,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: 'rgba(0, 0, 0, 0.5)'
  },
  middle: {
    flexBasis: 280,
    flexDirection: 'row',
    backgroundColor: 'transparent'
  },
  middleLeft: {
    flex: 1,
    backgroundColor: 'rgba(0, 0, 0, 0.5)'
  },
  middleCenter: {
    flexBasis: 280,
    borderWidth: 1,
    backgroundColor: 'transparent'
  },
  middleRight: {
    flex: 1,
    backgroundColor: 'rgba(0, 0, 0, 0.5)'
  },
  bottom: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: 'rgba(0, 0, 0, 0.5)'
  },
  titleTop: {
    color: colors.bg_text,
    fontSize: 26,
    fontFamily: fonts.bold,
    textAlign: 'center'
  },
  descTitle: {
    color: colors.bg_text,
    fontSize: 18,
    fontFamily: fonts.bold,
    paddingBottom: 10,
    textAlign: 'center'
  },
  descSecondary: {
    color: colors.bg_text,
    fontSize: 14,
    fontFamily: fonts.bold,
    paddingBottom: 20
  }
});
