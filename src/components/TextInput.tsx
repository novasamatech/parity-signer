// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Copyright 2021 Commonwealth Labs, Inc.
// This file is part of Layer Wallet.

// Layer Wallet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Layer Wallet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Layer Wallet. If not, see <http://www.gnu.org/licenses/>.

import React from 'react';
import {
	TextInput as TextInputOrigin,
	View,
	Text,
	TextStyle
} from 'react-native';

import { components, colors } from 'styles/index';

export default class TextInput extends React.PureComponent<
	{
		suffix?: string;
		autofocus?: boolean;
		label?: string;
		labelRight?: string;
		error?: boolean;
	},
	{}
> {
	// props
	static defaultProps = {
		autofocus: false
	};
	input: TextInputOrigin | null = null;

	// methods
	focus(): void {
		this.input?.focus();
	}

	componentDidUpdate(): void {
		const { autofocus } = this.props;
		autofocus && this.focus();
	}

	renderLabel(): React.ReactNode {
		const { label, labelRight } = this.props;
		if (!label && !labelRight) return;
		return (
			<View style={components.textInputLabel}>
				{typeof label === 'string' ? (
					<Text style={components.textInputLabelLeft}>{label}</Text>
				) : (
					label
				)}
				{typeof labelRight === 'string' ? (
					<Text style={components.textInputLabelRight}>{labelRight}</Text>
				) : (
					labelRight
				)}
			</View>
		);
	}

	render(): React.ReactElement {
		const { suffix, style, error } = this.props;
		const finalInputStyles: TextStyle[] = [components.textInputText];
		if (error) {
			finalInputStyles.push(components.textInputTextError);
		}

		return (
			<View style={components.textInput}>
				{this.renderLabel()}
				<View style={{ flexDirection: 'row' }}>
					<TextInputOrigin
						ref={(input: TextInputOrigin): any => (this.input = input)}
						autoCapitalize="none"
						keyboardAppearance="dark"
						underlineColorAndroid="transparent"
						{...this.props}
						style={[finalInputStyles, style]}
						placeholderTextColor={colors.text.faded}
						selectionColor={colors.text.cursor}
					/>
					{suffix && (
						<Text style={[finalInputStyles, components.textInputSuffix]}>
							{suffix}
						</Text>
					)}
				</View>
			</View>
		);
	}
}
