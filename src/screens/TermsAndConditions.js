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
import Icon from 'react-native-vector-icons/MaterialCommunityIcons';
import toc from '../../docs/terms-and-conditions.md';
import colors from '../colors';
import fonts from "../fonts";
import Button from '../components/Button';
import Markdown from '../components/Markdown';
import TouchableItem from '../components/TouchableItem';
import { saveToCAndPPConfirmation } from '../util/db';

export default class TermsAndConditions extends React.PureComponent {
  static navigationOptions = {
    title: 'Terms and conditions',
    headerBackTitle: 'Back'
  };

  state = {
    tocAgreement: false,
    ppAgreement: false
  };

  render() {
    const { navigation } = this.props;
    const { tocAgreement, ppAgreement } = this.state;
    return (
      <View style={styles.body}>
        <ScrollView contentContainerStyle={{}}>
          <Markdown>{toc}</Markdown>
        </ScrollView>

        <TouchableItem
          style={{
            flexDirection: 'row',
            alignItems: 'center'
          }}
          onPress={() => {
            this.setState({ tocAgreement: !tocAgreement });
          }}
        >
          <Icon
            name={tocAgreement ? 'checkbox-marked' : 'checkbox-blank-outline'}
            style={[styles.text, { fontSize: 30 }]}
          />

          <Text style={[styles.text, { fontSize: 16 }]}>
            {'  I agree to the terms and conditions'}
          </Text>
        </TouchableItem>
        <TouchableItem
          style={{
            flexDirection: 'row',
            alignItems: 'center'
          }}
          onPress={() => {
            this.setState({ ppAgreement: !ppAgreement });
          }}
        >
          <Icon
            name={ppAgreement ? 'checkbox-marked' : 'checkbox-blank-outline'}
            style={[styles.text, { fontSize: 30 }]}
          />

          <Text style={[styles.text, { fontSize: 16 }]}>
            <Text>{'  I agree to the '}</Text>
            <Text
              style={{ textDecorationLine: 'underline' }}
              onPress={() => {
                navigation.navigate('PrivacyPolicy');
              }}
            >
              privacy policy
            </Text>
          </Text>
        </TouchableItem>

        <Button
          buttonStyles={{ marginTop: 10, height: 60 }}
          title="Next"
          disabled={!ppAgreement || !tocAgreement}
          onPress={async () => {
            const firstScreenActions = navigation.getParam(
              'firstScreenActions'
            );
            await saveToCAndPPConfirmation();
            navigation.dispatch(firstScreenActions);
          }}
        />
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
    fontFamily: fonts.bold,
    paddingBottom: 20,
    textAlign: 'center'
  },
  title: {
    fontFamily: fonts.bold,
    color: colors.bg_text_sec,
    fontSize: 18,
    paddingBottom: 20
  },
  text: {
    marginTop: 10,
    fontFamily: fonts.regular,
    fontSize: 14,
    color: colors.card_bg
  }
});
