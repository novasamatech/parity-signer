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

import React, { useEffect, useState } from 'react';
import { StyleSheet, Text, View } from 'react-native';
import Clipboard from '@react-native-community/clipboard';
import { showMessage } from 'react-native-flash-message';

import { components } from 'styles';
import { NavigationProps } from 'types/props';
import { words } from 'utils/native';
import TouchableItem from 'components/TouchableItem';
import Button from 'components/Button';

function CreateWallet2({
	navigation,
	route
}: NavigationProps<'CreateWallet2'>): React.ReactElement {
	const [seedPhrase, setSeedPhrase] = useState('');
	const [wordsNumber, setWordsNumber] = useState(12);

	useEffect((): (() => void) => {
		const setSeedPhraseAsync = async (): Promise<void> => {
			setSeedPhrase(await words(wordsNumber));
		};

		setSeedPhraseAsync();
		return (): void => {
			setSeedPhrase('');
		};
	}, [route.params, wordsNumber]);

	return (
		<View style={components.page}>
			<Text style={components.textBlock}>
				Save this phrase somewhere secure.
			</Text>
			<Text style={components.textBlock}>
				Do not screenshot or save it on your computer, or anyone with access
				could compromise your account.
			</Text>
			<TouchableItem
				onPress={(): void => {
					// only allow the copy of the key phrase in dev environment
					if (__DEV__) {
						showMessage('Recovery phrase copied.');
						Clipboard.setString(seedPhrase);
					}
				}}
				style={components.textBlockPreformatted}
			>
				<Text style={components.textBlockPreformattedText}>{seedPhrase}</Text>
			</TouchableItem>
	    <View style={{ flexDirection: 'row', paddingBottom: 20 }}>
				<View style={{ flex: 1, flexDirection: 'row' }}>
					<Button
						title={'12 words'}
						onPress={(): void => setWordsNumber(12)}
						fluid={'left'}
						inactive={wordsNumber !== 12}
						secondary={wordsNumber === 12}
					/>
				</View>
				<View style={{ flex: 1, flexDirection: 'row', paddingRight: 10 }}>
					<Button
						title={'24 words'}
						onPress={(): void => setWordsNumber(24)}
						fluid={'right'}
						inactive={wordsNumber !== 24}
						secondary={wordsNumber === 24}
					/>
				</View>
			</View>
			<Button
				title={'Continue'}
				onPress={(): void =>
					navigation.navigate('CreateWallet3', { seedPhrase })
				}
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

export default CreateWallet2;
