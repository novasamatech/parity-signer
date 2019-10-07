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
import React, { useEffect, useState } from 'react';
import { Image, View } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';

import styles from '../styles';
import colors from '../colors';
import { NetworkProtocols } from '../constants'
import { blockiesIcon } from '../util/native';

export default function AccountIcon (props) {

  AccountIcon.propTypes = {
    address: PropTypes.string.isRequired,
    protocol: PropTypes.string.isRequired
  };

  const {address, protocol, style} = props;
  const [ethereumIconUri, setEthereumIconUri] = useState('');

  useEffect(() => {
    if (protocol === NetworkProtocols.ETHEREUM) {
      loadEthereumIcon(address);
    }
  },[protocol, address])

  const loadEthereumIcon = function (address){
    blockiesIcon('0x'+address)
    .then((ethereumIconUri) => {
      setEthereumIconUri(ethereumIconUri);
    })
    .catch(console.error)
  }

  if (protocol === NetworkProtocols.SUBSTRATE) {

    return (
      <View style={[styles.el_icon,  {borderRadius:styles.el_icon.width, overflow:'hidden'}]}>
        <Identicon
          value={address}
          size={styles.el_icon.width}
        />
      </View>
    );
  } else if (protocol === NetworkProtocols.ETHEREUM && ethereumIconUri){

    return (
      <View style={[styles.el_icon,  {borderRadius:styles.el_icon.width, overflow:'hidden'}]}>
        <Image 
          source={{ uri: ethereumIconUri }} 
          style={styles.el_icon}
        />
      </View>
    );
  } else {
    // if there's no protocol or it's unknown we return a warning
    return (
      <View style={[styles.el_icon,  {borderRadius:styles.el_icon.width, overflow:'hidden', backgroundColor:'black'}]}>
        <Icon
          color={colors.bg}
          name={'error'}
          size={style.width || 56 }
        />
      </View>
    )
  }
}
