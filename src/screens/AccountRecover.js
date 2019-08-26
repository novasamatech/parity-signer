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

import React from 'react';
import {
  Alert,
  findNodeHandle,
  SafeAreaView,
  StyleSheet,
  Text,
} from 'react-native';
import { Subscribe } from 'unstated';

import colors from '../colors';
import fonts from "../fonts";
import AccountCard from '../components/AccountCard';
import AccountSeed from '../components/AccountSeed';
import Background from '../components/Background';
import Button from '../components/Button';
import DerivationPathField from '../components/DerivationPathField';
import KeyboardScrollView from '../components/KeyboardScrollView';
import NetworkButton from '../components/NetworkButton';
import TextInput from '../components/TextInput';
import { NETWORK_LIST, NetworkProtocols } from '../constants';
import AccountsStore from '../stores/AccountsStore';
import { empty, validateSeed } from '../util/account';
import { debounce } from '../util/debounce';
import { brainWalletAddress, substrateAddress } from '../util/native';
import {constructSURI} from '../util/suri';

export default class AccountRecover extends React.Component {
  static navigationOptions = {
    title: 'Recover Account',
    headerBackTitle: 'Back'
  };

  render() {
    return (
      <Subscribe to={[AccountsStore]}>
        {accounts => <AccountRecoverView {...this.props} accounts={accounts} />}
      </Subscribe>
    );
  }
}

class AccountRecoverView extends React.Component {
  constructor(...args) {
    super(...args);

    this.state = {
      derivationPassword: '',
      derivationPath: '',
      isDerivationPathValid: true,
      seedPhrase: '',
      selectedAccount: undefined,
      selectedNetwork: undefined,
    };
  }

  static getDerivedStateFromProps(nextProps, prevState) {
    const selectedAccount = nextProps.accounts.getNew();
    const selectedNetwork = NETWORK_LIST[selectedAccount.networkKey];

    return {
      derivationPassword: prevState.derivationPassword,
      derivationPath: prevState.derivationPath,
      seedPhrase: prevState.seedPhrase,
      selectedAccount,
      selectedNetwork,
    }
  }

  clearNewAccount = function () {
    const { accounts } = this.props;

    accounts.updateNew({ address:'', derivationPath:'', derivationPassword:'', seed:'', seedPhrase:'', validBip39Seed: false })
  }

  addressGeneration = (seedPhrase, derivationPath = '', derivationPassword = '') => {
    const { accounts } = this.props;
    const { selectedNetwork:{protocol, prefix} } = this.state;

    if (!seedPhrase){
      this.clearNewAccount();
      
      return;
    }

    if (protocol === NetworkProtocols.ETHEREUM){
      brainWalletAddress(seedPhrase)
        .then(({ address, bip39 }) =>
          accounts.updateNew({ address, seed: seedPhrase, seedPhrase, validBip39Seed: bip39 })
        )
        .catch(console.error);
    } else {
      // Substrate
      try {
        const suri = constructSURI({
          derivePath: derivationPath,
          password: derivationPassword,
          phrase: seedPhrase
        });

        substrateAddress(suri, prefix)
          .then((address) => {
            accounts.updateNew({ address, derivationPath, derivationPassword, seed: suri, seedPhrase, validBip39Seed: true })
          })
          .catch(() => {
            //invalid phrase
            this.clearNewAccount();
          });
      } catch (e) {
        // invalid phrase or derivation path
        this.clearNewAccount();
      }
      
    }
  };

  debouncedAddressGeneration = debounce(this.addressGeneration, 200);

  componentWillUnmount = function() {
    // called when the user goes back, or finishes the whole recovery process
    this.props.accounts.updateNew(empty());
  };

  componentDidUpdate(_, prevState){
    const {derivationPassword, derivationPath, seedPhrase } = this.state;

    if (prevState.selectedNetwork !== this.state.selectedNetwork){
      this.addressGeneration(seedPhrase, derivationPath, derivationPassword);
    }
  }

  toggleAdvancedField = () => {
    this.setState({showAdvancedField: !this.state.showAdvancedField}) 
  }

