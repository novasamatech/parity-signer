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

'use strict';

import React, { useState } from 'react';
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import { withNavigation } from 'react-navigation';

import colors from '../colors';
import Button from '../components/Button';
import {
	NETWORK_LIST,
	UnknownNetworkKeys,
	SubstrateNetworkKeys,
	NetworkProtocols
} from '../constants';
import {
	navigateToPathsList,
	navigateToSubstrateRoot,
	unlockSeedPhrase
} from '../util/navigationHelpers';
import { withAccountStore } from '../util/HOC';
import { alertPathDerivationError } from '../util/alertUtils';
import {
	getExistedNetworkKeys,
	getIdentityName,
	getPathsWithSubstrateNetwork
} from '../util/identitiesUtils';
import testIDs from '../../e2e/testIDs';
import ScreenHeading, { IdentityHeading } from '../components/ScreenHeading';
import fontStyles from '../fontStyles';
import { NetworkCard } from '../components/AccountCard';

const excludedNetworks = [
	UnknownNetworkKeys.UNKNOWN,
	SubstrateNetworkKeys.KUSAMA_CC2
];
if (!__DEV__) {
	excludedNetworks.push(SubstrateNetworkKeys.SUBSTRATE_DEV);
	excludedNetworks.push(SubstrateNetworkKeys.KUSAMA_DEV);
}

