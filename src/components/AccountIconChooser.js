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
  TouchableOpacity,
  View
} from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';
import colors from '../colors';
import { brainWalletAddress, words } from '../util/native';

import AccountIcon from './AccountIcon';
import Address from './Address'

export default class AccountIconChooser extends React.PureComponent {
  constructor(props) {
    super(props);

    this.state = {};
  }

  componentDidMount() {
    this.refreshAccount();
  }

  refreshAccount = async () => {
    const { onChange } = this.props;
    try {
      const seed = await words();
      const { address, bip39 } = await brainWalletAddress(seed);

      const newAccountProperties = {
          address,
          bip39,
          seed
        };
      this.setState(newAccountProperties);
      onChange(newAccountProperties);
    } catch (e) {
      console.error(e);
    }
  };

  renderIcon = () => {
    const { address, bip39, seed } = this.state;

    return (
      <>
        <TouchableOpacity
          onPress={this.refreshAccount}
        ><Icon name={'refresh'} size={35} style={styles.refreshIcon} />
        </TouchableOpacity>
        <AccountIcon style={styles.icon} seed={'0x' + address} />
      </>
    );
  }

  render() {
    const { address } = this.state;

    if (!address){
      return null;
    }

    return (
      <View style={styles.body}>
        {this.renderIcon()}
        <Address 
          address={address}
          short
        />
      </View>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    alignItems: 'center',
    backgroundColor: colors.card_bg,
    display: 'flex',
    flexDirection:'row',
    marginBottom: 20,
    padding: 20,
    paddingLeft: 10,
  },
  icon: {
    backgroundColor: colors.card_bg,
    height: 50,
    margin: 6,
    padding: 5,
    width: 50,
  },
  addressText: {
    fontFamily: 'Roboto',
    color: colors.bg,
    fontWeight: '700',
    fontSize: 14,
  },
  refreshIcon :{
    color: colors.bg,
    marginRight: 5
  }
});
