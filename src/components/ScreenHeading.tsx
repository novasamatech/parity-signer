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

import TouchableItem from 'components/TouchableItem';
import testIDs from 'e2e/testIDs';
import React, { ReactElement, ReactNode, useContext } from 'react';
import { StyleSheet, Text, TextStyle,View, ViewStyle } from 'react-native';
import { Icon } from 'react-native-elements';
import AntIcon from 'react-native-vector-icons/AntDesign';
import colors from 'styles/colors';
import fonts from 'styles/fonts';
import fontStyles from 'styles/fontStyles';
import { ButtonListener } from 'types/props';

import { NetworksContext } from '../context';
import AccountIcon from './AccountIcon';
import ButtonIcon from './ButtonIcon';

const renderSubtitle = (subtitle?: string,
	hasSubtitleIcon?: boolean,
	isAlignLeft?: boolean,
	isError?: boolean,
	multiline?: boolean): ReactNode => {
	if (!subtitle || subtitle === '') return;
	const subtitleBodyStyle: ViewStyle[] = [baseStyles.subtitleBody],
		subtitleTextStyle: TextStyle[] = [
			fontStyles.t_codeS,
			{ color: colors.text.faded }
		];

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
				ellipsizeMode="middle"
				numberOfLines={multiline ? undefined : 1}
				style={subtitleTextStyle}
			>
				{subtitle}
			</Text>
		</View>
	);
};

const renderSubtitleIcon = (hasSubtitleIcon?: boolean): ReactNode => {
	if (!hasSubtitleIcon) return;

	return (
		<AntIcon
			color={colors.text.faded}
			name="user"
			size={10}
		/>
	);
};

const renderBack = (onPress?: ButtonListener): ReactNode => {
	if (!onPress) return;

	return (
		<ButtonIcon
			iconBgStyle={{ backgroundColor: 'transparent' }}
			iconName="arrowleft"
			iconType="antdesign"
			onPress={onPress}
			style={StyleSheet.flatten([baseStyles.icon, { left: 0 }])}
			testID={testIDs.Main.backButton}
		/>
	);
};

const renderIcon = (iconName?: string, iconType?: string): ReactNode => {
	if (!iconName) return;

	return (
		<View style={[baseStyles.icon, { paddingLeft: 16 }]}>
			<Icon color={colors.text.main}
				name={iconName}
				type={iconType} />
		</View>
	);
};

export function LeftScreenHeading({ hasSubtitleIcon, headMenu, networkKey, onPress, subtitle, title }: {
	title: string;
	subtitle?: string;
	hasSubtitleIcon?: boolean;
	headMenu?: React.ReactElement;
	networkKey?: string;
	onPress?: () => any;
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
	const { getNetwork } = useContext(NetworksContext);
	const isDisabled = onPress === undefined;

	return (
		<TouchableItem
			disabled={isDisabled}
			onPress={onPress}
			style={baseStyles.bodyWithIcon}
		>
			<View style={{ alignItems: 'center', flexDirection: 'row' }}>
				{ networkKey && (
					<AccountIcon
						address={''}
						network={getNetwork(networkKey)}
						style={baseStyles.networkIcon}
					/>
				)}
				<View>
					<Text style={subtitle ? titleStyleWithSubtitle : titleStyle}>
						{title}
					</Text>
					{renderSubtitle(subtitle, hasSubtitleIcon, true, false, false)}
				</View>
			</View>
			{headMenu}
		</TouchableItem>
	);
}

export function IdentityHeading({ hasSubtitleIcon, onPressBack, subtitle, title }: {
	title: string;
	subtitle?: string;
	hasSubtitleIcon?: boolean;
	onPressBack?: ButtonListener;
}): ReactElement {
	return (
		<View style={baseStyles.bodyWithIdentity}>
			<View style={baseStyles.identityName}>
				<Text
					ellipsizeMode="middle"
					numberOfLines={1}
					style={[baseStyles.text, baseStyles.t_left]}
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
		const { error, hasSubtitleIcon, headMenu, iconName, iconType, subtitle, subtitleL, title } = this.props;

		return (
			<View style={{ ...baseStyles.body, flexDirection: 'row' }}>
				{renderIcon(iconName, iconType)}
				<View style={baseStyles.titles}>
					<Text style={baseStyles.text}>{title}</Text>
					{renderSubtitle(subtitle, hasSubtitleIcon, subtitleL, error, true)}
				</View>
				{headMenu}
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
		justifyContent: 'space-between',
		marginBottom: 16,
		paddingRight: 16
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
	linkIcon: { marginLeft: 10 },
	// menu: {
	// 	alignSelf: 'flex-end'
	// },
	networkIcon: { paddingHorizontal: 16 },
	subtitleBody: {
		alignItems: 'center',
		flexDirection: 'row',
		justifyContent: 'center'
	},
	t_center: { textAlign: 'center' },
	t_error: { color: colors.signal.error },
	t_left: { textAlign: 'left' },
	t_normal: { fontFamily: fonts.roboto },
	text: {
		...fontStyles.h1,
		textAlign: 'center'
	},
	titles: {
		alignItems: 'center',
		flex: 1
	}
});
