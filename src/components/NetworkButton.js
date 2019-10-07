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

'use strict';

import TouchableItem from './TouchableItem';
import { Text } from 'react-native';
import React from 'react';
import { withNavigation } from 'react-navigation';
import styles from '../styles';

class NetworkButton extends React.PureComponent<{
  network: {
    title: number,
    color: string,
    secondaryColor: string
  }
}> {
  render() {
    const { navigation, network } = this.props;

    return (
      <TouchableItem
        style={[
          styles.card,
          {
            backgroundColor: network.color,
          }
        ]}
        onPress={() => navigation.navigate('AccountNetworkChooser')}
      >
        <Text style={[styles.t_h2, styles.t_bold, styles.t_center, { color: network.secondaryColor }]}>
          {network.title}
        </Text>
      </TouchableItem>
    );
  }
}

export default withNavigation(NetworkButton);
