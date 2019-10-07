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

import React from 'react';
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import { Subscribe } from 'unstated';
import styles from '../styles';
import AccountCard from '../components/AccountCard';
import TextInput from '../components/TextInput';
import AccountsStore from '../stores/AccountsStore';


export default class AccountEdit extends React.PureComponent {
  static navigationOptions = {
    title: 'Edit Account'
  };

  constructor(props) {
    super(props);
  }

  render() {
    return (
      <Subscribe to={[AccountsStore]}>
        {accounts => {
          const selected = accounts.getSelected();

          if (!selected) {
            return null;
          }

          return (
            <ScrollView style={styles.b_flex}>
              <View style={styles.b_paddingH}>
                <Text style={[styles.t_h1, styles.header]}>Edit Account</Text>
              </View>
              <AccountCard
                title={selected.name}
                address={selected.address}
                networkKey={selected.networkKey}
              />

              <View style={[styles.b_paddingH, {marginTop:16}]}>
                <Text style={styles.t_text}>Account Name</Text>
                <TextInput
                  style={[styles.b_textInput, styles.t_h2]}
                  onChangeText={async (name) => {
                    accounts.updateSelectedAccount({ name });
                    await accounts.save(accounts.getSelectedKey(), accounts.getSelected())
                  }}
                  value={selected.name}
                  placeholder="New name"
                />
              </View>
            </ScrollView>
          );
        }}
      </Subscribe>
    );
  }
}
