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
import PropTypes from 'prop-types';
import {
  View,
  Text,
  StyleSheet,
} from 'react-native';
import debounce from 'debounce';
import { StackActions, NavigationActions } from 'react-navigation';
import { Subscribe } from 'unstated';
import AccountsStore from '../stores/AccountsStore';
import ScannerStore from '../stores/ScannerStore';
import Background from '../components/Background';
import TextInput from '../components/TextInput';
import colors from '../colors';

export class AccountUnlockAndSign extends React.PureComponent {
  render() {
    return (
      <Subscribe to={[AccountsStore, ScannerStore]}>
        {(accounts, scannerStore) => (
          <AccountUnlockView
            {...this.props}
            accounts={accounts}
            nextButtonTitle="Sign"
            onChange={async (o) => {
              try {
                const txRequest = scannerStore.getTXRequest();
                let res = await scannerStore.signData(o.pin);
                const resetAction = StackActions.reset({
                  index: 2,
                  actions: [
                    NavigationActions.navigate({ routeName: 'QrScanner' }),
                    NavigationActions.navigate({ routeName: 'TxDetails' }),
                    NavigationActions.navigate({ routeName: 'SignedTx' })
                  ]
                });
                this.props.navigation.dispatch(resetAction);
              } catch (e) { }
            }}
          />
        )}
      </Subscribe>
    );
  }
}

export class AccountUnlock extends React.Component {
  state = {
    hasWrongPin: false
  };

  render() {
    console.log("AccountUnlock")
    return (
      <Subscribe to={[AccountsStore]}>
        {accounts => (
          <AccountUnlockView
            {...this.props}
            onChange={async (o) => {
              if (await accounts.unlockAccount(accounts.getSelected(), o.pin)) {
                const resetAction = StackActions.reset({
                  index: 3,
                  actions: [
                    NavigationActions.navigate({ routeName: 'AccountList' }),
                    NavigationActions.navigate({ routeName: 'AccountDetails' }),
                    NavigationActions.navigate({ routeName: 'AccountEdit' }),
                    NavigationActions.navigate({ routeName: 'AccountBackup' })
                  ]
                });
                this.props.navigation.dispatch(resetAction);
                console.log('verifyPinAndNavigate 1')
              } else if (o.submitted != undefined && o.submitted) {
                this.setState({ hasWrongPin: true });
                console.log('Submitted a wrong pin')
              }
            }}
            hasWrongPin={this.state.hasWrongPin}
            accounts={accounts}
          />
        )}
      </Subscribe>
    );
  }
}

// This is not used anywhere
/*
export class AccountUnlockAndChangePin extends React.PureComponent {
  render() {
    return (
      <Subscribe to={[AccountsStore]}>
        {accounts => (
          <AccountUnlockView
            {...this.props}
            onChange={async pin => {
              try {
                if (await accounts.unlockAccount(accounts.getSelected(), pin)) {
                  this.props.navigation.navigate('AccountPin', {
                    isChange: true
                  });
                }
              } catch (e) { }
            }}
            accounts={accounts}
          />
        )}
      </Subscribe>
    );
  }
}
*/

class AccountUnlockView extends React.PureComponent {
  state = {
    pin: '',
  };

  static propTypes = {
    onChange: PropTypes.func.isRequired,
    nextButtonTitle: PropTypes.string,
    hasWrongPin: PropTypes.bool
  };

  verifyAndSubmitPin = (submitted) => {
    console.log('verifyAndSubmitPin, pin: ' + this.state.pin + ', submitted: ' + submitted)
    debounce(this.props.onChange, 200)({
      pin: this.state.pin,
      submitted: submitted
    });
  }

  render() {
    console.log("hasWrongPin: " + this.props.hasWrongPin)
    return (
      <View style={styles.body}>
        <Background />
        <Text style={styles.titleTop}>UNLOCK ACCOUNT</Text>
        <Text style={styles.errorText}>{this.props.hasWrongPin == undefined || !this.props.hasWrongPin ? null : 'Wrong pin, please try again'}</Text>
        <Text style={styles.title}>PIN</Text>
        <PinInput
          onChangeText={(pin) => {
            const submitted = false
            this.setState({ pin });
            console.log("TextChange: " + this.state.pin + " Submitted: " + submitted)
            if (pin.length < 1) {
              return;
            }
            this.verifyAndSubmitPin(submitted);
          }}
          onSubmitEditing={() => {
            const submitted = true
            console.log("Text: " + this.state.pin + " submittedChanged : " + submitted)
            this.verifyAndSubmitPin(submitted)
          }}
          value={this.state.pin}
        />
      </View>
    );
  }
}

function PinInput(props) {
  return (
    <TextInput
      autoFocus
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
      {...props}
    />
  );
}

const styles = StyleSheet.create({
  body: {
    backgroundColor: colors.bg,
    padding: 20,
    flex: 1,
    overflow: 'hidden'
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
  },
  nextStep: {
    marginTop: 20
  }
});
