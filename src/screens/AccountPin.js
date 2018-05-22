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
import { View, Text, StyleSheet, KeyboardAvoidingView } from 'react-native';
import { StackActions, NavigationActions } from 'react-navigation';
import { Subscribe } from 'unstated';
import AccountsStore from '../stores/AccountsStore';
import TextInput from '../components/TextInput';
import Button from '../components/Button';
import colors from '../colors';

export default class AccountPin extends Component {
  render() {
    return (
      <Subscribe to={[AccountsStore]}>{accounts => <AccountPinView {...this.props} accounts={accounts} />}</Subscribe>
    );
  }
}

class AccountPinView extends Component {
  static propTypes = {
    type: PropTypes.string.isRequired
  };

  static defaultProps = {
    type: 'NEW'
  };

  static defa = {
    type: PropTypes.string.isRequired
  };

  state = {
    pin: '',
    confirmation: ''
  };

  render() {
    const { accounts, type } = this.props;
    console.log(this.props.navigation.state);
    const title = {
      NEW: 'ACCOUNT PIN',
      CHANGE: 'CHANGE PIN'
    }[type];
    return (
      <View style={styles.body}>
        <Text style={styles.titleTop}>{title}</Text>
        <Text style={styles.hintText}>Please make your PIN 6 or more digits</Text>
        <Text style={styles.title}>PIN</Text>
        <PinInput onChangeText={pin => this.setState({ pin })} value={this.state.pin} />
        <Text style={styles.title}>CONFIRM PIN</Text>
        <PinInput onChangeText={confirmation => this.setState({ confirmation })} value={this.state.confirmation} />
        <Button
          onPress={async () => {
            if (this.state.pin.length > 0 && this.state.pin === this.state.confirmation) {
              const account = accounts.getNew();
              await accounts.save(account, this.state.pin);
              accounts.submitNew();
              accounts.select(account.address);
              accounts.refreshList();
              if (this.props.navigation.getParam('isWelcome')) {
                this.props.navigation.navigate('Tabs');
                const resetAction = StackActions.reset({
                  index: 0,
                  actions: [NavigationActions.navigate({ routeName: 'QrScanner' })]
                });
                this.props.navigation.dispatch(resetAction);
              } else {
                const resetAction = StackActions.reset({
                  index: 0,
                  actions: [NavigationActions.navigate({ routeName: 'AccountList' })]
                });
                this.props.navigation.dispatch(resetAction);
                this.props.navigation.navigate('AccountDetails');
              }
            }
          }}
          color="green"
          title="Done"
          accessibilityLabel={'Done'}
        />
      </View>
    );
  }
}

function PinInput(props) {
  return (
    <TextInput
      autoFocus
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
      {...props}
    />
  );
}

const styles = StyleSheet.create({
  body: {
    backgroundColor: colors.bg,
    padding: 20,
    flex: 1
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
    fontFamily: 'Roboto',
    color: colors.bg_text_sec,
    fontSize: 18,
    fontWeight: 'bold',
    paddingBottom: 10
  },
  titleTop: {
    color: colors.bg_text_sec,
    fontSize: 24,
    fontWeight: 'bold',
    paddingBottom: 20,
    textAlign: 'center'
  },
  hintText: {
    fontFamily: 'Roboto',
    textAlign: 'center',
    color: colors.bg_text_sec,
    fontWeight: '700',
    fontSize: 12,
    paddingBottom: 20
  },
  pinInput: {
    marginBottom: 20
  },
  nextStep: {
    marginTop: 20
  }
});
