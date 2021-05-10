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

import React, { ReactElement, useContext, useMemo, useState } from 'react';
import { BackHandler, FlatList, FlatListProps } from 'react-native';
import { useFocusEffect } from '@react-navigation/native';

import { filterNetworks } from 'modules/network/utils';
import { NetworkCard } from 'components/AccountCard';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import ScreenHeading, { IdentityHeading } from 'components/ScreenHeading';
import testIDs from 'e2e/testIDs';
import { AlertStateContext } from 'stores/alertContext';
import { NetworksContext } from 'stores/NetworkContext';
import colors from 'styles/colors';
import {
	isEthereumNetworkParams,
	isSubstrateNetworkParams,
	NetworkParams,
	SubstrateNetworkParams
} from 'types/networkTypes';
import { NavigationAccountIdentityProps } from 'types/props';
import { alertPathDerivationError } from 'utils/alertUtils';
import { withCurrentIdentity } from 'utils/HOC';
import { getExistedNetworkKeys, getIdentityName } from 'utils/identitiesUtils';
import {
	navigateToPathDetails,
	navigateToPathsList,
	unlockSeedPhrase,
	useUnlockSeed
} from 'utils/navigationHelpers';
import { useSeedRef } from 'utils/seedRefHooks';
import QrScannerTab from 'components/QrScannerTab';

function NetworkSelector({
	accountsStore,
	navigation,
	route
}: NavigationAccountIdentityProps<'Main'>): React.ReactElement {
	const isNew = route.params?.isNew ?? false;
	const [shouldShowMoreNetworks, setShouldShowMoreNetworks] = useState(false);
	const { identities, currentIdentity } = accountsStore.state;
	const networkContextState = useContext(NetworksContext);
	const { getSubstrateNetwork, allNetworks } = networkContextState;
	const seedRefHooks = useSeedRef(currentIdentity.encryptedSeed);
	const { unlockWithoutPassword } = useUnlockSeed(seedRefHooks.isSeedRefValid);

	const { setAlert } = useContext(AlertStateContext);
	// catch android back button and prevent exiting the app
	// TODO: this just doesn't work and nobody noticed, let's fix later
	useFocusEffect(
		React.useCallback((): any => {
			const handleBackButton = (): boolean => {
				if (shouldShowMoreNetworks) {
					setShouldShowMoreNetworks(false);
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
		}, [shouldShowMoreNetworks])
	);

	const onAddCustomPath = (): Promise<void> =>
		unlockWithoutPassword({
			name: 'PathDerivation',
			params: { parentPath: '' }
		});

	const deriveSubstrateNetworkRootPath = async (
		networkKey: string,
		networkParams: SubstrateNetworkParams
	): Promise<void> => {
		const { pathId } = networkParams;
		await unlockSeedPhrase(navigation, seedRefHooks.isSeedRefValid);
		const fullPath = `//${pathId}`;
		try {
			await accountsStore.deriveNewPath(
				fullPath,
				seedRefHooks.substrateAddress,
				getSubstrateNetwork(networkKey),
				`${networkParams.title} root`,
				''
			);
			navigateToPathDetails(navigation, networkKey, fullPath);
		} catch (error) {
			alertPathDerivationError(setAlert, error.message);
		}
	};

	const deriveEthereumAccount = async (networkKey: string): Promise<void> => {
		await unlockSeedPhrase(navigation, seedRefHooks.isSeedRefValid);
		try {
			await accountsStore.deriveEthereumAccount(
				seedRefHooks.brainWalletAddress,
				networkKey,
				allNetworks
			);
			navigateToPathsList(navigation, networkKey);
		} catch (e) {
			alertPathDerivationError(setAlert, e.message);
		}
	};

	const getListOptions = (): Partial<FlatListProps<any>> => {
		if (isNew) return {};
		if (shouldShowMoreNetworks) {
			return {
				ListHeaderComponent: (
					<NetworkCard
						isAdd={true}
						onPress={onAddCustomPath}
						testID={testIDs.Main.addCustomNetworkButton}
						title="Create Custom Path"
						networkColor={colors.background.app}
					/>
				)
			};
		} else {
			return {
				ListFooterComponent: (
					<NetworkCard
						isAdd={true}
						onPress={(): void => setShouldShowMoreNetworks(true)}
						testID={testIDs.Main.addNewNetworkButton}
						title="Add Network Account"
						networkColor={colors.background.app}
					/>
				)
			};
		}
	};

	const renderScreenHeading = (): React.ReactElement => {
		if (isNew) {
			return <ScreenHeading title={'Create your first Keypair'} />;
		} else if (shouldShowMoreNetworks) {
			return (
				<IdentityHeading
					title={'Choose Network'}
					onPressBack={(): void => setShouldShowMoreNetworks(false)}
				/>
			);
		} else {
			const identityName = getIdentityName(currentIdentity, identities);
			return <IdentityHeading title={identityName} />;
		}
	};

	const onNetworkChosen = async (
		networkKey: string,
		networkParams: NetworkParams
	): Promise<void> => {
		if (isNew || shouldShowMoreNetworks) {
			if (isSubstrateNetworkParams(networkParams)) {
				await deriveSubstrateNetworkRootPath(networkKey, networkParams);
			} else {
				await deriveEthereumAccount(networkKey);
			}
		} else {
			navigation.navigate('PathsList', { networkKey });
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

				if (shouldShowMoreNetworks) {
					if (shouldExclude) return false;
					return !availableNetworks.includes(networkKey);
				}
				return availableNetworks.includes(networkKey);
			}),
		[availableNetworks, isNew, shouldShowMoreNetworks, allNetworks]
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
			{renderScreenHeading()}
			<FlatList
				bounces={false}
				data={networkList}
				keyExtractor={(item: [string, NetworkParams]): string => item[0]}
				renderItem={renderNetwork}
				testID={testIDs.Main.chooserScreen}
				{...getListOptions()}
			/>
			{!shouldShowMoreNetworks && !isNew && <QrScannerTab />}
		</SafeAreaViewContainer>
	);
}

export default withCurrentIdentity(NetworkSelector);
