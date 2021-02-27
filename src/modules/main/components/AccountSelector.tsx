// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Modifications Copyright (c) 2021 Thibaut Sardan

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

import { useFocusEffect } from '@react-navigation/native';
import { NetworkCard } from 'components/NetworkCard';
import QrScannerTab from 'components/QrScannerTab';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import ScreenHeading, { IdentityHeading } from 'components/ScreenHeading';
import testIDs from 'e2e/testIDs';
import { filterNetworks } from 'modules/network/utils';
import React, { ReactElement, useContext, useMemo, useState } from 'react';
import { BackHandler, FlatList, FlatListProps } from 'react-native';
import { isEthereumNetwork, isSubstrateNetwork, NetworkParams, SubstrateNetworkParams } from 'types/networkTypes';
import { NavigationAccountIdentityProps } from 'types/props';
import { alertPathDerivationError } from 'utils/alertUtils';
import { withCurrentIdentity } from 'utils/HOC';
import { getExistedNetworkKeys, getIdentityName } from 'utils/identitiesUtils';
import { navigateToPathDetails, navigateToPathsList, unlockSeedPhrase, useUnlockSeed } from 'utils/navigationHelpers';
import { useSeedRef } from 'utils/seedRefHooks';

import { AlertContext, NetworksContext } from '../../../context';

