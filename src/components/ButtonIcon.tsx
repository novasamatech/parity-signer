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

import React, { FunctionComponent, useState } from 'react';
import {
	Platform,
	TouchableNativeFeedback,
	TouchableOpacity,
	View,
	Text,
	ViewStyle,
	TextStyle,
	StyleSheet,
	TouchableNativeFeedbackProps
} from 'react-native';
import { Icon } from 'react-native-elements';
import AntIcon from 'react-native-vector-icons/AntDesign';

import colors from 'styles/colors';
import { ButtonListener } from 'types/props';

interface Props {
	dropdown?: boolean;
	renderDropdownElement?: () => React.ReactNode;
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

const ButtonIcon: FunctionComponent<Props> = ({
	dropdown = false,
	renderDropdownElement = (): null => null,
	iconName,
	iconType,
	iconColor,
	onPress,
	iconBgStyle,
	iconSize,
	testID,
	textStyle,
	title,
	style = {}
}) => {
	const size = iconSize || 28;

	const styles = StyleSheet.create({
		dropdownView: {
			marginRight: 8,
			marginTop: size / -12,
			marginVertical: 8
		},
		generalView: {
			display: 'flex',
			flexDirection: 'row',
			paddingVertical: 8
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
			backgroundColor: colors.card_bg,
			borderRadius: size,
			height: size,
			paddingLeft: 3,
			paddingTop: size / 8,
			width: size
		},
		title: {
			marginLeft: 8
		}
	});

	const Touchable: React.ComponentClass<TouchableNativeFeedbackProps> =
		Platform.OS === 'android' ? TouchableNativeFeedback : TouchableOpacity;

	const renderIcon = (): React.ReactElement => {
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
			<TouchableOpacity
				accessibilityComponentType="button"
				onPress={onPress}
				activeOpacity={0.5}
				style={{ ...styles.generalView, ...style }}
				testID={testID}
			>
				<View style={styles.iconTitleViewContainer}>
					<View style={styles.iconTitleView}>
						<View style={[styles.iconView, iconBgStyle]}>{renderIcon()}</View>
						{!!title && <Text style={[styles.title, textStyle]}>{title}</Text>}
					</View>
				</View>
			</TouchableOpacity>

			{dropdown && (
				<View>
					<Touchable onPress={(): void => setIsDropsdownOpen(!isDropdownOpen)}>
						<View style={styles.dropdownView}>
							<Icon
								color={colors.bg_text}
								size={size - 4}
								name={isDropdownOpen ? 'md-arrow-dropup' : 'md-arrow-dropdown'}
								type="ionicon"
							/>
						</View>
					</Touchable>
				</View>
			)}

			{isDropdownOpen && renderDropdownElement()}
		</>
	);
};

export default ButtonIcon;
