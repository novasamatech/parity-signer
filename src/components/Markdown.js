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

// @flow

import React from 'react';
import PropTypes from 'prop-types';
import { View, Text, Platform, StyleSheet } from 'react-native';
import { default as MarkdownRender } from 'react-native-markdown-renderer';
import colors from '../colors';

export default class Markdown extends React.PureComponent {
  render() {
    return (
      <MarkdownRender
        style={StyleSheet.create({
          text: {
            marginTop: 10,
            fontFamily: 'Roboto',
            fontSize: 14,
            color: colors.card_bg
          },
          listUnorderedItemIcon: {
            color: colors.card_bg,
            fontWeight: "900",
            marginRight: 3,
            marginTop: 19
          },
          listOrderedItemIcon: {
            color: colors.card_bg,
            marginTop: 19,
            marginRight: 3
          }
        })}
        {...this.props}
      />
    );
  }
}
