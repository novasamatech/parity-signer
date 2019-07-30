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

// import QrScan from '@polkadot/ui-qr';
// import decodeAddress from '@polkadot/util-crypto';
import PropTypes from 'prop-types';
import React from 'react';
import { Alert, StyleSheet, Text, View } from 'react-native';
import { RNCamera } from 'react-native-camera';
import { Subscribe } from 'unstated';
import colors from '../colors';
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';

export default class Scanner extends React.PureComponent {
  static navigationOptions = {
    title: 'Transaction Details',
    headerBackTitle: 'Scanner'
  };

  encodeString (value: string): Uint8Array {
    const u8a = new Uint8Array(value.length);
    
    for (let i = 0; i < value.length; i++) {
      u8a[i] = value.charCodeAt(i);
    }

    return u8a;
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
                if (scannerStore.isBusy()) {
                  return;
                }

                let data = {};

                if (txRequestData.data) { // then this is Ethereum Legacy
                  data = JSON.parse(txRequestData.data);

                  if (data.action === undefined) {
                    throw new Error('Could not determine action type.');
                  }
                  
                  if (!(await scannerStore.setData(data, accountsStore))) {
                    return;
                  } else {
                    if (scannerStore.getType() === 'transaction') {
                      this.props.navigation.navigate('TxDetails');
                    } else { // message
                      this.props.navigation.navigate('MessageDetails');
                    }
                  }
                }
                // parse past the frame information
                let raw = txRequestData.rawData;
                let rawAfterFrames = raw.slice(13);

                // can't scope variables to switch case blocks....fml
                let zerothByte = rawAfterFrames.slice(0, 2);
                let firstByte = rawAfterFrames.slice(2, 4);
                let action;
                let address;

                try {
                  // decode payload appropriately via UOS
                  switch (zerothByte) {
                    case '45': // Ethereum UOS payload
                      action = firstByte === '00' || firstByte === '02' ? 'signData' : firstByte === '01' ? 'signTransaction' : null;
                      address = rawAfterFrames.slice(2, 22);

                      data['action'] = action;
                      data['account'] = account;

                      if (action === 'signData') {
                        data['rlp'] = rawAfterFrames.slice(23);
                      } else if (action === 'signTransaction') {
                        data['data'] = rawAfterFrames.slice(23);
                      } else {
                        throw new Error('Could not determine action type.');
                      }
                      break;
                    case '53': // Substrate UOS payload
                      const secondByte = rawAfterFrames.slice(4, 6);
                      const crypto = firstByte === '00' ? 'ed25519' : firstByte === '01' ? 'sr25519' : null;
                      action = secondByte === '00' || secondByte === '01' ? 'signData': secondByte === '02' || secondByte === '03' ? 'signTransaction' : null;

                      data['crypto'] = crypto;
                      data['action'] = action;
                      data['account'] = rawAfterFrames.slice(6, 70);
                      data['data'] = rawAfterFrames.slice(70);

                      debugger;

                      break;
                    default:
                      throw new Error('we cannot handle the payload: ', txRequestData);
                  }

                  if (!(await scannerStore.setData(data, accountsStore))) {
                    return;
                  } else {
                    if (scannerStore.getType() === 'transaction') {
                      this.props.navigation.navigate('TxDetails');
                    } else { // message
                      this.props.navigation.navigate('MessageDetails');
                    }
                  }
                } catch (e) {
                  scannerStore.setBusy();
                  Alert.alert('Unable to parse transaction', e.message, [
                    {
                      text: 'Try again',
                      onPress: () => {
                        scannerStore.cleanup();
                      }
                    }
                  ]);
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

  componentWillMount() {
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
    fontFamily: 'Manifold CF',
    fontWeight: 'bold',
    textAlign: 'center'
  },
  descTitle: {
    color: colors.bg_text,
    fontSize: 18,
    fontFamily: 'Manifold CF',
    fontWeight: 'bold',
    paddingBottom: 10,
    textAlign: 'center'
  },
  descSecondary: {
    color: colors.bg_text,
    fontSize: 14,
    fontFamily: 'Manifold CF',
    fontWeight: 'bold',
    paddingBottom: 20,
  }
});
