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

import React, { useContext, useEffect, useState } from 'react';
import { StyleSheet, View } from 'react-native';

import { KeyboardAwareContainer } from 'modules/unlock/components/Container';
import testIDs from 'e2e/testIDs';
import { AccountsContext } from 'stores/AccountsContext';
import { AlertStateContext } from 'stores/alertContext';
import { NavigationProps } from 'types/props';
import Button from 'components/Button';
import TextInput from 'components/TextInput';
import { emptyIdentity } from 'utils/identitiesUtils';
import colors from 'styles/colors';
import { validateSeed } from 'utils/account';
import AccountSeed from 'components/AccountSeed';
import { navigateToNewIdentityNetwork, setPin } from 'utils/navigationHelpers';
import {
	alertError,
	alertIdentityCreationError,
	alertRisks
} from 'utils/alertUtils';
import ScreenHeading from 'components/ScreenHeading';
import { brainWalletAddress } from 'utils/native';
import { debounce } from 'utils/debounce';
import { useNewSeedRef } from 'utils/seedRefHooks';

function IdentityNew({
	navigation,
	route
}: NavigationProps<'IdentityNew'>): React.ReactElement {
	const accounts = useContext(AccountsContext);
	const defaultSeedValidObject = validateSeed('', false);
	const isRecoverDefaultValue = route.params?.isRecover ?? false;
	const [isRecover, setIsRecover] = useState(isRecoverDefaultValue);
	const [isSeedValid, setIsSeedValid] = useState(defaultSeedValidObject);
	const [seedPhrase, setSeedPhrase] = useState('');
	const { setAlert } = useContext(AlertStateContext);
	const createSeedRefWithNewSeed = useNewSeedRef();

	useEffect((): (() => void) => {
		const clearNewIdentity = (): void =>
			accounts.updateNewIdentity(emptyIdentity());
		clearNewIdentity();
		return clearNewIdentity;
	}, [accounts]);

	const updateName = (name: string): void => {
		accounts.updateNewIdentity({ name });
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
		const pin = await setPin(navigation);
		try {
			if (isSeedValid.bip39) {
				await accounts.saveNewIdentity(
					seedPhrase.trimEnd(),
					pin,
					createSeedRefWithNewSeed
				);
			} else {
				await accounts.saveNewIdentity(
					seedPhrase,
					pin,
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
		navigation.navigate('IdentityBackup', {
			isNew: true
		});
	};

	const renderRecoverView = (): React.ReactElement => (
		<>
			<AccountSeed
				testID={testIDs.IdentityNew.seedInput}
				onChangeText={onSeedTextInput}
				onSubmitEditing={onRecoverConfirm}
				returnKeyType="done"
				valid={isSeedValid.valid}
				value={seedPhrase}
			/>
			<View style={styles.btnBox}>
				<Button
					title="Recover"
					testID={testIDs.IdentityNew.recoverButton}
					onPress={onRecoverConfirm}
					small={true}
				/>
				<Button
					title="or create new identity"
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
				title="Create"
				testID={testIDs.IdentityNew.createButton}
				onPress={onCreateNewIdentity}
				small={true}
			/>
			<Button
				title="or recover existing identity"
				onPress={(): void => setIsRecover(true)}
				small={true}
				onlyText={true}
			/>
		</View>
	);

	return (
		<KeyboardAwareContainer>
			<ScreenHeading title={'New Identity'} />
			<TextInput
				onChangeText={updateName}
				testID={testIDs.IdentityNew.nameInput}
				value={accounts.getNewIdentity().name}
				placeholder="Identity Name"
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
