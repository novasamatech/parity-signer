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
import React, { useEffect, useState } from 'react';
import { Dimensions, Image, StyleSheet, View } from 'react-native';
import { qrCode, qrHex } from '../util/native';

QrView.propTypes = {
  data: PropTypes.string.isRequired
};

export default function QrView(props) {

  const [qr, setQr] = useState(null);

  useEffect(() => {
    async function displayQrCode(data) {
      try {
        const qr = isHex(data) ? await qrHex(data) : await qrCode(data);
        setQr(qr);
      } catch (e) {
        console.error(e);
      }
    }
    displayQrCode(props.data);
  }, [props.data]);


  const { width: deviceWidth } = Dimensions.get('window');
  let size = props.size || deviceWidth - 80;
  let flexBasis = props.height || deviceWidth - 40;

  return (
    <View
      style={[
        styles.rectangleContainer,
        { flexBasis, height: flexBasis },
        props.style
      ]}
    >
      <Image source={{ uri: qr }} style={{ width: size, height: size }} />
    </View>
  );
}

const styles = StyleSheet.create({
  rectangleContainer: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: 'transparent'
  }
});
