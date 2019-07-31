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
  FlatList,
  StyleSheet,
  Text,
  TouchableOpacity,
  View
} from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';

import AccountIcon from './AccountIcon';
import Address from './Address'
import colors from '../colors';
import { brainWalletAddress, words } from '../util/native';

export default class AccountIconChooser extends React.PureComponent {
  constructor(props) {
    super(props);

    this.state = { icons: [] };
  }

  componentDidMount() {
    this.refreshAccount();
  }

  refreshAccount = async () => {
    try {
      const icons = await Promise.all(
        Array(4)
          .join(' ')
          .split(' ')
          .map(async () => {
            const seed = await words();
            const { address, bip39 } = await brainWalletAddress(seed);

            return {
              address,
              bip39,
              seed,
            };
          })
      );

      this.setState({ icons });
    } catch (e) {
      console.error(e);
    }
  }

  renderAddress = () => {
    const {value} = this.props;

    if (!!value) {
      return (
        <Address 
          address={value}
          style = {styles.addressText}
        />
      );
    } else {
      return <Text>Select an icon.</Text>
    }
  }
 
  renderIcon = ({ item, index }) => {
    const { value, onSelect } = this.props;
    const { address, bip39, seed } = item;
    const isSelected = address.toLowerCase() === value.toLowerCase();

    return (
        <TouchableOpacity
          key={index}
          style={[styles.iconBorder, isSelected ? styles.selected : {}]}
          onPress={() => onSelect({ address, bip39, seed })}
        >
          <AccountIcon
            style={styles.icon}
            seed={'0x' + address}
          />
        </TouchableOpacity>
    );
  }

  onRefresh = () => {
    const { onSelect } = this.props;

    this.refreshAccount();
    onSelect({ address: '', bip39: false, seed: ''});
  }

  render() {
    const { value } = this.props;
    const { icons } = this.state;

    return (
      <View style={styles.body}>
        <View style={styles.firstRow}>
          <FlatList
            data={icons}
            extraData={value}
            horizontal
            keyExtractor={item => item.address}
            renderItem={this.renderIcon}
            style={styles.icons}
          />
          <TouchableOpacity
            onPress={this.onRefresh}
          >
            <Icon
              name={'refresh'}
              size={35}
              style={styles.refreshIcon}
            />
          </TouchableOpacity>
        </View>
          {this.renderAddress()}
        </View>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    backgroundColor: colors.card_bg,
    display: 'flex',
    flexDirection:'column',
    marginBottom: 20,
    padding: 20,
  },
  firstRow: {
    flex: 1,
    display: 'flex',
    flexDirection:'row',
    alignItems: 'center',
    marginBottom: 10
  },
  iconBorder: {
     borderWidth: 6,
    borderColor: colors.card_bg
  },
  icon: {
    width: 50,
    backgroundColor: colors.card_bg,
    height: 50,
    padding: 5,
  },
  addressText: {
    paddingLeft: 6
  },
  refreshIcon :{
    color: colors.bg,
  },
  selected: {
    borderColor: colors.bg,
  }
});
