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
import {
  Alert,
  ScrollView,
  View,
  Text,
  TouchableOpacity,
  Share,
  StyleSheet,
  KeyboardAvoidingView,
  findNodeHandle
} from 'react-native';
import { KeyboardAwareScrollView } from 'react-native-keyboard-aware-scroll-view';
import { Subscribe } from 'unstated';
import Icon from 'react-native-vector-icons/MaterialIcons';
import { brainWalletAddress } from '../util/native';
import AccountsStore from '../stores/AccountsStore';
import AccountSeed from '../components/AccountSeed';
import AccountCard from '../components/AccountCard';
import AccountIconChooser from '../components/AccountIconChooser';
import TextInput from '../components/TextInput';
import Button from '../components/Button';
import colors from '../colors';

export default class AccountRecover extends Component {
  static navigationOptions = {
    title: 'Recover Account',
    headerBackTitle: 'Back'
  };
  render() {
    return (
      <Subscribe to={[AccountsStore]}>
        {accounts => <AccountRecoverView {...this.props} accounts={accounts} />}
      </Subscribe>
    );
  }
}

class AccountRecoverView extends Component {
  render() {
    const { accounts } = this.props;
    const selected = accounts.getNew();
    return (
      <View style={styles.body}>
        <KeyboardAwareScrollView
          style={{ padding: 20 }}
          containerStyle={styles.bodyContainer}
          enableOnAndroid
          scrollEnabled={true}
          extraHeight={120}
          innerRef={ref => {
            this.scroll = ref;
          }}
        >
          <Text style={styles.titleTop}>RECOVER ACCOUNT</Text>
          <AccountCard
            address={selected.address || ''}
            title={selected.name || 'no name'}
          />
          <Text style={styles.title}>ACCOUNT NAME</Text>
          <TextInput
            onChangeText={name => accounts.updateNew({ name })}
            value={selected && selected.name}
            placeholder="Enter an account name"
          />

          <Text style={[styles.title, { marginTop: 20 }]}>
            ENTER RECOVERY WORDS
          </Text>
          <TextInput
            onFocus={e => {
              this.scroll.props.scrollToFocusedInput(findNodeHandle(e.target));
            }}
            onChangeText={async seed =>
              accounts.updateNew({
                seed,
                address: await brainWalletAddress(seed)
              })
            }
            style={{ height: 140, lineHeight: 30 }}
            editable={true}
            value={selected.seed}
            multiline={true}
          />
          <Button
            buttonStyles={{ marginTop: 20, marginBottom: 40 }}
            title="Next Step"
            onPress={() => {
              this.props.navigation.navigate('AccountPin', {
                isWelcome: this.props.navigation.getParam('isWelcome')
              });
            }}
          />
        </KeyboardAwareScrollView>
      </View>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    backgroundColor: colors.bg,
    flex: 1
  },
  bodyContainer: {
    paddingBottom: 20
  },
  titleTop: {
    fontFamily: 'Roboto',
    color: colors.bg_text_sec,
    fontSize: 24,
    fontWeight: 'bold',
    paddingBottom: 20,
    textAlign: 'center'
  },
  title: {
    fontFamily: 'Roboto',
    color: colors.bg_text_sec,
    fontSize: 18,
    fontWeight: 'bold',
    paddingBottom: 20
  }
});
