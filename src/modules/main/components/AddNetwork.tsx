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

import React, { ReactElement, useContext, useMemo, useState } from 'react';
import { BackHandler, FlatList, FlatListProps } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';
import { useFocusEffect } from '@react-navigation/native';

import { NetworkCard } from './NetworkCard';

import TouchableItem from 'components/TouchableItem';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import AccountPrefixedTitle from 'components/AccountPrefixedTitle';
import ScreenHeading, { IdentityHeading } from 'components/ScreenHeading';
import {
	SubstrateNetworkKeys,
	UnknownNetworkKeys
} from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
import { AlertStateContext } from 'stores/alertContext';
import { NetworksContext } from 'stores/NetworkContext';
import colors from 'styles/colors';
import {
	isEthereumNetworkParams,
	isSubstrateNetworkParams,
	NetworkParams
} from 'types/networkTypes';
import { NavigationAccountIdentityProps } from 'types/props';
import { alertPathDerivationError } from 'utils/alertUtils';
import { withCurrentIdentity } from 'utils/HOC';
import { getExistedNetworkKeys, getIdentityName } from 'utils/identitiesUtils';
import {
	resetNavigationTo,
	navigateToAddToPolkadotJs
} from 'utils/navigationHelpers';
import { useSeedRef } from 'utils/seedRefHooks';
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

function AddNetwork({
	accountsStore,
	navigation,
	route
}: NavigationAccountIdentityProps<'Main'>): React.ReactElement {
	const isNew = route.params?.isNew ?? false;
	const [isAddingNetwork, setIsAddingNetwork] = useState(false);
	const { identities, currentIdentity } = accountsStore.state;
	const networkContextState = useContext(NetworksContext);
	const { getSubstrateNetwork, allNetworks } = networkContextState;
	const seedRefHooks = useSeedRef(currentIdentity.encryptedSeed);

	const { setAlert } = useContext(AlertStateContext);
	// catch android back button and prevent exiting the app
	useFocusEffect(
		React.useCallback((): any => {
			const handleBackButton = (): boolean => {
				if (isAddingNetwork) {
					setIsAddingNetwork(false);
					return true;
				} else {
					return false;
				}
			};
			const backHandler = BackHandler.addEventListener(
				'hardwareBackPress',
				handleBackButton
			);
			return (): void => backHandler.remove();
		}, [isAddingNetwork])
	);

	const getListOptions = (): Partial<FlatListProps<any>> => {
		if (isNew) return {};
		return {
			ListFooterComponent: (
				<>
					<TouchableItem
						onPress={(): void => resetNavigationTo(navigation, 'SignTx')}
						style={{
							display: 'flex',
							flexDirection: 'row'
						}}
					>
						<Icon name="add" color={colors.text.main} size={30} />
						<AccountPrefixedTitle title="Sign a polkadot-js transaction" />
					</TouchableItem>
					<TouchableItem
						onPress={(): void => setIsAddingNetwork(true)}
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
		if (isNew || isAddingNetwork) {
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
					alertPathDerivationError(setAlert, error.message);
				}
				resetNavigationTo(navigation, 'Main');
			} else {
				// derive ethereum account
				try {
					await accountsStore.deriveEthereumAccount(
						seedRefHooks.brainWalletAddress,
						networkKey,
						allNetworks
					);
				} catch (e) {
					alertPathDerivationError(setAlert, e.message);
				}
				resetNavigationTo(navigation, 'Main');
			}
		} else {
			if (isSubstrateNetworkParams(networkParams)) {
				// navigate to substrate account
				const { pathId } = networkParams;
				const fullPath = `//${pathId}`;
				navigateToAddToPolkadotJs(navigation, networkKey, fullPath);
			} else {
				// navigate to ethereum account
				navigateToAddToPolkadotJs(navigation, networkKey, networkKey);
			}
		}
	};

	const availableNetworks = useMemo(
		() => getExistedNetworkKeys(currentIdentity, networkContextState),
		[currentIdentity, networkContextState]
	);

	const networkList = useMemo(
		() =>
			filterNetworks(allNetworks, (networkKey, shouldExclude) => {
				if (isNew && !shouldExclude) return true;

				if (isAddingNetwork) {
					if (shouldExclude) return false;
					return !availableNetworks.includes(networkKey);
				}
				return availableNetworks.includes(networkKey);
			}),
		[availableNetworks, isNew, isAddingNetwork, allNetworks]
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
			{isNew ? (
				<ScreenHeading title={'Select a network'} />
			) : isAddingNetwork ? (
				<IdentityHeading
					title={'Add a network'}
					onPressBack={(): void => setIsAddingNetwork(false)}
				/>
			) : (
				<IdentityHeading title={getIdentityName(currentIdentity, identities)} />
			)}
			<FlatList
				data={networkList}
				keyExtractor={(item: [string, NetworkParams]): string => item[0]}
				renderItem={renderNetwork}
				testID={testIDs.Main.chooserScreen}
				{...(!isAddingNetwork && !isNew && getListOptions())}
			/>
			{!isAddingNetwork && !isNew && <NavigationTab />}
		</SafeAreaViewContainer>
	);
}

export default withCurrentIdentity(AddNetwork);
