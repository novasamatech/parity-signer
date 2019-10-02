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

import PropTypes from 'prop-types';
import React from 'react';
import { FlatList, StyleSheet, Text, View } from 'react-native';
import { Subscribe } from 'unstated';

import colors from '../colors';
import AccountCard from '../components/AccountCard';
import Background from '../components/Background';
import Button from '../components/Button';
import PopupMenu from '../components/PopupMenu';
import fonts from "../fonts";
import AccountsStore from '../stores/AccountsStore';

export default class AccountList extends React.PureComponent {
  static navigationOptions = {
    title: 'Accounts',
  };

  render() {

    return (
      <Subscribe to={[AccountsStore]}>
        {accounts => {
          return (
            <AccountListView
              {...this.props}
              accounts={accounts.getAccounts()}
              onAccountSelected={key => {
                accounts.select(key);
                this.props.navigation.navigate('AccountDetails');
              }}
            />
          );
        }}
      </Subscribe>
    );
  }
}

class AccountListView extends React.PureComponent {
  static propTypes = {
    accounts: PropTypes.object.isRequired,
    onAccountSelected: PropTypes.func.isRequired,
  };

  constructor(props) {
    super(props);
  }

  showOnboardingMessage = () => {
    const { navigate } = this.props.navigation;
    const createLink = (text, navigation) => (
      <Text style={styles.link} onPress={() => navigate(navigation)}>
        {text}
      </Text>
    );

    return (
      <View style={styles.onboardingWrapper}>
        <Text style={styles.onboardingText}>
          No account yet?{'\n'}
          {createLink('Create', 'AccountNew')} or{' '}
          {createLink('recover', 'AccountRecover')} an account to get started.
        </Text>
      </View>
    );
  };

  render() {
    const { accounts, navigation, onAccountSelected } = this.props;
    const hasNoAccount = accounts.length < 1;
    const { navigate } = navigation;;
console.log('account list',accounts)
    return (
      <View style={styles.body}>
        <Background />
        <View style={styles.header}>
          <Text style={styles.title}>ACCOUNTS</Text>
          <View style={styles.menuView}>
            <PopupMenu
              onSelect={value => navigate(value)}
              menuTriggerIconName={'add'}
              menuItems={[
                { value: 'AccountNew', text: 'New Account' },
                { value: 'AccountRecover', text: 'Recover Account' },
                { value: 'About', text: 'About' },
              ]}
            />
          </View>
        </View>
        {hasNoAccount && this.showOnboardingMessage()}
        <FlatList
          ref={list => {
            this.list = list;
          }}
          style={styles.content}
          data={[...accounts.entries()]}
          keyExtractor={([key, value]) => key}
          ItemSeparatorComponent={() => <View style={{ height: 20 }} />}
          renderItem={({ item: [key, value] }) => {
            const account = value;
            // console.log('key',key,'value',value)
            return (
              <AccountCard
                address={account.address}
                networkKey={account.networkKey}
                onPress={() => {
                  onAccountSelected(key);
                }}
                style={{ paddingBottom: null }}
                title={account.name}
              />
            );
          }}
          enableEmptySections
        />
        {!hasNoAccount && (
          <View style={styles.bottom}>
            <Button
              buttonStyles={{ height: 60 }}
              title="Scan"
              onPress={() => navigate('QrScanner')}
            />
          </View>
        )}
      </View>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    backgroundColor: colors.bg,
    flex: 1,
    flexDirection: 'column',
    padding: 20,
  },
  bottom: {
    marginTop: 20
  },
  content: {
    flex: 1,
  },
  header: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingBottom: 20,
    justifyContent: 'center'
  },
  menuView: {
    flex: 1,
    alignItems: 'flex-end'
  },
  title: {
    color: colors.bg_text_sec,
    fontSize: 18,
    fontFamily: fonts.bold,
    flexDirection: 'column',
    justifyContent: 'center'
  },
  link: {
    textDecorationLine: 'underline',
  },
  onboardingWrapper: {
    flex: 1,
    flexDirection: 'row',
    alignItems: 'flex-end'
  },
  onboardingText: {
    fontFamily: fonts.regular,
    fontSize: 20,
    color: colors.bg_text_sec
  }
});
