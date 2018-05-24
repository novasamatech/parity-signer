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
import colors from '../colors';

export default class AccountList extends Component {
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
              onAccountSelected={address => {
                accounts.select(address);
                this.props.navigation.navigate('AccountDetails');
              }}
            />
          );
        }}
      </Subscribe>
    );
  }
}

class AccountListView extends Component {
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
  }

  render() {
    return (
      <View style={styles.body}>
        <Text style={styles.title}>ACCOUNTS</Text>
        <FlatList
          style={styles.content}
          data={this.props.accounts}
          keyExtractor={item => item.address}
          ItemSeparatorComponent={() => <View style={{ height: 20 }} />}
          renderItem={({ item }) => {
            return (
              <AccountCard
                title={item.name ? item.name : 'no name'}
                style={{ paddingBottom: null }}
                address={item.address}
                onPress={() => {
                  this.props.onAccountSelected(item.address);
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
    fontFamily: 'Roboto',
    fontWeight: 'bold',
    paddingBottom: 20
  },
  body: {
    flex: 1,
    flexDirection: 'column',
    padding: 20,
    backgroundColor: colors.bg
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
