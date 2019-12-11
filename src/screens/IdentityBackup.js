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

import { words } from '../util/native';
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import TouchableItem from '../components/TouchableItem';
import colors from '../colors';
import fontStyles from '../fontStyles';
import ButtonMainAction from '../components/ButtonMainAction';
import { withNavigation } from 'react-navigation';
import {
	navigateToNewIdentityNetwork,
	setPin,
	unlockSeedPhrase
} from '../util/navigationHelpers';
import { unlockIdentitySeedWithBiometric } from '../util/identitiesUtils';
import { withAccountStore } from '../util/HOC';
import testIDs from '../../e2e/testIDs';
import ScreenHeading from '../components/ScreenHeading';
import { alertBackupDone, alertCopyBackupPhrase } from '../util/alertUtils';

function IdentityBackup({ navigation, accounts }) {
	const { currentIdentity } = accounts.state;
	const [seedPhrase, setSeedPhrase] = useState('');
	const isNew = navigation.getParam('isNew', false);
	const onBackupDone = async () => {
		const pin = await setPin(navigation);
		await accounts.saveNewIdentity(seedPhrase, pin);
		setSeedPhrase('');
		navigateToNewIdentityNetwork(navigation);
	};

	useEffect(() => {
		const onUnlockSuccess = backupSeedPhrase => {
			navigation.pop();
			setSeedPhrase(backupSeedPhrase);
		};
		const setSeedPhraseAsync = async () => {
			if (isNew) {
				setSeedPhrase(await words());
			} else {
				let backupSeedPhrase;
				if (currentIdentity.biometricEnabled) {
					backupSeedPhrase = await unlockIdentitySeedWithBiometric(
						currentIdentity
					);
					if (backupSeedPhrase) return onUnlockSuccess(backupSeedPhrase);
				}
				backupSeedPhrase = await unlockSeedPhrase(navigation);
				onUnlockSuccess(backupSeedPhrase);
			}
		};

		setSeedPhraseAsync();
		return () => {
			setSeedPhrase('');
		};
	}, [currentIdentity, isNew, navigation]);

	return (
		<ScrollView style={styles.body}>
			<ScreenHeading
				title={'Recovery Phrase'}
				subtitle={
					' Write these words down on paper. Keep the backup paper safe. These words allow anyone to recover this account and access its funds.'
				}
			/>
			<View />
			<TouchableItem
				onPress={() => {
					// only allow the copy of the recovery phrase in dev environment
					if (__DEV__) {
						alertCopyBackupPhrase(seedPhrase);
					}
				}}
			>
				<Text
					style={fontStyles.t_seed}
					testID={testIDs.IdentityBackup.seedText}
				>
					{seedPhrase}
				</Text>
			</TouchableItem>
			{isNew && (
				<ButtonMainAction
					title={'Next'}
					testID={testIDs.IdentityBackup.nextButton}
					bottom={false}
					onPress={() => alertBackupDone(onBackupDone)}
				/>
			)}
		</ScrollView>
	);
}

export default withAccountStore(withNavigation(IdentityBackup));

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column',
		padding: 16
	}
});
