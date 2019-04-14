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

export default class AccountSeed extends Component {
  state = {
    cursorPosition: 0,
    useBIP39WordList: true,
    useParityWordList: true,
  };

  handleCursorPosition = ({
    nativeEvent: {
      selection: { start, end }
    }
  }) => {
    if (start !== end) {
      return;
    }
    this.setState({ cursorPosition: start });
  }

  // Parse the imput field and return the position of the word
  // that is currently under the user's cursor
  getWordPosition = () => {
    const { value } = this.props;
    const { cursorPosition } = this.state;
    let wordPosition = 0;
    let i = 0;
    let char = '';
    let wasLetter = false;
    // if the input field is empty
    if (0 === value.length) {
      return 0;
    }
    while (true) {
      // get out if the i met the cursor or the i is at the end of the recovery phrase
      if (cursorPosition === i || (char = value.charAt(i)) === '') {
        break;
      }
      // if the character rigth after the i isn't a white space or new line
      if ([' ', '\n', '\r'].indexOf(char) === -1) {
        wasLetter = true;
      } else {
        // otherwise if we had a letter before, it means we reached the end of a word
        if (wasLetter) {
          wordPosition++;
        }
        wasLetter = false;
      }
      i++;
    }
    return wordPosition;
  }

  getSuggestions = (words, word_list) => {
    const input = words[words.length - 1]
    // the word list is sorted, get the index we should start searching
    // for the word
    let fromIndex = word_list.findIndex(w => w.startsWith(input));
    if (fromIndex === -1) {
      return [];
    }
    // if the whole word has already been typed in
    if (word_list[fromIndex] === input) {
      return [];
    }
    const SUGGESTIONS_COUNT = 5;
    const result = [];
    let yielded = 0;
    while (yielded < SUGGESTIONS_COUNT && fromIndex < word_list.length) {
      if (!word_list[fromIndex].startsWith(input)) {
        return result;
      }

      //do not suggest words that where already added
      if (words.indexOf(word_list[fromIndex]) === -1) {
        result.push(word_list[fromIndex]);
        yielded++
      }
      fromIndex++;
    }
    return result;
  }

  narrowWordList = (words) => {
    // if the last word is only present in BIP39 word list
    if (BIP39_WORDS.indexOf(words[words.length - 2]) !== -1 && PARITY_WORDS.indexOf(words[words.length - 2]) === -1) {
      this.setState({ useParityWordList: false })
      // if the second last word is only present in Parity word list
    } else if (BIP39_WORDS.indexOf(words[words.length - 2]) === -1 && PARITY_WORDS.indexOf(words[words.length - 2]) !== -1) {
      this.setState({ useBIP39WordList: false })
    }
  }

  generateSuggestions = (words) => {
    const { useBIP39WordList, useParityWordList } = this.state;

    // try to narrow down the word list using the last word typed, 
    // as soon as a second word is being typed
    words.length > 1 && useBIP39WordList && useParityWordList && this.narrowWordList(words);

    let suggestions = []
    useParityWordList && suggestions.push(...this.getSuggestions(words, PARITY_WORDS));
    useBIP39WordList && suggestions.push(...this.getSuggestions(words, BIP39_WORDS));

    //return a deduplicated sorted array if both word lists are still used
    return (useBIP39WordList && useParityWordList) ? [...new Set(suggestions.sort())] : suggestions;
  }

  renderSuggestions () {
    const { value } = this.props;
    // array of the words in the input field 
    const words = value.length ? value.toLowerCase().split(' ') : [];
    // at what word number the user's cursor is
    const wordPosition = this.getWordPosition();
    const suggestions = this.generateSuggestions(words);

    return (
      <View
        style={[styles.suggestions, { height: this.state.suggestionsHeight }]}
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

  render () {
    const { valid } = this.props;
    const invalidStyles = !valid ? styles.invalidInput : {};
    return (
      <View>
        <TextInput
          style={[styles.input, invalidStyles]}
          multiline
          autoCapitalize="none"
          onSelectionChange={this.handleCursorPosition}
          {...this.props}
        />
        {this.renderSuggestions()}
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
