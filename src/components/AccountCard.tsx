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

import React, { ReactElement } from 'react';
import { StyleSheet, Text, View, ViewStyle } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';

import AccountIcon from './AccountIcon';
import Address from './Address';
import TouchableItem from './TouchableItem';
import AccountPrefixedTitle from './AccountPrefixedTitle';

import Separator from 'components/Separator';
import { NETWORK_LIST, NetworkProtocols } from 'constants/networkSpecs';
import fontStyles from 'styles/fontStyles';
import colors from 'styles/colors';
import { NetworkParams } from 'types/networkSpecsTypes';
import { ButtonListener } from 'types/props';

const CardSeparator = (): ReactElement => (
	<Separator
		shadow={true}
		style={{
			backgroundColor: 'transparent',
			height: 0,
			marginVertical: 0
		}}
	/>
);

const NetworkFooter = ({
	networkColor,
	network
}: {
	networkColor: string;
	network: NetworkParams;
}): React.ReactElement => (
	<View
		style={[
			styles.footer,
			{
				backgroundColor: networkColor || network.color
			}
		]}
	/>
);

export function NetworkCard({
	isAdd,
	networkColor,
	networkKey,
	onPress,
	testID,
	title
}: {
	isAdd?: boolean;
	networkColor?: string;
	networkKey?: string;
	onPress: ButtonListener;
	testID?: string;
	title: string;
}): ReactElement {
	const network =
		networkKey !== undefined
			? NETWORK_LIST[networkKey]
			: NETWORK_LIST[NetworkProtocols.UNKNOWN];

	return (
		<TouchableItem testID={testID} disabled={false} onPress={onPress}>
			<CardSeparator />
			<View style={styles.content}>
				{isAdd ? (
					<View style={{ height: 40, width: 40 }}>
						<Icon name="add" color={colors.bg_text} size={32} />
					</View>
				) : (
					<AccountIcon address={''} network={network} style={styles.icon} />
				)}
				<View style={styles.desc}>
					<AccountPrefixedTitle title={title} />
				</View>
				<NetworkFooter
					network={network}
					networkColor={networkColor ?? network.color}
				/>
			</View>
		</TouchableItem>
	);
}

type AccountCardProps = {
	address: string;
	networkKey?: string;
	onPress?: ButtonListener;
	seedType?: string;
	style?: ViewStyle;
	testID?: string;
	title: string;
	titlePrefix?: string;
};

export default function AccountCard({
	address,
	networkKey,
	onPress,
	seedType,
	style,
	testID,
	title,
	titlePrefix
}: AccountCardProps): ReactElement {
	const defaultTitle = 'No name';
	const displayTitle = title.length > 0 ? title : defaultTitle;
	const seedTypeDisplay = seedType || '';
	const network =
		networkKey !== undefined
			? NETWORK_LIST[networkKey]
			: NETWORK_LIST[NetworkProtocols.UNKNOWN];

	return (
		<TouchableItem
			accessibilityComponentType="button"
			testID={testID}
			disabled={false}
			onPress={onPress}
		>
			<CardSeparator />
			<View style={[styles.content, style]}>
				<AccountIcon address={address} network={network} style={styles.icon} />
				<View style={styles.desc}>
					<View>
						<Text style={[fontStyles.t_regular, { color: colors.bg_text_sec }]}>
							{`${network.title}${seedTypeDisplay} `}
						</Text>
					</View>
					<AccountPrefixedTitle
						title={displayTitle}
						titlePrefix={titlePrefix}
					/>
					{address !== '' && (
						<Address address={address} protocol={network.protocol} />
					)}
				</View>
				<NetworkFooter network={network} networkColor={network.color} />
			</View>
		</TouchableItem>
	);
}

const styles = StyleSheet.create({
	content: {
		alignItems: 'center',
		flexDirection: 'row',
		paddingLeft: 16
	},
	desc: {
		flex: 1,
		flexDirection: 'column',
		justifyContent: 'space-between',
		paddingLeft: 16
	},
	footer: {
		alignSelf: 'stretch',
		height: 88,
		marginLeft: 8,
		width: 8
	},
	icon: {
		height: 40,
		width: 40
	}
});
