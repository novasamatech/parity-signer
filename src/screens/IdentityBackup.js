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

import React, { useEffect, useState } from 'react';

import { words } from '../util/native';
import {
	Alert,
	Clipboard,
	ScrollView,
	StyleSheet,
	Text,
	View
} from 'react-native';
import TouchableItem from '../components/TouchableItem';
import colors from '../colors';
import fonts from '../fonts';
import fontStyles from '../fontStyles';
import Button from '../components/Button';
import { withNavigation } from 'react-navigation';
import {
	navigateToNewIdentityNetwork,
	setPin,
	unlockSeed
} from '../util/navigationHelpers';
import { withAccountStore } from '../util/HOC';

function IdentityBackup({ navigation, accounts }) {
	const [seedPhrase, setSeedPhrase] = useState('');
	const isNew = navigation.getParam('isNew', false);

	useEffect(() => {
		const setSeedPhraseAsync = async () => {
			if (isNew) {
				setSeedPhrase(await words());
			} else {
				const backupSeedPhrase = await unlockSeed(navigation);
				navigation.pop();
				setSeedPhrase(backupSeedPhrase);
			}
		};

		setSeedPhraseAsync();
		return () => {
			setSeedPhrase('');
		};
	}, [isNew, navigation]);

	return (
		<ScrollView style={styles.body}>
			<Text style={styles.titleTop}>BACKUP ACCOUNT</Text>
			<View>
				<Text style={styles.titleTop}>RECOVERY PHRASE</Text>
				<Text style={styles.hintText}>
					Write these words down on paper. Keep the backup paper safe. These
					words allow anyone to recover this account and access its funds.
				</Text>
			</View>
			<TouchableItem
				onPress={() => {
					// only allow the copy of the recovery phrase in dev environment
					if (__DEV__) {
						Alert.alert(
							'Write this recovery phrase on paper',
							`It is not recommended to transfer or store a recovery phrase digitally and unencrypted. Anyone in possession of this recovery phrase is able to spend funds from this account.
                `,
							[
								{
									onPress: () => {
										Clipboard.setString(seedPhrase);
									},
									style: 'default',
									text: 'Copy anyway'
								},
								{
									style: 'cancel',
									text: 'Cancel'
								}
							]
						);
					}
				}}
			>
				<Text style={fontStyles.t_seed}>{seedPhrase}</Text>
			</TouchableItem>
			{isNew && (
				<Button
					title="Next"
					onPress={async () => {
						const pin = await setPin(navigation);
						await accounts.saveNewIdentity(seedPhrase, pin);
						setSeedPhrase('');
						navigateToNewIdentityNetwork(navigation);
					}}
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
	hintText: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 12,
		paddingBottom: 20,
		textAlign: 'center'
	},
	title: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 18,
		paddingBottom: 20
	},
	titleTop: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 24,
		paddingBottom: 20,
		textAlign: 'center'
	}
});
