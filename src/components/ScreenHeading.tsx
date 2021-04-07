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

import React, { ReactElement, useContext } from 'react';
import { View, StyleSheet, Text, TextStyle } from 'react-native';

import AccountIcon from './AccountIcon';

import { fonts, fontStyles } from 'styles/index';
import { NetworksContext } from 'stores/NetworkContext';
import TouchableItem from 'components/TouchableItem';
import { ButtonListener } from 'types/props';

export function ScreenHeadingWithNetworkIcon({
	title,
	headMenu,
	networkKey,
	onPress
}: {
	title: string;
	headMenu?: React.ReactElement;
	networkKey: string;
	onPress?: () => any;
}): ReactElement {
	const titleStyle: TextStyle = {
		fontFamily: fonts.bold,
		...fontStyles.h2
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

export default class ScreenHeading extends React.PureComponent<{
	title: string;
	onPress?: ButtonListener;
}> {
	render(): ReactElement {
		const { title } = this.props;

		return (
			<View style={{ ...baseStyles.body, flexDirection: 'row' }}>
				<View style={{ flex: 1 }}>
					<Text style={fontStyles.h2}>{title}</Text>
				</View>
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
	icon: {
		marginLeft: 5,
		paddingLeft: 16,
		position: 'absolute'
	},
	networkIcon: {
		paddingHorizontal: 16
	}
});
