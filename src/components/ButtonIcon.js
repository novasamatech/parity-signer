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
	TouchableNativeFeedback,
	TouchableOpacity,
	View,
	ViewPropTypes,
	Text
} from 'react-native';
import { Icon } from 'react-native-elements';
import colors from '../colors';

export default class ButtonIcon extends React.PureComponent<{
	onPress: () => any
}> {
	static propTypes = {
		iconBgStyle: ViewPropTypes.style,
		iconName: PropTypes.string.isRequired,
		iconSize: PropTypes.number,
		iconType: PropTypes.string,
		onPress: PropTypes.func.isRequired,
		style: ViewPropTypes.style,
		textStyle: Text.propTypes.style,
		title: PropTypes.string
	};

	render() {
		const {
			iconName,
			iconType,
			onPress,
			iconBgStyle,
			iconSize,
			textStyle,
			title,
			style
		} = this.props;
		const Touchable =
			Platform.OS === 'android' ? TouchableNativeFeedback : TouchableOpacity;
		return (
			<Touchable accessibilityComponentType="button" onPress={onPress}>
				<View
					style={[
						{
							alignItems: 'center',
							flexDirection: 'row',
							marginLeft: 8,
							marginVertical: 8
						},
						style
					]}
				>
					<View
						style={[
							{
								alignItems: 'center',
								backgroundColor: colors.card_bg,
								borderRadius: iconSize || 24,
								height: iconSize || 24,
								justifyContent: 'center',
								width: iconSize || 24
							},
							iconBgStyle
						]}
					>
						<Icon
							color={colors.bg_text}
							size={iconSize - 4 || 20}
							name={iconName}
							type={iconType}
						/>
					</View>

					{title && <Text style={[{ marginLeft: 8 }, textStyle]}>{title}</Text>}
				</View>
			</Touchable>
		);
	}
}
