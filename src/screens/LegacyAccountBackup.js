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

import React, { useEffect } from 'react';
import { AppState, ScrollView, StyleSheet, Text, View } from 'react-native';
import { withNavigation } from 'react-navigation';

import colors from '../colors';
import fonts from '../fonts';
import fontStyles from '../fontStyles';
import AccountCard from '../components/AccountCard';
import Background from '../components/Background';
import Button from '../components/Button';
import ScreenHeading from '../components/ScreenHeading';
import TouchableItem from '../components/TouchableItem';
import DerivationPasswordVerify from '../components/DerivationPasswordVerify';
import { withAccountStore } from '../util/HOC';
import { NetworkProtocols, NETWORK_LIST } from '../constants';
import { alertBackupDone, alertCopyBackupPhrase } from '../util/alertUtils';

function LegacyAccountBackup({ navigation, accounts }) {
	useEffect(() => {
		const handleAppStateChange = nextAppState => {
			if (nextAppState === 'inactive') {
				navigation.goBack();
			}
		};

		AppState.addEventListener('change', handleAppStateChange);
		return () => {
			const selectedKey = accounts.getSelectedKey();

			if (selectedKey) {
				accounts.lockAccount(selectedKey);
			}

			AppState.removeEventListener('change', handleAppStateChange);
		};
	}, [navigation, accounts]);

	const { navigate } = navigation;
	const isNew = navigation.getParam('isNew');
	const {
		address,
		derivationPassword,
		derivationPath,
		name,
		networkKey,
		seed,
		seedPhrase
	} = isNew ? accounts.getNew() : accounts.getSelected();
	const protocol =
		(NETWORK_LIST[networkKey] && NETWORK_LIST[networkKey].protocol) ||
		NetworkProtocols.UNKNOWN;

	return (
		<ScrollView style={styles.body}>
			<Background />
			<ScreenHeading
				title="Recovery Phrase"
				subtitle="Write these words down on paper. Keep the backup paper safe. These
				words allow anyone to recover this account and access its funds."
			/>

			<AccountCard address={address} networkKey={networkKey} title={name} />
			<View style={styles.bodyContent}>
				<TouchableItem
					onPress={() => {
						// only allow the copy of the recovery phrase in dev environment
						if (__DEV__) {
							if (protocol === NetworkProtocols.SUBSTRATE) {
								alertCopyBackupPhrase(`${seedPhrase}${derivationPath}`);
							} else {
								alertCopyBackupPhrase(seed);
							}
						}
					}}
				>
					<Text style={fontStyles.t_seed}>{seedPhrase || seed}</Text>
				</TouchableItem>
				{!!derivationPath && (
					<Text style={styles.derivationText}>{derivationPath}</Text>
				)}
				{!!derivationPassword && (
					<DerivationPasswordVerify password={derivationPassword} />
				)}
				{isNew && (
					<Button
						buttonStyles={[styles.nextStep, { marginBottom: 20 }]}
						title="Backup Done"
						onPress={() => {
							alertBackupDone(() => {
								navigate('AccountPin', { isNew });
							});
						}}
					/>
				)}
			</View>
		</ScrollView>
	);
}

export default withAccountStore(withNavigation(LegacyAccountBackup));

const styles = StyleSheet.create({
	body: {
		alignContent: 'flex-start',
		backgroundColor: colors.bg,
		flex: 1,
		paddingBottom: 40,
		paddingTop: 24
	},
	bodyContent: {
		padding: 16
	},
	derivationText: {
		backgroundColor: colors.card_bg,
		fontFamily: fonts.regular,
		fontSize: 20,
		lineHeight: 26,
		marginTop: 20,
		minHeight: 30,
		padding: 10
	},
	nextStep: {
		marginTop: 20
	},
	title: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 18,
		paddingBottom: 20
	}
});
