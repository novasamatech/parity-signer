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

// @flow

import React from 'react';
import {
  StyleSheet,
  Text,
} from 'react-native';

import colors from '../colors';
import fonts from "../fonts";
import {NetworkProtocols} from '../constants'

export default function Address (props) {
  const {address, protocol = NetworkProtocols.SUBSTRATE, short = false ,style = {}} = props;
  const prefix = protocol === NetworkProtocols.ETHEREUM ? '0x' : '';
  let result = address;

  if (short) {
    result = `${address.slice(0, 6)}…${address.slice(-6)}`;
  }

  return (
      <Text numberOfLines={1} style={[style, styles.secondaryText]}>
        {prefix}{result}
      </Text>
  );
}

const styles = StyleSheet.create({
  secondaryText: {
    fontFamily: fonts.regular,
    color: colors.bg_text_sec,
    fontSize: 12
  }
});