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
import { StyleSheet, View, StatusBar, Alert } from 'react-native';
import Camera from 'react-native-camera';
import { Subscribe } from 'unstated';
import ScannerStore from '../stores/ScannerStore';
import AccountsStore from '../stores/AccountsStore';
import colors from '../colors';

export default class Scanner extends Component {
  static navigationOptions = {
    title: 'Transaction Details',
    headerBackTitle: 'Scanner'
  };

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
                try {
                  const data = JSON.parse(txRequestData.data);
                  if (!(await scannerStore.setTXRequest(data, accountsStore))) {
                    return;
                  } else {
                    this.props.navigation.navigate('TxDetails');
                  }
                } catch (e) {
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

export class QrScannerView extends Component {
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
      return <View style={ styles.body } />;
    }
    return (
      <Camera onBarCodeRead={this.props.onBarCodeRead} style={styles.view}>
        <StatusBar barStyle="light-content" />
        {this.renderRects()}
      </Camera>
    );
  }

  renderRects() {
    return (
      <View style={styles.rectangleContainer}>
        <View style={styles.rectangle}>
          <View style={styles.innerRectangle} />
        </View>
      </View>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    backgroundColor: colors.bg,
    padding: 20,
    flex: 1,
    flexDirection: 'column'
  },
  view: {
    flex: 1,
    backgroundColor: 'black'
  },
  rectangleContainer: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: 'transparent'
  },
  rectangle: {
    borderWidth: 2,
    borderRadius: 25,
    alignItems: 'center',
    justifyContent: 'center',
    height: 250,
    width: 250,
    borderColor: '#ccc',
    backgroundColor: 'transparent'
  },
  innerRectangle: {
    height: 248,
    width: 248,
    borderWidth: 2,
    borderRadius: 25,
    borderColor: '#ddd',
    backgroundColor: 'transparent'
  },
});
