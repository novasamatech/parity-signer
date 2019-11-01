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

import React, { useEffect, useState } from 'react';
import { StyleSheet, Text, View } from 'react-native';
import { withNavigation } from 'react-navigation';
import colors from '../colors';
import AccountIconChooser from '../components/AccountIconChooser';
import Background from '../components/Background';
import Button from '../components/Button';
import DerivationPathField from '../components/DerivationPathField';
import KeyboardScrollView from '../components/KeyboardScrollView';
import TextInput from '../components/TextInput';
import { NETWORK_LIST, NetworkProtocols } from '../constants';
import fonts from '../fonts';
import { emptyAccount, validateSeed } from '../util/account';
import { constructSURI } from '../util/suri';
import AccountCard from '../components/AccountCard';
import { withAccountStore } from '../util/HOC';

/**
 * @return {null}
 */
function AccountNew({ accounts, navigation }) {
	const initialState = {
		derivationPassword: '',
		derivationPath: '',
		isDerivationPathValid: true,
		selectedAccount: undefined,
		selectedNetwork: undefined
	};

	const [state, setState] = useState(initialState);
	const updateState = delta => setState({ ...state, ...delta });

	useEffect(() => {
		accounts.updateNew(emptyAccount());
	}, [accounts]);

	/* eslint-disable react-hooks/exhaustive-deps*/
	useEffect(() => {
		const selectedAccount = accounts.getNew();
		const selectedNetwork = NETWORK_LIST[selectedAccount.networkKey];
		updateState({
			derivationPassword: state.derivationPassword,
			derivationPath: state.derivationPath,
			selectedAccount,
			selectedNetwork
		});
	}, [accounts.getNew()]);

	const {
		derivationPassword,
		derivationPath,
		isDerivationPathValid,
		selectedAccount,
		selectedNetwork
	} = state;
	if (!selectedAccount) return null;

	const { address, name, seed, validBip39Seed } = selectedAccount;
	const isSubstrate = selectedNetwork.protocol === NetworkProtocols.SUBSTRATE;

	return (
		<KeyboardScrollView>
			<View style={styles.body}>
				<Background />
				<Text style={styles.titleTop}>CREATE ACCOUNT</Text>
				<Text style={styles.title}>NETWORK</Text>
			</View>
			<AccountCard
				address={''}
				title={selectedNetwork.title}
				networkKey={selectedAccount.networkKey}
				onPress={() => navigation.navigate('AccountNetworkChooser')}
			/>
			<View style={styles.body}>
				<Text style={styles.title}>ICON & ADDRESS</Text>
				<AccountIconChooser
					derivationPassword={derivationPassword}
					derivationPath={derivationPath}
					onSelect={({ newAddress, isBip39, newSeed }) => {
						if (newAddress && isBip39 && newSeed) {
							if (isSubstrate) {
								try {
									const suri = constructSURI({
										derivePath: derivationPath,
										password: derivationPassword,
										phrase: newSeed
									});

									accounts.updateNew({
										address: newAddress,
										derivationPassword,
										derivationPath,
										seed: suri,
										seedPhrase: newSeed,
										validBip39Seed: isBip39
									});
								} catch (e) {
									console.error(e);
								}
							} else {
								// Ethereum account
								accounts.updateNew({
									address: newAddress,
									seed: newSeed,
									validBip39Seed: isBip39
								});
							}
						} else {
							accounts.updateNew({
								address: '',
								seed: '',
								validBip39Seed: false
							});
						}
					}}
					network={selectedNetwork}
					value={address && address}
				/>
				<Text style={styles.title}>NAME</Text>
				<TextInput
					onChangeText={name => accounts.updateNew({ name })}
					value={name}
					placeholder="Enter a new account name"
				/>
				{isSubstrate && (
					<DerivationPathField
						onChange={newDerivationPath => {
							updateState({
								derivationPassword: newDerivationPath.derivationPassword,
								derivationPath: newDerivationPath.derivationPath,
								isDerivationPathValid: newDerivationPath.isDerivationPathValid
							});
						}}
						styles={styles}
					/>
				)}
				<View style={styles.bottom}>
					<Text style={styles.hintText}>
						Next, you will be asked to backup your account, get a pen and some
						paper.
					</Text>
					<Button
						buttonStyles={styles.nextStep}
						title="Next Step"
						disabled={
							!validateSeed(seed, validBip39Seed).valid ||
							!isDerivationPathValid
						}
						onPress={() => {
							validateSeed(seed, validBip39Seed).valid &&
								navigation.navigate('LegacyAccountBackup', {
									isNew: true
								});
						}}
					/>
				</View>
			</View>
		</KeyboardScrollView>
	);
}

export default withAccountStore(withNavigation(AccountNew));

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		overflow: 'hidden',
		padding: 16
	},
	bodyContainer: {
		flex: 1,
		flexDirection: 'column',
		justifyContent: 'space-between'
	},
	bottom: {
		flexBasis: 50,
		paddingBottom: 15
	},
	hintText: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 12,
		paddingTop: 20,
		textAlign: 'center'
	},
	nextStep: {
		marginTop: 15
	},
	title: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 18
	},
	titleTop: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 24,
		paddingBottom: 20,
		textAlign: 'center'
	}
});
