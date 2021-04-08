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

import React, {
	ReactElement,
	useContext,
	useEffect,
	useMemo,
	useState
} from 'react';
import { View, BackHandler, FlatList, FlatListProps } from 'react-native';
import { useFocusEffect } from '@react-navigation/native';
import BN from 'bn.js';

import { NetworkCard } from '../components/NetworkCard';
import OnBoardingView from '../components/OnBoarding';
import NoCurrentIdentity from '../components/NoCurrentIdentity';

import { components } from 'styles/index';
import { UnknownNetworkKeys } from 'constants/networkSpecs';
import { NetworksContext } from 'stores/NetworkContext';
import { AccountsContext } from 'stores/AccountsContext';
import testIDs from 'e2e/testIDs';
import {
	isEthereumNetworkParams,
	isSubstrateNetworkParams,
	NetworkParams
} from 'types/networkTypes';
import { NavigationProps } from 'types/props';
import {
	getAddressWithPath,
	getExistedNetworkKeys
} from 'utils/identitiesUtils';
import { navigateToReceiveBalance } from 'utils/navigationHelpers';
import Button from 'components/Button';
import NavigationTab from 'components/NavigationTab';
import { ApiContext } from 'stores/ApiContext';
import { RegistriesContext } from 'stores/RegistriesContext';

const filterNetworks = (
	networkList: Map<string, NetworkParams>,
	extraFilter?: (networkKey: string, shouldExclude: boolean) => boolean
): Array<[string, NetworkParams]> => {
	const excludedNetworks = [UnknownNetworkKeys.UNKNOWN];
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

interface State {
	freeBalance: string;
}

const EMPTY_STATE: State = {
	freeBalance: 'Loading...'
};

function Wallet({ navigation }: NavigationProps<'Wallet'>): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const { identities, currentIdentity, loaded } = accountsStore.state;
	const networkContextState = useContext(NetworksContext);
	const registriesContext = useContext(RegistriesContext);
	const { selectNetwork, state, disconnect } = useContext(ApiContext);
	const [balance, setBalance] = useState(EMPTY_STATE);
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

	const availableNetworks = useMemo(
		() =>
			currentIdentity
				? getExistedNetworkKeys(currentIdentity, networkContextState)
				: [],
		[currentIdentity, networkContextState]
	);

	const networkList = useMemo(
		() =>
			filterNetworks(allNetworks, networkKey => {
				return availableNetworks.includes(networkKey);
			}),
		[availableNetworks, allNetworks]
	);

	// initialize the API using the first network the user has, if they have any
	const firstNetwork = networkList[0];

	// TODO: fix this hook!!!
	useEffect((): void => {
		if (!firstNetwork) return;
		const [networkKey, networkParams] = firstNetwork;

		if (!isSubstrateNetworkParams(networkParams)) return;
		if (networkKey !== state.apiNetworkKey) {
			console.log('unequal network keys!');
			disconnect();
			selectNetwork(networkKey, networkContextState, registriesContext);
		} else if (state.apiError) {
			// TODO: is this error handling code correct?
			console.error(`API ERROR: ${state.apiError}`);
		} else if (state.isApiReady) {
			console.log('API READY!');
			const path = `//${networkParams.pathId}`;
			const address = getAddressWithPath(path, currentIdentity);
			if (state.api?.query?.balances) {
				console.log('FETCHING BALANCES...');
				state.api.query.balances.account(address).then(fetchedBalance => {
					console.log('BALANCES FETCHED!');
					const base = new BN(10).pow(new BN(networkParams.decimals));
					const div = fetchedBalance.free.div(base);
					const mod = fetchedBalance.free.mod(base);
					const nDisplayDecimals = 3;
					setBalance({
						freeBalance: div + '.' + mod.toString(10, nDisplayDecimals)
					});
				});
			}
		}
		return;
	}, [currentIdentity, firstNetwork, state]);

	if (!loaded) return <View />;
	if (identities.length === 0) return <OnBoardingView />;
	if (currentIdentity === null) return <NoCurrentIdentity />;

	const getListOptions = (): Partial<FlatListProps<any>> => {
		return {
			ListFooterComponent: (
				<View style={{ marginBottom: 12, paddingHorizontal: 15 }}>
					<Button
						title="Switch network"
						onPress={(): void => navigation.navigate('AddNetwork')}
						fluid={true}
					/>
				</View>
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
			navigateToReceiveBalance(navigation, networkKey, fullPath);
		} else {
			// navigate to ethereum account
			navigateToReceiveBalance(navigation, networkKey, networkKey);
		}
	};

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
				testID={testIDs.Wallet.networkButton + networkIndexSuffix}
				networkKey={networkKey}
				onPress={(): Promise<void> =>
					onNetworkChosen(networkKey, networkParams)
				}
				balance={balance.freeBalance}
				title={networkParams.title}
			/>
		);
	};

	return (
		<>
			<View style={components.pageWide}>
				<FlatList
					data={networkList}
					keyExtractor={(item: [string, NetworkParams]): string => item[0]}
					renderItem={renderNetwork}
					testID={testIDs.Wallet.chooserScreen}
					{...getListOptions()}
				/>
			</View>
			<NavigationTab />
		</>
	);
}

export default Wallet;
