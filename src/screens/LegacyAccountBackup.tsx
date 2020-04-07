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
import { AppState, AppStateStatus, StyleSheet, Text, View } from 'react-native';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import { NetworkProtocols, NETWORK_LIST } from 'constants/networkSpecs';
import { UnlockedAccount } from 'types/identityTypes';
import { NavigationAccountProps } from 'types/props';
import colors from 'styles/colors';
import fonts from 'styles/fonts';
import fontStyles from 'styles/fontStyles';
import AccountCard from 'components/AccountCard';
import Button from 'components/Button';
import ScreenHeading from 'components/ScreenHeading';
import TouchableItem from 'components/TouchableItem';
import DerivationPasswordVerify from 'components/DerivationPasswordVerify';
import { withAccountStore } from 'utils/HOC';
import { alertBackupDone, alertCopyBackupPhrase } from 'utils/alertUtils';

function LegacyAccountBackup({
	accounts,
	navigation,
	route
}: NavigationAccountProps<'LegacyAccountBackup'>): React.ReactElement {
	useEffect(() => {
		const handleAppStateChange = (nextAppState: AppStateStatus): void => {
			if (nextAppState === 'inactive') {
				navigation.goBack();
			}
		};

		AppState.addEventListener('change', handleAppStateChange);
		return (): void => {
			const selectedKey = accounts.getSelectedKey();

			if (selectedKey) {
				accounts.lockAccount(selectedKey);
			}

			AppState.removeEventListener('change', handleAppStateChange);
		};
	}, [navigation, accounts]);

	const { navigate } = navigation;
	const isNew = route.params?.isNew ?? false;
	const {
		address,
		derivationPassword = '',
		derivationPath = '',
		name,
		networkKey,
		seed = '',
		seedPhrase = ''
	} = isNew ? accounts.getNew() : (accounts.getSelected() as UnlockedAccount);
	const protocol =
		(NETWORK_LIST[networkKey] && NETWORK_LIST[networkKey].protocol) ||
		NetworkProtocols.UNKNOWN;

	return (
		<SafeAreaScrollViewContainer style={styles.body}>
			<ScreenHeading
				title="Recovery Phrase"
				subtitle="Write these words down on paper. Keep the backup paper safe. These
				words allow anyone to recover this account and access its funds."
			/>

			<AccountCard address={address} networkKey={networkKey} title={name} />
			<View style={styles.bodyContent}>
				<TouchableItem
					onPress={(): void => {
						// only allow the copy of the recovery phrase in dev environment
						if (__DEV__) {
							if (protocol === NetworkProtocols.SUBSTRATE) {
								alertCopyBackupPhrase(`${seedPhrase}${derivationPath}`);
							} else {
								alertCopyBackupPhrase(seedPhrase === '' ? seed : seedPhrase);
							}
						}
					}}
				>
					<Text style={fontStyles.t_seed}>
						{seedPhrase === '' ? seed : seedPhrase}
					</Text>
				</TouchableItem>
				{derivationPath !== '' && (
					<Text style={styles.derivationText}>{derivationPath || ''}</Text>
				)}
				{derivationPassword !== '' && (
					<DerivationPasswordVerify password={derivationPassword} />
				)}
				{isNew && (
					<Button
						buttonStyles={StyleSheet.flatten([
							styles.nextStep,
							{ marginBottom: 20 }
						])}
						title="Backup Done"
						onPress={(): void => {
							alertBackupDone(() => {
								navigate('AccountPin', { isNew });
							});
						}}
					/>
				)}
			</View>
		</SafeAreaScrollViewContainer>
	);
}

export default withAccountStore(LegacyAccountBackup);

const styles = StyleSheet.create({
	body: {
		alignContent: 'flex-start',
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
