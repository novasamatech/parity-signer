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

import React, { useContext, useEffect, useState } from 'react';
import { StyleSheet, Text, View } from 'react-native';

import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import { AlertStateContext } from 'stores/alertContext';
import { NavigationProps } from 'types/props';
import { getSeedPhraseForBackup } from 'utils/native';
import TouchableItem from 'components/TouchableItem';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { navigateToLandingPage } from 'utils/navigationHelpers';
import ScreenHeading from 'components/ScreenHeading';
import {
	alertBackupDone,
	alertCopyBackupPhrase,
	alertIdentityCreationError
} from 'utils/alertUtils';
import Button from 'components/Button';

function SeedBackup({
	navigation,
	route
}: NavigationProps<'SeedBackup'>): React.ReactElement {
	const [seedPhrase, setSeedPhrase] = useState('');
	const seedName = route.params.seedName;
	const { setAlert } = useContext(AlertStateContext);
	const onBackupDone = async (): Promise<void> => {
		navigateToLandingPage(navigation);
	};

	const renderTextButton = (buttonWordsNumber: number): React.ReactElement => {
		const textStyles = wordsNumber === buttonWordsNumber && {
			color: colors.signal.main
		};
		return (
			<Button
				title={`${buttonWordsNumber} words`}
				onPress={(): void => setWordsNumber(buttonWordsNumber)}
				onlyText
				small
				textStyles={{ ...textStyles }}
			/>
		);
	};

	useEffect((): (() => void) => {
		const setSeedPhraseAsync = async (): Promise<void> => {
			try {
				const fetchedSeedPhrase = await getSeedPhraseForBackup(
					seedName,
					'000000'
				);
				setSeedPhrase(fetchedSeedPhrase);
			} catch (e) {
				console.log('seed phrase fetch failed, system corrupted');
				console.log(e);
				setAlert(e);
			}
		};
		setSeedPhraseAsync();
	}, [seedName]);

	return (
		<SafeAreaViewContainer>
			<ScreenHeading
				title={'Recovery Phrase'}
				subtitle={
					'Write these words down on paper. Keep the backup paper safe. These words allow anyone to recover your identities and gain full access to them.'
				}
			/>
			<Text
				style={[fontStyles.t_seed, { marginHorizontal: 16 }]}
				testID={testIDs.IdentityBackup.seedText}
			>
				{seedPhrase}
			</Text>
			<Button
				title={'Done'}
				testID={testIDs.IdentityBackup.nextButton}
				onPress={(): void => alertBackupDone(setAlert, onBackupDone)}
				aboveKeyboard
			/>
		</SafeAreaViewContainer>
	);
}

export default SeedBackup;

const styles = StyleSheet.create({
	body: {
		padding: 16
	},
	mnemonicSelectionButton: {
		backgroundColor: colors.background.app,
		flex: 1,
		height: 30,
		paddingHorizontal: 4,
		paddingVertical: 4
	},
	mnemonicSelectionRow: {
		flexDirection: 'row',
		justifyContent: 'space-around'
	}
});
