// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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

import React from 'react';
import {
	Platform,
	StyleSheet,
	Text,
	TextStyle,
	TouchableNativeFeedback,
	TouchableOpacity,
	ViewStyle,
	View
} from 'react-native';

import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { ButtonListener } from 'types/props';

export default class Button extends React.PureComponent<{
	title: string;
	onPress: ButtonListener;
	textStyles?: TextStyle;
	aboveKeyboard?: boolean;
	disabled?: boolean;
	small?: boolean;
	onlyText?: boolean;
	testID?: string;
	style?: ViewStyle;
}> {
	render(): React.ReactElement {
		const {
			onPress,
			title,
			aboveKeyboard,
			disabled,
			small,
			textStyles,
			onlyText,
			testID,
			style
		} = this.props;

		const finalTextStyles = [styles.buttonText, {}];
		const finalButtonStyles = [styles.button, {}];

		if (small) {
			finalTextStyles.push({ fontSize: 14 });
			finalButtonStyles.push(styles.buttonSmall);
		}
		if (onlyText) {
			finalTextStyles.push({ color: colors.text.main });
			finalButtonStyles.push(styles.buttonOnlyText);
		}
		if (disabled) {
			finalButtonStyles.push(styles.buttonDisabled);
		}
		if (aboveKeyboard) {
			finalButtonStyles.push(styles.buttonAboveKeyboard);
		}

		return Platform.OS === 'android' ? (
			<TouchableNativeFeedback
				accessibilityComponentType="button"
				disabled={disabled}
				onPress={onPress}
				testID={testID}
			>
				<View style={[finalButtonStyles, style]}>
					<Text
						style={[
							fontStyles.h1,
							styles.buttonText,
							finalTextStyles,
							textStyles
						]}
					>
						{title}
					</Text>
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
				<Text
					style={[
						fontStyles.h1,
						styles.buttonText,
						finalTextStyles,
						textStyles
					]}
				>
					{title}
				</Text>
			</TouchableOpacity>
		);
	}
}

const styles = StyleSheet.create({
	button: {
		alignSelf: 'center',
		backgroundColor: colors.text.main,
		borderRadius: 60,
		height: 48,
		justifyContent: 'center',
		marginVertical: 40,
		paddingHorizontal: 40
	},
	buttonAboveKeyboard: {
		bottom: 56,
		position: 'absolute'
	},
	buttonDisabled: {
		backgroundColor: colors.background.card
	},
	buttonOnlyText: {
		backgroundColor: colors.background.app,
		elevation: 8
	},
	buttonSmall: {
		height: 40,
		marginVertical: 8,
		paddingHorizontal: 32
	},
	buttonText: {
		...fontStyles.a_button
	}
});
