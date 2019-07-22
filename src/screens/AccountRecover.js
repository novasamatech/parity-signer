// Copyright 2015-2017 Parity Technologies (UK) Ltd.
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
import { Alert, findNodeHandle, SafeAreaView, StyleSheet, ScrollView, Text, View } from 'react-native';
import { KeyboardAwareScrollView } from 'react-native-keyboard-aware-scroll-view'
import { Subscribe } from 'unstated';

import colors from '../colors';
import AccountCard from '../components/AccountCard';
import AccountSeed from '../components/AccountSeed';
import Background from '../components/Background';
import Button from '../components/Button';
import TextInput from '../components/TextInput';
import TouchableItem from '../components/TouchableItem';
import { DEFAULT_NETWORK_COLOR, NETWORK_COLOR, NETWORK_TITLES } from '../constants';
import AccountsStore from '../stores/AccountsStore';
import { validateSeed } from '../util/account';

export default class AccountRecover extends React.Component {
  static navigationOptions = {
    title: 'Recover Account',
    headerBackTitle: 'Back'
  };
  render () {
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
  }

  render () {
    const { accounts } = this.props;
    const selected = accounts.getNew();
    const chainId = selected.chainId;

    return (
      <SafeAreaView style={styles.safeAreaView}>
        <KeyboardAwareScrollView style={styles.bodyContainer}>
          <Background />
          <ScrollView
            contentContainerStyle={{ justifyContent: 'flex-end' }}
            style={{ flex: 1 }}
            enableOnAndroid
            scrollEnabled
            keyboardShouldPersistTaps="handled"
            extraHeight={230}
            innerRef={ref => {
              this.scroll = ref;
            }}
          >
            <Text style={styles.titleTop}>RECOVER ACCOUNT</Text>
            <Text style={styles.title}>CHOOSE NETWORK</Text>
            <TouchableItem
              style={[
                styles.card,
                {
                  backgroundColor:
                    NETWORK_COLOR[chainId] || DEFAULT_NETWORK_COLOR,
                  marginBottom: 20
                }
              ]}
              onPress={() =>
                this.props.navigation.navigate('AccountNetworkChooser')
              }
            >
              <Text
                style={[
                  styles.cardText,
                  {
                    color: NETWORK_COLOR[chainId]
                      ? colors.card_bg
                      : colors.card_text
                  }
                ]}
              >
                {NETWORK_TITLES[chainId]}
              </Text>
            </TouchableItem>
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
              valid={validateSeed(selected.seed, selected.validBip39Seed).valid}
              onChangeText={seed => {
                accounts.updateNew({ seed });
              }}
              value={selected.seed}
            />
            <AccountCard
              style={{ marginTop: 20 }}
              address={selected.address || ''}
              chainId={selected.chainId || ''}
              title={selected.name}
              seedType={selected.validBip39Seed ? 'bip39' : 'brain wallet'}
            />
            <Button
              buttonStyles={{ marginBottom: 40 }}
              title="Next Step"
              onPress={() => {
                const validation = validateSeed(selected.seed, selected.validBip39Seed);
                if (!validation.valid) {
                  Alert.alert(
                    'Warning:',
                    `${validation.reason}`,
                    [
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
                    ]
                  );
                  return;
                }
                this.props.navigation.navigate('AccountPin', {
                  isWelcome: this.props.navigation.getParam('isWelcome'),
                  isNew: true
                });
              }}
            />
          </ScrollView>
        </KeyboardAwareScrollView>
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
  },
  card: {
    backgroundColor: colors.card_bg,
    padding: 20
  },
  cardText: {
    color: colors.card_text,
    fontFamily: 'Manifold CF',
    fontSize: 20,
    fontWeight: 'bold'
  }
});
