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

import colors from '../colors';
import AccountIconChooser from '../components/AccountIconChooser';
import Background from '../components/Background';
import Button from '../components/Button';
import TextInput from '../components/TextInput';
import TouchableItem from '../components/TouchableItem';
import { NETWORK_LIST, NetworkProtocols, SubstrateNetworkKeys } from '../constants';
import AccountsStore from '../stores/AccountsStore';
import { validateSeed } from '../util/account';
import NetworkButton from '../components/NetworkButton';
import KeyboardScrollView from '../components/KeyboardScrollView';

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
          <ScrollView
            style={{ padding: 20 }}
            keyboardDismissMode="on-drag"
            keyboardShouldPersistTaps="always"
            containerStyle={styles.bodyContainer}
          >
            <View style={styles.top}>
              <Text style={styles.titleTop}>CREATE ACCOUNT</Text>
              <Text style={styles.title}>CHOOSE NETWORK</Text>
              <NetworkButton network={network}/>
              <Text style={[styles.title, { marginTop: 20 }]}>
                CHOOSE AN IDENTICON
              </Text>
              <AccountIconChooser
                value={selected && selected.seed && selected.address}
                onSelect={({ address, bip39, seed }) => {
                  accounts.updateNew({ address, seed, validBip39Seed: bip39 });
                }}
              />
              <Text style={styles.title}>ACCOUNT NAME</Text>
              <TextInput
                onChangeText={name => accounts.updateNew({ name })}
                value={selected && selected.name}
                placeholder="Enter a new account name"
              />
            </View>
            <View style={styles.bottom}>
              <Text style={styles.hintText}>
                On the next step you will be asked to backup your account, get pen
                and paper ready
              </Text>
              <Button
                buttonStyles={styles.nextStep}
                title="Next Step"
                disabled={ !validateSeed(selected.seed, selected.validBip39Seed).valid }
                onPress={() => {
                  // TODO remove this hardcoded address for SUBSTRATE
                  if (selected.networkKey === SubstrateNetworkKeys.SUBSTRATE) {
                    accounts.updateNew({ address:'5EjSNPzM9T6Nb19zb38TcwBQh5hNWG47noi7anXQT64BBJBx', seed:'this is sparta', publicKey:0x123 ,validBip39Seed: false });
                  }

                  validateSeed(selected.seed, selected.validBip39Seed).valid &&
                    this.props.navigation.navigate('AccountBackup', {
                      isNew: true,
                      isWelcome: this.props.navigation.getParam('isWelcome')
                    });
                }}
              />
            </View>
          </ScrollView>
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
    fontFamily: 'Manifold CF',
    color: colors.bg_text_sec,
    fontSize: 18,
    fontWeight: 'bold',
    paddingBottom: 20
  },
  titleTop: {
    color: colors.bg_text_sec,
    fontSize: 24,
    fontWeight: 'bold',
    paddingBottom: 20,
    textAlign: 'center'
  },
  hintText: {
    fontFamily: 'Manifold CF',
    textAlign: 'center',
    paddingTop: 20,
    color: colors.bg_text_sec,
    fontWeight: '800',
    fontSize: 12
  },
  nextStep: {
    marginTop: 15
  }
});
