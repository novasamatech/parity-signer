// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Copyright 2021 Commonwealth Labs, Inc.
// This file is part of Layer Wallet.

// Layer Wallet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Layer Wallet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Layer Wallet. If not, see <http://www.gnu.org/licenses/>.

import React, { useContext, useState } from 'react';
import { View } from 'react-native';
import { showMessage } from 'react-native-flash-message';

import { components } from 'styles';
import { AccountsContext } from 'stores/AccountsContext';
import Button from 'components/Button';
import { NavigationProps } from 'types/props';
import { validateSeed } from 'utils/account';
import AccountSeedTextInput from 'components/AccountSeedTextInput';
import { resetNavigationTo } from 'utils/navigationHelpers';
import { brainWalletAddress } from 'utils/native';
import { debounce } from 'utils/debounce';
import { useNewSeedRef } from 'utils/seedRefHooks';

function CreateWalletImport({
	navigation
}: NavigationProps<'CreateWalletImport'>): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const defaultSeedValidObject = validateSeed('', false);
	const [isSeedValid, setIsSeedValid] = useState(defaultSeedValidObject);
	const [seedPhrase, setSeedPhrase] = useState('');
	const createSeedRefWithNewSeed = useNewSeedRef();

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
			resetNavigationTo(navigation, 'Wallet', { isNew: true });
		} catch (e) {
			showMessage('Could not find a valid wallet for the seed: ' + e.message);
		}
	};

	const onRecoverConfirm = (): void | Promise<void> => {
		if (!isSeedValid.valid) {
			return showMessage(isSeedValid.reason);
		}
		return onRecoverIdentity();
	};

	return (
		<View style={components.page}>
			<AccountSeedTextInput
				onChangeText={onSeedTextInput}
				onSubmitEditing={onRecoverConfirm}
				returnKeyType="done"
				valid={isSeedValid.valid}
			/>
			<Button
				title={'Import'}
				disabled={!isSeedValid.valid}
				onPress={onRecoverConfirm}
				fluid={true}
			/>
			<Button
				title={'Go back'}
				onPress={(): void => navigation.goBack()}
				fluid={true}
			/>
		</View>
	);
}

export default CreateWalletImport;
