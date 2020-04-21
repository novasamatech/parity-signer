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

import React, { useState } from 'react';
import { ScrollView } from 'react-native';

import { NetworkCard } from 'components/AccountCard';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import ScreenHeading, { IdentityHeading } from 'components/ScreenHeading';
import {
	NETWORK_LIST,
	NetworkProtocols,
	SubstrateNetworkKeys,
	UnknownNetworkKeys
} from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
import colors from 'styles/colors';
import {
	isEthereumNetworkParams,
	isSubstrateNetworkParams,
	isUnknownNetworkParams,
	NetworkParams,
	SubstrateNetworkParams
} from 'types/networkSpecsTypes';
import { NavigationAccountProps } from 'types/props';
import { alertPathDerivationError } from 'utils/alertUtils';
import {
	getExistedNetworkKeys,
	getIdentityName,
	getPathsWithSubstrateNetworkKey
} from 'utils/identitiesUtils';
import {
	navigateToPathDetails,
	navigateToPathsList,
	unlockSeedPhrase
} from 'utils/navigationHelpers';
import { useSeedRef } from 'utils/seedRefHooks';

const excludedNetworks = [
	UnknownNetworkKeys.UNKNOWN,
	SubstrateNetworkKeys.KUSAMA_CC2
];
if (!__DEV__) {
	excludedNetworks.push(SubstrateNetworkKeys.SUBSTRATE_DEV);
	excludedNetworks.push(SubstrateNetworkKeys.KUSAMA_DEV);
}

export default function NetworkSelector({
	accounts,
	navigation,
	route
}: NavigationAccountProps<'Main'>): React.ReactElement {
	const isNew = route.params?.isNew ?? false;
	const [shouldShowMoreNetworks, setShouldShowMoreNetworks] = useState(false);
	const { identities, currentIdentity } = accounts.state;
	const seedRefHooks = useSeedRef(currentIdentity!.encryptedSeed);

	const sortNetworkKeys = (
		[, params1]: [any, NetworkParams],
		[, params2]: [any, NetworkParams]
	): number => {
		if (params1.order > params2.order) {
			return 1;
		} else if (params1.order < params2.order) {
			return -1;
		} else {
			return 0;
		}
	};

	const filterNetworkKeys = ([networkKey]: [string, any]): boolean => {
		const shouldExclude = excludedNetworks.includes(networkKey);
		if (isNew && !shouldExclude) return true;

		if (shouldShowMoreNetworks) {
			if (shouldExclude) return false;
			return !availableNetworks.includes(networkKey);
		}
		return availableNetworks.includes(networkKey);
	};

	const deriveSubstrateNetworkRootPath = async (
		networkKey: string,
		networkParams: SubstrateNetworkParams
	): Promise<void> => {
		const { pathId } = networkParams;
		await unlockSeedPhrase(navigation, seedRefHooks.isSeedRefValid);
		const fullPath = `//${pathId}`;
		try {
			await accounts.deriveNewPath(
				fullPath,
				seedRefHooks.substrateAddress,
				networkKey,
				`${networkParams.title} root`,
				''
			);
			navigateToPathDetails(navigation, networkKey, fullPath);
		} catch (error) {
			alertPathDerivationError(error.message);
		}
	};

	const deriveEthereumAccount = async (networkKey: string): Promise<void> => {
		await unlockSeedPhrase(navigation, seedRefHooks.isSeedRefValid);
		try {
			await accounts.deriveEthereumAccount(
				seedRefHooks.brainWalletAddress,
				networkKey
			);
			navigateToPathsList(navigation, networkKey);
		} catch (e) {
			alertPathDerivationError(e.message);
		}
	};

	const renderCustomPathCard = (): React.ReactElement => (
		<NetworkCard
			isAdd={true}
			onPress={(): void =>
				navigation.navigate('PathDerivation', { parentPath: '' })
			}
			testID={testIDs.Main.addCustomNetworkButton}
			title="Create Custom Path"
			networkColor={colors.bg}
		/>
	);

	const renderAddButton = (): React.ReactElement => {
		if (isNew) return renderCustomPathCard();
		if (!shouldShowMoreNetworks) {
			return (
				<NetworkCard
					isAdd={true}
					onPress={(): void => setShouldShowMoreNetworks(true)}
					testID={testIDs.Main.addNewNetworkButton}
					title="Add Network Account"
					networkColor={colors.bg}
				/>
			);
		} else {
			return renderCustomPathCard();
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
			const identityName = getIdentityName(currentIdentity!, identities);
			return <IdentityHeading title={identityName} />;
		}
	};

	const onNetworkChosen = async (
		networkKey: string,
		networkParams: NetworkParams
	): Promise<void> => {
		if (isNew) {
			if (isSubstrateNetworkParams(networkParams)) {
				await deriveSubstrateNetworkRootPath(networkKey, networkParams);
			} else {
				await deriveEthereumAccount(networkKey);
			}
		} else {
			const paths = Array.from(currentIdentity!.meta.keys());
			if (
				isSubstrateNetworkParams(networkParams) ||
				isUnknownNetworkParams(networkParams)
			) {
				const listedPaths = getPathsWithSubstrateNetworkKey(
					currentIdentity!,
					networkKey
				);
				if (listedPaths.length === 0 && isSubstrateNetworkParams(networkParams))
					return await deriveSubstrateNetworkRootPath(
						networkKey,
						networkParams
					);
			} else if (
				networkParams.protocol === NetworkProtocols.ETHEREUM &&
				!paths.includes(networkKey)
			) {
				return await deriveEthereumAccount(networkKey);
			}
			navigation.navigate('PathsList', { networkKey });
		}
	};

	const availableNetworks = getExistedNetworkKeys(currentIdentity!);
	const networkList = Object.entries(NETWORK_LIST).filter(filterNetworkKeys);
	networkList.sort(sortNetworkKeys);

	return (
		<SafeAreaViewContainer>
			{renderScreenHeading()}
			<ScrollView bounces={false} testID={testIDs.Main.chooserScreen}>
				{networkList.map(([networkKey, networkParams]) => {
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
				})}
				{renderAddButton()}
			</ScrollView>
		</SafeAreaViewContainer>
	);
}
