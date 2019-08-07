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

import Identicon from '@polkadot/reactnative-identicon';
import PropTypes from 'prop-types';
import React from 'react';
import { Image } from 'react-native';

import { NetworkProtocols } from '../constants'
import { blockiesIcon } from '../util/native';

export default class AccountIcon extends React.PureComponent {

  static propTypes = {
    address: PropTypes.string.isRequired,
    protocol: PropTypes.string.isRequired
  };

  constructor(props) {
    super(props);

    this.state = { ethereumIconUri: '' };
  }

  componentDidMount() {
    const {address, protocol} = this.props;

    if (protocol === NetworkProtocols.ETHEREUM) {
      this.loadEthereumIcon(address);
    }
  }

  componentDidUpdate (prevProps) {
    const {address, protocol} = this.props
    const {oldAddress} = prevProps;

    if (protocol === NetworkProtocols.ETHEREUM && address !== oldAddress) {
      this.loadEthereumIcon(address);
    }
  }

  loadEthereumIcon(address){
    blockiesIcon('0x'+address)
    .then((ethereumIconUri) => {
      this.setState({ethereumIconUri});
    })
    .catch(console.error)
  }

  render() {
    const { address, protocol, style } = this.props;
    const { ethereumIconUri } = this.state;

    if (protocol === NetworkProtocols.SUBSTRATE && address) {

      return (
        <Identicon
          value={address}
          size={style.width || 50 }
        />
      );
    } else if (protocol === NetworkProtocols.ETHEREUM && ethereumIconUri){

      return (
        <Image 
          source={{ uri: ethereumIconUri }} 
          style={style || { width: 47, height: 47 }}
        />
      );
    }

    return null;
  }
}
