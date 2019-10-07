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

import React, { Component } from 'react';
import { StyleSheet, Text, View } from 'react-native';
import { NavigationActions, StackActions } from 'react-navigation';
import { Subscribe } from 'unstated';
import styles from '../styles';
import Background from '../components/Background';
import Button from '../components/Button';
import TextInput from '../components/TextInput';
import AccountsStore from '../stores/AccountsStore';

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
    const account = accountCreation
      ? accounts.getNew()
      : accounts.getSelected();
    if (this.state.pin.length >= 6 && this.state.pin === this.state.confirmation) {
      if (accountCreation) {
        await accounts.submitNew(pin);
        const resetAction = StackActions.reset({
          index: 0,
          key: undefined, // FIXME workaround for now, use SwitchNavigator later: https://github.com/react-navigation/react-navigation/issues/1127#issuecomment-295841343
          actions: [NavigationActions.navigate({ routeName: 'AccountList' })]
        });
        this.props.navigation.dispatch(resetAction);
      } else {
        await accounts.save(accounts.getSelectedKey(), account, pin);
        const resetAction = StackActions.reset({
          index: 1,
          key: undefined, // FIXME workaround for now, use SwitchNavigator later: https://github.com/react-navigation/react-navigation/issues/1127#issuecomment-295841343
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
      return <Text style={[styles.t_hintText, styles.t_errorText]}>Your pin must be at least 6 digits long!</Text>
    } else if (this.state.pinMismatch) {
      return <Text style={[styles.t_hintText, styles.t_errorText]}>Pin codes don't match!</Text>
    }
    return (<Text style={styles.t_hintText}>Choose a PIN code with 6 or more digits</Text>)
  }

  onPinInputChange = (stateName, pinInput) => {
    if (/^\d+$|^$/.test(pinInput)) {
      this.setState({[stateName]: pinInput, pinMismatch: false, pinTooShort: false})
    }
  }

  render() {
    const title = 'Account PIN';
    return (
      <View style={[styles.b_flex, styles.b_paddingH]}>
        <Background />
        <Text style={[styles.t_h1, styles.header]}>{title}</Text>
        {this.showHintOrError()}
        <Text style={[styles.t_text, styles.b_marginV_xs]}>PIN</Text>
        <PinInput
          autoFocus
          returnKeyType="next"
          onFocus={() => this.setState({ focusConfirmation: false })}
          onSubmitEditing={() => {
            this.setState({ focusConfirmation: true });
          }}
          onChangeText={pin => this.onPinInputChange('pin', pin)}
          value={this.state.pin}
        />
        <Text style={[styles.t_text, styles.b_marginV_xs]}>Confirm PIN</Text>
        <PinInput
          returnKeyType="done"
          focus={this.state.focusConfirmation}
          onChangeText={confirmation => this.onPinInputChange('confirmation', confirmation)}
          value={this.state.confirmation}
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
        style={[styles.t_parityXL, styles.pinInput]}
        {...this.props}
      />
    );
  }
}
