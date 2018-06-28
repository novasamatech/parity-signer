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

import React from 'react';
import PropTypes from 'prop-types';
import { Image, View, StyleSheet } from 'react-native';
import { blockiesIcon } from '../util/native';

export default class AccountIcon extends React.PureComponent {

  constructor(...args) {
    super(...args);
    this.displayIcon = this.displayIcon.bind(this);
  }

  static propTypes = {
    seed: PropTypes.string.isRequired
  };

  state = {};

  async displayIcon(seed) {
    try {
      let icon = await blockiesIcon(seed);
      this.setState({
        icon: icon
      });
    } catch (e) {
      console.log(e);
    }
  }

  componentDidMount() {
    this.displayIcon(this.props.seed);
  }

  componentWillReceiveProps(newProps) {
    if (newProps.seed !== this.props.seed) {
      this.displayIcon(newProps.seed);
    }
  }

  render() {
    return (
      <View style={styles.identicon}>
        <Image
          style={this.props.style || {}}
          source={{ uri: this.state.icon }}
        />
      </View>
    );
  }
}

const styles = StyleSheet.create({
  identicon: {
    alignItems: 'center'
  }
});