function AccountSelector({ accountsStore, navigation, route }: NavigationAccountIdentityProps<'Main'>): React.ReactElement {
	const isNew = route.params?.isNew ?? false;
	const [shouldShowMoreNetworks, setShouldShowMoreNetworks] = useState(false);
	const { currentIdentity, identities } = accountsStore.state;
	const networkContextState = useContext(NetworksContext);
	const { allNetworks, getSubstrateNetwork } = networkContextState;
	const { brainWalletAddress, isSeedRefValid, substrateAddress } = useSeedRef(currentIdentity.encryptedSeed);
	const { unlockWithoutPassword } = useUnlockSeed(isSeedRefValid);

	const { setAlert } = useContext(AlertContext);

	// catch android back button and prevent exiting the app
	useFocusEffect(React.useCallback((): any => {
		const handleBackButton = (): boolean => {
			if (shouldShowMoreNetworks) {
				setShouldShowMoreNetworks(false);

				return true;
			} else {
				return false;
			}
		};

		const backHandler = BackHandler.addEventListener('hardwareBackPress', handleBackButton);

		return (): void => backHandler.remove();
	}, [shouldShowMoreNetworks]));

	const onAddCustomPath = (): Promise<void> =>
		unlockWithoutPassword({ name: 'PathDerivation', params: { parentPath: '' } });

	const deriveSubstrateNetworkRootPath = async (networkKey: string,
		networkParams: SubstrateNetworkParams): Promise<void> => {
		const { pathId } = networkParams;

		await unlockSeedPhrase(navigation, isSeedRefValid);
		const fullPath = `//${pathId}`;

		try {
			await accountsStore.deriveNewPath(fullPath,
				substrateAddress,
				getSubstrateNetwork(networkKey),
				`${networkParams.title} root`,
				'');
			navigateToPathDetails(navigation, networkKey, fullPath);
		} catch (error) {
			alertPathDerivationError(setAlert, error.message);
		}
	};

	const deriveEthereumAccount = async (networkKey: string): Promise<void> => {
		await unlockSeedPhrase(navigation, isSeedRefValid);

		try {
			await accountsStore.deriveEthereumAccount(brainWalletAddress, networkKey);
			navigateToPathsList(navigation, networkKey);
		} catch (e) {
			alertPathDerivationError(setAlert, e.message);
		}
	};

	const getListOptions = (): Partial<FlatListProps<any>> => {
		// if (isNew) return {};
		// if (shouldShowMoreNetworks) {
		return {
			ListHeaderComponent: (
				<NetworkCard
					isAdd={true}
					onPress={onAddCustomPath}
					testID={testIDs.Main.addCustomNetworkButton}
					title="Create Custom Path"
				/>
			)
		};
		// }
		// else {
		// 	return {
		// 		ListFooterComponent: (
		// 			<NetworkCard
		// 				isAdd={true}
		// 				onPress={(): void => setShouldShowMoreNetworks(true)}
		// 				testID={testIDs.Main.addNewNetworkButton}
		// 				title="Add Network Account"
		// 			/>
		// 		)
		// 	};
		// }
	};

	const renderScreenHeading = (): React.ReactElement => {
		if (isNew) {
			return <ScreenHeading title={'Create your first Keypair'} />;
		} else if (shouldShowMoreNetworks) {
			return (
				<IdentityHeading
					onPressBack={(): void => setShouldShowMoreNetworks(false)}
					title={'Choose Network'}
				/>
			);
		} else {
			const identityName = getIdentityName(currentIdentity, identities);

			return <IdentityHeading title={identityName} />;
		}
	};

	const onNetworkChosen = async (networkKey: string,
		networkParams: NetworkParams): Promise<void> => {
		if (isNew || shouldShowMoreNetworks) {
			if (isSubstrateNetwork(networkParams)) {
				await deriveSubstrateNetworkRootPath(networkKey, networkParams);
			} else {
				await deriveEthereumAccount(networkKey);
			}
		} else {
			navigation.navigate('PathsList', { networkKey });
		}
	};

	const availableNetworks = useMemo(() => {
		const networks = getExistedNetworkKeys(currentIdentity, networkContextState)

		return networks;
	},
	[currentIdentity, networkContextState]);

	const networkList = useMemo(() =>
		filterNetworks(allNetworks, (networkKey, shouldExclude) => {
			if (isNew && !shouldExclude) return true;

			if (shouldShowMoreNetworks) {
				if (shouldExclude) return false;

				return !availableNetworks.includes(networkKey);
			}

			return availableNetworks.includes(networkKey);
		}),
	[availableNetworks, isNew, shouldShowMoreNetworks, allNetworks]);

	// interface AccountInfo {
	// 		address: string;
	// 		network?: string;
	// }

	// Identity
	// {
	// 	"addresses": Map {
	// 		"GiFE7t56Dc4cWZoitYkFaJ6pKSVr36EATztX5LX45X7jSHy" => "",
	// 		"15XpV4xc3aFFVgsfMSQsUszWdkVjdjs7AzG94ygVmX32whhg" => "//some/1"
	// 	},
	// 	"derivationPassword": "",
	// 	"encryptedSeed": "{\"cipher\":\"aes-128-ctr\",\"cipherparams\":{\"iv\":\"7039494bdca0c839c58d538a554d705e\"},\"ciphertext\":\"6ad656d7cd094893ca27e01ed80cc4c85784dd43ed4639555af5fbcc979465fe178dfc43e3ffd9a39ed55f3f9d385026e63841a91bcf0d83246bd0eaddf709a032fde35b9ecbf2\",\"kdf\":\"pbkdf2\",\"kdfparams\":{\"c\":10240,\"dklen\":32,\"prf\":\"hmac-sha256\",\"salt\":\"aea3b3cc027a08877a70200f79fcc560e7d2f748da7dfd934475e5aaf9841b02\"},\"mac\":\"459152615d894a1ef68572323dea992d66c80758ce6fe89ab01a34d65d05d8cc\"}",
	// 	"meta": Map {
	// 		"" => {
	// 			"address": "GiFE7t56Dc4cWZoitYkFaJ6pKSVr36EATztX5LX45X7jSHy",
	// 			"createdAt": 1612624676279,
	// 			"hasPassword": false,
	// 			"name": "",
	// 			"networkPathId": "kusama",
	// 			"updatedAt": 1612624676279
	// 		},
	//      "//some/1" => {
	// 			"address": "15XpV4xc3aFFVgsfMSQsUszWdkVjdjs7AzG94ygVmX32whhg",
	// 			"createdAt": 1612627886130,
	// 			"hasPassword": false,
	// 			"name": "",
	// 			"networkPathId": "polkadot",
	// 			"updatedAt": 1612627886130
	// 		}
	// 	},
	// 	"name": "bla"
	// }

	// const accountList = useMemo(()=> {
	// 	return identities.map((id): AccountInfo | null => {
	// 		const addresses = Array.from(id.addresses);

	// 		if (!addresses.length){
	// 			return null;
	// 		}

	// 		// select the first account from the identity.address, the key from the map is the address
	// 		const address = Array.from(id.addresses)[0][0];
	// 		// select the first account from the identity.map, the value from the map has a networkPathId
	// 		const network = Array.from(id.meta)[0][1].networkPathId;

	// 		return {
	// 			address,
	// 			network
	// 		}
	// 	});
	// }, [identities])

	// const renderAccount = ({ item }: { item: [string, NetworkParams] }): ReactElement => {
	// 	const [networkKey, networkParams] = item;
	// 	const networkIndexSuffix = isEthereumNetworkParams(networkParams)
	// 		? networkParams.ethereumChainId
	// 		: networkParams.pathId;

	// 	return (
	// 		<CompatibleCard
	// 			key={networkKey}
	// 			networkKey={networkKey}
	// 			onPress={(): Promise<void> =>
	// 				onNetworkChosen(networkKey, networkParams)
	// 			}
	// 			testID={testIDs.Main.networkButton + networkIndexSuffix}
	// 			title={networkParams.title}
	// 		/>
	// 	);
	// };

	const renderNetwork = ({ item }: { item: [string, NetworkParams] }): ReactElement => {
		const [networkKey, networkParams] = item;
		const networkIndexSuffix = isEthereumNetwork(networkParams)
			? networkParams.ethereumChainId
			: networkParams.pathId;

		return (
			<NetworkCard
				key={networkKey}
				networkKey={networkKey}
				onPress={(): Promise<void> =>
					onNetworkChosen(networkKey, networkParams)
				}
				testID={testIDs.Main.networkButton + networkIndexSuffix}
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

export default withCurrentIdentity(AccountSelector);
