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
import { StyleSheet, View } from 'react-native';
import { withNavigation } from 'react-navigation';

import Button from '../components/Button';
import TextInput from '../components/TextInput';
import { emptyIdentity } from '../util/identitiesUtils';
import colors from '../colors';
import fonts from '../fonts';
import { withAccountStore } from '../util/HOC';
import { validateSeed } from '../util/account';
import AccountSeed from '../components/AccountSeed';
import {
	navigateToNewIdentityNetwork,
	setPin
} from '../util/navigationHelpers';
import {
	alertErrorWithMessage,
	alertIdentityCreationError,
	alertRisks
} from '../util/alertUtils';
import testIDs from '../../e2e/testIDs';
import ScreenHeading from '../components/ScreenHeading';
import KeyboardScrollView from '../components/KeyboardScrollView';
import { brainWalletAddress } from '../util/native';
import { debounce } from '../util/debounce';

function IdentityNew({ accounts, navigation }) {
	const isRecoverDefaultValue = navigation.getParam('isRecover', false);
	const [isRecover, setIsRecover] = useState(isRecoverDefaultValue);
	const [isSeedValid, setIsSeedValid] = useState(false);
	const [seedPhrase, setSeedPhrase] = useState('');

	useEffect(() => {
		const clearNewIdentity = () => accounts.updateNewIdentity(emptyIdentity());
		clearNewIdentity();
		return clearNewIdentity;
	}, [accounts]);

	const updateName = name => {
		accounts.updateNewIdentity({ name });
	};

	const onSeedTextInput = inputSeedPhrase => {
		setSeedPhrase(inputSeedPhrase);
		const addressGeneration = () =>
			brainWalletAddress(inputSeedPhrase)
				.then(({ bip39 }) => {
					setIsSeedValid(validateSeed(inputSeedPhrase, bip39));
				})
				.catch(() => setIsSeedValid(inputSeedPhrase, false));
		const debouncedAddressGeneration = debounce(addressGeneration, 200);
		debouncedAddressGeneration();
	};

	const onRecoverIdentity = async () => {
		const pin = await setPin(navigation);
		try {
			await accounts.saveNewIdentity(seedPhrase, pin);
			setSeedPhrase('');
			navigateToNewIdentityNetwork(navigation);
		} catch (e) {
			alertIdentityCreationError();
		}
	};

	const onRecoverConfirm = () => {
		if (!isSeedValid.valid) {
			if (isSeedValid.accountRecoveryAllowed) {
				return alertRisks(`${isSeedValid.reason}`, onRecoverIdentity);
			} else {
				return alertErrorWithMessage(`${isSeedValid.reason}`, 'Back');
			}
		}
		return onRecoverIdentity();
	};

	const onCreateNewIdentity = () => {
		setSeedPhrase('');
		navigation.navigate('IdentityBackup', {
			isNew: true
		});
	};

	const renderRecoverView = () => (
		<>
			<AccountSeed
				testID={testIDs.IdentityNew.seedInput}
				valid={isSeedValid.valid}
				onChangeText={onSeedTextInput}
				value={seedPhrase}
			/>
			<View style={styles.btnBox}>
				<Button
					title="Create"
					onPress={() => {
						setIsRecover(false);
					}}
					small={true}
					onlyText={true}
				/>
				<Button
					title="Recover Identity"
					testID={testIDs.IdentityNew.recoverButton}
					onPress={onRecoverConfirm}
					small={true}
				/>
			</View>
		</>
	);

	const renderCreateView = () => (
		<View style={styles.btnBox}>
			<Button
				title="Recover Identity"
				onPress={() => setIsRecover(true)}
				small={true}
				onlyText={true}
			/>
			<Button
				title="Create"
				testID={testIDs.IdentityNew.createButton}
				onPress={onCreateNewIdentity}
				small={true}
			/>
		</View>
	);

	return (
		<KeyboardScrollView style={styles.body}>
			<ScreenHeading title={'New Identity'} />
			<TextInput
				onChangeText={updateName}
				testID={testIDs.IdentityNew.nameInput}
				value={accounts.getNewIdentity().name}
				placeholder="Identity Name"
				focus={false}
			/>
			{isRecover ? renderRecoverView() : renderCreateView()}
		</KeyboardScrollView>
	);
}

export default withAccountStore(withNavigation(IdentityNew));

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		overflow: 'hidden'
	},
	btnBox: {
		alignContent: 'center',
		flexDirection: 'row',
		flexWrap: 'wrap',
		justifyContent: 'space-around',
		marginTop: 32
	},
	title: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 18
	}
});
