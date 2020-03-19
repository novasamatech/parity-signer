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

/**
 * This is the current app's main landing page
 */

import React, { FunctionComponent, useState } from 'react';
import { ScrollView, StyleSheet, Text, View } from 'react-native';

import {
	SafeAreaScrollViewContainer,
	SafeAreaViewContainer
} from 'components/SafeAreaContainer';
import {
	NETWORK_LIST,
	UnknownNetworkKeys,
	SubstrateNetworkKeys,
	NetworkProtocols
} from 'constants/networkSpecs';
import testIDs from 'e2e/testIDs';
import colors from 'styles/colors';
import Button from 'components/Button';
import { NavigationAccountProps } from 'types/props';
import {
	navigateToPathsList,
	unlockSeedPhrase,
	navigateToPathDetails
} from 'utils/navigationHelpers';
import { withAccountStore } from 'utils/HOC';
import { alertPathDerivationError } from 'utils/alertUtils';
import {
	getExistedNetworkKeys,
	getIdentityName,
	getPathsWithSubstrateNetworkKey
} from 'utils/identitiesUtils';
import ScreenHeading, { IdentityHeading } from 'components/ScreenHeading';
import fontStyles from 'styles/fontStyles';
import { NetworkCard } from 'components/AccountCard';
import {
	NetworkParams,
	SubstrateNetworkParams,
	isSubstrateNetworkParams,
	isEthereumNetworkParams
} from 'types/networkSpecsTypes';

const excludedNetworks = [
	UnknownNetworkKeys.UNKNOWN,
	SubstrateNetworkKeys.KUSAMA_CC2
];
if (!__DEV__) {
	excludedNetworks.push(SubstrateNetworkKeys.SUBSTRATE_DEV);
	excludedNetworks.push(SubstrateNetworkKeys.KUSAMA_DEV);
}

function AccountNetworkChooser({
	accounts,
	navigation,
	route
}: NavigationAccountProps<'AccountNetworkChooser'>): React.ReactElement {
	const isNew = route.params?.isNew ?? false;
	const [shouldShowMoreNetworks, setShouldShowMoreNetworks] = useState(false);
	const { identities, currentIdentity, loaded } = accounts.state;
	const hasLegacyAccount = accounts.getAccounts().size !== 0;

	const TextButton: FunctionComponent<{ text: string; isRecover: boolean }> = ({
		text,
		isRecover
	}) => (
		<Text
			style={[fontStyles.quote, { textDecorationLine: 'underline' }]}
			testID={
				isRecover
					? testIDs.AccountNetworkChooser.recoverButton
					: testIDs.AccountNetworkChooser.createButton
			}
			onPress={(): void => navigation.navigate('IdentityNew', { isRecover })}
		>
			{text}
		</Text>
	);

	const showOnboardingMessage = (): React.ReactElement => (
		<SafeAreaScrollViewContainer
			testID={testIDs.AccountNetworkChooser.noAccountScreen}
			contentContainerStyle={styles.scrollContent}
		>
			<View style={styles.onboardingWrapper}>
				<TextButton text="Create" isRecover={false} />
				<Text style={fontStyles.quote}> or </Text>
				<TextButton text="recover" isRecover={true} />
				<Text style={fontStyles.quote}>your identity to get started.</Text>
				{hasLegacyAccount && (
					<Button
						title="Show Legacy Accounts"
						onPress={(): void => navigation.navigate('LegacyAccountList')}
						small={true}
						onlyText={true}
						style={{ marginLeft: 0 }}
					/>
				)}
			</View>
		</SafeAreaScrollViewContainer>
	);

	const showNoCurrentIdentityMessage = (): React.ReactElement => (
		<SafeAreaScrollViewContainer contentContainerStyle={styles.scrollContent}>
			<View style={styles.onboardingWrapper}>
				<Text style={fontStyles.quote}>
					Select one of your identity to get started.
				</Text>
			</View>
		</SafeAreaScrollViewContainer>
	);

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
		const availableNetworks = getExistedNetworkKeys(currentIdentity!);
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
		const seedPhrase = await unlockSeedPhrase(navigation);
		const fullPath = `//${pathId}`;
		const derivationSucceed = await accounts.deriveNewPath(
			fullPath,
			seedPhrase,
			networkKey,
			`${networkParams.title} root`
		);
		if (derivationSucceed) {
			navigateToPathDetails(navigation, networkKey, fullPath);
		} else {
			alertPathDerivationError();
		}
	};

	const deriveEthereumAccount = async (networkKey: string): Promise<void> => {
		const seedPhrase = await unlockSeedPhrase(navigation);
		const derivationSucceed = await accounts.deriveEthereumAccount(
			seedPhrase,
			networkKey
		);
		if (derivationSucceed) {
			navigateToPathsList(navigation, networkKey);
		} else {
			alertPathDerivationError();
		}
	};

	const renderCustomPathCard = (): React.ReactElement => (
		<NetworkCard
			isAdd={true}
			onPress={(): void =>
				navigation.navigate('PathDerivation', { parentPath: '' })
			}
			testID={testIDs.AccountNetworkChooser.addCustomNetworkButton}
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
					testID={testIDs.AccountNetworkChooser.addNewNetworkButton}
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
			if (isSubstrateNetworkParams(networkParams)) {
				const listedPaths = getPathsWithSubstrateNetworkKey(paths, networkKey);
				if (listedPaths.length === 0)
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

	if (!loaded) return <SafeAreaViewContainer />;
	if (identities.length === 0) return showOnboardingMessage();
	if (!currentIdentity) return showNoCurrentIdentityMessage();

	const networkList = Object.entries(NETWORK_LIST).filter(filterNetworkKeys);
	networkList.sort(sortNetworkKeys);

	return (
		<SafeAreaViewContainer>
			{renderScreenHeading()}
			<ScrollView
				bounces={false}
				testID={testIDs.AccountNetworkChooser.chooserScreen}
			>
				{networkList.map(([networkKey, networkParams]) => {
					const networkIndexSuffix = isEthereumNetworkParams(networkParams)
						? networkParams.ethereumChainId
						: networkParams.pathId;
					return (
						<NetworkCard
							key={networkKey}
							testID={
								testIDs.AccountNetworkChooser.networkButton + networkIndexSuffix
							}
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

export default withAccountStore(AccountNetworkChooser);

const styles = StyleSheet.create({
	onboardingWrapper: {
		alignItems: 'center',
		flexDirection: 'row',
		flexWrap: 'wrap'
	},
	scrollContent: {
		flex: 1,
		justifyContent: 'center',
		padding: 16,
		paddingBottom: 100
	}
});
