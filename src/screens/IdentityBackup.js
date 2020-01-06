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
import { withAccountStore } from '../util/HOC';
import testIDs from '../../e2e/testIDs';
import ScreenHeading from '../components/ScreenHeading';
import { alertBackupDone, alertCopyBackupPhrase } from '../util/alertUtils';
import Button from '../components/Button';

function IdentityBackup({ navigation, accounts }) {
	const [seedPhrase, setSeedPhrase] = useState('');
	const [wordsNumber, setWordsNumber] = useState(12);
	const isNew = navigation.getParam('isNew', false);
	const onBackupDone = async () => {
		const pin = await setPin(navigation);
		await accounts.saveNewIdentity(seedPhrase, pin);
		setSeedPhrase('');
		navigateToNewIdentityNetwork(navigation);
	};

	const renderTextButton = buttonWordsNumber => {
		const textStyles =
			wordsNumber === buttonWordsNumber
				? { ...fontStyles.t_codeS, color: colors.label_text }
				: fontStyles.t_codeS;
		return (
			<Button
				buttonStyles={styles.mnemonicSelectionButton}
				textStyles={textStyles}
				title={`${buttonWordsNumber} words`}
				onPress={() => setWordsNumber(buttonWordsNumber)}
			/>
		);
	};
	useEffect(() => {
		const setSeedPhraseAsync = async () => {
			if (isNew) {
				setSeedPhrase(await words(wordsNumber));
			} else {
				const backupSeedPhrase = await unlockSeedPhrase(navigation);
				navigation.pop();
				setSeedPhrase(backupSeedPhrase);
			}
		};

		setSeedPhraseAsync();
		return () => {
			setSeedPhrase('');
		};
	}, [isNew, navigation, wordsNumber]);

	return (
		<ScrollView style={styles.body}>
			<ScreenHeading
				title={'Recovery Phrase'}
				subtitle={
					' Write these words down on paper. Keep the backup paper safe. These words allow anyone to recover this account and access its funds.'
				}
			/>
			<View />
			{isNew && (
				<View style={styles.mnemonicSelectionRow}>
					{renderTextButton(12)}
					{renderTextButton(24)}
				</View>
			)}
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
	},
	mnemonicSelectionButton: {
		backgroundColor: colors.bg,
		flex: 1,
		height: 30,
		paddingHorizontal: 5,
		paddingVertical: 5
	},
	mnemonicSelectionRow: {
		flex: 1,
		flexDirection: 'row',
		justifyContent: 'space-around'
	}
});
