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

const composeStyle = StyleSheet.compose;

const extendComponentStyle = (styleKey, extendStyle) => {
	componentStyles[styleKey] = StyleSheet.compose(
		componentStyles[styleKey],
		extendStyle
	);
};

const renderSubtitle = (subtitle, subtitleIcon, styles) => {
	if (!subtitle) return;
	return (
		<View style={styles.finalSubtitleIconStyle}>
			{renderSubtitleIcon(subtitleIcon)}
			<Text style={[styles.finalTextStyles, styles.finalSubtitleStyle]}>
				{subtitle}
			</Text>
		</View>
	);
};
const renderSubtitleIcon = subtitleIcon => {
	if (!subtitleIcon) return;
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
	subtitleIcon,
	networkKey,
	onPress
}) {
	extendComponentStyle('finalTextStyles', baseStyles.t_left);
	extendComponentStyle('finalSubtitleIconStyle', {
		justifyContent: 'flex-start'
	});
	return (
		<TouchableItem style={baseStyles.bodyWithIcon} onPress={onPress}>
			<AccountIcon
				address={''}
				network={NETWORK_LIST[networkKey]}
				style={baseStyles.networkIcon}
			/>
			<View>
				<Text style={componentStyles.finalTextStyles}>{title}</Text>
				{renderSubtitle(subtitle, subtitleIcon, componentStyles)}
			</View>
		</TouchableItem>
	);
}

export default class ScreenHeading extends React.PureComponent {
	static propTypes = {
		big: PropTypes.bool,
		onPress: PropTypes.func,
		small: PropTypes.bool,
		subtitle: PropTypes.string,
		title: PropTypes.string
	};
	render() {
		const {
			title,
			subtitle,
			subtitleL,
			subtitleIcon,
			error,
			onPress,
			iconName,
			iconType
		} = this.props;

		if (error) {
			extendComponentStyle('finalSubtitleStyle', baseStyles.t_error);
		}
		if (subtitleL) {
			extendComponentStyle('finalSubtitleStyle', { textAlign: 'left' });
		}

		return (
			<View style={[baseStyles.body, baseStyles.bodyL]}>
				<Text style={componentStyles.finalTextStyles}>{title}</Text>
				{renderSubtitle(subtitle, subtitleIcon, componentStyles)}
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
	bodyL: {
		paddingLeft: 72,
		paddingRight: 16
	},
	bodyWithIcon: {
		alignItems: 'center',
		flexDirection: 'row',
		marginBottom: 16
	},
	icon: {
		marginLeft: 5,
		position: 'absolute'
	},
	networkIcon: {
		paddingHorizontal: 16
	},
	subtitleIcon: {
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
	}
});

const componentStyles = {
	finalSubtitleIconStyle: baseStyles.subtitleIcon,
	finalSubtitleStyle: fontStyles.t_codeS,
	finalTextStyles: StyleSheet.compose(
		fontStyles.h1,
		baseStyles.t_center
	)
};
