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

import React, { Component } from 'react';
import PropTypes from 'prop-types';
import { View, Text, FlatList, StatusBar, StyleSheet } from 'react-native';
import { Subscribe } from 'unstated';
import AccountsStore from '../stores/AccountsStore';
import AccountCard from '../components/AccountCard';
import Button from '../components/Button';
import Background from '../components/Background';
import colors from '../colors';
import { accountId } from '../util/account';

export default class AccountList extends React.PureComponent {
  static navigationOptions = {
    title: 'Accounts'
  };
  render() {
    return (
      <Subscribe to={[AccountsStore]}>
        {accounts => {
          return (
            <AccountListView
              {...this.props}
              accounts={accounts.getAccounts()}
              onAccountSelected={async address => {
                await accounts.select(address);
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
    accounts: PropTypes.arrayOf(
      PropTypes.shape({
        address: PropTypes.string.isRequired
      })
    ).isRequired,
    onAccountSelected: PropTypes.func.isRequired
  };

  constructor(props) {
    super(props);
    this.scrollToIndex = this.scrollToIndex.bind(this);
  }

  scrollToIndex() {
    const { accounts, navigation } = this.props;
    const id = navigation.getParam('accountId');
    const index = id
      ? accounts.findIndex(a => id === accountId(a))
      : navigation.getParam('index', -1);
    if (this.list && typeof index === 'number' && index !== -1) {
      navigation.setParams({ accountId: undefined, index: undefined });
      this.list.scrollToIndex({ index });
    }
  }

  componentDidMount() {
    this.scrollToIndex();
  }

  componentDidUpdate() {
    this.scrollToIndex();
  }

  render() {
    return (
      <View style={styles.body}>
        <Background />
        <Text style={styles.title}>ACCOUNTS</Text>
        <FlatList
          ref={list => {
            this.list = list;
          }}
          style={styles.content}
          data={this.props.accounts}
          keyExtractor={account => accountId(account)}
          ItemSeparatorComponent={() => <View style={{ height: 20 }} />}
          renderItem={({ item: account }) => {
            return (
              <AccountCard
                title={account.name}
                style={{ paddingBottom: null }}
                address={account.address}
                chainId={account.chainId}
                onPress={() => {
                  this.props.onAccountSelected(account);
                }}
              />
            );
          }}
          enableEmptySections
        />
        <View style={styles.bottom}>
          <Button
            buttonStyles={{ height: 60 }}
            title="Add Account"
            onPress={() => {
              this.props.navigation.navigate('AccountAdd');
            }}
          />
        </View>
      </View>
    );
  }
}

const styles = StyleSheet.create({
  title: {
    color: colors.bg_text_sec,
    fontSize: 18,
    fontFamily: 'Manifold CF',
    fontWeight: 'bold',
    paddingBottom: 20
  },
  body: {
    backgroundColor: colors.bg,
    flex: 1,
    flexDirection: 'column',
    padding: 20
  },
  content: {
    flex: 1
  },
  bottom: {
    marginTop: 20,
    flexBasis: 60
  },
  introContainer: {
    padding: 30,
    flex: 1,
    flexDirection: 'column',
    justifyContent: 'center'
  },
  introText: {
    textAlign: 'center',
    fontSize: 16,
    marginBottom: 20
  }
});
