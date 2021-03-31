// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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

import React, { useContext, useEffect, useRef, useState } from 'react';
import { StyleSheet, View } from 'react-native';

import testIDs from 'e2e/testIDs';
import { AccountsContext } from 'stores/AccountsContext';
import { AlertStateContext } from 'stores/alertContext';
import Button from 'components/Button';
import TextInput from 'components/TextInput';
import { NavigationProps } from 'types/props';
import { emptyIdentity } from 'utils/identitiesUtils';
import colors from 'styles/colors';
import { validateSeed } from 'utils/account';
import AccountSeed from 'components/AccountSeed';
import { navigateToNewIdentityNetwork } from 'utils/navigationHelpers';
import {
	alertError,
	alertIdentityCreationError,
	alertRisks
} from 'utils/alertUtils';
import ScreenHeading from 'components/ScreenHeading';
import { brainWalletAddress } from 'utils/native';
import { debounce } from 'utils/debounce';
import { useNewSeedRef } from 'utils/seedRefHooks';
import KeyboardScrollView from 'components/KeyboardScrollView';

function CreateWallet({
	navigation,
	route
}: NavigationProps<'CreateWallet'>): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const defaultSeedValidObject = validateSeed('', false);
	const isRecoverDefaultValue = route.params?.isRecover ?? false;
	const [isRecover, setIsRecover] = useState(isRecoverDefaultValue);
	const [isSeedValid, setIsSeedValid] = useState(defaultSeedValidObject);
	const [seedPhrase, setSeedPhrase] = useState('');
	const { setAlert } = useContext(AlertStateContext);
	const createSeedRefWithNewSeed = useNewSeedRef();
	const clearIdentity = useRef(() =>
		accountsStore.updateNewIdentity(emptyIdentity())
	);

	useEffect((): (() => void) => {
		clearIdentity.current();
		return clearIdentity.current;
	}, [clearIdentity]);

	const updateName = (name: string): void => {
		accountsStore.updateNewIdentity({ name });
	};

	const onSeedTextInput = (inputSeedPhrase: string): void => {
		setSeedPhrase(inputSeedPhrase);
		const addressGeneration = (): Promise<void> =>
			brainWalletAddress(inputSeedPhrase.trimEnd())
				.then(({ bip39 }) => {
					setIsSeedValid(validateSeed(inputSeedPhrase, bip39));
				})
				.catch(() => setIsSeedValid(defaultSeedValidObject));
		const debouncedAddressGeneration = debounce(addressGeneration, 200);
		debouncedAddressGeneration();
	};

	const onRecoverIdentity = async (): Promise<void> => {
		try {
			if (isSeedValid.bip39) {
				await accountsStore.saveNewIdentity(
					seedPhrase.trimEnd(),
					createSeedRefWithNewSeed
				);
			} else {
				await accountsStore.saveNewIdentity(
					seedPhrase,
					createSeedRefWithNewSeed
				);
			}
			setSeedPhrase('');
			navigateToNewIdentityNetwork(navigation);
		} catch (e) {
			alertIdentityCreationError(setAlert, e.message);
		}
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

	const onCreateNewIdentity = (): void => {
		setSeedPhrase('');
		navigation.navigate('ShowRecoveryPhrase', {
			isNew: true
		});
	};

	const renderRecoverView = (): React.ReactElement => (
		<>
			<AccountSeed
				testID={testIDs.CreateWallet.seedInput}
				onChangeText={onSeedTextInput}
				onSubmitEditing={onRecoverConfirm}
				returnKeyType="done"
				valid={isSeedValid.valid}
			/>
			<View style={styles.btnBox}>
				<Button
					title="Import Wallet"
					testID={testIDs.CreateWallet.recoverButton}
					onPress={onRecoverConfirm}
					small={true}
				/>
				<Button
					title="or create new wallet"
					onPress={(): void => {
						setIsRecover(false);
					}}
					small={true}
					onlyText={true}
				/>
			</View>
		</>
	);

	const renderCreateView = (): React.ReactElement => (
		<View style={styles.btnBox}>
			<Button
				title="Create New"
				testID={testIDs.CreateWallet.createButton}
				onPress={onCreateNewIdentity}
				small={true}
			/>
			<Button
				title="Import Wallet"
				onPress={(): void => setIsRecover(true)}
				small={true}
			/>
		</View>
	);

	return (
		<KeyboardScrollView
			bounces={false}
			style={styles.body}
			testID={testIDs.CreateWallet.scrollScreen}
		>
			<ScreenHeading title={'New Wallet'} />
			<TextInput
				onChangeText={updateName}
				testID={testIDs.CreateWallet.nameInput}
				value={accountsStore.state.newIdentity.name}
				placeholder="Wallet name"
			/>
			{isRecover ? renderRecoverView() : renderCreateView()}
		</KeyboardScrollView>
	);
}

export default CreateWallet;

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.background.app,
		flex: 1,
		overflow: 'hidden'
	},
	btnBox: {
		alignContent: 'center',
		marginTop: 32
	}
});
