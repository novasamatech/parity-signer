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

import React from 'react';
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
import AccountCard from '../components/AccountCard';
import Background from '../components/Background';
import Button from '../components/Button';
import TouchableItem from '../components/TouchableItem';
import DerivationPasswordVerify from '../components/DerivationPasswordVerify';
import AccountsStore from '../stores/AccountsStore';
import { NetworkProtocols, NETWORK_LIST } from '../constants';

export default class AccountBackup extends React.PureComponent {
	static navigationOptions = {
		title: 'Account Backup'
	};
	render() {
		return (
			<Subscribe to={[AccountsStore]}>
				{accounts => <AccountBackupView {...this.props} accounts={accounts} />}
			</Subscribe>
		);
	}
}

class AccountBackupView extends React.PureComponent {
	constructor(...args) {
		super(...args);
		this.handleAppStateChange = this.handleAppStateChange.bind(this);
	}

	componentDidMount() {
		AppState.addEventListener('change', this.handleAppStateChange);
	}

	handleAppStateChange = nextAppState => {
		if (nextAppState === 'inactive') {
			this.props.navigation.goBack();
		}
	};

	componentWillUnmount() {
		const { accounts } = this.props;
		const selectedKey = accounts.getSelectedKey();

		if (selectedKey) {
			accounts.lockAccount(selectedKey);
		}

		AppState.removeEventListener('change', this.handleAppStateChange);
	}

	render() {
		const { accounts, navigation } = this.props;
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
			<ScrollView
				style={styles.body}
				contentContainerStyle={styles.bodyContent}
			>
				<Background />
				<Text style={styles.titleTop}>BACKUP ACCOUNT</Text>
				<AccountCard address={address} networkKey={networkKey} title={name} />
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
					<Text style={styles.seedText}>{seedPhrase || seed}</Text>
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
			</ScrollView>
		);
	}
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column',
		padding: 20
	},
	bodyContainer: {
		flex: 1,
		flexDirection: 'column',
		justifyContent: 'space-between'
	},
	bodyContent: {
		paddingBottom: 40
	},
	bottom: {
		flexBasis: 50,
		paddingBottom: 15
	},
	deleteButton: {
		backgroundColor: colors.bg_alert
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
	hintText: {
		color: colors.bg_text_sec,
		fontFamily: fonts.bold,
		fontSize: 12,
		paddingBottom: 20,
		textAlign: 'center'
	},
	nextStep: {
		marginTop: 20
	},
	seedText: {
		backgroundColor: colors.card_bg,
		fontFamily: fonts.regular,
		fontSize: 20,
		lineHeight: 26,
		minHeight: 160,
		padding: 10
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
	},
	top: {
		flex: 1
	}
});
