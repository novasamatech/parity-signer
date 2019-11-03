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
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import { Subscribe } from 'unstated';
import colors from '../colors';
import fonts from '../fonts';
import AccountCard from '../components/AccountCard';
import QrView from '../components/QrView';
import AccountsStore from '../stores/AccountsStore';
import TxStore from '../stores/TxStore';
import PopupMenu from '../components/PopupMenu';
import { NETWORK_LIST, NetworkProtocols } from '../constants';
import { alertDeleteAccount } from '../util/alertUtils';
import { navigateToLegacyAccountList } from '../util/navigationHelpers';

export default class AccountDetails extends React.Component {
	render() {
		return (
			<Subscribe to={[AccountsStore, TxStore]}>
				{(accounts, txStore) => (
					<AccountDetailsView
						{...this.props}
						txStore={txStore}
						accounts={accounts}
						selected={accounts.getSelected() && accounts.getSelected().address}
					/>
				)}
			</Subscribe>
		);
	}
}

class AccountDetailsView extends React.Component {
	constructor(props) {
		super(props);
	}

	onDelete = () => {
		const accounts = this.props.accounts;
		const selected = accounts.getSelected();
		const selectedKey = accounts.getSelectedKey();

		alertDeleteAccount(
			selected.name || selected.address || 'this account',
			async () => {
				await accounts.deleteAccount(selectedKey);
				navigateToLegacyAccountList(this.props.navigation);
			}
		);
	};

	onOptionSelect = value => {
		const navigate = this.props.navigation.navigate;

		if (value !== 'AccountEdit') {
			navigate('AccountUnlock', {
				next: value,
				onDelete: this.onDelete
			});
		} else {
			navigate(value);
		}
	};

	renderWarningUnknownAccount = function() {
		return (
			<View style={styles.warningView}>
				<Text style={{ ...styles.title, ...styles.warningTitle }}>Warning</Text>
				<Text>
					This account wasn't retrieved successfully. This could be because its
					network isn't supported, or you upgraded Parity Signer without wiping
					your device and this account couldn't be migrated.
					{'\n'}
					{'\n'}
					To be able to use this account you need to:{'\n'}- write down its
					recovery phrase{'\n'}- delete it{'\n'}- recover it{'\n'}
				</Text>
			</View>
		);
	};

	render() {
		const { accounts } = this.props;
		const account = accounts.getSelected();
		const selectedKey = accounts.getSelectedKey();

		if (!account) {
			return null;
		}

		const protocol =
			(account.networkKey &&
				NETWORK_LIST[account.networkKey] &&
				NETWORK_LIST[account.networkKey].protocol) ||
			NetworkProtocols.UNKNOWN;

		return (
			<ScrollView contentContainerStyle={styles.body}>
				<View style={styles.bodyContent}>
					<View style={styles.header}>
						<Text style={styles.title}>ACCOUNT</Text>
						<View style={styles.menuView}>
							<PopupMenu
								onSelect={this.onOptionSelect}
								menuTriggerIconName={'more-vert'}
								menuItems={[
									{ text: 'Edit', value: 'AccountEdit' },
									{ text: 'Change Pin', value: 'AccountPin' },
									{
										text: 'View Recovery Phrase',
										value: 'LegacyAccountBackup'
									},
									{
										text: 'Delete',
										textStyle: styles.deleteText,
										value: 'AccountDelete'
									}
								]}
							/>
						</View>
					</View>
				</View>
				<AccountCard
					address={account.address}
					networkKey={account.networkKey}
					title={account.name}
				/>
				<View style={styles.bodyContent}>
					<View style={styles.qr}>
						{protocol !== NetworkProtocols.UNKNOWN ? (
							<QrView data={selectedKey} />
						) : (
							this.renderWarningUnknownAccount()
						)}
					</View>
				</View>
			</ScrollView>
		);
	}
}

const styles = StyleSheet.create({
	body: {
		alignContent: 'flex-start',
		backgroundColor: colors.bg,
		flex: 1,
		paddingBottom: 40,
		paddingTop: 8
	},
	bodyContent: {
		padding: 16
	},
	deleteText: {
		color: colors.bg_alert
	},
	header: {
		alignItems: 'center',
		flexDirection: 'row',
		justifyContent: 'center'
	},
	menuView: {
		alignItems: 'flex-end',
		flex: 1
	},
	qr: {
		backgroundColor: colors.card_bg,
		marginTop: 20
	},
	title: {
		color: colors.bg_text_sec,
		flexDirection: 'column',
		fontFamily: fonts.bold,
		fontSize: 18,
		justifyContent: 'center'
	},
	warningTitle: {
		color: colors.bg_alert,
		fontSize: 20,
		marginBottom: 10
	},
	warningView: {
		padding: 20
	}
});
