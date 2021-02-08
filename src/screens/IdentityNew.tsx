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

import AccountSeed from 'components/AccountSeed';
import Button from 'components/Button';
import ScreenHeading from 'components/ScreenHeading';
import TextInput from 'components/TextInput';
import testIDs from 'e2e/testIDs';
import { KeyboardAwareContainer } from 'modules/unlock/components/Container';
import React, { useContext, useEffect, useRef, useState } from 'react';
import { StyleSheet, View } from 'react-native';
import { AccountsContext } from 'stores/AccountsContext';
import { AlertStateContext } from 'stores/alertContext';
import colors from 'styles/colors';
import { NavigationProps } from 'types/props';
import { validateSeed } from 'utils/account';
import { alertError, alertIdentityCreationError, alertRisks } from 'utils/alertUtils';
import { debounce } from 'utils/debounce';
import { emptyIdentity } from 'utils/identitiesUtils';
import { brainWalletAddress } from 'utils/native';
import { navigateToNewIdentityNetwork, setPin } from 'utils/navigationHelpers';
import { useNewSeedRef } from 'utils/seedRefHooks';

function IdentityNew({ navigation, route }: NavigationProps<'IdentityNew'>): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const defaultSeedValidObject = validateSeed('', false);
	const isRecoverDefaultValue = route.params?.isRecover ?? false;
	const [isRecover, setIsRecover] = useState(isRecoverDefaultValue);
	const [isSeedValid, setIsSeedValid] = useState(defaultSeedValidObject);
	const [seedPhrase, setSeedPhrase] = useState('');
	const { setAlert } = useContext(AlertStateContext);
	const createSeedRefWithNewSeed = useNewSeedRef();
	const clearIdentity = useRef(() =>
		accountsStore.updateNewIdentity(emptyIdentity()));
	const [isValidBIP39, setIsBIP39Valid] = useState(false);

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
					setIsBIP39Valid(bip39)
					setIsSeedValid(validateSeed(inputSeedPhrase, bip39));
				})
				.catch(() => setIsSeedValid(defaultSeedValidObject));
		const debouncedAddressGeneration = debounce(addressGeneration, 200);

		debouncedAddressGeneration();
	};

	const onRecoverIdentity = async (): Promise<void> => {
		const pin = await setPin(navigation);

		try {
			if (isSeedValid.bip39) {
				await accountsStore.saveNewIdentity(seedPhrase.trimEnd(),
					pin,
					createSeedRefWithNewSeed);
			} else {
				await accountsStore.saveNewIdentity(seedPhrase,
					pin,
					createSeedRefWithNewSeed);
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
		navigation.navigate('IdentityBackup', { isNew: true });
	};

	const renderRecoverView = (): React.ReactElement => (
		<>
			<AccountSeed
				onChangeText={onSeedTextInput}
				onSubmitEditing={onRecoverConfirm}
				returnKeyType="done"
				testID={testIDs.IdentityNew.seedInput}
				valid={isValidBIP39}
			/>
			<View style={styles.btnBox}>
				<Button
					onPress={onRecoverConfirm}
					small={true}
					testID={testIDs.IdentityNew.recoverButton}
					title="Recover"
				/>
				<Button
					onPress={(): void => {
						setIsRecover(false);
					}}
					onlyText={true}
					small={true}
					title="or create new identity"
				/>
			</View>
		</>
	);

	const renderCreateView = (): React.ReactElement => (
		<View style={styles.btnBox}>
			<Button
				onPress={onCreateNewIdentity}
				small={true}
				testID={testIDs.IdentityNew.createButton}
				title="Create"
			/>
			<Button
				onPress={(): void => setIsRecover(true)}
				onlyText={true}
				small={true}
				title="or recover existing identity"
			/>
		</View>
	);

	return (
		<KeyboardAwareContainer>
			<ScreenHeading title={'New Identity'} />
			<TextInput
				onChangeText={updateName}
				placeholder="Identity Name"
				testID={testIDs.IdentityNew.nameInput}
				value={accountsStore.state.newIdentity.name}
			/>
			{isRecover ? renderRecoverView() : renderCreateView()}
		</KeyboardAwareContainer>
	);
}

export default IdentityNew;

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
