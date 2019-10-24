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

// @flow

import PropTypes from 'prop-types';
import React from 'react';
import {
	Platform,
	StyleSheet,
	Text,
	TouchableNativeFeedback,
	TouchableOpacity,
	View,
	ViewPropTypes
} from 'react-native';
import colors from '../colors';
import fonts from '../fonts';

export default class Button extends React.PureComponent<{
	title: string,
	onPress: () => any,
	textStyles?: ?StyleSheet.Styles,
	buttonStyles?: ?StyleSheet.Styles,
	disabled?: ?boolean
}> {
	static propTypes = {
		disabled: PropTypes.bool,
		onPress: PropTypes.func.isRequired,
		style: ViewPropTypes.style,
		textStyles: Text.propTypes.style,
		title: PropTypes.string.isRequired
	};

	render() {
		const { buttonStyles, disabled, onPress, textStyles, title } = this.props;

		const finalTextStyles = [styles.text, textStyles];
		const finalButtonStyles = [styles.button, buttonStyles];

		if (disabled) {
			finalTextStyles.push(styles.textDisabled);
			finalButtonStyles.push(styles.buttonDisabled);
		}

		const Touchable =
			Platform.OS === 'android' ? TouchableNativeFeedback : TouchableOpacity;
		return (
			<Touchable
				accessibilityComponentType="button"
				disabled={disabled}
				onPress={onPress}
			>
				<View style={finalButtonStyles}>
					<Text style={finalTextStyles} disabled={disabled}>
						{title}
					</Text>
				</View>
			</Touchable>
		);
	}
}

const styles = StyleSheet.create({
	button: {
		alignItems: 'center',
		backgroundColor: colors.bg_button,
		elevation: 4,
		height: 60,
		justifyContent: 'center'
	},
	buttonDisabled: {
		backgroundColor: '#dfdfdf',
		elevation: 0
	},
	text: {
		color: 'white',
		fontFamily: fonts.bold,
		fontSize: 20,
		padding: 8
	},
	textDisabled: {
		color: '#a1a1a1'
	}
});
