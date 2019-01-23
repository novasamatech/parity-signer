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
import { ScrollView, StyleSheet, View } from 'react-native';
import privacyPolicy from '../../docs/privacy-policy.md';
import colors from '../colors';
import Markdown from '../components/Markdown';

export default class PrivacyPolicy extends React.PureComponent {
  static navigationOptions = {
    title: 'Privacy policy',
    headerBackTitle: 'Back'
  };

  render() {
    const { navigation } = this.props;
    const isWelcome = navigation.getParam('isWelcome');
    return (
      <View style={styles.body}>
        <ScrollView contentContainerStyle={{}}>
          <Markdown>{privacyPolicy}</Markdown>
        </ScrollView>
      </View>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    flex: 1,
    flexDirection: 'column',
    overflow: 'hidden',
    backgroundColor: colors.bg,
    padding: 20
  },
  top: {
    flex: 1
  },
  bottom: {
    flexBasis: 50,
    paddingBottom: 15
  },
  titleTop: {
    color: colors.bg_text_sec,
    fontSize: 24,
    fontFamily: 'Manifold CF',
    fontWeight: 'bold',
    paddingBottom: 20,
    textAlign: 'center'
  },
  title: {
    fontFamily: 'Manifold CF',
    color: colors.bg_text_sec,
    fontSize: 18,
    fontWeight: 'bold',
    paddingBottom: 20
  },
  text: {
    marginTop: 10,
    fontFamily: 'Roboto',
    fontSize: 14,
    color: colors.card_bg
  }
});
