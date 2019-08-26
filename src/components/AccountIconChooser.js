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
import Address from './Address';
import colors from '../colors';
import { NetworkProtocols } from '../constants';
import fonts from "../fonts";
import { debounce } from '../util/debounce';
import { brainWalletAddress, substrateAddress, words } from '../util/native';
import {constructSURI} from '../util/suri'

export default class AccountIconChooser extends React.PureComponent {
  constructor(props) {
    super(props);

    this.state = {
      icons: []
    };
  }

  refreshIcons = async () => {
    const {derivationPassword, derivationPath, network : {protocol, prefix}, onSelect} = this.props;

    // clean previous selection
    onSelect({ newAddress: '', isBip39: false, newSeed: ''});
    try {
      const icons = await Promise.all(
        Array(4)
          .join(' ')
          .split(' ')
          .map(async () => {
            let result = {
              address: '',
              bip39: false,
              seed: ''
            }
            result.seed = await words();

            if (protocol === NetworkProtocols.ETHEREUM) {
              Object.assign(result, await brainWalletAddress(result.seed));
            } else {
              // Substrate
              try {
                const suri = constructSURI({
                  derivePath: derivationPath,
                  password: derivationPassword,
                  phrase: result.seed
                });
                
                result.address = await substrateAddress(suri, prefix);
                result.bip39 = true;
              } catch (e){
                // invalid seed or derivation path
                console.error(e);
              }
            }
            return result;
          })
      );
      this.setState({ icons });
    } catch (e) {
      console.error(e);
    }
  }

  renderAddress = () => {
    const {network: {protocol}, value} = this.props;

    if (value) {
      return (
        <Address 
          address={value}
          protocol={protocol}
          style = {styles.addressText}
        />
      );
    } else {
      return <Text style={styles.addressSelectionText} >Select an icon.</Text>
    }
  }
 
  renderIcon = ({ item, index }) => {
    const { onSelect, network : {protocol}, value } = this.props;
    const { address, bip39, seed } = item;
    const isSelected = address.toLowerCase() === value.toLowerCase();

    if (!address) {
      //return an empty view to prevent the screen from jumping
      return <View
        style={styles.icon}
      />
    }

    return (
        <TouchableOpacity
          key={index}
          style={[styles.iconBorder, isSelected ? styles.selected : {}]}
          onPress={() => onSelect({ newAddress: address, isBip39: bip39, newSeed: seed })}
        >
          <AccountIcon
            address={address}
            protocol={protocol}
            style={styles.icon}
          />
        </TouchableOpacity>
    );
  }

  componentDidMount() {
    this.refreshIcons();
  }

  debouncedRefreshIcons = debounce(this.refreshIcons, 200);

  componentDidUpdate(prevProps){
    const {derivationPassword, derivationPath, network} = this.props;

    if ((prevProps.network !== network) || 
      (prevProps.derivationPassword !== derivationPassword) ||
      (prevProps.derivationPath !== derivationPath)){
        this.debouncedRefreshIcons();
      }
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
            keyExtractor={item => item.seed}
            renderItem={this.renderIcon}
            style={styles.icons}
          />
          <TouchableOpacity
            onPress={this.refreshIcons}
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
  addressSelectionText: {
    fontFamily: fonts.bold,
    color: colors.bg,
    fontSize: 14,
    paddingLeft: 6
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
