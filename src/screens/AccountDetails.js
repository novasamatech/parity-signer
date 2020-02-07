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
import colors from '../colors';
import fonts from '../fonts';
import AccountCard from '../components/AccountCard';
import QrView from '../components/QrView';
import PopupMenu from '../components/PopupMenu';
import { NETWORK_LIST, NetworkProtocols } from '../constants';
import { alertDeleteLegacyAccount } from '../util/alertUtils';
import {
	navigateToLandingPage,
	navigateToLegacyAccountList
} from '../util/navigationHelpers';
import fontStyles from '../fontStyles';
import UnknownAccountWarning from '../components/UnknownAccountWarning';
import { withAccountStore } from '../util/HOC';
import AccountIcon from '../components/AccountIcon';

export default withAccountStore(AccountDetails);

function AccountDetails({ accounts, navigation }) {
	const account = accounts.getSelected();
	const selectedKey = accounts.getSelectedKey();

	if (!account) return null;

	const protocol =
		(account.networkKey &&
			NETWORK_LIST[account.networkKey] &&
			NETWORK_LIST[account.networkKey].protocol) ||
		NetworkProtocols.UNKNOWN;

	const onDelete = () => {
		alertDeleteLegacyAccount(
			account.name || account.address || 'this account',
			async () => {
				await accounts.deleteAccount(selectedKey);
				if (accounts.getAccounts().size === 0) {
					return navigateToLandingPage(navigation);
				}
				navigateToLegacyAccountList(navigation);
			}
		);
	};

	const onOptionSelect = value => {
		if (value !== 'AccountEdit') {
			navigation.navigate('AccountUnlock', {
				next: value,
				onDelete
			});
		} else {
			navigation.navigate(value);
		}
	};

	return (
		<ScrollView contentContainerStyle={styles.body}>
			<View style={styles.bodyContent}>
				<View style={styles.header}>
					<AccountIcon
						address={''}
						network={NETWORK_LIST[account.networkKey]}
						style={styles.icon}
					/>
					<Text style={fontStyles.h2}>Public Address</Text>
					<View style={styles.menuView}>
						<PopupMenu
							onSelect={onOptionSelect}
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
			<View>
				{protocol !== NetworkProtocols.UNKNOWN ? (
					<QrView
						data={account.name ? `${selectedKey}:${account.name}` : selectedKey}
					/>
				) : (
					<UnknownAccountWarning />
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
		paddingTop: 8
	},
	deleteText: {
		color: colors.bg_alert
	},
	header: {
		alignItems: 'center',
		flexDirection: 'row',
		paddingBottom: 24,
		paddingRight: 19
	},
	icon: {
		paddingHorizontal: 16
	},
	menuView: {
		alignItems: 'flex-end',
		flex: 1
	},
	title: {
		color: colors.bg_text_sec,
		flexDirection: 'column',
		fontFamily: fonts.bold,
		fontSize: 18,
		justifyContent: 'center'
	}
});
