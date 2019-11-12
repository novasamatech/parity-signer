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

import React, { useState } from 'react';
import {
	Platform,
	TouchableNativeFeedback,
	TouchableOpacity,
	View,
	Text
} from 'react-native';
import { Icon } from 'react-native-elements';
import colors from '../colors';

const ButtonIcon = props => {
	const {
		dropdown = false,
		renderDropdownElement,
		iconName,
		iconType,
		onPress,
		iconBgStyle,
		iconSize,
		textStyle,
		title,
		style
	} = props;

	const styles = {
		dropdownView: {
			marginRight: 8,
			marginVertical: 8
		},
		generalView: {
			display: 'flex',
			flexDirection: 'row',
			marginVertical: 8
		},
		iconTitleView: {
			alignItems: 'center',
			display: 'flex',
			flexDirection: 'row',
			marginHorizontal: 8,
			marginLeft: 8
		},
		iconTitleViewContainer: {
			flex: dropdown && title ? 1 : 0
		},
		iconView: {
			backgroundColor: colors.card_bg,
			borderRadius: iconSize || 24,
			height: iconSize || 24,
			justifyContent: 'center',
			width: iconSize || 24
		},
		title: {
			marginLeft: 8
		}
	};

	const Touchable =
		Platform.OS === 'android' ? TouchableNativeFeedback : TouchableOpacity;

	const [isDropdownOpen, setIsDropsdownOpen] = useState(false);

	return (
		<>
			<View style={[styles.generalView, style]}>
				<View style={styles.iconTitleViewContainer}>
					<Touchable accessibilityComponentType="button" onPress={onPress}>
						<View style={styles.iconTitleView}>
							<TouchableOpacity
								style={[styles.iconView, iconBgStyle]}
								onPress={onPress}
							>
								<Icon
									color={colors.bg_text}
									size={iconSize - 4 || 20}
									name={iconName}
									type={iconType}
								/>
							</TouchableOpacity>
							{!!title && (
								<Text style={[styles.title, textStyle]}>{title}</Text>
							)}
						</View>
					</Touchable>
				</View>
				{dropdown && (
					<View>
						<Touchable onPress={() => setIsDropsdownOpen(!isDropdownOpen)}>
							<View style={styles.dropdownView}>
								<Icon
									color={colors.bg_text}
									size={iconSize - 4 || 20}
									name={
										isDropdownOpen ? 'md-arrow-dropup' : 'md-arrow-dropdown'
									}
									type="ionicon"
								/>
							</View>
						</Touchable>
					</View>
				)}
			</View>
			{isDropdownOpen && renderDropdownElement()}
		</>
	);
};

export default ButtonIcon;
