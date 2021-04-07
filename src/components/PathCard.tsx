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

import Clipboard from '@react-native-community/clipboard';
import React, { useContext, useEffect, useState } from 'react';
import { StyleSheet, View } from 'react-native';
import { showMessage } from 'react-native-flash-message';

import AccountIcon from './AccountIcon';
import AccountPrefixedTitle from './AccountPrefixedTitle';
import Address from './Address';
import TouchableItem from './TouchableItem';

import { colors, fontStyles } from 'styles';
import { NetworksContext } from 'stores/NetworkContext';
import {
	defaultNetworkKey,
	NETWORK_LIST,
	UnknownNetworkKeys,
	NetworkProtocols
} from 'constants/networkSpecs';
import { Identity } from 'types/identityTypes';
import { getAddressWithPath, getNetworkKeyByPath } from 'utils/identitiesUtils';
import { useSeedRef } from 'utils/seedRefHooks';

export default function PathCard({
	identity,
	isPathValid = true,
	path,
	networkKey,
	testID,
	titlePrefix
}: {
	identity: Identity;
	isPathValid?: boolean;
	path: string;
	networkKey?: string;
	testID?: string;
	titlePrefix?: string;
}): React.ReactElement {
	const networksContext = useContext(NetworksContext);
	const { networks, allNetworks } = networksContext;
	const { isSeedRefValid, substrateAddress } = useSeedRef(
		identity.encryptedSeed
	);
	const [address, setAddress] = useState('');
	const computedNetworkKey =
		networkKey ||
		getNetworkKeyByPath(path, identity.meta.get(path)!, networksContext);
	useEffect(() => {
		const getAddress = async (): Promise<void> => {
			const existedAddress = getAddressWithPath(path, identity);
			if (existedAddress !== '') return setAddress(existedAddress);
			if (isSeedRefValid && isPathValid && networks.has(computedNetworkKey)) {
				const prefix = networks.get(computedNetworkKey)!.prefix;
				const generatedAddress = await substrateAddress(path, prefix);
				return setAddress(generatedAddress);
			}
			setAddress('');
		};
		getAddress();
	}, [
		path,
		identity,
		isPathValid,
		networkKey,
		computedNetworkKey,
		isSeedRefValid,
		substrateAddress,
		networks
	]);

	const isUnknownAddress = address === '';
	const networkParams =
		computedNetworkKey === UnknownNetworkKeys.UNKNOWN &&
		!isUnknownAddress &&
		!allNetworks.has(computedNetworkKey)
			? NETWORK_LIST[defaultNetworkKey]
			: allNetworks.get(computedNetworkKey)!;

	return (nonSubstrateCard = (
		<TouchableItem
			accessibilityComponentType="button"
			onPress={(): void => {
				showMessage('Address copied.');
				Clipboard.setString(
					(networkParams.protocol === NetworkProtocols.ETHEREUM ? '0x' : '') +
						address
				);
			}}
			style={styles.body}
		>
			<View style={styles.content} testID={testID}>
				<AccountIcon
					address={address}
					network={networkParams}
					style={styles.identicon}
				/>
				<View style={styles.desc}>
					<AccountPrefixedTitle title={networkParams.title} />
					<Address address={address} protocol={networkParams.protocol} />
				</View>
			</View>
		</TouchableItem>
	));
}

const styles = StyleSheet.create({
	body: {
		borderBottomWidth: 1,
		borderColor: colors.background.app,
		borderTopWidth: 1
	},
	content: {
		alignItems: 'center',
		backgroundColor: colors.background.card,
		flexDirection: 'row',
		paddingLeft: 16,
		paddingVertical: 8
	},
	desc: {
		flex: 1,
		paddingHorizontal: 16
	},
	footer: {
		height: 80,
		marginLeft: 8,
		width: 4
	},
	iconLock: {
		marginLeft: 4,
		...fontStyles.h2
	},
	identicon: {
		height: 40,
		width: 40
	},
	row: {
		alignItems: 'flex-end',
		flexDirection: 'row'
	}
});
