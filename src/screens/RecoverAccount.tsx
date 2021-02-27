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

import { useNavigation } from '@react-navigation/native';
import AccountCard from 'components/AccountCard';
import AccountSeed from 'components/AccountSeed';
import Button from 'components/Button';
import { NetworkCard } from 'components/NetworkCard';
import ScreenHeading from 'components/ScreenHeading';
import TextInput from 'components/TextInput';
import testIDs from 'e2e/testIDs';
import { KeyboardAwareContainer } from 'modules/unlock/components/Container';
import React, { useCallback, useContext, useEffect, useMemo, useState } from 'react';
import { StyleSheet, Text, View } from 'react-native';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { isEthereumNetwork, NetworkParams, SubstrateNetworkParams } from 'types/networkTypes';
import { NavigationProps } from 'types/props';
import { emptyAccount, validateSeed } from 'utils/account';
import { alertError, alertRisks } from 'utils/alertUtils';
import { debounce } from 'utils/debounce';
import { brainWalletAddress, substrateAddress } from 'utils/native';
import { constructSURI } from 'utils/suri';

import { AccountsContext, AlertContext, NetworksContext } from '../context';

interface State {
	derivationPassword: string;
	derivationPath: string;
	isDerivationPathValid: boolean;
	selectedNetwork?:NetworkParams | null;
	newAccount?: Account;
}

