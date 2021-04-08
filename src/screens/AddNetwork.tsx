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

import React, { ReactElement, useContext, useMemo } from 'react';
import { View, FlatList } from 'react-native';
import { showMessage } from 'react-native-flash-message';

import { components } from 'styles';
import Separator from 'components/Separator';
import { AddNetworkCard } from 'components/AddNetworkCard';
import { UnknownNetworkKeys } from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
import { NetworksContext } from 'stores/NetworkContext';
import {
	isEthereumNetworkParams,
	isSubstrateNetworkParams,
	NetworkParams
} from 'types/networkTypes';
import { NavigationAccountIdentityProps } from 'types/props';
import { withCurrentIdentity } from 'utils/HOC';
import { getExistedNetworkKeys } from 'utils/identitiesUtils';
import { resetNavigationTo } from 'utils/navigationHelpers';
import { useSeedRef } from 'utils/seedRefHooks';

function AddNetwork({
	accountsStore,
	navigation,
	route
}: NavigationAccountIdentityProps<'AddNetwork'>): React.ReactElement {
	const isNew = route.params?.isNew ?? false;
	const { currentIdentity } = accountsStore.state;
	const networkContextState = useContext(NetworksContext);
	const { getSubstrateNetwork, allNetworks } = networkContextState;
	const seedRefHooks = useSeedRef(currentIdentity.encryptedSeed);

	const onNetworkChosen = async (
		networkKey: string,
		networkParams: NetworkParams
	): Promise<void> => {
		if (isSubstrateNetworkParams(networkParams)) {
			// derive substrate account
			const { pathId } = networkParams;
			const fullPath = `//${pathId}`;
			try {
				await accountsStore.deriveNewPath(
					fullPath,
					seedRefHooks.substrateAddress,
					getSubstrateNetwork(networkKey),
					`${networkParams.title} root`
				);
			} catch (error) {
				showMessage(
					'Could not derive a valid account from the seed: ' + error.message
				);
			}
			resetNavigationTo(navigation, 'Wallet');
		} else {
			// derive ethereum account
			try {
				await accountsStore.deriveEthereumAccount(
					seedRefHooks.brainWalletAddress,
					networkKey,
					allNetworks
				);
			} catch (error) {
				showMessage(
					'Could not derive a valid account from the seed: ' + error.message
				);
			}
			resetNavigationTo(navigation, 'Wallet');
		}
	};

	const availableNetworks = useMemo(
		() => getExistedNetworkKeys(currentIdentity, networkContextState),
		[currentIdentity, networkContextState]
	);

	const networkList = Array.from(allNetworks.entries())
		.filter(([networkKey, network]) => {
			if (networkKey === UnknownNetworkKeys.UNKNOWN) return false;
			if (isNew) return true;
			if (availableNetworks.includes(networkKey)) return false;
			return true;
		})
		.sort((a, b) => a[1].order - b[1].order);
	const networkListMainnets = networkList.filter(n => !n[1].isTestnet);
	const networkListTestnets = networkList.filter(n => n[1].isTestnet);

	const renderNetwork = ({
		item
	}: {
		item: [string, NetworkParams];
	}): ReactElement => {
		const [networkKey, networkParams] = item;
		const networkIndexSuffix = isEthereumNetworkParams(networkParams)
			? networkParams.ethereumChainId
			: networkParams.pathId;
		return (
			<AddNetworkCard
				key={networkKey}
				testID={testIDs.Wallet.networkButton + networkIndexSuffix}
				networkKey={networkKey}
				onPress={(): Promise<void> =>
					onNetworkChosen(networkKey, networkParams)
				}
				title={networkParams.title}
			/>
		);
	};

	return (
		<View style={components.page}>
			<FlatList
				data={networkListMainnets}
				keyExtractor={(item: [string, NetworkParams]): string => item[0]}
				renderItem={renderNetwork}
			/>
			{networkListMainnets.length > 0 && networkListTestnets.length > 0 && (
				<Separator />
			)}
			<FlatList
				data={networkListTestnets}
				keyExtractor={(item: [string, NetworkParams]): string => item[0]}
				renderItem={renderNetwork}
			/>
		</View>
	);
}

export default withCurrentIdentity(AddNetwork);
