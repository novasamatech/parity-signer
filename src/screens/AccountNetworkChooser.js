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
import fonts from '../fonts';
import {
	NETWORK_LIST,
	UnknownNetworkKeys,
	SubstrateNetworkKeys,
	NetworkProtocols
} from '../constants';
import { navigateToPathsList, unlockSeed } from '../util/navigationHelpers';
import { withAccountStore } from '../util/HOC';
import { alertPathDerivationError } from '../util/alertUtils';
import { getAvailableNetworkKeys } from '../util/identitiesUtils';
import testIDs from '../../e2e/testIDs';

function AccountNetworkChooser({ navigation, accounts }) {
	const isNew = navigation.getParam('isNew', false);
	const [shouldShowMoreNetworks, setShouldShowMoreNetworks] = useState(false);
	const excludedNetworks = [UnknownNetworkKeys.UNKNOWN];
	if (!__DEV__) {
		excludedNetworks.push(SubstrateNetworkKeys.SUBSTRATE_DEV);
		excludedNetworks.push(SubstrateNetworkKeys.KUSAMA_DEV);
	}
	const { identities, currentIdentity, loaded } = accounts.state;
	const hasNoAccount =
		accounts.getAccounts().size === 0 && identities.length === 0;

	const TextButton = ({ text, isRecover }) => (
		<Text
			style={styles.link}
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
			>
				<View style={styles.onboardingWrapper}>
					<Text style={styles.onboardingText}>No Identity yet?{'\n'}</Text>
					<TextButton text="Create" isRecover={false} />
					<Text style={styles.onboardingText}> Or {'\n'}</Text>
					<TextButton text="Recover" isRecover={true} />
					<Text style={styles.onboardingText}>an account to get started.</Text>
				</View>
			</ScrollView>
		);
	};

	const getNetworkKeys = ([networkKey]) => {
		const availableNetworks = getAvailableNetworkKeys(
			currentIdentity || identities[0]
		);
		if (excludedNetworks.includes(networkKey)) return false;
		if (isNew) return true;
		if (shouldShowMoreNetworks) {
			return !availableNetworks.includes(networkKey);
		}
		return availableNetworks.includes(networkKey);
	};

	const renderShowMoreButton = () => {
		if (isNew) return;
		if (!shouldShowMoreNetworks) {
			return (
				<>
					<Button
						testID={testIDs.AccountNetworkChooser.addNewNetworkButton}
						title="Add Network Account"
						onPress={() => setShouldShowMoreNetworks(true)}
					/>
					<Button
						title="Scan"
						onPress={() => navigation.navigate('QrScanner')}
					/>
				</>
			);
		} else {
			return (
				<Button
					testID={testIDs.AccountNetworkChooser.showExistedButton}
					title="Show Existed Network Account"
					onPress={() => setShouldShowMoreNetworks(false)}
				/>
			);
		}
	};

	if (!loaded) return <ScrollView style={styles.body} />;

	if (hasNoAccount) return showOnboardingMessage();

	return (
		<ScrollView
			style={styles.body}
			testID={testIDs.AccountNetworkChooser.chooserScreen}
		>
			<Text style={styles.title}>
				{isNew ? 'CREATE YOUR FIRST KEYPAIR' : 'CHOOSE NETWORK'}{' '}
			</Text>
			{Object.entries(NETWORK_LIST)
				.filter(getNetworkKeys)
				.map(([networkKey, networkParams], index) => (
					<AccountCard
						address={''}
						key={networkKey}
						testID={testIDs.AccountNetworkChooser.networkButton + index}
						networkKey={networkKey}
						onPress={async () => {
							if (isNew) {
								const { prefix, pathId, protocol } = networkParams;
								const seed = await unlockSeed(navigation);
								let derivationSucceed;
								if (protocol === NetworkProtocols.SUBSTRATE) {
									derivationSucceed = await accounts.deriveNewPath(
										`//${pathId}//default`,
										seed,
										prefix,
										networkKey
									);
								} else {
									derivationSucceed = await accounts.deriveEthereumAccount(
										seed,
										networkKey
									);
								}
								if (derivationSucceed) {
									navigateToPathsList(navigation, networkKey);
								} else {
									alertPathDerivationError();
								}
							} else {
								navigation.navigate('PathsList', { networkKey });
							}
						}}
						title={networkParams.title}
					/>
				))}
			{renderShowMoreButton()}
		</ScrollView>
	);
}

export default withAccountStore(withNavigation(AccountNetworkChooser));

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column'
	},
	header: {
		alignItems: 'center',
		flexDirection: 'row',
		justifyContent: 'center'
	},
	onboardingText: {
		color: colors.bg_text_sec,
		fontFamily: fonts.regular,
		fontSize: 20
	},
	onboardingWrapper: {
		alignItems: 'center',
		flex: 1
	},
	title: {
		color: colors.bg_text_sec,
		flexDirection: 'column',
		fontFamily: fonts.bold,
		fontSize: 18,
		justifyContent: 'center',
		marginTop: 16,
		paddingLeft: 72
	}
});
