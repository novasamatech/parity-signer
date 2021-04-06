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
import { StyleSheet } from 'react-native';

import { colors } from 'styles';
import { AccountsContext } from 'stores/AccountsContext';
import { AlertStateContext } from 'stores/alertContext';
import Button from 'components/Button';
import { NavigationProps } from 'types/props';
import { validateSeed } from 'utils/account';
import AccountSeed from 'components/AccountSeed';
import { resetNavigationTo } from 'utils/navigationHelpers';
import { alertError, alertIdentityCreationError } from 'utils/alertUtils';
import ScreenHeading from 'components/ScreenHeading';
import { brainWalletAddress } from 'utils/native';
import { debounce } from 'utils/debounce';
import { useNewSeedRef } from 'utils/seedRefHooks';
import KeyboardScrollView from 'components/KeyboardScrollView';

function CreateWalletImport({
	navigation
}: NavigationProps<'CreateWalletImport'>): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const defaultSeedValidObject = validateSeed('', false);
	const [isSeedValid, setIsSeedValid] = useState(defaultSeedValidObject);
	const [seedPhrase, setSeedPhrase] = useState('');
	const { setAlert } = useContext(AlertStateContext);
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
			alertIdentityCreationError(setAlert, e.message);
		}
	};

	const onRecoverConfirm = (): void | Promise<void> => {
		if (!isSeedValid.valid) {
			return alertError(setAlert, `${isSeedValid.reason}`);
		}
		return onRecoverIdentity();
	};

	return (
		<KeyboardScrollView bounces={false} style={styles.body}>
			<ScreenHeading title={'Import Wallet'} />
			<AccountSeed
				onChangeText={onSeedTextInput}
				onSubmitEditing={onRecoverConfirm}
				returnKeyType="done"
				valid={isSeedValid.valid}
			/>
			<Button
				title={'Import'}
				onPress={onRecoverConfirm}
				disabled={!isSeedValid.valid}
			/>
			<Button title={'Go back'} onPress={(): void => navigation.goBack()} />
		</KeyboardScrollView>
	);
}

export default CreateWalletImport;

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
