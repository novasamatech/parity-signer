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
import { StyleSheet, View } from 'react-native';

import AccountIcon from 'components/AccountIcon';
import TouchableItem from 'components/TouchableItem';
import AccountPrefixedTitle from 'components/AccountPrefixedTitle';
import { NetworksContext } from 'stores/NetworkContext';
import Separator from 'components/Separator';
import { ButtonListener } from 'types/props';

export function AddNetworkCard({
	networkKey,
	onPress,
	testID,
	title
}: {
	networkKey?: string;
	onPress?: ButtonListener;
	testID?: string;
	title: string;
}): ReactElement {
	const { getNetwork } = useContext(NetworksContext);
	const networkParams = getNetwork(networkKey ?? '');
	const isDisabled = onPress === undefined;
	return (
		<TouchableItem testID={testID} disabled={isDisabled} onPress={onPress}>
			<Separator
				style={{
					backgroundColor: 'transparent',
					height: 0,
					marginVertical: 0
				}}
			/>
			<View style={styles.content}>
				<AccountIcon address={''} network={networkParams} style={styles.icon} />
				<View style={styles.desc}>
					<AccountPrefixedTitle title={title} />
				</View>
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
	}
});
