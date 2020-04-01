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

import React, { Component, ReactElement } from 'react';
import {
	NativeSyntheticEvent,
	StyleSheet,
	Text,
	TextInputProps,
	TextInputSelectionChangeEventData,
	View
} from 'react-native';

import TextInput from './TextInput';
import TouchableItem from './TouchableItem';

import colors from 'styles/colors';
import fonts from 'styles/fonts';
import fontStyles from 'styles/fontStyles';
import PARITY_WORDS from 'res/parity_wordlist.json';
import BIP39_WORDS from 'res/bip39_wordlist.json';
import { binarySearch } from 'utils/array';

// Combined, de-duplicated, sorted word list (could be a precompute from json as well)
const ALL_WORDS = Array.from(new Set(PARITY_WORDS.concat(BIP39_WORDS))).sort();
const SUGGESTIONS_COUNT = 5;

interface Props extends TextInputProps {
	onChangeText: (text: string) => void;
	valid: boolean;
	value: string;
}

export default class AccountSeed extends Component<
	Props,
	{
		cursorPosition: number;
	}
> {
	state = {
		cursorPosition: 0
	};

	handleCursorPosition = (
		event: NativeSyntheticEvent<TextInputSelectionChangeEventData>
	): void => {
		const { start, end } = event.nativeEvent.selection;

		if (start !== end) {
			return;
		}
		this.setState({ cursorPosition: start });
	};

	/**
	 * Generate a list of suggestions for input
	 */
	generateSuggestions(input: string, wordList: string[]): string[] {
		const fromIndex = binarySearch(wordList, input).index; // index to start search from

		let suggestions = wordList.slice(fromIndex, fromIndex + SUGGESTIONS_COUNT);

		const lastValidIndex = suggestions.findIndex(
			word => !word.startsWith(input)
		);

		if (lastValidIndex !== -1) {
			suggestions = suggestions.slice(0, lastValidIndex);
		}

		return suggestions;
	}

	selectWordList(otherWords: string[]): string[] {
		for (const word of otherWords) {
			const isBIP39 = binarySearch(BIP39_WORDS, word).hit;
			const isParity = binarySearch(PARITY_WORDS, word).hit;

			if (!isBIP39 && isParity) {
				return PARITY_WORDS;
			} else if (isBIP39 && !isParity) {
				return BIP39_WORDS;
			}
		}

		return ALL_WORDS as string[];
	}

	renderSuggestions(): ReactElement {
		const { value } = this.props;
		const { cursorPosition } = this.state;

		let left = value.substring(0, cursorPosition).split(' ');
		let right = value.substring(cursorPosition).split(' ');

		// combine last nibble before cursor and first nibble after cursor into a word
		const input = left[left.length - 1] + right[0];

		left = left.slice(0, -1);
		right = right.slice(1);

		// find a wordList using words around as discriminator
		const wordList = this.selectWordList(left.concat(right));
		const suggestions = this.generateSuggestions(input.toLowerCase(), wordList);

		return (
			<View style={styles.suggestions}>
				{suggestions.map((suggestion, i) => {
					const sepStyle =
						i !== suggestions.length - 1
							? { borderColor: colors.card_bg_text_sec, borderRightWidth: 0.3 }
							: {};
					return (
						<TouchableItem
							key={i}
							onPress={(): void => {
								let phrase = left.concat(suggestion, right).join(' ').trimEnd();
								const is24words = phrase.split(' ').length === 24;
								if (!is24words) phrase += ' ';
								this.props.onChangeText(phrase);
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

	render(): ReactElement {
		const { valid, value } = this.props;
		const invalidStyles = !valid ? styles.invalidInput : {};
		return (
			<View>
				<TextInput
					style={StyleSheet.flatten([
						fontStyles.t_seed,
						styles.input,
						invalidStyles
					])}
					multiline
					autoCorrect={false}
					autoCompleteType="off"
					autoCapitalize="none"
					returnKeyType="done"
					blurOnSubmit={true}
					textAlignVertical="top"
					onSelectionChange={this.handleCursorPosition}
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
		borderBottomColor: colors.card_bg
	},
	invalidInput: {
		borderBottomColor: colors.bg_alert,
		borderColor: colors.bg_alert
	},
	suggestion: {
		padding: 14,
		paddingVertical: 4
	},
	suggestionText: {
		color: colors.bg_button,
		fontFamily: fonts.regular
	},
	suggestions: {
		alignItems: 'center',
		backgroundColor: colors.card_bg,
		borderColor: colors.card_bg,
		borderWidth: 0.5,
		flexDirection: 'row',
		height: 40,
		marginHorizontal: 16,
		marginTop: -8,
		overflow: 'hidden'
	}
});
