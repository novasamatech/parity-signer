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
	TouchableNativeFeedback,
	TouchableOpacity,
	View,
	ViewPropTypes
} from 'react-native';
import { Icon } from 'react-native-elements';
import colors from '../colors';

export default class ButtonIcon extends React.PureComponent<{
	onPress: () => any
}> {
	static propTypes = {
		iconColor: PropTypes.string,
		iconName: PropTypes.string.isRequired,
		iconType: PropTypes.string,
		onPress: PropTypes.func,
		style: ViewPropTypes.style
	};

	render() {
		const { iconColor, iconName, iconType, onPress } = this.props;

		const Touchable =
			Platform.OS === 'android' ? TouchableNativeFeedback : TouchableOpacity;
		return (
			<Touchable accessibilityComponentType="button" onPress={onPress}>
				<View style={styles.button}>
					<Icon
						color={iconColor || colors.bg_text_sec}
						size={26}
						name={iconName}
						type={iconType}
					/>
				</View>
			</Touchable>
		);
	}
}

const styles = StyleSheet.create({
	button: {
		alignItems: 'center',
		backgroundColor: colors.card_bg,
		borderRadius: 24,
		height: 32,
		justifyContent: 'center',
		marginLeft: 8,
		width: 32
	}
});
