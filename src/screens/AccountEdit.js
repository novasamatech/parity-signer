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
import { ScrollView, StyleSheet, Text } from 'react-native';
import { Subscribe } from 'unstated';
import colors from '../colors';
import fonts from "../fonts";
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
            <ScrollView
              style={styles.body}
              contentContainerStyle={styles.bodyContainer}
            >
              <Text style={styles.titleTop}>EDIT ACCOUNT</Text>
              <AccountCard
                title={selected.name}
                address={selected.address}
                networkKey={selected.networkKey}
              />
              <Text style={styles.title}>ACCOUNT NAME</Text>
              <TextInput
                style={{ marginBottom: 40 }}
                onChangeText={async (name) => {
                  accounts.updateSelected({ name });
                  await accounts.save(accounts.getSelected())
                }}
                value={selected.name}
                placeholder="New name"
              />
            </ScrollView>
          );
        }}
      </Subscribe>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    backgroundColor: colors.bg,
    flex: 1,
    flexDirection: 'column',
    overflow: 'hidden'
  },
  bodyContainer: {
    flex: 1,
    padding: 20
  },
  top: {
    flex: 1
  },
  bottom: {
    flexBasis: 50,
    paddingBottom: 15
  },
  title: {
    fontFamily: fonts.bold,
    color: colors.bg_text_sec,
    fontSize: 18,
    paddingBottom: 20
  },
  titleTop: {
    color: colors.bg_text_sec,
    fontSize: 24,
    fontFamily: fonts.bold,
    paddingBottom: 20,
    textAlign: 'center'
  },
  hintText: {
    fontFamily: fonts.bold,
    textAlign: 'center',
    paddingTop: 20,
    color: colors.bg_text_sec,
    fontSize: 12
  },
  deleteButton: {
    backgroundColor: 'red'
  }
});
