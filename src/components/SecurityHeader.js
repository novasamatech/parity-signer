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

import NetInfo from '@react-native-community/netinfo';
import React from 'react';
import { StyleSheet, Text, View } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';
import { withNavigation } from 'react-navigation';

import colors from '../colors';
import TouchableItem from './TouchableItem';

class SecurityHeader extends React.Component {
  state = {
    isConnected: false
  };

  componentDidMount() {
    this.unsubscribe = NetInfo.addEventListener(state => {
      this.setState({ isConnected: state.isConnected });
    });
  }

  componentWillUnmount() {
    this.unsubscribe();
  }

  render() {
    const { isConnected } = this.state;

    if (!isConnected) {
      return null
    }

    return (
      <TouchableItem onPress={() => this.props.navigation.navigate('Security')}>
        <View style={{ flexDirection: 'row', alignItems: 'center' }}>
          <Icon style={styles.headerSecureIcon} name="security" />
          <Text style={styles.headerTextRight}>Not Secure</Text>
        </View>
      </TouchableItem>
    );
  }
}

const styles = StyleSheet.create({
  headerSecureIcon: {
    marginLeft: 0,
    fontSize: 20,
    fontWeight: 'bold',
    paddingRight: 5,
    color: colors.bg_alert
  },
  headerTextRight: {
    marginLeft: 0,
    fontSize: 17,
    fontFamily: 'Manifold CF',
    fontWeight: 'bold',
    color: colors.bg_alert,
    paddingRight: 14,
  }
});

export default withNavigation(SecurityHeader);
