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
  StyleSheet
} from 'react-native';
import { Subscribe } from 'unstated';
import AccountsStore from '../stores/AccountsStore';
import Background from '../components/Background';
import AccountSeed from '../components/AccountSeed';
import AccountCard from '../components/AccountCard';
import AccountIconChooser from '../components/AccountIconChooser';
import TextInput from '../components/TextInput';
import Button from '../components/Button';
import colors from '../colors';

export default class AccountBackup extends Component {
  static navigationOptions = {
    title: 'Account Backup'
  };
  render() {
    return (
      <Subscribe to={[AccountsStore]}>
        {accounts => <AccountBackupView {...this.props} accounts={accounts} />}
      </Subscribe>
    );
  }
}

class AccountBackupView extends Component {
  render() {
    const { accounts, navigation } = this.props;
    const selected = navigation.getParam('isNew')
      ? accounts.getNew()
      : accounts.getSelected();
    return (
      <View style={styles.body}>
        <Background />
        <Text style={styles.titleTop}>BACKUP ACCOUNT</Text>
        <AccountCard
          address={selected.address}
          title={selected.name || 'no name'}
        />
        <Text style={styles.titleTop}>RECOVERY WORDS</Text>
        <Text style={styles.hintText}>
          Write these words down on paper. Keep it safe. These words allow
          anyone to recover this account.
        </Text>
        <TextInput
          style={{ height: 140, lineHeight: 30 }}
          editable={false}
          value={selected.seed}
          multiline={true}
        />
        <Button
          buttonStyles={styles.nextStep}
          title="Done"
          onPress={() => {
            if (navigation.getParam('isNew')) {
              this.props.navigation.navigate('AccountPin', {
                isWelcome: navigation.getParam('isWelcome')
              });
            } else {
              navigation.navigate('AccountList');
            }
          }}
        />
      </View>
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
    paddingBottom: 20
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
  nextStep: {
    marginTop: 20
  }
});
