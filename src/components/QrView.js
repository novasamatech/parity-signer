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

import { isHex } from '@polkadot/util';
import PropTypes from 'prop-types';
import React from 'react';
import { Dimensions, Image, StyleSheet, Text, View } from 'react-native';

import { qrCode, qrHex } from '../util/native';

export default class QrView extends React.PureComponent {
  static propTypes = {
    data: PropTypes.string.isRequired // arbitrary message/txn string or `${networkType}:0x${address.toLowerCase()}@${networkKey}`
  }

  constructor(props) {
    super(props);

    this.state = {
      qr: null
    };
  }

  componentDidMount() {
    const { data } = this.props;
  
    this.displayQrCode(data);
  }

  componentWillReceiveProps(newProps) {
    if (newProps.text !== this.props.text) {
      this.displayIcon(newProps.text);
    }
  }

  async displayQrCode (data) {
    try {
      const qr = isHex(data) ? await qrHex(data) : await qrCode(data);

      this.setState({
        qr: qr
      })
    } catch (e) {
      console.error(e);
    }
  }

  render() {
    if (this.props.screen) {
      return <View style={AppStyles.view}>{this.renderQr()}</View>;
    }

    return this.renderQr();
  }

  renderQr() {
    const { width: deviceWidth } = Dimensions.get('window');
    let size = this.props.size || deviceWidth - 80;
    let flexBasis = this.props.height || deviceWidth - 40;

    return (
      <View style={[styles.rectangleContainer, { flexBasis, height: flexBasis }, this.props.style]}>
        <Image
          source={{ uri: this.state.qr }}
          style={{ width: size, height: size }}
        />
      </View>
    );
  }
}

const styles = StyleSheet.create({
  rectangleContainer: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: 'transparent'
  }
});
