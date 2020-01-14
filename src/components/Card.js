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

export default class Card extends React.PureComponent<{
	title: string,
	secondaryText?: ?string,
	labelText?: ?string,
	footerStyle?: ?StyleSheet.Styles,
	onPress: () => any
}> {
	static propTypes = {
		footerStyle: ViewPropTypes.style,
		labelText: PropTypes.string,
		onPress: PropTypes.func,
		secondaryText: PropTypes.string,
		title: PropTypes.string.isRequired
	};

	render() {
		const {
			title,
			secondaryText,
			labelText,
			footerStyle,
			onPress
		} = this.props;

		const finalBodyStyle = [styles.body, footerStyle];
		const finalContentStyle = [styles.content];
		const finalFooterStyle = [styles.footer, footerStyle];
		const finalTitleTextStyle = [styles.titleText];
		const finalSecondaryTextStyle = [styles.secondaryText];
		const finalFooterTextStyle = [styles.footerText];

		const Touchable =
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
}

const styles = StyleSheet.create({
	body: {},
	content: {
		backgroundColor: colors.card_bg,
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
