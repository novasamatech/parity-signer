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
import { Alert, ScrollView, StyleSheet, Text, View } from 'react-native';
import { Subscribe } from 'unstated';
import {
  Menu,
  MenuOptions,
  MenuOption,
  MenuTrigger,
} from 'react-native-popup-menu';
import Icon from 'react-native-vector-icons/MaterialIcons';

import colors from '../colors';
import AccountCard from '../components/AccountCard';
import QrView from '../components/QrView';
import AccountsStore from '../stores/AccountsStore';
import TxStore from '../stores/TxStore';
import { accountId } from '../util/account';

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

  componentDidMount() {
    this.subscription = this.props.navigation.addListener('willFocus', t => {
      this.props.txStore.loadTxsForAccount(this.props.accounts.getSelected());
    });
  }

  onDelete = () => {
    const accounts = this.props.accounts
    const selected = accounts.getSelected();

    Alert.alert(
      'Delete Account',
      `Do you really want to delete ${selected.name || selected.address}?
This account can only be recovered with its associated recovery phrase.`,
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
  }

  componentWillUnmount() {
    this.subscription.remove();
  }

  onOptionSelect = (value) => {
    const navigate = this.props.navigation.navigate

    if (value !== 'AccountEdit') {
      navigate('AccountUnlock', {
        next: value,
        onDelete: this.onDelete
      });
    } else {
      navigate(value);
    }
  }

  showEditMenu = () => {
    const editIcon = <Icon name="more-vert" size={35} color={colors.bg_text_sec} />

    return (
      <View style={styles.menuView}>
        <Menu
          onSelect={this.onOptionSelect}
        >
          <MenuTrigger children={editIcon} />
          <MenuOptions customStyles={menuOptionsStyles}>
            <MenuOption value={'AccountEdit'} text='Edit' />
            <MenuOption value={'AccountPin'} text='Change Pin' />
            <MenuOption value={'AccountBackup'} text='View Recovery Phrase' />
            <MenuOption value={'AccountDelete'} ><Text style={styles.deleteText}>Delete</Text></MenuOption>
          </MenuOptions>
        </Menu>
      </View>
    )
  };

  render() {
    const account = this.props.accounts.getSelected();
    if (!account) {
      return null;
    }

    return (
      <ScrollView
        contentContainerStyle={styles.bodyContent}
        style={styles.body}
      >
        <View style={styles.header}>
          <Text style={styles.title}>ACCOUNT</Text>
          {this.showEditMenu()}
        </View>
        <AccountCard
          address={account.address}
          chainId={account.chainId}
          title={account.name}
        />
        <View style={styles.qr}>
          <QrView text={accountId(account)} />
        </View>
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
  qr: {
    marginTop: 20,
    backgroundColor: colors.card_bg
  },
  deleteText: {
    color: 'red'
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
    fontFamily: 'Manifold CF',
    fontWeight: 'bold',
    flexDirection: 'column',
    justifyContent: 'center',
  }
});

const menuOptionsStyles = {
  optionWrapper: {
    padding: 15,
  },
  optionText: {
    fontFamily: 'Roboto',
    fontSize: 16
  }
}
