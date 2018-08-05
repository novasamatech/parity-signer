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
import { Text, View, StyleSheet, Keyboard } from 'react-native';
import TouchableItem from './TouchableItem';
import TextInput from './TextInput';
import WORDS from '../../res/wordlist.json';
import colors from '../colors';

const WORDS_INDEX = WORDS.reduce(
  (res, w) => Object.assign(res, { [w]: 1 }),
  {}
);

export default class AccountSeed extends Component {
  state = {
    cursorPosition: 0,
    keyboard: false
  };

  constructor(...args) {
    super(...args);
    this.getSuggestions = this.getSuggestions.bind(this);
    this.handleCursorPosition = this.handleCursorPosition.bind(this);
    this.getWordPosition = this.getWordPosition.bind(this);
    this.keyboardDidShow = this.keyboardDidShow.bind(this);
    this.keyboardDidHide = this.keyboardDidHide.bind(this);
  }

  componentDidMount() {
    this.keyboardDidShowListener = Keyboard.addListener(
      'keyboardDidShow',
      this.keyboardDidShow
    );
    this.keyboardDidHideListener = Keyboard.addListener(
      'keyboardDidHide',
      this.keyboardDidHide
    );
  }

  componentWillUnmount() {
    this.keyboardDidShowListener.remove();
    this.keyboardDidHideListener.remove();
  }

  keyboardDidShow() {
    this.setState({ keyboard: true });
  }

  keyboardDidHide() {
    this.setState({ keyboard: false });
  }

  handleCursorPosition({
    nativeEvent: {
      selection: { start, end }
    }
  }) {
    if (start !== end) {
      return;
    }
    this.setState({ cursorPosition: start });
  }

  getSearchInput() {
    const { value } = this.props;
    const { cursorPosition } = this.state;
    let startFrom = cursorPosition;
    // find the first space or start of string
    while (startFrom > 0 && [' '].indexOf(value.charAt(startFrom - 1)) === -1) {
      --startFrom;
    }
    return value.substring(startFrom, cursorPosition);
  }

  getWordPosition() {
    const { value } = this.props;
    const { cursorPosition } = this.state;
    let wordPosition = 0;
    let cursor = 0;
    let char = '';
    let wasLetter = false;
    if (0 === value.length) {
      return 0;
    }
    while (true) {
      if (cursorPosition === cursor || (char = value.charAt(cursor)) === '') {
        break;
      }
      if ([' ', '\n', '\r'].indexOf(char) === -1) {
        wasLetter = true;
      } else {
        if (wasLetter) {
          ++wordPosition;
        }
        wasLetter = false;
      }
      ++cursor;
    }
    return wordPosition;
  }

  getSuggestions(input, words) {
    let fromIndex = WORDS.findIndex(w => w.startsWith(input));
    if (fromIndex === -1) {
      return [];
    }
    if (WORDS[fromIndex] === input) {
      fromIndex = 0;
    }
    const SUGGETIONS_COUNT = 5;
    const result = [];
    let yielded = 0;
    while (yielded < SUGGETIONS_COUNT && WORDS[fromIndex] !== undefined) {
      ++fromIndex;
      if (words.indexOf(WORDS[fromIndex]) !== -1) {
        continue;
      }
      result.push(WORDS[fromIndex]);
      ++yielded;
    }
    return result;
  }

  renderSuggestions() {
    const { value } = this.props;
    const words = value.length ? value.split(' ') : [];
    const wordPosition = this.getWordPosition();
    let searchInput = this.getSearchInput();
    if (WORDS_INDEX[searchInput]) {
      searchInput = '';
    }
    const suggestions = this.getSuggestions(searchInput, words);
    return (
      <View style={styles.suggestions}>
        {/* <View style={styles.suggestion}>suggestion</View> */}
        {suggestions.map((suggestion, i) => {
          const sepStyle =
            i !== suggestions.length - 1
              ? { borderRightWidth: 1, borderColor: colors.card_bg }
              : {};
          return (
            <TouchableItem
              onPress={e => {
                words[wordPosition] = suggestion;
                this.props.onChangeText(words.join(' '));
              }}
            >
              <View key={suggestion} style={[styles.suggestion, sepStyle]}>
                <Text style={styles.suggestionText}>{suggestion}</Text>
              </View>
            </TouchableItem>
          );
        })}
      </View>
    );
  }

  render() {
    const { keyboard } = this.state;
    return (
      <View>
        <TextInput
          style={styles.input}
          editable={true}
          multiline={true}
          onSelectionChange={this.handleCursorPosition}
          {...this.props}
        />
        {keyboard && this.renderSuggestions()}
      </View>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    flexDirection: 'column'
  },
  input: {
    height: 120,
    lineHeight: 26,
    fontSize: 20
  },
  suggestions: {
    backgroundColor: '#E5E5E5',
    paddingHorizontal: 5,
    height: 35,
    flexDirection: 'row',
    alignItems: 'center'
  },
  suggestion: {
    paddingVertical: 9,
    padding: 10
  },
  suggestionText: {
    fontFamily: 'Roboto',
    color: colors.card_bg_text
  }
});
