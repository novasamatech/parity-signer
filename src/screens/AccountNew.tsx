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

import AccountIconChooser from 'components/AccountIconChooser';
import Button from 'components/Button';
import DerivationPathField from 'components/DerivationPathField';
import KeyboardScrollView from 'components/KeyboardScrollView';
import { NetworkCard } from 'components/NetworkCard';
import TextInput from 'components/TextInput';
import { NetworkProtocols } from 'constants/networkSpecs';
import React, { useCallback, useContext, useEffect, useReducer } from 'react';
import { StyleSheet, Text, View } from 'react-native';
import { AccountsContext } from 'stores/AccountsContext';
import { NetworksContext } from 'stores/NetworkContext';
import colors from 'styles/colors';
import fonts from 'styles/fonts';
import fontStyles from 'styles/fontStyles';
import { Account, UnlockedAccount } from 'types/identityTypes';
import { NetworkParams } from 'types/networkTypes';
import { NavigationProps } from 'types/props';
import { emptyAccount, validateSeed } from 'utils/account';
import { constructSURI } from 'utils/suri';

interface State {
	derivationPassword: string;
	derivationPath: string;
	isDerivationPathValid: boolean;
	selectedAccount?: Account;
	selectedNetwork?:NetworkParams | null;
	newAccount?: Account;
}

export default function AccountNew({ navigation }: NavigationProps<'AccountNew'>): React.ReactElement {
	const initialState = {
		derivationPassword: '',
		derivationPath: '',
		isDerivationPathValid: true,
		selectedAccount: undefined,
		selectedNetwork: undefined
	};
	const reducer = (state: State, delta: Partial<State>): State => ({
		...state,
		...delta
	});
	const [state, updateState] = useReducer(reducer, initialState);
	const { derivationPassword, derivationPath, isDerivationPathValid, selectedAccount, selectedNetwork } = state;
	const accountsStore = useContext(AccountsContext);
	const { getNetwork } = useContext(NetworksContext);
	const seed = (selectedAccount as UnlockedAccount)?.seed;
	const isSubstrate = selectedNetwork?.protocol === NetworkProtocols.SUBSTRATE;

	useEffect((): void => {
		accountsStore.updateNew(emptyAccount('', ''));
		// we get an infinite loop if we add anything here.
	// eslint-disable-next-line react-hooks/exhaustive-deps
	}, []);

	useEffect((): void => {
		const selectedAccount = accountsStore.state.newAccount;
		const selectedNetwork = getNetwork(selectedAccount.networkKey);

		updateState({
			selectedAccount,
			selectedNetwork
		});
	}, [accountsStore.state.newAccount, getNetwork]);

	const onAccountSelect = useCallback(({ isBip39, newAddress, newSeed }): void => {
		if (newAddress && isBip39 && newSeed) {
			if (isSubstrate) {
				try {
					const suri = constructSURI({
						derivePath: derivationPath,
						password: derivationPassword,
						phrase: newSeed
					});

					accountsStore.updateNew({
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
				accountsStore.updateNew({
					address: newAddress,
					seed: newSeed,
					validBip39Seed: isBip39
				});
			}
		} else {
			accountsStore.updateNew({
				address: '',
				seed: '',
				validBip39Seed: false
			});
		}
	},[accountsStore, derivationPassword, derivationPath, isSubstrate])

	if (!selectedAccount) {
		return <View />;
	}

	const { address, name, validBip39Seed } = selectedAccount;

	return (
		<KeyboardScrollView>
			<View style={styles.bodyContainer}>
				<Text style={styles.titleTop}>CREATE ACCOUNT</Text>
				<View style={styles.step}>
					<Text style={styles.title}>NAME</Text>
					<TextInput
						onChangeText={(input: string): void =>
							accountsStore.updateNew({ name: input })
						}
						placeholder="new name"
						value={name}
					/>
				</View>
				<View style={styles.step}>
					<Text style={styles.title}>NETWORK</Text>
					<NetworkCard
						networkKey={selectedAccount.networkKey}
						onPress={(): void => navigation.navigate('LegacyNetworkChooser')}
						title={selectedNetwork?.title || 'Select Network'}
					/>
				</View>
				{ selectedNetwork && (
					<View>
						<View style={styles.step}>
							<Text style={styles.title}>ICON & ADDRESS</Text>
							<AccountIconChooser
								derivationPassword={derivationPassword}
								derivationPath={derivationPath}
								network={selectedNetwork!}
								onSelect={onAccountSelect}
								value={address && address}
							/>
						</View>
						{isSubstrate && (
							<View style={StyleSheet.flatten([styles.step, styles.lastStep])}>
								<DerivationPathField
									onChange={(newDerivationPath: { derivationPassword: string; derivationPath: string; isDerivationPathValid: boolean; }): void => {
										updateState({
											derivationPassword: newDerivationPath.derivationPassword,
											derivationPath: newDerivationPath.derivationPath,
											isDerivationPathValid: newDerivationPath.isDerivationPathValid
										});
									}}
									styles={styles}
								/>
							</View>
						)}
						<View style={styles.bottom}>
							<Button
								disabled={
									!validateSeed(seed, validBip39Seed).valid ||
							!isDerivationPathValid
								}
								onPress={(): void => {
									navigation.navigate('LegacyAccountBackup', { isNew: true });
								}}
								title="Next Step"
							/>
						</View>
					</View>
				)}
			</View>
		</KeyboardScrollView>
	);
}

const styles = StyleSheet.create({
	bodyContainer: {
		flex: 1,
		flexDirection: 'column',
		justifyContent: 'space-between'
	},
	bottom: {
		flexBasis: 50,
		paddingBottom: 15
	},
	lastStep: {
		paddingTop: 0
	},
	step: {
		padding: 16
	},
	title: {
		...fontStyles.h_subheading,
		color: colors.text.main
	},
	titleTop: {
		color: colors.text.main,
		fontFamily: fonts.bold,
		fontSize: 24,
		paddingBottom: 20,
		textAlign: 'center'
	}
});
