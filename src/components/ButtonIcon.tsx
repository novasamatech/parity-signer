// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Modifications Copyright (c) 2021 Thibaut Sardan

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

import React, { FunctionComponent } from 'react';
import { StyleSheet, Text, TextStyle, TouchableOpacity, View, ViewStyle } from 'react-native';
import { Icon } from 'react-native-elements';
import AntIcon from 'react-native-vector-icons/AntDesign';
import colors from 'styles/colors';
import { ButtonListener } from 'types/props';

interface Props {
	iconName: string;
	iconType: string;
	iconColor?: string;
	onPress: ButtonListener;
	iconBgStyle?: ViewStyle;
	iconSize?: number;
	testID?: string;
	textStyle?: TextStyle;
	title?: string;
	style?: ViewStyle;
}

const ButtonIcon: FunctionComponent<Props> = ({ iconName, iconType, iconColor, onPress, iconBgStyle, iconSize, testID, textStyle, title, style = {} }) => {
	const size = iconSize || 28;

	const styles = StyleSheet.create({
		generalView: {
			display: 'flex',
			flexDirection: 'row',
			paddingVertical: 8
		},
		iconTitleView: {
			alignItems: 'center',
			flexDirection: 'row',
			marginLeft: 8
		},
		iconView: {
			height: size,
			paddingLeft: 3,
			paddingTop: size / 8,
			width: size
		},
		title: { marginLeft: 8 }
	});

	const renderIcon = (): React.ReactElement => {
		if (iconType === 'antdesign') {
			return (
				<AntIcon
					color={iconColor || colors.text.main}
					name={iconName}
					size={size - 6}
				/>
			);
		}

		return (
			<Icon
				color={iconColor || colors.text.main}
				name={iconName}
				size={size - 6}
				type={iconType}
			/>
		);
	};

	return (
		<TouchableOpacity
			activeOpacity={0.5}
			onPress={onPress}
			style={{ ...styles.generalView, ...style }}
			testID={testID}
		>
			<View style={styles.iconTitleView}>
				<View style={[styles.iconView, iconBgStyle]}>{renderIcon()}</View>
				{!!title && <Text style={[styles.title, textStyle]}>{title}</Text>}
			</View>
		</TouchableOpacity>
	);
};

export default ButtonIcon;
