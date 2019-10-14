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
import { Alert, ScrollView, StyleSheet, Text, View } from 'react-native';
import { NavigationActions, StackActions } from 'react-navigation';
import { Subscribe } from 'unstated';

import colors from '../colors';
import fonts from "../fonts";
import AccountCard from '../components/AccountCard';
import QrView from '../components/QrView';
import AccountsStore from '../stores/AccountsStore';
import TxStore from '../stores/TxStore';
import PopupMenu from '../components/PopupMenu'
import { NETWORK_LIST, NetworkProtocols } from '../constants';

export default class AccountDetails extends React.Component {
  static navigationOptions = {
    title: 'Account Details'
  };

  render() {
    return (
      <Subscribe to={[AccountsStore, TxStore]}>
        {(accounts, txStore) => (
          <AccountDetailsView
            {...this.props}
            txStore={txStore}
            accounts={accounts}
            selected={accounts.getSelected() && accounts.getSelected().address}
          />
        )}
      </Subscribe>
    );
  }
}

class AccountDetailsView extends React.Component {
  constructor(props) {
    super(props);
  }

  onDelete = () => {
    const accounts = this.props.accounts
    const selected = accounts.getSelected();
    const selectedKey = accounts.getSelectedKey();

    Alert.alert(
      'Delete Account',
      `Do you really want to delete ${selected.name || selected.address || 'this account'}?
This account can only be recovered with its associated recovery phrase.`,
      [
        {
          text: 'Delete',
          style: 'destructive',
          onPress: () => {
            accounts.deleteAccount(selectedKey);
            const resetAction = StackActions.reset({
              index: 0,
              key: undefined, // FIXME workaround for now, use SwitchNavigator later: https://github.com/react-navigation/react-navigation/issues/1127#issuecomment-295841343
              actions: [NavigationActions.navigate({ routeName: 'AccountList' })]
            });
            this.props.navigation.dispatch(resetAction);
          }
        },
        {
          text: 'Cancel',
          style: 'cancel'
        }
      ]
    );
  }

  onOptionSelect = async (value) => {
    const navigate = this.props.navigation.navigate
    const accounts = this.props.accounts;

    if (value !== 'AccountEdit') {
      if (accounts.getSelected().biometricEnabled) { 
          try {
              await accounts.unlockAccountWithBiometric(accounts.getSelectedKey());
              if (value === 'AccountDelete') {
                  this.onDelete();
              } else if (value === 'AccountBiometric') {
                  await accounts.disableBiometric(accounts.getSelectedKey());
              } else {
                  navigate(value);
              }
          } catch (e) {
              console.log(e);
              Alert.alert('Biometric Error', e.message, [
                  { 
                      text: 'Ok',
                      style: 'default',
                      onPress: () => {
                          navigate('AccountUnlock', {
                            next: value,
                            onDelete: this.onDelete
                          });
                      },
                      onDismiss: () => {
                          navigate('AccountUnlock', {
                            next: value,
                            onDelete: this.onDelete
                          });
                      }
                  }
              ]);
          }
      } else {
          navigate('AccountUnlock', {
            next: value,
            onDelete: this.onDelete
          });
      }
    } else {
      navigate(value);
    }
  }

  renderWarningUnknownAccount = function () {
    return (
      <View style={styles.warningView}>
        <Text style={{...styles.title, ...styles.warningTitle}}>Warning</Text>
        <Text>
          This account wasn't retrieved successfully. This could be because its network isn't supported,
          or you upgraded Parity Signer without wiping your device and this account couldn't be migrated.
          {'\n'}{'\n'}
          To be able to use this account you need to:{'\n'}
          - write down its recovery phrase{'\n'}
          - delete it{'\n'}
          - recover it{'\n'}
        </Text>
      </View>
    )
  }

  render() {
    const { accounts } = this.props
    const account = accounts.getSelected();
    const selectedKey = accounts.getSelectedKey();

    if (!account) {
      return null;
    }

    const protocol = account.networkKey && NETWORK_LIST[account.networkKey] && NETWORK_LIST[account.networkKey].protocol || NetworkProtocols.UNKNOWN ;

    return (
      <Subscribe to={[AccountsStore]}>
        {(accounts) => (
          <ScrollView
            contentContainerStyle={styles.bodyContent}
            style={styles.body}>
            <View style={styles.header}>
              <Text style={styles.title}>ACCOUNT</Text>
              <View style={styles.menuView}>
                <PopupMenu
                  onSelect={this.onOptionSelect}
                  menuTriggerIconName={"more-vert"}
                  menuItems={[
                    { value: 'AccountEdit', text: 'Edit' },
                    { value: 'AccountPin', text: 'Change Pin' },
                    { value: 'AccountBiometric', text: accounts.getSelected().biometricEnabled ? "Disable Biometric" : "Enable Biometric" },
                    { value: 'AccountBackup', text: 'View Recovery Phrase' },
                    { value: 'AccountDelete', text: 'Delete', textStyle: styles.deleteText }]}
                />
              </View>
            </View>
          <AccountCard
            address={account.address}
            networkKey={account.networkKey}
            title={account.name}
          />
          <View style={styles.qr}>
          {
            protocol !== NetworkProtocols.UNKNOWN
              ? <QrView data={selectedKey} />
              : this.renderWarningUnknownAccount()
           }
           </View>
         </ScrollView>
       )}
       </Subscribe>
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
  qr: {
    marginTop: 20,
    backgroundColor: colors.card_bg
  },
  deleteText: {
    color: colors.bg_alert
  },
  header: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingBottom: 20,
    justifyContent: 'center',
  },
  menuView: {
    flex: 1,
    alignItems: 'flex-end',
  },
  title: {
    color: colors.bg_text_sec,
    fontSize: 18,
    fontFamily: fonts.bold,
    flexDirection: 'column',
    justifyContent: 'center',
  },
  warningTitle: {
    color: colors.bg_alert,
    fontSize: 20,
    marginBottom: 10
  },
  warningView: {
    padding: 20
  }
});
