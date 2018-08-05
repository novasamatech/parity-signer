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
import PropTypes from 'prop-types';
import {
  Alert,
  ScrollView,
  View,
  Text,
  TouchableOpacity,
  Share,
  StyleSheet,
  Clipboard,
  AppState
} from 'react-native';
import { Subscribe } from 'unstated';
import AccountsStore from '../stores/AccountsStore';
import Background from '../components/Background';
import AccountSeed from '../components/AccountSeed';
import AccountCard from '../components/AccountCard';
import AccountIconChooser from '../components/AccountIconChooser';
import TextInput from '../components/TextInput';
import TouchableItem from '../components/TouchableItem';
import Button from '../components/Button';
import colors from '../colors';

export default class AccountBackup extends React.PureComponent {
  static navigationOptions = {
    title: 'Account Backup'
  };
  render() {
    return (
      <Subscribe to={[AccountsStore]}>
        {accounts => <AccountBackupView {...this.props} accounts={accounts} />}
      </Subscribe>
    );
  }
}

class AccountBackupView extends React.PureComponent {
  constructor(...args) {
    super(...args);
    this.handleAppStateChange = this.handleAppStateChange.bind(this);
  }

  componentDidMount() {
    AppState.addEventListener('change', this.handleAppStateChange);
  }

  handleAppStateChange = (nextAppState) => {
    if (nextAppState === 'background') {
      this.props.navigation.goBack();
    }
  }

  componentWillUnmount() {
    const { accounts } = this.props;
    const selected =
      accounts.getNew().address && accounts.getNew().address.length
        ? accounts.getNew()
        : accounts.getSelected();
    accounts.lockAccount(selected);
    AppState.removeEventListener('change', this._handleAppStateChange);
  }
  render() {
    const { accounts, navigation } = this.props;
    const isNew = navigation.getParam('isNew');
    const selected = isNew ? accounts.getNew() : accounts.getSelected();
    return (
      <ScrollView
        style={styles.body}
        contentContainerStyle={styles.bodyContent}
      >
        <Background />
        <Text style={styles.titleTop}>BACKUP ACCOUNT</Text>
        <AccountCard
          address={selected.address}
          chainId={selected.chainId}
          title={selected.name}
        />
        <Text style={styles.titleTop}>RECOVERY WORDS</Text>
        <Text style={styles.hintText}>
          Write these words down on paper. Keep it safe. These words allow
          anyone to recover this account.
        </Text>
        <TouchableItem
          onPress={() => {
            Alert.alert(
              'Use paper to store seed phrases',
              `It's not recommended to transfer or store seed phrases digitally and unencrypted. Everyone who have the phrase is able to spend funds from this account.
              `,
              [
                {
                  text: 'Copy anyway',
                  style: 'default',
                  onPress: () => {
                    Clipboard.setString(selected.seed);
                  }
                },
                {
                  text: 'Cancel',
                  style: 'cancel'
                }
              ]
            );
          }}
        >
          <Text
            style={{
              padding: 10,
              height: 120,
              lineHeight: 26,
              fontSize: 20,
              backgroundColor: colors.card_bg
            }}
          >
            {selected.seed}
          </Text>
        </TouchableItem>
        <Button
          buttonStyles={[styles.nextStep, { marginBottom: 20 }]}
          title="Done Backup"
          onPress={() => {
            if (isNew) {
              Alert.alert(
                'Important information',
                "Make sure you've backed up recovery words for your account. Recovery words are the only way to restore access to your account in case of device failure/lost.",
                [
                  {
                    text: 'Proceed',
                    onPress: () => {
                      this.props.navigation.navigate('AccountPin', {
                        isWelcome: navigation.getParam('isWelcome'),
                        isNew
                      });
                    }
                  },
                  {
                    text: 'Cancel',
                    style: 'cancel'
                  }
                ]
              );
            } else {
              navigation.navigate('AccountList');
            }
          }}
        />

        {!isNew && (
          <Button
            buttonStyles={{ marginBottom: 40 }}
            title="Change PIN"
            onPress={() => {
              navigation.navigate('AccountPin', { isNew });
            }}
          />
        )}

        {!isNew && (
          <Button
            buttonStyles={styles.deleteButton}
            title="Delete Account"
            onPress={() => {
              Alert.alert(
                'Delete Account',
                `Are you sure to delete ${selected.name || selected.address} and its private key?`,
                [
                  {
                    text: 'Delete',
                    style: 'destructive',
                    onPress: () => {
                      accounts.deleteAccount(selected);
                      this.props.navigation.navigate('AccountList');
                    }
                  },
                  {
                    text: 'Cancel',
                    style: 'cancel'
                  }
                ]
              );
            }}
          />
        )}
      </ScrollView>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    backgroundColor: colors.bg,
    flex: 1,
    flexDirection: 'column',
    padding: 20
  },
  bodyContent: {
    paddingBottom: 40
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
    fontFamily: 'Manifold CF',
    fontWeight: 'bold',
    paddingBottom: 20,
    textAlign: 'center'
  },
  hintText: {
    fontFamily: 'Manifold CF',
    textAlign: 'center',
    color: colors.bg_text_sec,
    fontWeight: '700',
    fontSize: 12,
    paddingBottom: 20
  },
  nextStep: {
    marginTop: 20
  },
  deleteButton: {
    backgroundColor: colors.bg_alert
  }
});
