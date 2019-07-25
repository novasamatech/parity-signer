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

import PropTypes from 'prop-types';
import React from 'react';
import {
  FlatList,
  StyleSheet,
  Text,
  TouchableOpacity,
  View
} from 'react-native';
import colors from '../colors';
import { brainWalletAddress, words } from '../util/native';
import AccountIcon from './AccountIcon';

export default class AccountIconChooser extends React.PureComponent<{
  value: string,
  onChange: () => any
}> {
  static propTypes = {
    value: PropTypes.string,
    onChange: PropTypes.func
  };

  constructor(props) {
    super(props);

    this.state = { icons: [] };
  }

  componentDidMount() {
    this.refreshIcons();
  }

  refreshIcons = async () => {
    try {
      const icons = await Promise.all(
        Array(10)
          .join(' ')
          .split(' ')
          .map(async () => {
            const seed = await words();
            const { address } = await brainWalletAddress(seed);

            return {
              seed,
              address
            };
          })
      );

      this.setState({
        icons
      });
    } catch (e) {
      console.error(e);
    }
  };

  renderIcon = ({ item, index }) => {
    const { value, onChange } = this.props;
    const iSelected = item.address.toLowerCase() === value.toLowerCase();
    const style = [styles.icon];

    return (
      <TouchableOpacity
        key={index}
        style={[styles.iconBorder, iSelected ? styles.selected : {}]}
        onPress={() =>
          onChange({
            address: item.address,
            seed: item.seed
          })
        }
      >
        <AccountIcon style={style} seed={'0x' + item.address} />
      </TouchableOpacity>
    );
  }

  render() {
    const { value } = this.props;
    const { icons } = this.state;

    return (
      <View style={styles.body}>
        <FlatList
          data={icons}
          extraData={value}
          horizontal
          keyExtractor={item => item.address}
          renderItem={this.renderIcon}
          style={styles.icons}
        />
        <Text
          numberOfLines={1}
          adjustsFontSizeToFit
          minimumFontScale={0.01}
          style={styles.addressText}
        >
          {value ? `0x${value}` : `Select an identicon`}
        </Text>
      </View>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    flex: 1,
    flexDirection: 'column',
    marginBottom: 20,
    padding: 20,
    backgroundColor: colors.card_bg
  },
  icons: {
    backgroundColor: colors.card_bg
  },
  icon: {
    width: 50,
    height: 50,
    margin: 6,
    padding: 5
  },
  iconBorder: {
    borderWidth: 6,
    borderColor: colors.card_bg
  },
  selected: {
    borderColor: colors.bg
  },
  addressText: {
    fontFamily: 'Roboto',
    paddingTop: 20,
    color: colors.bg,
    fontWeight: '700',
    fontSize: 14
  }
});
