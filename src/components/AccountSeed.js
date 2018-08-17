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
import { Animated, Text, View, StyleSheet } from 'react-native';
import TouchableItem from './TouchableItem';
import TextInput from './TextInput';
import { WORDS, WORDS_INDEX } from '../util/account';
import colors from '../colors';

export default class AccountSeed extends Component {
  state = {
    cursorPosition: 0,
    keyboard: false,
    animation: false,
    suggestionsHeight: new Animated.Value(0) // Initial value for opacity: 0
  };

  constructor(...args) {
    super(...args);
    this.getSuggestions = this.getSuggestions.bind(this);
    this.handleCursorPosition = this.handleCursorPosition.bind(this);
    this.getWordPosition = this.getWordPosition.bind(this);
    this.keyboardDidShow = this.keyboardDidShow.bind(this);
    this.keyboardDidHide = this.keyboardDidHide.bind(this);
  }

  keyboardDidShow() {
    this.setState({ keyboard: true, animation: true });
    Animated.timing(
      // Animate over time
      this.state.suggestionsHeight, // The animated value to drive
      {
        toValue: 35, // Animate to opacity: 1 (opaque)
        duration: 200 // Make it take a while
      }
    ).start(() => this.setState({ animation: false }));
  }

  keyboardDidHide() {
    this.setState({ animation: true });
    Animated.timing(
      // Animate over time
      this.state.suggestionsHeight, // The animated value to drive
      {
        toValue: 0, // Animate to opacity: 1 (opaque)
        duration: 200 // Make it take a while
      }
    ).start(() => {
      this.setState({ keyboard: false, animation: false });
    });
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
    const SUGGESTIONS_COUNT = 5;
    const result = [];
    let yielded = 0;
    while (yielded < SUGGESTIONS_COUNT && WORDS[fromIndex] !== undefined) {
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
    const { value, valid } = this.props;

    const invalidStyles = !valid ? styles.invalidInput : {};
    const words = value.length ? value.split(' ') : [];
    const wordPosition = this.getWordPosition();
    let searchInput = this.getSearchInput();
    if (WORDS_INDEX[searchInput]) {
      searchInput = '';
    }
    const suggestions = this.getSuggestions(searchInput, words);
    return (
      <Animated.View
        style={[styles.suggestions, { height: this.state.suggestionsHeight }]}
      >
        {suggestions.map((suggestion, i) => {
          const sepStyle =
            !this.state.animation && i !== suggestions.length - 1
              ? { borderRightWidth: 0.3, borderColor: colors.card_bg_text_sec }
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
      </Animated.View>
    );
  }

  render() {
    const { keyboard } = this.state;
    const { valid } = this.props;
    const invalidStyles = !valid ? styles.invalidInput : {};
    return (
      <View>
        <TextInput
          style={[styles.input, invalidStyles]}
          multiline
          onBlur={this.keyboardDidHide}
          onSelectionChange={this.handleCursorPosition}
          {...this.props}
          onFocus={(...args) => {
            this.keyboardDidShow();
            return this.props.onFocus(...args);
          }}
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
    fontSize: 20,
    backgroundColor: '#e4fee4'
  },
  invalidInput: {
    backgroundColor: '#fee3e3'
  },
  suggestions: {
    backgroundColor: colors.card_bg,
    borderTopWidth: 0.3,
    borderColor: colors.card_bg_text_sec,
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
