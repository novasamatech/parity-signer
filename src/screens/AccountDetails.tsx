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

import React from 'react';
import { StyleSheet, Text, View } from 'react-native';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import { NETWORK_LIST, NetworkProtocols } from 'constants/networkSpecs';
import colors from 'styles/colors';
import fonts from 'styles/fonts';
import AccountCard from 'components/AccountCard';
import QrView from 'components/QrView';
import PopupMenu from 'components/PopupMenu';
import { alertDeleteLegacyAccount } from 'utils/alertUtils';
import {
	navigateToLandingPage,
	navigateToLegacyAccountList
} from 'utils/navigationHelpers';
import fontStyles from 'styles/fontStyles';
import UnknownAccountWarning from 'components/UnknownAccountWarning';
import { withAccountStore } from 'utils/HOC';
import AccountIcon from 'components/AccountIcon';
import { NavigationAccountProps } from 'types/props';

function AccountDetails({
	accounts,
	navigation
}: NavigationAccountProps<'AccountDetails'>): React.ReactElement {
	const account = accounts.getSelected();
	const selectedKey = accounts.getSelectedKey();

	if (!account) return <View />;

	const protocol =
		(account.networkKey &&
			NETWORK_LIST[account.networkKey] &&
			NETWORK_LIST[account.networkKey].protocol) ||
		NetworkProtocols.UNKNOWN;

	const onDelete = (): void => {
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

	const onOptionSelect = (value: string): void => {
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
		<SafeAreaScrollViewContainer contentContainerStyle={styles.scrollBody}>
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
		</SafeAreaScrollViewContainer>
	);
}

export default withAccountStore(AccountDetails);

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1
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
	scrollBody: {
		alignContent: 'flex-start',
		flex: 1,
		paddingBottom: 40,
		paddingTop: 8
	},
	title: {
		color: colors.bg_text_sec,
		flexDirection: 'column',
		fontFamily: fonts.bold,
		fontSize: 18,
		justifyContent: 'center'
	}
});
