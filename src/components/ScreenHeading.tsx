// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Copyright 2021 Commonwealth Labs, Inc.
// This file is part of Layer Wallet.

// Layer Wallet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Layer Wallet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Layer Wallet. If not, see <http://www.gnu.org/licenses/>.

import React, { ReactElement, ReactNode, useContext } from 'react';
import { View, StyleSheet, Text, ViewStyle, TextStyle } from 'react-native';
import AntIcon from 'react-native-vector-icons/AntDesign';
import { Icon } from 'react-native-elements';

import ButtonIcon from './ButtonIcon';
import AccountIcon from './AccountIcon';

import { colors, fonts, fontStyles } from 'styles';
import { NetworksContext } from 'stores/NetworkContext';
import TouchableItem from 'components/TouchableItem';
import testIDs from 'e2e/testIDs';
import { ButtonListener } from 'types/props';

export function ScreenHeadingWithNetworkIcon({
	title,
	subtitle,
	hasSubtitleIcon,
	headMenu,
	networkKey,
	onPress
}: {
	title: string;
	subtitle?: string;
	hasSubtitleIcon?: boolean;
	headMenu?: React.ReactElement;
	networkKey: string;
	onPress?: () => any;
}): ReactElement {
	const titleStyle: TextStyle = {
		fontFamily: fonts.bold,
		...fontStyles.h2,
		...baseStyles.t_left,
	};
	const { getNetwork } = useContext(NetworksContext);
	const isDisabled = onPress === undefined;
	return (
		<TouchableItem
			style={baseStyles.bodyWithIcon}
			onPress={onPress}
			disabled={isDisabled}
		>
			<View style={{ alignItems: 'center', flexDirection: 'row' }}>
				<AccountIcon
					address={''}
					network={getNetwork(networkKey)}
					style={baseStyles.networkIcon}
				/>
				<View>
					<Text style={titleStyle}>{title}</Text>
				</View>
			</View>
			{headMenu}
		</TouchableItem>
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
			{onPressBack &&
				(<ButtonIcon
					iconName="arrowleft"
					iconType="antdesign"
					onPress={onPress}
					testID={testIDs.Wallet.backButton}
					style={StyleSheet.flatten([baseStyles.icon, { left: 0 }])}
					iconBgStyle={{ backgroundColor: 'transparent' }}
				/>)
			}
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
				<View style={[baseStyles.icon, { paddingLeft: 16 }]}>
					<Icon name={iconName} type={iconType} color={colors.text.main} />
				</View>
				<View style={baseStyles.titles}><Text style={baseStyles.text}>{title}</Text></View>
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
	linkIcon: {
		marginLeft: 10
	},
	// menu: {
	// 	alignSelf: 'flex-end'
	// },
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
		color: colors.signal.error
	},
	t_left: {
		textAlign: 'left'
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
