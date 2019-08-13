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
import { StyleSheet, Text, TouchableOpacity, View } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';
import { Subscribe } from 'unstated';

import colors from '../colors';
import AccountIconChooser from '../components/AccountIconChooser';
import Background from '../components/Background';
import Button from '../components/Button';
import KeyboardScrollView from '../components/KeyboardScrollView';
import NetworkButton from '../components/NetworkButton';
import TextInput from '../components/TextInput';
import { NETWORK_LIST, NetworkProtocols } from '../constants';
import fonts from "../fonts";
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
      selectedAccount: undefined,
      selectedNetwork: undefined,
      showAdvancedField: false
    };
  }

  static getDerivedStateFromProps(nextProps, prevState) {
    const selectedAccount = nextProps.accounts.getNew();
    const selectedNetwork = NETWORK_LIST[selectedAccount.networkKey];

    return {
      selectedAccount,
      selectedNetwork,
      showAdvancedField: prevState.showAdvancedField
    }
  }

  renderAdvanced () {
    const { selectedNetwork, showAdvancedField } = this.state;

    if (selectedNetwork.protocol === NetworkProtocols.ETHEREUM){
      return null;
    }

    return (
      <>
        <TouchableOpacity
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
        </TouchableOpacity>
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
    const { selectedAccount, selectedNetwork } = this.state;
    const {address, name, seed, validBip39Seed} = selectedAccount;

    if (!selectedAccount) {
      return null;
    }

    return (
      <View style={styles.body}>
        <KeyboardScrollView style={{ padding: 20 }}>
          <Background />
          <View style={styles.top}>
            <Text style={styles.titleTop}>CREATE ACCOUNT</Text>
            <Text style={styles.title}>NETWORK</Text>
            <NetworkButton network={selectedNetwork}/>
            <Text style={styles.title}>ICON & ADDRESS</Text>
            <AccountIconChooser
              onSelect={({ newAddress, isBip39, newSeed }) => {
                accounts.updateNew({ 
                  address: newAddress,
                  seed: newSeed,
                  validBip39Seed: isBip39
                });
              }}
              protocol={selectedNetwork.protocol}
              value={seed && address}
            />
            <Text style={styles.title}>NAME</Text>
            <TextInput
              onChangeText={name => accounts.updateNew({ name })}
              value={name}
              placeholder="Enter a new account name"
            />
            {this.renderAdvanced()}
          </View>
          <View style={styles.bottom}>
            <Text style={styles.hintText}>
              Next, you will be asked to backup your account, get a pen and some paper.
            </Text>
            <Button
              buttonStyles={styles.nextStep}
              title="Next Step"
              disabled={!validateSeed(seed, validBip39Seed).valid}
              onPress={() => {
                validateSeed(seed, validBip39Seed).valid &&
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
