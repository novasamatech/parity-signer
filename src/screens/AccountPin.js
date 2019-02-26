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
import { StyleSheet, Text, View } from 'react-native';
import { NavigationActions, StackActions } from 'react-navigation';
import { Subscribe } from 'unstated';
import colors from '../colors';
import Background from '../components/Background';
import Button from '../components/Button';
import TextInput from '../components/TextInput';
import AccountsStore from '../stores/AccountsStore';
import { accountId } from '../util/account';

export default class AccountPin extends React.PureComponent {
  render() {
    return (
      <Subscribe to={[AccountsStore]}>
        {accounts => <AccountPinView {...this.props} accounts={accounts} />}
      </Subscribe>
    );
  }
}

class AccountPinView extends React.PureComponent {
  constructor(...args) {
    super(...args);
    this.submit = this.submit.bind(this);
  }

  state = {
    pin: '',
    confirmation: '',
    focusConfirmation: false,
    pinTooShort: false,
    pinMismatch: false
  };

  async submit() {
    const { accounts, navigation } = this.props;
    const accountCreation = navigation.getParam('isNew');
    const { pin } = this.state;
    if (
      this.state.pin.length >= 6 &&
      this.state.pin === this.state.confirmation
    ) {
      let account = null;
      if (accountCreation) {
        account = accounts.getNew();
        await accounts.submitNew(pin);
        await accounts.select(account);
        accounts.refreshList();
        navigation.popToTop();
        navigation.navigate('AccountList', {
          accountId: accountId(account)
        });
      } else {
        account = accounts.getSelected();
        await accounts.save(account, pin);
        const resetAction = StackActions.reset({
          index: 1,
          actions: [
            NavigationActions.navigate({ routeName: 'AccountList' }),
            NavigationActions.navigate({ routeName: 'AccountDetails' })
          ]
        });
        this.props.navigation.dispatch(resetAction);
      }
    } else {
      if (this.state.pin.length < 6) {
        this.setState({ pinTooShort: true });
      } else if (this.state.pin !== this.state.confirmation)
        this.setState({ pinMismatch: true });
    }
  }

  showHintOrError = () => {
    if (this.state.pinTooShort) {
      return <Text style={styles.errorText}>Your pin must be at least 6 digits long!</Text>
    } else if (this.state.pinMismatch) {
      return <Text style={styles.errorText}>Pin codes don't match!</Text>
    }
    return (<Text style={styles.hintText}>Choose a PIN code with 6 or more digits</Text>)
  }

  render() {
    const title = 'ACCOUNT PIN';
    return (
      <View style={styles.body}>
        <Background />
        <Text style={styles.titleTop}>{title}</Text>
        {this.showHintOrError()}
        <Text style={styles.title}>PIN</Text>
        <PinInput
          autoFocus
          returnKeyType="next"
          onFocus={() => this.setState({ focusConfirmation: false })}
          onSubmitEditing={() => {
            this.setState({ focusConfirmation: true });
          }}
          onChangeText={pin => this.setState({ pin: pin, pinMismatch: false, pinTooShort: false })}
          value={this.state.pin}
        />
        <Text style={styles.title}>CONFIRM PIN</Text>
        <PinInput
          returnKeyType="done"
          focus={this.state.focusConfirmation}
          onChangeText={confirmation => this.setState({ confirmation: confirmation, pinMismatch: false, pinTooShort: false })}
          value={this.state.confirmation}
          onSubmitEditing={this.submit}
        />
        <Button
          onPress={this.submit}
          color="green"
          title="Done"
          accessibilityLabel={'Done'}
        />
      </View>
    );
  }
}

class PinInput extends Component {
  render() {
    return (
      <TextInput
        keyboardAppearance="dark"
        clearTextOnFocus
        editable
        fontSize={24}
        keyboardType="numeric"
        multiline={false}
        autoCorrect={false}
        numberOfLines={1}
        returnKeyType="next"
        secureTextEntry
        style={styles.pinInput}
        {...this.props}
      />
    );
  }
}

const styles = StyleSheet.create({
  body: {
    backgroundColor: colors.bg,
    padding: 20,
    flex: 1,
    overflow: 'hidden'
  },
  title: {
    fontFamily: 'Manifold CF',
    color: colors.bg_text_sec,
    fontSize: 18,
    fontWeight: 'bold',
    paddingBottom: 10
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
  errorText: {
    fontFamily: 'Manifold CF',
    textAlign: 'center',
    color: colors.bg_alert,
    fontWeight: '700',
    fontSize: 12,
    paddingBottom: 20
  },
  pinInput: {
    marginBottom: 20
  }
});
