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

import React, { useEffect } from 'react';
import {
	Alert,
	AppState,
	Clipboard,
	ScrollView,
	StyleSheet,
	Text,
	View
} from 'react-native';
import { Subscribe } from 'unstated';

import colors from '../colors';
import fonts from '../fonts';
import fontStyles from '../fontStyles';
import AccountCard from '../components/AccountCard';
import Background from '../components/Background';
import Button from '../components/Button';
import ScreenHeading from '../components/ScreenHeading';
import TouchableItem from '../components/TouchableItem';
import DerivationPasswordVerify from '../components/DerivationPasswordVerify';
import AccountsStore from '../stores/AccountsStore';
import { NetworkProtocols, NETWORK_LIST } from '../constants';

export default class LegacyAccountBackup extends React.PureComponent {
	render() {
		return (
			<Subscribe to={[AccountsStore]}>
				{accounts => (
					<LegacyAccountBackupView {...this.props} accounts={accounts} />
				)}
			</Subscribe>
		);
	}
}

function LegacyAccountBackupView(props) {
	useEffect(() => {
		const handleAppStateChange = nextAppState => {
			if (nextAppState === 'inactive') {
				props.navigation.goBack();
			}
		};

		AppState.addEventListener('change', handleAppStateChange);
		return function() {
			const { accounts } = props;
			const selectedKey = accounts.getSelectedKey();

			if (selectedKey) {
				accounts.lockAccount(selectedKey);
			}

			AppState.removeEventListener('change', handleAppStateChange);
		};
	}, []);

	const { accounts, navigation } = props;
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
		(NETWORK_LIST[networkKey] && NETWORK_LIST[networkKey].protocol) || '';

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
							Alert.alert(
								'Write this recovery phrase on paper',
								`It is not recommended to transfer or store a recovery phrase digitally and unencrypted. Anyone in possession of this recovery phrase is able to spend funds from this account.
                `,
								[
									{
										onPress: () => {
											if (protocol === NetworkProtocols.SUBSTRATE) {
												Clipboard.setString(`${seedPhrase}${derivationPath}`);
											} else {
												Clipboard.setString(seed);
											}
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
							Alert.alert(
								'Important',
								"Make sure you've backed up this recovery phrase. It is the only way to restore your account in case of device failure/lost.",
								[
									{
										onPress: () => {
											navigate('AccountPin', { isNew });
										},
										text: 'Proceed'
									},
									{
										style: 'cancel',
										text: 'Cancel'
									}
								]
							);
						}}
					/>
				)}
			</View>
		</ScrollView>
	);
}

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
