// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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
import { StyleSheet, View } from 'react-native';

import TouchableItem from './TouchableItem';
import AccountPrefixedTitle from './AccountPrefixedTitle';

import NetworkIcon from 'components/NetworkIcon';
import { CardSeparator } from 'components/CardSeparator';
import { ButtonListener } from 'types/props';

const NetworkFooter = ({ color }: { color: string }): React.ReactElement => (
	<View
		style={[
			styles.footer,
			{
				backgroundColor: color
			}
		]}
	/>
);

export function NetworkCard({
	network,
	onPress,
	testID
}: {
	network: any;
	onPress?: ButtonListener;
	testID?: string;
}): ReactElement {
	const isDisabled = onPress === undefined;
	return (
		<TouchableItem testID={testID} disabled={isDisabled} onPress={onPress}>
			<CardSeparator />
			<View style={styles.content}>
				<View style={styles.icon}>
					<NetworkIcon
						logo={network.logo} //TODO: dynamic logo storage
						style={styles.logo}
					/>
				</View>

				<View style={styles.desc}>
					<AccountPrefixedTitle title={network.title} />
				</View>
				<NetworkFooter color={network.color} />
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
		height: 80,
		marginLeft: 8,
		width: 4
	},
	icon: {
		height: 40,
		width: 40
	},
	logo: {
		alignItems: 'center',
		height: 36,
		justifyContent: 'center',
		marginHorizontal: 2,
		opacity: 0.7,
		width: 36
	}
});