function AccountNetworkChooser({ navigation, accounts }) {
	const isNew = navigation.getParam('isNew', false);
	const [shouldShowMoreNetworks, setShouldShowMoreNetworks] = useState(false);
	const { identities, currentIdentity, loaded } = accounts.state;
	const hasLegacyAccount = accounts.getAccounts().size !== 0;

	const TextButton = ({ text, isRecover }) => (
		<Text
			style={[fontStyles.quote, { textDecorationLine: 'underline' }]}
			testID={
				isRecover
					? testIDs.AccountNetworkChooser.recoverButton
					: testIDs.AccountNetworkChooser.createButton
			}
			onPress={() => navigation.navigate('IdentityNew', { isRecover })}
		>
			{text}
		</Text>
	);

	const showOnboardingMessage = () => {
		return (
			<ScrollView
				testID={testIDs.AccountNetworkChooser.noAccountScreen}
				style={styles.body}
				contentContainerStyle={{
					flex: 1,
					justifyContent: 'center',
					padding: 16,
					paddingBottom: 100
				}}
			>
				<View style={styles.onboardingWrapper}>
					<TextButton text="Create" isRecover={false} />
					<Text style={fontStyles.quote}> or </Text>
					<TextButton text="recover" isRecover={true} />
					<Text style={fontStyles.quote}>your identity to get started.</Text>
					{hasLegacyAccount && (
						<Button
							title="Show Legacy Accounts"
							onPress={() => navigation.navigate('LegacyAccountList')}
							small={true}
							onlyText={true}
							style={{ marginLeft: 0 }}
						/>
					)}
				</View>
			</ScrollView>
		);
	};

	const sortNetworkKeys = ([, networkParams]) =>
		networkParams.protocol !== NetworkProtocols.SUBSTRATE ? 1 : -1;

	const getNetworkKeys = ([networkKey]) => {
		const shouldExclude = excludedNetworks.includes(networkKey);
		if (isNew && !shouldExclude) return true;
		const availableNetworks = getExistedNetworkKeys(currentIdentity);
		if (shouldShowMoreNetworks) {
			if (shouldExclude) return false;
			return !availableNetworks.includes(networkKey);
		}
		return availableNetworks.includes(networkKey);
	};

	const onDerivationFinished = (
		derivationSucceed,
		networkKey,
		isSubstrateRoot
	) => {
		if (derivationSucceed) {
			if (isSubstrateRoot) {
				return navigateToSubstrateRoot(navigation, networkKey);
			}
			navigateToPathsList(navigation, networkKey);
		} else {
			alertPathDerivationError();
		}
	};

	const deriveSubstrateRoot = async (networkKey, networkParams) => {
		const { pathId } = networkParams;
		const seedPhrase = await unlockSeedPhrase(navigation);
		const derivationSucceed = await accounts.deriveNewPath(
			`//${pathId}`,
			seedPhrase,
			networkKey,
			`${networkParams.title} root`
		);
		onDerivationFinished(derivationSucceed, networkKey, true);
	};

	const deriveEthereumAccount = async networkKey => {
		const seedPhrase = await unlockSeedPhrase(navigation);
		const derivationSucceed = await accounts.deriveEthereumAccount(
			seedPhrase,
			networkKey
		);
		onDerivationFinished(derivationSucceed, networkKey, false);
	};

	const renderCustomPathCard = () => (
		<NetworkCard
			isAdd={true}
			onPress={() => navigation.navigate('PathDerivation', { parentPath: '' })}
			testID={testIDs.AccountNetworkChooser.addCustomNetworkButton}
			title="Create Custom Path"
			networkColor={colors.bg}
		/>
	);

	const renderAddButton = () => {
		if (isNew) return renderCustomPathCard();
		if (!shouldShowMoreNetworks) {
			return (
				<NetworkCard
					isAdd={true}
					onPress={() => setShouldShowMoreNetworks(true)}
					testID={testIDs.AccountNetworkChooser.addNewNetworkButton}
					title="Add Network Account"
					networkColor={colors.bg}
				/>
			);
		} else {
			return renderCustomPathCard();
		}
	};

	const renderScreenHeading = () => {
		if (isNew) {
			return <ScreenHeading title={'Create your first Keypair'} />;
		} else if (shouldShowMoreNetworks) {
			return (
				<IdentityHeading
					title={'Choose Network'}
					onPressBack={() => setShouldShowMoreNetworks(false)}
				/>
			);
		} else {
			const identityName = getIdentityName(currentIdentity, identities);
			return <IdentityHeading title={identityName} />;
		}
	};

	const onNetworkChosen = async (networkKey, networkParams) => {
		if (isNew) {
			if (networkParams.protocol === NetworkProtocols.SUBSTRATE) {
				await deriveSubstrateRoot(networkKey, networkParams);
			} else {
				await deriveEthereumAccount(networkKey);
			}
		} else {
			const paths = Array.from(currentIdentity.meta.keys());
			const listedPaths = getPathsWithSubstrateNetwork(paths, networkKey);
			if (networkParams.protocol === NetworkProtocols.SUBSTRATE) {
				if (listedPaths.length === 0)
					return await deriveSubstrateRoot(networkKey, networkParams);
			} else if (
				networkParams.protocol === NetworkProtocols.ETHEREUM &&
				!paths.includes(networkKey)
			) {
				return await deriveEthereumAccount(networkKey);
			}
			navigation.navigate('PathsList', { networkKey });
		}
	};

	if (!loaded) return <ScrollView style={styles.body} />;
	if (identities.length === 0) return showOnboardingMessage();
	if (!currentIdentity) return <ScrollView style={styles.body} />;

	const networkList = Object.entries(NETWORK_LIST).filter(getNetworkKeys);
	networkList.sort(sortNetworkKeys);

	return (
		<View style={styles.body}>
			{renderScreenHeading()}
			<ScrollView testID={testIDs.AccountNetworkChooser.chooserScreen}>
				{networkList.map(([networkKey, networkParams], index) => (
					<NetworkCard
						key={networkKey}
						testID={testIDs.AccountNetworkChooser.networkButton + index}
						networkKey={networkKey}
						onPress={() => onNetworkChosen(networkKey, networkParams)}
						title={networkParams.title}
					/>
				))}
				{renderAddButton()}
			</ScrollView>
		</View>
	);
}

export default withAccountStore(withNavigation(AccountNetworkChooser));

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column'
	},
	onboardingWrapper: {
		alignItems: 'center',
		flexDirection: 'row',
		flexWrap: 'wrap'
	}
});
