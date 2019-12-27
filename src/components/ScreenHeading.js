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

'use strict';

import PropTypes from 'prop-types';
import React from 'react';
import { View, StyleSheet, Text } from 'react-native';
import AntIcon from 'react-native-vector-icons/AntDesign';
import fontStyles from '../fontStyles';
import fonts from '../fonts';
import ButtonIcon from './ButtonIcon';
import { Icon } from 'react-native-elements';
import colors from '../colors';
import AccountIcon from './AccountIcon';
import { NETWORK_LIST } from '../constants';
import TouchableItem from './TouchableItem';
import FontAwesome from 'react-native-vector-icons/FontAwesome';

const composeStyle = StyleSheet.compose;

const renderSubtitle = (
	subtitle,
	hasSubtitleIcon,
	isAlignLeft,
	isError,
	multiline
) => {
	if (!subtitle || subtitle === '') return;
	let subtitleBodyStyle = [baseStyles.subtitleBody],
		subtitleTextStyle = [fontStyles.t_codeS];
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
const renderSubtitleIcon = hasSubtitleIcon => {
	if (!hasSubtitleIcon) return;
	return <AntIcon name="user" size={10} color={colors.bg_text_sec} />;
};

const renderBack = onPress => {
	if (!onPress) return;
	return (
		<ButtonIcon
			iconName="arrowleft"
			iconType="antdesign"
			onPress={onPress}
			style={[baseStyles.icon, { left: 0, top: -8 }]}
			iconBgStyle={{ backgroundColor: 'transparent' }}
		/>
	);
};
const renderIcon = (iconName, iconType) => {
	if (!iconName) return;
	return (
		<View style={[baseStyles.icon, { paddingLeft: 16 }]}>
			<Icon name={iconName} type={iconType} color={colors.bg_text} />
		</View>
	);
};

export function PathCardHeading({ title, networkKey }) {
	const titleStyle = composeStyle(
		fontStyles.h2,
		baseStyles.t_left,
		baseStyles.t_normal
	);
	return (
		<View style={baseStyles.bodyWithIcon}>
			<AccountIcon
				address={''}
				network={NETWORK_LIST[networkKey]}
				style={baseStyles.networkIcon}
			/>
			<View>
				<Text style={titleStyle}>{title}</Text>
			</View>
		</View>
	);
}

export function PathListHeading({
	title,
	subtitle,
	hasSubtitleIcon,
	testID,
	networkKey,
	onPress
}) {
	return (
		<TouchableItem
			style={baseStyles.bodyWithIcon}
			onPress={onPress}
			testID={testID}
		>
			<AccountIcon
				address={''}
				network={NETWORK_LIST[networkKey]}
				style={baseStyles.networkIcon}
			/>
			<View>
				<Text style={[baseStyles.text, baseStyles.t_left]}>{title}</Text>
				{renderSubtitle(subtitle, hasSubtitleIcon, true)}
			</View>
		</TouchableItem>
	);
}

export function IdentityHeading({ onPress, title, subtitle, hasSubtitleIcon }) {
	return (
		<TouchableItem style={baseStyles.bodyWithIdentity} onPress={onPress}>
			<View style={baseStyles.touchable}>
				<View style={baseStyles.identityName}>
					<Text
						style={[baseStyles.text, baseStyles.t_left]}
						numberOfLines={1}
						ellipsizeMode="middle"
					>
						{title}
					</Text>
					<FontAwesome
						style={baseStyles.linkIcon}
						name="external-link"
						color={colors.bg_button}
						size={18}
					/>
				</View>
				{renderSubtitle(subtitle, hasSubtitleIcon, true)}
			</View>
		</TouchableItem>
	);
}

export default class ScreenHeading extends React.PureComponent {
	static propTypes = {
		multiline: PropTypes.bool,
		onPress: PropTypes.func,
		subtitle: PropTypes.string,
		title: PropTypes.string
	};
	render() {
		const {
			title,
			subtitle,
			subtitleL,
			hasSubtitleIcon,
			error,
			onPress,
			iconName,
			iconType,
			multiline
		} = this.props;

		return (
			<View style={baseStyles.body}>
				<Text style={baseStyles.text}>{title}</Text>
				{renderSubtitle(subtitle, hasSubtitleIcon, subtitleL, error, multiline)}
				{renderBack(onPress)}
				{renderIcon(iconName, iconType)}
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
		height: 42,
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
	touchable: {
		flex: 1,
		flexDirection: 'column',
		justifyContent: 'center'
	}
});
