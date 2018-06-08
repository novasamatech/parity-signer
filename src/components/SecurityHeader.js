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
import { Text, View, StyleSheet, NetInfo } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';
import colors from '../colors';

export default class SecurityHeader extends React.Component {
  constructor(...args) {
    super(...args);
    this.listener = null;
    this.connectionChangeListener = this.connectionChangeListener.bind(this);
    this.state = {
      warning: 'red'
    };
  }

  connectionChangeListener(connectionInfo) {
    if (connectionInfo.type !== 'none') {
      this.setState({ warning: 'red' });
    } else {
      this.setState({ warning: 'green' });
    }
  }

  componentDidMount() {
    NetInfo.addEventListener('connectionChange', this.connectionChangeListener);
  }

  componentWillUnmount() {
    NetInfo.removeEventListener(
      'connectionChange',
      this.connectionChangeListener
    );
  }

  render() {
    const { warning } = this.state;
    const color = {
      green: 'green',
      red: 'red'
    }[warning];

    const message = {
      green: 'Secured',
      red: 'Unsecured'
    }[warning];

    return (
      <View style={{ flexDirection: 'row' }}>
        <Icon style={[styles.headerSecureIcon, { color }]} name="security" />
        <Text style={[styles.headerTextRight, { color }]}>{ message }</Text>
      </View>
    );
  }
}

const styles = StyleSheet.create({
  headerSecureIcon: {
    marginLeft: 0,
    fontSize: 20,
    fontWeight: 'bold',
    paddingRight: 5,
    color: colors.bg_text_positive
  },
  headerTextRight: {
    marginLeft: 0,
    fontSize: 17,
    fontFamily: 'Roboto',
    fontWeight: 'bold',
    color: colors.bg_text_positive
  }
});
