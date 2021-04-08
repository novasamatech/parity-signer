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
	Platform,
	Text,
	TouchableNativeFeedback,
	TouchableOpacity,
	ViewStyle,
	View
} from 'react-native';

import { components, colors } from 'styles/index';
import { ButtonListener } from 'types/props';

export default class Button extends React.PureComponent<{
	title: string;
	onPress: ButtonListener;
	disabled?: boolean;
	inactive?: boolean;
	fluid?: boolean | string;
	negative?: boolean;
	secondary?: boolean;
	active?: boolean;
	testID?: string;
	style?: ViewStyle;
}> {
	render(): React.ReactElement {
		const {
			onPress,
			title,
			disabled,
			inactive,
			fluid,
			negative,
			secondary,
			active,
			testID,
			style
		} = this.props;

		const finalTextStyles = [components.buttonText, {}];
		const finalButtonStyles = [components.button, {}];

		if (disabled) {
			finalButtonStyles.push(components.buttonDisabled);
		}
		if (inactive) {
			finalButtonStyles.push(components.buttonDisabled);
			finalTextStyles.push({ color: colors.text.light });
		}
		if (fluid) {
			finalButtonStyles.push(
				fluid === 'left'
					? components.buttonFluidLeft
					: fluid === 'right'
					? components.buttonFluidRight
					: components.buttonFluid
			);
			finalTextStyles.push({ textAlign: 'center' });
		}
		if (negative) {
			finalButtonStyles.push({ backgroundColor: colors.background.negative });
		}
		if (secondary) {
			finalButtonStyles.push({
				backgroundColor: colors.background.accentMedium
			});
			finalTextStyles.push({ color: colors.text.dark });
		}
		if (active) {
			finalTextStyles.push({ fontWeight: 'bold' });
		}

		return Platform.OS === 'android' ? (
			<TouchableNativeFeedback
				accessibilityComponentType="button"
				disabled={disabled}
				onPress={onPress}
				testID={testID}
			>
				<View style={[finalButtonStyles, style]}>
					<Text style={[finalTextStyles]}>{title}</Text>
				</View>
			</TouchableNativeFeedback>
		) : (
			<TouchableOpacity
				accessibilityComponentType="button"
				disabled={disabled}
				onPress={onPress}
				testID={testID}
				style={[finalButtonStyles, style]}
			>
				<Text style={[finalTextStyles]}>{title}</Text>
			</TouchableOpacity>
		);
	}
}
