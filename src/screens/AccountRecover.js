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
import {
  Alert,
  SafeAreaView,
  StyleSheet,
  Text,
  findNodeHandle
} from 'react-native';
import { Subscribe } from 'unstated';

import colors from '../colors';
import AccountCard from '../components/AccountCard';
import AccountSeed from '../components/AccountSeed';
import Background from '../components/Background';
import Button from '../components/Button';
import NetworkButton from '../components/NetworkButton';
import TextInput from '../components/TextInput';
import { NETWORK_LIST } from '../constants';
import AccountsStore from '../stores/AccountsStore';
import { validateSeed } from '../util/account';
import { debounce } from '../util/debounce';
import { brainWalletAddress } from '../util/native';
import KeyboardScrollView from '../components/KeyboardScrollView';

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

    this.state = { seed: '' };
  }

  addressGeneration = seed => {
    const { accounts } = this.props;

    brainWalletAddress(seed)
      .then(({ address, bip39 }) =>
        accounts.updateNew({ address, seed, validBip39Seed: bip39 })
      )
      .catch(console.error);
  };

  debouncedAddressGeneration = debounce(this.addressGeneration, 200);

  componentWillUnmount = function() {
    // called when the user goes back, or finishes the whole recovery process
    this.props.accounts.updateNew({ seed: '' });
  };

  render() {
    const { accounts } = this.props;
    const selected = accounts.getNew();
    const networkKey = selected.networkKey;
    const network = NETWORK_LIST[networkKey];
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
          <NetworkButton network={network} />
          <Text style={styles.title}>ACCOUNT NAME</Text>
          <TextInput
            onChangeText={name => accounts.updateNew({ name })}
            value={selected && selected.name}
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
            valid={validateSeed(selected.seed, selected.validBip39Seed).valid}
            onChangeText={seed => {
              this.debouncedAddressGeneration(seed);
              this.setState({ seed });
            }}
            value={this.state.seed}
          />
          <AccountCard
            style={{ marginTop: 20 }}
            address={selected.address || ''}
            networkKey={selected.networkKey || ''}
            title={selected.name}
            seedType={selected.validBip39Seed ? 'bip39' : 'brain wallet'}
          />
          <Button
            buttonStyles={{ marginBottom: 40 }}
            title="Next Step"
            onPress={() => {
              const validation = validateSeed(
                selected.seed,
                selected.validBip39Seed
              );

              if (!validation.valid) {
                if (validation.accountRecoveryAllowed) {
                  return Alert.alert('Warning:', `${validation.reason}`, [
                    {
                      text: 'I understand the risks',
                      style: 'default',
                      onPress: () => {
                        this.props.navigation.navigate('AccountPin', {
                          isWelcome: this.props.navigation.getParam(
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
                  return Alert.alert('Error:', `${validation.reason}`, [
                    {
                      text: 'Back',
                      style: 'cancel'
                    }
                  ]);
                }
              }

              this.props.navigation.navigate('AccountPin', {
                isWelcome: this.props.navigation.getParam('isWelcome'),
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
    fontFamily: 'Manifold CF',
    color: colors.bg_text_sec,
    fontSize: 24,
    fontWeight: 'bold',
    paddingBottom: 20,
    textAlign: 'center'
  },
  title: {
    fontFamily: 'Manifold CF',
    color: colors.bg_text_sec,
    fontSize: 18,
    fontWeight: 'bold',
    paddingBottom: 20
  }
});
