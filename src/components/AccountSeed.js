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
import { StyleSheet, Text, View } from 'react-native';
import colors from '../colors';
import PARITY_WORDS from '../../res/parity_wordlist.json';
import BIP39_WORDS from '../../res/bip39_wordlist.json';;
import TextInput from './TextInput';
import TouchableItem from './TouchableItem';
import { binarySearch } from '../util/array';

// Combined, de-duplicated, sorted word list (could be a precompute from json as well)
const ALL_WORDS = Array.from(new Set(PARITY_WORDS.concat(BIP39_WORDS))).sort();
const SUGGESTIONS_COUNT = 5;

export default class AccountSeed extends Component {
  generateSuggestions (inputWordArray) {
    if (inputWordArray.length < 1) {
      return [];
    }

    const input = inputWordArray[inputWordArray.length - 1]; // last word
    const wordList = this.selectWordList(inputWordArray);
    const fromIndex = binarySearch(wordList, input).index; // index to start search from

    let suggestions = wordList.slice(fromIndex, fromIndex + SUGGESTIONS_COUNT);

    const lastValidIndex = suggestions.findIndex((word) => !word.startsWith(input));

    if (lastValidIndex !== -1) {
      suggestions = suggestions.slice(0, lastValidIndex);
    }

    return suggestions;
  }

  selectWordList (inputWordArray) {
    const previousWords = inputWordArray.slice(0, -1); // input without last word

    let words = [];

    for (const word of previousWords) {
      const isBIP39 = binarySearch(BIP39_WORDS, word).hit;
      const isParity = binarySearch(PARITY_WORDS, word).hit;

      if (!isBIP39 && isParity) {
        return PARITY_WORDS;
      } else if (isBIP39 && !isParity) {
        return BIP39_WORDS;
      }
    }

    return ALL_WORDS;
  }

  renderSuggestions () {
    const { value } = this.props;
    // array of the words in the input field
    const inputWordArray = value.length ? value.split(' ') : [];
    const suggestions = this.generateSuggestions(inputWordArray);

    return (
      <View
        style={[styles.suggestions]}
      >
        {suggestions.map((suggestion, i) => {
          const sepStyle =
            i !== suggestions.length - 1
              ? { borderRightWidth: 0.3, borderColor: colors.card_bg_text_sec }
              : {};
          return (
            <TouchableItem
              key={i}
              onPress={e => {
                inputWordArray[inputWordArray.length-1] = suggestion;
                this.props.onChangeText(inputWordArray.join(' '));
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

  render () {
    const { valid, value } = this.props;
    const invalidStyles = !valid ? styles.invalidInput : {};
    return (
      <View>
        <TextInput
          style={[styles.input, invalidStyles]}
          multiline
          autoCapitalize="none"
          {...this.props}
        />
        {value.length > 0 && this.renderSuggestions()}
      </View>
    );
  }
}

const styles = StyleSheet.create({
  body: {
    flexDirection: 'column'
  },
  input: {
    height: 160,
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
    alignItems: 'center',

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
