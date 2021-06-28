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

import React, { useContext, useEffect, useRef, useState } from 'react';
import { StyleSheet, View } from 'react-native';

import { KeyboardAwareContainer } from 'modules/unlock/components/Container';
import testIDs from 'e2e/testIDs';
import { AlertStateContext } from 'stores/alertContext';
import Button from 'components/Button';
import TextInput from 'components/TextInput';
import { NavigationProps } from 'types/props';
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
import PinInput from 'modules/unlock/components/PinInput';
import {tryCreateSeed, tryRecoverSeed} from 'utils/native';

function RootSeedNew({
	navigation,
	route
}: NavigationProps<'RootSeedNew'>): React.ReactElement {
	const isRecoverDefaultValue = route.params?.isRecover ?? false;
	const [isRecover, setIsRecover] = useState(isRecoverDefaultValue);
	const [seedPhrase, setSeedPhrase] = useState('');
	const [pin, setPin] = useState('');
	const { setAlert } = useContext(AlertStateContext);
	const [seedName, setSeedName] = useState('');
	const [disabled, setDisabled] = useState(false);
	const [error, setError] = useState('');
	const [cryptoType, setCryptoType] = useState('sr25519');

	const updateName = (name: string): void => {
		setSeedName(name);
		setDisabled(false);
		setError('');
	};

	const onSeedTextInput = (inputSeedPhrase: string): void => {
		setSeedPhrase(inputSeedPhrase);
		setDisabled(false);
		setError('');
	};

	const onRecoverConfirm = async (): void => {
		setDisabled(true);
		try {
			await tryRecoverSeed(seedName, cryptoType, seedPhrase, pin);
			setSeedPhrase('');
			navigation.navigate('SeedBackup', {	
				seedName
			});
		} catch (e) {
			setError(e);
		}
	};

	const onCreateNewIdentity = async (): void => {
		setDisabled(true);
		try {
			await tryCreateSeed(seedName, cryptoType, pin);
			setSeedPhrase('');
			navigation.navigate('SeedBackup', {	
				seedName
			});
		} catch (e) {
			console.log(e.toString());
			console.log(typeof e.toString());
			setError(e.toString());
		}
	};

	const renderRecoverView = (): React.ReactElement => (
		<>
			<AccountSeed
				testID={testIDs.IdentityNew.seedInput}
				onChangeText={onSeedTextInput}
				onSubmitEditing={onRecoverConfirm}
				returnKeyType="done"
				valid={isSeedValid.valid}
			/>
			<View style={styles.btnBox}>
				<Button
					title="Recover"
					testID={testIDs.IdentityNew.recoverButton}
					onPress={onRecoverConfirm}
					small={true}
					disabled={disabled || (pin.length < 6) || (seedName.length < 1) || (seedPhrase.length < 24)}
				/>
				<Button
					title="or create new seed"
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
				disabled={ disabled || (pin.length < 6) || (seedName.length < 1) }
			/>
			<Button
				title="or recover existing seed"
				onPress={(): void => setIsRecover(true)}
				small={true}
				onlyText={true}
			/>
		</View>
	);

	return (
		<KeyboardAwareContainer>
			<ScreenHeading title={'New Root Seed'} error={!!error} subtitle={error} />
			<PinInput 
				label={'Input pin'}
				returnKeyType="done"
				onChangeText={(newInput: string): void => {
					setDisabled(false);
					setPin(newInput);
					setError('');
				}}
				value={pin}
			/>
			<TextInput
				onChangeText={updateName}
				testID={testIDs.IdentityNew.nameInput}
				value={seedName}
				placeholder="Seed Name"
			/>
			{isRecover ? renderRecoverView() : renderCreateView()}
		</KeyboardAwareContainer>
	);
}

export default RootSeedNew;

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
