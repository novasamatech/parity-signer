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
import AccountCard from '../components/AccountCard';
import Button from '../components/Button';
import {
	NETWORK_LIST,
	UnknownNetworkKeys,
	SubstrateNetworkKeys,
	NetworkProtocols
} from '../constants';
import { navigateToPathsList, unlockSeed } from '../util/navigationHelpers';
import { withAccountStore } from '../util/HOC';
import { alertPathDerivationError } from '../util/alertUtils';
import {
	getAvailableNetworkKeys,
	getPathsWithSubstrateNetwork
} from '../util/identitiesUtils';
import testIDs from '../../e2e/testIDs';
import ButtonMainAction from '../components/ButtonMainAction';
import ScreenHeading from '../components/ScreenHeading';
import Separator from '../components/Separator';
import fontStyles from '../fontStyles';

function AccountNetworkChooser({ navigation, accounts }) {
	const isNew = navigation.getParam('isNew', false);
	const [shouldShowMoreNetworks, setShouldShowMoreNetworks] = useState(false);
	const excludedNetworks = [UnknownNetworkKeys.UNKNOWN];
	if (!__DEV__) {
		excludedNetworks.push(SubstrateNetworkKeys.SUBSTRATE_DEV);
		excludedNetworks.push(SubstrateNetworkKeys.KUSAMA_DEV);
	}
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
		const availableNetworks = getAvailableNetworkKeys(currentIdentity);
		if (excludedNetworks.includes(networkKey)) return false;
		if (isNew) return true;
		if (shouldShowMoreNetworks) {
			return !availableNetworks.includes(networkKey);
		}
		return availableNetworks.includes(networkKey);
	};

	const onDerivationFinished = (derivationSucceed, networkKey) => {
		if (derivationSucceed) {
			navigateToPathsList(navigation, networkKey);
		} else {
			alertPathDerivationError();
		}
	};

	const deriveSubstrateDefault = async (networkKey, networkParams) => {
		const { prefix, pathId } = networkParams;
		const seed = await unlockSeed(navigation);
		const derivationSucceed = await accounts.deriveNewPath(
			`//${pathId}//default`,
			seed,
			prefix,
			networkKey,
			'Default'
		);
		onDerivationFinished(derivationSucceed, networkKey);
	};

	const deriveEthereumAccount = async networkKey => {
		const seed = await unlockSeed(navigation);
		const derivationSucceed = await accounts.deriveEthereumAccount(
			seed,
			networkKey
		);
		onDerivationFinished(derivationSucceed, networkKey);
	};

	const renderShowMoreButton = () => {
		if (isNew) return;
		if (!shouldShowMoreNetworks) {
			return (
				<>
					<AccountCard
						isAdd={true}
						isNetworkCard={true}
						onPress={() => setShouldShowMoreNetworks(true)}
						title="Add Network Account"
						networkColor={colors.bg}
					/>
					<Separator style={{ backgroundColor: 'transparent', height: 120 }} />
				</>
			);
		}
	};

	const renderScreenHeading = () => {
		if (isNew) {
			return <ScreenHeading title={'Create your first Keypair'} />;
		} else if (shouldShowMoreNetworks) {
			return (
				<ScreenHeading
					title={'Choose Network'}
					onPress={() => setShouldShowMoreNetworks(false)}
				/>
			);
		}
	};

	const renderScanButton = () => {
		if (isNew) return;
		else if (shouldShowMoreNetworks) return;
		return (
			<ButtonMainAction
				testID={testIDs.AccountNetworkChooser.scanButton}
				title={'Scan'}
				onPress={() => navigation.navigate('QrScanner')}
			/>
		);
	};

	const onNetworkChosen = async (networkKey, networkParams) => {
		if (isNew) {
			if (networkParams.protocol === NetworkProtocols.SUBSTRATE) {
				await deriveSubstrateDefault(networkKey, networkParams);
			} else {
				await deriveEthereumAccount(networkKey);
			}
		} else {
			const paths = Array.from(currentIdentity.meta.keys());
			const listedPaths = getPathsWithSubstrateNetwork(paths, networkKey);
			if (networkParams.protocol === NetworkProtocols.SUBSTRATE) {
				if (listedPaths.length === 0)
					return navigation.navigate('PathDerivation', {
						networkKey
					});
			} else if (!paths.includes(networkKey)) {
				return await deriveEthereumAccount(networkKey);
			}
			navigation.navigate('PathsList', { networkKey });
		}
	};

	if (!loaded || !currentIdentity) return <ScrollView style={styles.body} />;

	if (identities.length === 0) return showOnboardingMessage();

	const networkList = Object.entries(NETWORK_LIST).filter(getNetworkKeys);
	networkList.sort(sortNetworkKeys);

	return (
		<View
			style={styles.body}
			testID={testIDs.AccountNetworkChooser.chooserScreen}
		>
			{renderScreenHeading()}
			<ScrollView>
				{networkList.map(([networkKey, networkParams], index) => (
					<AccountCard
						isNetworkCard={true}
						accountId={''}
						key={networkKey}
						testID={testIDs.AccountNetworkChooser.networkButton + index}
						networkKey={networkKey}
						onPress={() => onNetworkChosen(networkKey, networkParams)}
						title={networkParams.title}
					/>
				))}
				{renderShowMoreButton()}
			</ScrollView>
			{renderScanButton()}
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
