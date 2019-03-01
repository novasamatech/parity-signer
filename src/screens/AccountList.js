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

import PropTypes from 'prop-types';
import React from 'react';
import { FlatList, Image, StyleSheet, Text, View } from 'react-native';
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
import Background from '../components/Background';
import Button from '../components/Button';
import AccountsStore from '../stores/AccountsStore';
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

  showOnboardingMessage = () => {
    return (
      <View style={styles.onboardingWrapper} >
        <Text style={styles.onboardingText}>Nothing here yet.
        Click on the + to add an account</Text>
        <View style={styles.onboardingArrowWrapper}><Image style={styles.onboardingArrow} source={require('../assets/img/onboardingArrow.png')}
        /></View>
      </View>
    )
  }

  AccountMenu = () => {
    const addIcon = <Icon name="add" size={35} color={colors.bg_text_sec} />

    return (
      <View style={styles.menuView}>
        <Menu
          onSelect={value => this.props.navigation.navigate(value)}
        >
          <MenuTrigger children={addIcon} />
          <MenuOptions customStyles={menuOptionsStyles}>
            <MenuOption value={'AccountNew'} text='New Account' />
            <MenuOption value={'AccountRecover'} text='Recover Account' />
            <MenuOption value={'About'} text='About' />
          </MenuOptions>
        </Menu>
      </View>
    )
  };

  render() {
    return (
      <View style={styles.body}>
        <Background />
        <View style={styles.header}>
          <Text style={styles.title}>ACCOUNTS</Text>
          {this.AccountMenu()}
        </View>
        {this.props.accounts.length < 1 && this.showOnboardingMessage()}
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
            title="Scan"
            onPress={() => this.props.navigation.navigate('QrScanner')}
          />
        </View>
      </View>
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
  bottom: {
    marginTop: 20,
  },
  content: {
    flex: 1
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
  },
  onboardingWrapper: {
    flex: 1,
    flexDirection: 'row',
    alignItems: 'flex-end'
  },
  onboardingArrowWrapper:
  {
    flex: 1,
    paddingBottom: 20,
    paddingLeft: 5
  },
  onboardingArrow: {
    resizeMode: 'contain'
  },
  onboardingText: {
    flex: 5,
    fontFamily: 'Roboto',
    fontSize: 20,
    color: colors.bg_text_sec,
  }
});

const menuOptionsStyles = {
  optionWrapper: {
    padding: 15,
  },
  optionText: {
    fontFamily: 'Roboto',
    fontSize: 16
  },
};