  render() {
    const { accounts, navigation } = this.props;
    const { derivationPassword, derivationPath, isDerivationPathValid, selectedAccount, selectedNetwork} = this.state;
    const {address, name, networkKey, seedPhrase, validBip39Seed} = selectedAccount;
    const isSubstrate = selectedNetwork.protocol === NetworkProtocols.SUBSTRATE;

    return (
      <SafeAreaView style={styles.safeAreaView}>
        <KeyboardScrollView
          style={styles.bodyContainer}
          innerRef={ref => {
            this.scroll = ref;
          }}
          extraHeight={200}
          contentContainerStyle={{ justifyContent: 'flex-end' }}
        >
          <Background />
          <Text style={styles.titleTop}>RECOVER ACCOUNT</Text>
          <Text style={styles.title}>CHOOSE NETWORK</Text>
          <NetworkButton network={selectedNetwork} />
          <Text style={styles.title}>ACCOUNT NAME</Text>
          <TextInput
            onChangeText={name => accounts.updateNew({ name })}
            value={name}
            placeholder="Enter an account name"
          />
          <Text style={[styles.title, { marginTop: 20 }]}>
            ENTER RECOVERY WORDS
          </Text>
          <AccountSeed
            onFocus={event => {
              this.scroll.props.scrollToFocusedInput(
                findNodeHandle(event.target)
              );
            }}
            ref={this._seed}
            valid={validateSeed(seedPhrase, validBip39Seed).valid || (isSubstrate && address)}
            onChangeText={seedPhrase => {
              this.debouncedAddressGeneration(seedPhrase, derivationPath, derivationPassword);
              this.setState({ seedPhrase });
            }}
            value={this.state.seedPhrase}
          />
          {isSubstrate && <DerivationPathField
            onChange = { ({derivationPassword, derivationPath, isDerivationPathValid}) => {
              this.debouncedAddressGeneration(seedPhrase, derivationPath, derivationPassword);
              this.setState({ derivationPath, derivationPassword, isDerivationPathValid });
            }}
            styles={styles}
          />}
          <AccountCard
            style={{ marginTop: 20 }}
            address={address || ''}
            networkKey={networkKey || ''}
            title={name}
            seedType={validBip39Seed ? 'bip39' : 'brain wallet'}
          />
          <Button
            buttonStyles={{ marginBottom: 40 }}
            disabled={isSubstrate && (!address || !isDerivationPathValid)}
            title="Next Step"
            onPress={() => {
              const validation = validateSeed(
                seedPhrase,
                validBip39Seed
              );

              if (!validation.valid) {
                if (validation.accountRecoveryAllowed) {
                  return Alert.alert('Warning', `${validation.reason}`, [
                    {
                      text: 'I understand the risks',
                      style: 'default',
                      onPress: () => {
                        navigation.navigate('AccountPin', {
                          isWelcome: navigation.getParam(
                            'isWelcome'
                          ),
                          isNew: true
                        });
                      }
                    },
                    {
                      text: 'Back',
                      style: 'cancel'
                    }
                  ]);
                } else {
                  return Alert.alert('Error', `${validation.reason}`, [
                    {
                      text: 'Back',
                      style: 'cancel'
                    }
                  ]);
                }
              }

              navigation.navigate('AccountPin', {
                isWelcome: navigation.getParam('isWelcome'),
                isNew: true
              });
            }}
          />
        </KeyboardScrollView>
      </SafeAreaView>
    );
  }
}

const styles = StyleSheet.create({
  bodyContainer: {
    backgroundColor: colors.bg,
    flex: 1,
    padding: 20
  },
  safeAreaView: {
    flex: 1
  },
  titleTop: {
    fontFamily: fonts.bold,
    color: colors.bg_text_sec,
    fontSize: 24,
    paddingBottom: 20,
    textAlign: 'center'
  },
  title: {
    fontFamily: fonts.bold,
    color: colors.bg_text_sec,
    fontSize: 18,
    paddingBottom: 20
  }
});
