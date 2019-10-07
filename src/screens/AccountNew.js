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
import { StyleSheet, Text, View } from 'react-native';
import { Subscribe } from 'unstated';

import styles from '../styles';
import AccountIconChooser from '../components/AccountIconChooser';
import Background from '../components/Background';
import Button from '../components/Button';
import DerivationPathField from '../components/DerivationPathField'
import KeyboardScrollView from '../components/KeyboardScrollView';
import NetworkButton from '../components/NetworkButton';
import TextInput from '../components/TextInput';
import { NETWORK_LIST, NetworkProtocols } from '../constants';
import AccountsStore from '../stores/AccountsStore';
import { empty, validateSeed } from '../util/account';
import {constructSURI} from '../util/suri';

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
      derivationPassword: '',
      derivationPath: '',
      isDerivationPathValid: true,
      selectedAccount: undefined,
      selectedNetwork: undefined,
    };
  }

  componentWillUnmount = function() {
    // called when the user goes back, or finishes the whole process
    this.props.accounts.updateNew(empty());
  };

  static getDerivedStateFromProps(nextProps, prevState) {
    const selectedAccount = nextProps.accounts.getNew();
    const selectedNetwork = NETWORK_LIST[selectedAccount.networkKey];

    return {
      derivationPassword: prevState.derivationPassword,
      derivationPath: prevState.derivationPath,
      selectedAccount,
      selectedNetwork
    }
  }

  render() {
    const { accounts, navigation } = this.props;
    const { derivationPassword, derivationPath, isDerivationPathValid, selectedAccount, selectedNetwork } = this.state;
    const {address, name, seed, validBip39Seed} = selectedAccount;
    const isSubstrate = selectedNetwork.protocol === NetworkProtocols.SUBSTRATE;

    if (!selectedAccount) {
      return null;
    }

    return (
      <View style={[styles.b_flex, styles.b_bg]}>
        <KeyboardScrollView>
          <Background />
          <View style={styles.b_paddingH}>
            <Text style={[styles.t_h1, styles.header]}>Create Account</Text>
            <Text style={[styles.t_text, styles.b_marginV_xs, {marginTop:0}]}>Network</Text>
          </View>
          <View style={styles.b_marginBottom}>
            <NetworkButton network={selectedNetwork}/>
          </View>
          <View style={styles.b_paddingH}>
            <Text style={styles.t_text}>Icon & Address</Text>
          </View>
          <View style={styles.b_marginBottom}>
            <AccountIconChooser
              derivationPassword={derivationPassword}
              derivationPath={derivationPath}
              onSelect={({ newAddress, isBip39, newSeed }) => {
                if (newAddress && isBip39 && newSeed){
                  if (isSubstrate) {
                    try {
                      const suri = constructSURI({
                        derivePath: derivationPath,
                        password: derivationPassword,
                        phrase: newSeed
                      });

                      accounts.updateNew({
                        address: newAddress,
                        derivationPassword,
                        derivationPath,
                        seed: suri,
                        seedPhrase: newSeed,
                        validBip39Seed: isBip39
                      });
                    } catch (e) {
                      console.error(e);
                    }
                  } else {
                    // Ethereum account
                    accounts.updateNew({
                      address: newAddress,
                      seed: newSeed,
                      validBip39Seed: isBip39
                    });
                  }
                } else {
                  accounts.updateNew({ address: '', seed: '', validBip39Seed: false})
                }
              }}
              network={selectedNetwork}
              value={address && address}
            />
          </View>
          <View style={[styles.b_paddingH, styles.b_marginBottom]}>
            <Text style={styles.t_text}>NAME</Text>
            <TextInput
              onChangeText={name => accounts.updateNew({ name })}
              value={name}
              placeholder="Enter a new account name"
              style={[styles.b_textInput, styles.b_marginBottom, styles.t_h2] }
            />
            {isSubstrate && <DerivationPathField
              onChange = { ({derivationPassword, derivationPath, isDerivationPathValid}) => {
                this.setState({ derivationPath, derivationPassword, isDerivationPathValid });
              }}
              styles={styles}
          />}
          </View>
          <View style={styles.b_paddingH}>
            <Text style={styles.t_hintText}>
              Next, you will be asked to backup your account, get a pen and some paper.
            </Text>
            <Button
              buttonStyles={styles.nextStep}
              title="Next Step"
              disabled={!validateSeed(seed, validBip39Seed).valid || !isDerivationPathValid}
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
