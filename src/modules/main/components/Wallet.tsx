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
import { BackHandler, FlatList, FlatListProps } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';
import { useFocusEffect } from '@react-navigation/native';

import { NetworkCard } from './NetworkCard';

import TouchableItem from 'components/TouchableItem';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import AccountPrefixedTitle from 'components/AccountPrefixedTitle';
import { IdentityHeading } from 'components/ScreenHeading';
import {
	SubstrateNetworkKeys,
	UnknownNetworkKeys
} from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
import { NetworksContext } from 'stores/NetworkContext';
import colors from 'styles/colors';
import {
	isEthereumNetworkParams,
	isSubstrateNetworkParams,
	NetworkParams
} from 'types/networkTypes';
import { NavigationAccountIdentityProps } from 'types/props';
import { withCurrentIdentity } from 'utils/HOC';
import { getExistedNetworkKeys, getIdentityName } from 'utils/identitiesUtils';
import { navigateToAddToPolkadotJs } from 'utils/navigationHelpers';
import NavigationTab from 'components/NavigationTab';

const filterNetworks = (
	networkList: Map<string, NetworkParams>,
	extraFilter?: (networkKey: string, shouldExclude: boolean) => boolean
): Array<[string, NetworkParams]> => {
	const excludedNetworks = [UnknownNetworkKeys.UNKNOWN];
	if (!__DEV__) {
		excludedNetworks.push(SubstrateNetworkKeys.SUBSTRATE_DEV);
		excludedNetworks.push(SubstrateNetworkKeys.KUSAMA_DEV);
	}

	const filterNetworkKeys = ([networkKey]: [string, any]): boolean => {
		const shouldExclude = excludedNetworks.includes(networkKey);
		if (extraFilter !== undefined)
			return extraFilter(networkKey, shouldExclude);
		return !shouldExclude;
	};
	return Array.from(networkList.entries())
		.filter(filterNetworkKeys)
		.sort((a, b) => a[1].order - b[1].order);
};

function Wallet({
	accountsStore,
	navigation
}: NavigationAccountIdentityProps<'Main'>): React.ReactElement {
	const { identities, currentIdentity } = accountsStore.state;
	const networkContextState = useContext(NetworksContext);
	const { allNetworks } = networkContextState;
	// catch android back button and prevent exiting the app
	useFocusEffect(
		React.useCallback((): any => {
			const handleBackButton = (): boolean => true;
			const backHandler = BackHandler.addEventListener(
				'hardwareBackPress',
				handleBackButton
			);
			return (): void => backHandler.remove();
		}, [])
	);

	const getListOptions = (): Partial<FlatListProps<any>> => {
		return {
			ListFooterComponent: (
				<>
					<TouchableItem
						onPress={(): void => navigation.navigate('SignTx')}
						style={{
							display: 'flex',
							flexDirection: 'row'
						}}
					>
						<Icon name="add" color={colors.text.main} size={30} />
						<AccountPrefixedTitle title="Sign a polkadot-js transaction" />
					</TouchableItem>
					<TouchableItem
						onPress={(): void => navigation.navigate('AddNetwork')}
						style={{
							display: 'flex',
							flexDirection: 'row'
						}}
					>
						<Icon name="add" color={colors.text.main} size={30} />
						<AccountPrefixedTitle title="Add a network" />
					</TouchableItem>
				</>
			)
		};
	};

	const onNetworkChosen = async (
		networkKey: string,
		networkParams: NetworkParams
	): Promise<void> => {
		if (isSubstrateNetworkParams(networkParams)) {
			// navigate to substrate account
			const { pathId } = networkParams;
			const fullPath = `//${pathId}`;
			navigateToAddToPolkadotJs(navigation, networkKey, fullPath);
		} else {
			// navigate to ethereum account
			navigateToAddToPolkadotJs(navigation, networkKey, networkKey);
		}
	};

	const availableNetworks = useMemo(
		() => getExistedNetworkKeys(currentIdentity, networkContextState),
		[currentIdentity, networkContextState]
	);

	const networkList = useMemo(
		() =>
			filterNetworks(allNetworks, networkKey => {
				return availableNetworks.includes(networkKey);
			}),
		[availableNetworks, allNetworks]
	);

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
			<NetworkCard
				key={networkKey}
				testID={testIDs.Main.networkButton + networkIndexSuffix}
				networkKey={networkKey}
				onPress={(): Promise<void> =>
					onNetworkChosen(networkKey, networkParams)
				}
				title={networkParams.title}
			/>
		);
	};

	return (
		<SafeAreaViewContainer>
			<IdentityHeading title={getIdentityName(currentIdentity, identities)} />
			<FlatList
				data={networkList}
				keyExtractor={(item: [string, NetworkParams]): string => item[0]}
				renderItem={renderNetwork}
				testID={testIDs.Main.chooserScreen}
				{...getListOptions()}
			/>
			<NavigationTab />
		</SafeAreaViewContainer>
	);
}

export default withCurrentIdentity(Wallet);
