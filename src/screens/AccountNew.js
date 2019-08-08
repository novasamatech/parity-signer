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

'use strict';

import React from 'react';
import { StyleSheet, ScrollView, Text, View } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';
import { Subscribe } from 'unstated';

import colors from '../colors';
import AccountIconChooser from '../components/AccountIconChooser';
import Background from '../components/Background';
import Button from '../components/Button';
import KeyboardScrollView from '../components/KeyboardScrollView';
import NetworkButton from '../components/NetworkButton';
import TextInput from '../components/TextInput';
import TouchableItem from '../components/TouchableItem';
import { NETWORK_LIST, NetworkProtocols, SubstrateNetworkKeys } from '../constants';
import AccountsStore from '../stores/AccountsStore';
import { validateSeed } from '../util/account';

export default class AccountNew extends React.Component {
  static navigationOptions = {
    title: 'New Account',
    headerBackTitle: 'Back'
  };
  render() {
    return (
      <Subscribe to={[AccountsStore]}>
        {accounts => <AccountNewView {...this.props} accounts={accounts} />}
      </Subscribe>
    );
  }
}

class AccountNewView extends React.Component {

  constructor(props) {
    super(props);

    this.state = {
      showAdvancedField: false,
    };
  }

  renderAdvanced (network) {
    const { showAdvancedField } = this.state;

    if (network.protocol === NetworkProtocols.ETHEREUM){
      return null;
    }

    return (
      <>
        <TouchableItem
          onPress={this.toggleAdvancedField}
          style={{diplay:'flex'}}
        >
          <View
            style={{justifyContent:'center'}}
          >
            <Text style={[styles.title, styles.advancedText]}>
              ADVANCED
              <Icon 
                name={showAdvancedField ? 'arrow-drop-up' : 'arrow-drop-down'}
                size={20}
              />
            </Text>
          </View>
        </TouchableItem>
        {showAdvancedField && 
          <TextInput
            // onChangeText={name => this.setState({ derivationPath })}
            placeholder="secret derivation path"
          />
        }
      </>
    )
  }

  toggleAdvancedField = () => {
    this.setState({showAdvancedField: !this.state.showAdvancedField}) 
  }

  render() {
    const { accounts, navigation } = this.props;
    const selected = accounts.getNew();
    const network = NETWORK_LIST[selected.networkKey];

    if (!selected) {
      return null;
    }

    return (
      <View style={styles.body}>
        <KeyboardScrollView style={{ padding: 20 }}>
          <Background />
          <View style={styles.top}>
            <Text style={styles.titleTop}>CREATE ACCOUNT</Text>
            <Text style={styles.title}>NETWORK</Text>
            <NetworkButton network={network}/>
            <Text style={styles.title}>ICON & ADDRESS</Text>
            <AccountIconChooser
              onSelect={({ address, bip39, seed }) => {
                accounts.updateNew({ address, seed, validBip39Seed: bip39 });
              }}
              protocol={network.protocol}
              value={selected && selected.seed && selected.address}
            />
            <Text style={styles.title}>NAME</Text>
            <TextInput
              onChangeText={name => accounts.updateNew({ name })}
              value={selected && selected.name}
              placeholder="Enter a new account name"
            />
            {this.renderAdvanced(network)}
          </View>
          <View style={styles.bottom}>
            <Text style={styles.hintText}>
              Next, you will be asked to backup your account, get a pen and some paper.
            </Text>
            <Button
              buttonStyles={styles.nextStep}
              title="Next Step"
              disabled={!validateSeed(selected.seed, selected.validBip39Seed).valid}
              onPress={() => {
                validateSeed(selected.seed, selected.validBip39Seed).valid &&
                  navigation.navigate('AccountBackup', {
                    isNew: true,
                    isWelcome: navigation.getParam('isWelcome')
                  });
              }}
            />
          </View>
        </KeyboardScrollView>
      </View>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    backgroundColor: colors.bg,
    paddingBottom: 20,
    flex: 1,
    overflow: 'hidden'
  },
  bodyContainer: {
    flex: 1,
    flexDirection: 'column',
    justifyContent: 'space-between'
  },
  top: {
    flex: 1
  },
  bottom: {
    flexBasis: 50,
    paddingBottom: 15
  },
  title: {
    fontFamily: fonts.bold,
    color: colors.bg_text_sec,
    fontSize: 18,
    paddingBottom: 20
  },
  titleTop: {
    color: colors.bg_text_sec,
    fontFamily: fonts.bold,
    fontSize: 24,
    paddingBottom: 20,
    textAlign: 'center'
  },
  advancedText: {
    paddingBottom: 0,
    paddingTop:20
  },
  hintText: {
    fontFamily: fonts.bold,
    textAlign: 'center',
    paddingTop: 20,
    color: colors.bg_text_sec,
    fontSize: 12
  },
  nextStep: {
    marginTop: 15
  }
});
