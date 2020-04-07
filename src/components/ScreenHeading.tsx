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

import React, { ReactElement, ReactNode } from 'react';
import { View, StyleSheet, Text, ViewStyle, TextStyle } from 'react-native';
import AntIcon from 'react-native-vector-icons/AntDesign';
import { Icon } from 'react-native-elements';

import ButtonIcon from './ButtonIcon';
import AccountIcon from './AccountIcon';

import { NETWORK_LIST } from 'constants/networkSpecs';
import fontStyles from 'styles/fontStyles';
import fonts from 'styles/fonts';
import colors from 'styles/colors';
import { ButtonListener } from 'types/props';

const renderSubtitle = (
	subtitle?: string,
	hasSubtitleIcon?: boolean,
	isAlignLeft?: boolean,
	isError?: boolean,
	multiline?: boolean
): ReactNode => {
	if (!subtitle || subtitle === '') return;
	const subtitleBodyStyle: ViewStyle[] = [baseStyles.subtitleBody],
		subtitleTextStyle: TextStyle[] = [fontStyles.t_codeS];
	if (isAlignLeft) {
		subtitleBodyStyle.push({ justifyContent: 'flex-start' });
		subtitleTextStyle.push({ textAlign: 'left' });
	}
	if (isError) {
		subtitleTextStyle.push(baseStyles.t_error);
	}

	return (
		<View style={subtitleBodyStyle}>
			{renderSubtitleIcon(hasSubtitleIcon)}
			<Text
				style={subtitleTextStyle}
				numberOfLines={multiline ? undefined : 1}
				ellipsizeMode="middle"
			>
				{subtitle}
			</Text>
		</View>
	);
};
const renderSubtitleIcon = (hasSubtitleIcon?: boolean): ReactNode => {
	if (!hasSubtitleIcon) return;
	return <AntIcon name="user" size={10} color={colors.bg_text_sec} />;
};

const renderBack = (onPress?: ButtonListener): ReactNode => {
	if (!onPress) return;
	return (
		<ButtonIcon
			iconName="arrowleft"
			iconType="antdesign"
			onPress={onPress}
			style={StyleSheet.flatten([baseStyles.icon, { left: 0 }])}
			iconBgStyle={{ backgroundColor: 'transparent' }}
		/>
	);
};
const renderIcon = (iconName?: string, iconType?: string): ReactNode => {
	if (!iconName) return;
	return (
		<View style={[baseStyles.icon, { paddingLeft: 16 }]}>
			<Icon name={iconName} type={iconType} color={colors.bg_text} />
		</View>
	);
};

export function LeftScreenHeading({
	title,
	subtitle,
	hasSubtitleIcon,
	headMenu,
	networkKey
}: {
	title: string;
	subtitle?: string;
	hasSubtitleIcon?: boolean;
	headMenu?: React.ReactElement;
	networkKey: string;
}): ReactElement {
	const titleStyle: TextStyle = {
		...fontStyles.h2,
		...baseStyles.t_left,
		...baseStyles.t_normal
	};
	const titleStyleWithSubtitle: TextStyle = {
		...baseStyles.text,
		...baseStyles.t_left
	};
	return (
		<View style={baseStyles.bodyWithIcon}>
			<AccountIcon
				address={''}
				network={NETWORK_LIST[networkKey]}
				style={baseStyles.networkIcon}
			/>
			<View>
				<Text style={subtitle ? titleStyleWithSubtitle : titleStyle}>
					{title}
				</Text>
				{renderSubtitle(subtitle, hasSubtitleIcon, true, false, false)}
			</View>
			<View style={baseStyles.menu}>{headMenu}</View>
		</View>
	);
}

export function IdentityHeading({
	title,
	subtitle,
	hasSubtitleIcon,
	onPressBack
}: {
	title: string;
	subtitle?: string;
	hasSubtitleIcon?: boolean;
	onPressBack?: ButtonListener;
}): ReactElement {
	return (
		<View style={baseStyles.bodyWithIdentity}>
			<View style={baseStyles.identityName}>
				<Text
					style={[baseStyles.text, baseStyles.t_left]}
					numberOfLines={1}
					ellipsizeMode="middle"
				>
					{title}
				</Text>
			</View>
			{onPressBack && renderBack(onPressBack)}
			{renderSubtitle(subtitle, hasSubtitleIcon, true, false, false)}
		</View>
	);
}

export default class ScreenHeading extends React.PureComponent<{
	subtitle?: string;
	subtitleL?: boolean;
	hasSubtitleIcon?: boolean;
	headMenu?: React.ReactElement;
	title: string;
	onPress?: ButtonListener;
	error?: boolean;
	iconName?: string;
	iconType?: string;
}> {
	render(): ReactElement {
		const {
			title,
			subtitle,
			subtitleL,
			hasSubtitleIcon,
			headMenu,
			error,
			iconName,
			iconType
		} = this.props;

		return (
			<View style={{ ...baseStyles.body, flexDirection: 'row' }}>
				{renderIcon(iconName, iconType)}
				<View style={baseStyles.titles}>
					<Text style={baseStyles.text}>{title}</Text>
					{renderSubtitle(subtitle, hasSubtitleIcon, subtitleL, error, true)}
				</View>
				<View style={baseStyles.menu}>{headMenu}</View>
			</View>
		);
	}
}

const baseStyles = StyleSheet.create({
	body: {
		marginBottom: 16,
		paddingHorizontal: 16
	},
	bodyWithIcon: {
		alignItems: 'center',
		flexDirection: 'row',
		marginBottom: 16
	},
	bodyWithIdentity: {
		flexDirection: 'column',
		height: 42,
		justifyContent: 'center',
		paddingLeft: 72,
		paddingRight: 32
	},
	icon: {
		marginLeft: 5,
		position: 'absolute'
	},
	identityName: {
		alignItems: 'center',
		flexDirection: 'row'
	},
	linkIcon: {
		marginLeft: 10
	},
	menu: {
		marginLeft: 'auto',
		marginRight: 10
	},
	networkIcon: {
		paddingHorizontal: 16
	},
	subtitleBody: {
		alignItems: 'center',
		flexDirection: 'row',
		justifyContent: 'center'
	},
	t_center: {
		textAlign: 'center'
	},
	t_error: {
		color: colors.bg_alert
	},
	t_left: {
		textAlign: 'left'
	},
	t_normal: {
		fontFamily: fonts.roboto
	},
	text: {
		...fontStyles.h1,
		textAlign: 'center'
	},
	titles: {
		alignItems: 'center',
		flex: 1
	}
});