function RecoverAccount({ navigation, route }: NavigationProps<'RecoverAccount'>): React.ReactElement {
	const initialState = {
		derivationPassword: '',
		derivationPath: '',
		isDerivationPathValid: true,
		selectedNetwork: undefined
	};

	const [derivationPath, setDerivationPath] = useState('');
	const [derivationPassword, setDerivationPassword] = useState('');
	const { newAccount, updateNew } = useContext(AccountsContext);
	const defaultSeedValidObject = validateSeed('', false);
	const [isSeedValid, setIsSeedValid] = useState(defaultSeedValidObject);
	const [seedPhrase, setSeedPhrase] = useState('');
	const { setAlert } = useContext(AlertContext);
	const { getNetwork } = useContext(NetworksContext)
	const selectedNetwork = useMemo(() => getNetwork(newAccount.networkKey), [getNetwork, newAccount.networkKey])
	const { navigate } = useNavigation()

	const goToPin = useCallback(() => {
		navigate('AccountPin', { isNew: true });
	}, [navigate])

	useEffect((): void => {
		console.log('----> empty it')
		updateNew(emptyAccount('', ''));
	// we get an infinite loop if we add anything here.
	// eslint-disable-next-line react-hooks/exhaustive-deps
	}, []);

	// useEffect((): void => {
	// 	const selectedNetwork = getNetwork(newAccount.networkKey);

	// 	updateState({ selectedNetwork });
	// }, [newAccount, getNetwork]);

	const onSeedTextInput = (inputSeedPhrase: string): void => {
		setSeedPhrase(inputSeedPhrase);
		const addressGeneration = (): Promise<void> =>
			brainWalletAddress(inputSeedPhrase.trimEnd())
				.then(({ bip39 }) => {
					setIsSeedValid(validateSeed(inputSeedPhrase, bip39));
					generateAddress()
				})
				.catch(() => setIsSeedValid(defaultSeedValidObject));
		const debouncedAddressGeneration = debounce(addressGeneration, 200);

		debouncedAddressGeneration();
	};

	const generateAddress = useCallback(() => {
		console.log('<--generate')

		if (!selectedNetwork) {
			console.error('No network selected')

			return null
		}

		if (isEthereumNetwork(selectedNetwork)) {
			brainWalletAddress(seedPhrase)
				.then(({ address, bip39 }) =>
					updateNew({
						address,
						seed: seedPhrase,
						seedPhrase,
						validBip39Seed: bip39
					}))
				.catch(console.error);
		} else {
			// Substrate
			try {
				const { prefix } = selectedNetwork as SubstrateNetworkParams
				const suri = constructSURI({
					derivePath: derivationPath,
					password: derivationPassword,
					phrase: seedPhrase
				});

				substrateAddress(suri, prefix)
					.then(address => {
						updateNew({
							address,
							derivationPassword,
							derivationPath,
							seed: suri,
							seedPhrase,
							validBip39Seed: true
						});
					})
					.catch((e) => {
						//invalid phrase
						console.error('invalid phrase',e)
					});
			} catch (e) {
				// invalid phrase or derivation path
				console.error('invalid phrase or path', e)
			}
		}
	}, [derivationPassword, derivationPath, seedPhrase, selectedNetwork, updateNew]);

	// derivationPassword, derivationPath, seedPhrase, selecteNetwork, updateNew
	useEffect(() => {
		console.log('useEffect isSeedValid')
		isSeedValid.bip39 && generateAddress()
	}, [generateAddress, isSeedValid.bip39])

	const onRecoverIdentity = async (): Promise<void> => {
		// const pin = await setPin(navigation);

		// try {
		// if (isSeedValid.bip39) {
		goToPin()
		// 	} else {
		// 		await accountsStore.saveNewIdentity(seedPhrase,
		// 			pin,
		// 			createSeedRefWithNewSeed);
		// 	}

		// 	setSeedPhrase('');
		// 	navigateToNewIdentityNetwork(navigation);
		// } catch (e) {
		// 	alertIdentityCreationError(setAlert, e.message);
		// }
	};

	const onRecoverConfirm = (): void | Promise<void> => {
		if (!isSeedValid.valid) {
			if (isSeedValid.accountRecoveryAllowed) {
				return alertRisks(setAlert, `${isSeedValid.reason}`, onRecoverIdentity);
			} else {
				return alertError(setAlert, `${isSeedValid.reason}`);
			}
		}

		return onRecoverIdentity();
	};

	// const onCreateNewIdentity = (): void => {
	// 	setSeedPhrase('');
	// 	navigation.navigate('IdentityBackup', { isNew: true });
	// };
	const { address, name, networkKey } = newAccount;

	return (
		<KeyboardAwareContainer>
			<ScreenHeading title={'New Account'} />
			<View style={styles.step}>
				<Text style={styles.title}>Name</Text>
				<TextInput
					onChangeText={(input: string): void =>
						updateNew({ name: input })
					}
					placeholder="new name"
					value={name}
				/>
			</View>
			<View style={styles.step}>
				<Text style={styles.title}>NETWORK</Text>
				<NetworkCard
					networkKey={networkKey}
					onPress={(): void => navigation.navigate('LegacyNetworkChooser')}
					title={selectedNetwork?.title || 'Select Network'}
				/>
			</View>
			<View style={styles.step}>
				<Text style={styles.title}>Mnemonic</Text>
				<AccountSeed
					onChangeText={onSeedTextInput}
					returnKeyType="done"
					testID={testIDs.RecoverAccount.seedInput}
					valid={isSeedValid.bip39}
				/>
			</View>
			{/* {isSubstrate && (
				<DerivationPathField
					onChange={newDerivationPath => {
						this.debouncedAddressGeneration(seedPhrase,
							newDerivationPath.derivationPath,
							newDerivationPath.derivationPassword);
						this.setState({
							derivationPassword: newDerivationPath.derivationPassword,
							derivationPath: newDerivationPath.derivationPath,
							isDerivationPathValid: newDerivationPath.isDerivationPathValid
						});
					}}
					styles={styles}
				/>
			)} */}
			{ isSeedValid.bip39 &&
				(<AccountCard
					address={address}
					networkKey={networkKey}
					title={name}
				/>)
			}
			<View style={styles.btnBox}>
				<Button
					onPress={onRecoverConfirm}
					small={true}
					testID={testIDs.RecoverAccount.recoverButton}
					title="Recover"
				/>
			</View>
		</KeyboardAwareContainer>
	);
}

export default RecoverAccount;

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.background.app,
		flex: 1,
		overflow: 'hidden'
	},
	btnBox: {
		alignContent: 'center',
		marginTop: 32
	},
	step: {
		padding: 16
	},
	title: {
		...fontStyles.h_subheading,
		color: colors.text.main
	}
});
