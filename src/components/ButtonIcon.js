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
import AntIcon from 'react-native-vector-icons/AntDesign';
import colors from '../colors';

const ButtonIcon = props => {
	const {
		dropdown = false,
		renderDropdownElement,
		iconName,
		iconType,
		iconColor,
		onPress,
		iconBgStyle,
		iconSize,
		textStyle,
		title,
		style
	} = props;

	const size = iconSize || 28;

	const styles = {
		dropdownView: {
			marginRight: 8,
			marginTop: size / -12,
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
			marginLeft: 8
		},
		iconTitleViewContainer: {
			flex: dropdown && title ? 1 : 0
		},
		iconView: {
			alignItems: 'center',
			backgroundColor: colors.card_bg,
			borderRadius: size,
			height: size,
			justifyContent: 'center',
			width: size
		},
		title: {
			marginLeft: 8
		}
	};

	const Touchable =
		Platform.OS === 'android' ? TouchableNativeFeedback : TouchableOpacity;

	const renderIcon = () => {
		if (iconType === 'antdesign') {
			return (
				<AntIcon
					color={iconColor || colors.bg_text}
					size={size - 6}
					name={iconName}
				/>
			);
		}
		return (
			<Icon
				color={iconColor || colors.bg_text}
				size={size - 6}
				name={iconName}
				type={iconType}
			/>
		);
	};
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
								{renderIcon()}
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
									size={size - 4}
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
