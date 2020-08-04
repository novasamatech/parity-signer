// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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
	TouchableNativeFeedback,
	TouchableNativeFeedbackProps,
	TouchableOpacity,
	View,
	ViewStyle
} from 'react-native';

import colors from 'styles/colors';
import fonts from 'styles/fonts';

export default function NetworkCard(props: {
	title: string;
	secondaryText?: string;
	labelText?: string;
	footerStyle?: ViewStyle;
	onPress: () => any;
}) {
	const { title, secondaryText, labelText, footerStyle, onPress } = props;

	const finalBodyStyle = [styles.body, footerStyle];
	const finalContentStyle = [styles.content];
	const finalFooterStyle = [styles.footer, footerStyle];
	const finalTitleTextStyle = [styles.titleText];
	const finalSecondaryTextStyle = [styles.secondaryText];
	const finalFooterTextStyle = [styles.footerText];

	const Touchable: React.ComponentClass<TouchableNativeFeedbackProps> =
		Platform.OS === 'android' ? TouchableNativeFeedback : TouchableOpacity;
	return (
		<Touchable
			accessibilityComponentType="button"
			disabled={false}
			onPress={onPress}
		>
			<View style={finalBodyStyle}>
				<View style={finalContentStyle}>
					<View>
						<Text style={finalTitleTextStyle}>{title}</Text>
						<Text style={finalSecondaryTextStyle}>{secondaryText}</Text>
					</View>
				</View>
				<View style={finalFooterStyle}>
					<Text style={finalFooterTextStyle}>{labelText}</Text>
				</View>
			</View>
		</Touchable>
	);
}

const styles = StyleSheet.create({
	body: {},
	content: {
		backgroundColor: colors.background.app,
		padding: 30
	},
	footer: {},
	footerText: {},
	image: {
		height: 80,
		width: 80
	},
	secondaryText: {
		fontFamily: fonts.regular
	},
	titleText: {
		fontFamily: fonts.bold
	}
});
