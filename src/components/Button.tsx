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

import React from 'react';
import {
	Platform,
	StyleSheet,
	Text,
	TextStyle,
	TouchableNativeFeedback,
	TouchableNativeFeedbackProps,
	TouchableOpacity,
	View,
	ViewStyle
} from 'react-native';

import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { ButtonListener } from 'types/props';

export default class Button extends React.PureComponent<{
	title: string;
	onPress: ButtonListener;
	textStyles?: TextStyle;
	buttonStyles?: ViewStyle;
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
			disabled,
			small,
			textStyles,
			onlyText,
			buttonStyles,
			testID,
			style
		} = this.props;

		const finalTextStyles = [textStyles];
		const finalButtonStyles = [styles.button, buttonStyles];

		if (small) {
			finalTextStyles.push(fontStyles.t_important);
			finalButtonStyles.push(styles.buttonSmall);
		}
		if (onlyText) {
			finalTextStyles.push({ color: colors.bg_button });
			finalButtonStyles.push(styles.buttonOnlyText);
		}
		if (disabled) {
			finalTextStyles.push(styles.textDisabled);
			finalButtonStyles.push(styles.buttonDisabled);
		}

		const Touchable: React.ComponentClass<TouchableNativeFeedbackProps> =
			Platform.OS === 'android' ? TouchableNativeFeedback : TouchableOpacity;
		return (
			<Touchable
				accessibilityComponentType="button"
				disabled={disabled}
				onPress={onPress}
				testID={testID}
			>
				<View style={[finalButtonStyles, style]}>
					<Text style={[fontStyles.h1, finalTextStyles]}>{title}</Text>
				</View>
			</Touchable>
		);
	}
}

const styles = StyleSheet.create({
	button: {
		alignItems: 'center',
		backgroundColor: colors.bg_button,
		borderRadius: 60,
		elevation: 4,
		height: 56,
		justifyContent: 'center',
		margin: 8,
		paddingHorizontal: 64
	},
	buttonDisabled: {
		backgroundColor: colors.card_bgSolid,
		elevation: 0
	},
	buttonOnlyText: {
		backgroundColor: colors.bg,
		elevation: 0,
		paddingHorizontal: 0
	},
	buttonSmall: {
		height: 48,
		paddingHorizontal: 48
	},
	textDisabled: {
		color: colors.card_bg
	}
});
