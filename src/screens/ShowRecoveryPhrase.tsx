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

import React, { useContext, useEffect, useState } from 'react';
import { StyleSheet, Text, View } from 'react-native';
import Clipboard from '@react-native-community/clipboard';

import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import { AccountsContext } from 'stores/AccountsContext';
import { AlertStateContext } from 'stores/alertContext';
import { NavigationProps } from 'types/props';
import { words } from 'utils/native';
import TouchableItem from 'components/TouchableItem';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { resetNavigationTo } from 'utils/navigationHelpers';
import ScreenHeading from 'components/ScreenHeading';
import {
	alertIdentityCreationError
} from 'utils/alertUtils';
import Button from 'components/Button';
import { useNewSeedRef } from 'utils/seedRefHooks';

function ShowRecoveryPhrase({
	navigation,
	route
}: NavigationProps<'ShowRecoveryPhrase'>): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const [seedPhrase, setSeedPhrase] = useState('');
	const [wordsNumber, setWordsNumber] = useState(12);
	const { setAlert } = useContext(AlertStateContext);
	const createSeedRefWithNewSeed = useNewSeedRef();
	const isNew = route.params.isNew ?? false;
	const createWallet = async (): Promise<void> => {
		try {
			await accountsStore.saveNewIdentity(seedPhrase, createSeedRefWithNewSeed);
			setSeedPhrase('');
			resetNavigationTo(navigation, 'Main', { isNew: true });
		} catch (e) {
			alertIdentityCreationError(setAlert, e.message);
		}
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
			if (route.params.isNew) {
				setSeedPhrase(await words(wordsNumber));
			} else {
				setSeedPhrase(route.params.seedPhrase);
			}
		};

		setSeedPhraseAsync();
		return (): void => {
			setSeedPhrase('');
		};
	}, [route.params, wordsNumber]);

	return (
		<SafeAreaViewContainer>
			<ScreenHeading
				title={'Recovery Phrase'}
				subtitle={
					'Write these words down on paper. Keep the backup paper safe. These words allow anyone to recover this account and access its funds.'
				}
			/>
			{isNew && (
				<View style={styles.mnemonicSelectionRow}>
					{renderTextButton(12)}
					{renderTextButton(24)}
				</View>
			)}
			<TouchableItem
				onPress={(): void => {
					// only allow the copy of the recovery phrase in dev environment
					if (__DEV__) {
						Clipboard.setString(seedPhrase);
					}
				}}
			>
				<Text
					style={[fontStyles.t_seed, { marginHorizontal: 16 }]}
					testID={testIDs.ShowRecoveryPhrase.seedText}
				>
					{seedPhrase}
				</Text>
			</TouchableItem>
			{isNew && (
				<Button
					title={'Next'}
					testID={testIDs.ShowRecoveryPhrase.nextButton}
					onPress={createWallet}
					aboveKeyboard
				/>
			)}
		</SafeAreaViewContainer>
	);
}

export default ShowRecoveryPhrase;

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
