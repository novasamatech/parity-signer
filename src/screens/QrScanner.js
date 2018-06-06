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
import { StyleSheet, View, StatusBar } from 'react-native';
import Camera from 'react-native-camera';
import { Subscribe } from 'unstated';
import ScannerStore from '../stores/ScannerStore';

export default class Scanner extends Component {
  static navigationOptions = {
    title: 'Transaction Details',
    headerBackTitle: 'Scanner'
  };

  render() {
    return (
      <Subscribe to={[ScannerStore]}>
        {scannerStore => {
          return (
            <QrScannerView
              onBarCodeRead={txRequestData => {
                try {
                  scannerStore.setTXRequest(JSON.parse(txRequestData.data));
                  this.props.navigation.navigate('TxDetails');
                } catch (e) {
                  scannerStore.setErrorMsg(e.message);
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
  }

  static propTypes = {
    onBarCodeRead: PropTypes.func.isRequired
  };

  render() {
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
  }
});
