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

import React from 'react';
import PropTypes from 'prop-types';
import {
  TextInput as TextInputOrigin,
  Platform,
  StyleSheet
} from 'react-native';
import colors from '../colors';

export default class TextInput extends React.PureComponent {
  static defaultProps = {
    focus: false
  };

  // Methods:
  focus() {
    this.input.focus();
  }

  componentWillReceiveProps(nextProps) {
    const { focus } = nextProps;

    focus && this.focus();
  }
  render() {
    return (
      <TextInputOrigin
        ref={input => {
          this.input = input;
        }}
        keyboardAppearance="dark"
        underlineColorAndroid='transparent'
        {...this.props}
        style={[styles.input, this.props.style]}
      />
    );
  }
}

const styles = StyleSheet.create({
  input: {
    fontSize: 24,
    height: 60,
    justifyContent: 'center',
    alignItems: 'center',
    elevation: 4,
    paddingHorizontal: 18,
    backgroundColor: colors.card_bg
  }
});
