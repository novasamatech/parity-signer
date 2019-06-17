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
import { Alert, AppState, Clipboard, ScrollView, StyleSheet, Text } from 'react-native';
import { Subscribe } from 'unstated';
import colors from '../colors';
import AccountCard from '../components/AccountCard';
import Background from '../components/Background';
import Button from '../components/Button';
import TouchableItem from '../components/TouchableItem';
import AccountsStore from '../stores/AccountsStore';

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
    if (nextAppState === 'inactive') {
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
        <Text style={styles.titleTop}>RECOVERY PHRASE</Text>
        <Text style={styles.hintText}>
          Write these words down on paper. Keep it safe. These words allow
          anyone to recover and access the funds of this account.
        </Text>
        <TouchableItem
          onPress={() => {
            Alert.alert(
              'Write this recovery phrase on paper',
              `It is not recommended to transfer or store a recovery phrase digitally and unencrypted. Anyone in possession of this recovery phrase is able to spend funds from this account.
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
              height: 160,
              lineHeight: 26,
              fontSize: 20,
              backgroundColor: colors.card_bg
            }}
          >
            {selected.seed}
          </Text>
        </TouchableItem>
        {(isNew) &&
          <Button
            buttonStyles={[styles.nextStep, { marginBottom: 20 }]}
            title="Backup Done"
            onPress={() => {

              Alert.alert(
                'Important',
                "Make sure you've backed up this recovery phrase. It is the only way to restore your account in case of device failure/lost.",
                [
                  {
                    text: 'Proceed',
                    onPress: () => {
                      this.props.navigation.navigate('AccountPin', { isNew });
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
        }
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
