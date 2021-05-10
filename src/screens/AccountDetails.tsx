// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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

import React, { useContext } from 'react';
import { ScrollView, StyleSheet, Text, View } from 'react-native';

import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { NetworkProtocols } from 'constants/networkSpecs';
import { AccountsContext } from 'stores/AccountsContext';
import { AlertStateContext } from 'stores/alertContext';
import { NetworksContext } from 'stores/NetworkContext';
import colors from 'styles/colors';
import AccountCard from 'components/AccountCard';
import QrView from 'components/QrView';
import PopupMenu from 'components/PopupMenu';
import { alertDeleteLegacyAccount } from 'utils/alertUtils';
import {
	navigateToLandingPage,
	navigateToLegacyAccountList
} from 'utils/navigationHelpers';
import fontStyles from 'styles/fontStyles';
import { UnknownAccountWarning } from 'components/Warnings';
import AccountIcon from 'components/AccountIcon';
import { NavigationProps } from 'types/props';
import QrScannerTab from 'components/QrScannerTab';

export default function AccountDetails({
	navigation
}: NavigationProps<'AccountDetails'>): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const account = accountsStore.getSelected();
	const { getNetwork } = useContext(NetworksContext);
	const { setAlert } = useContext(AlertStateContext);
	const { accounts, selectedKey } = accountsStore.state;

	if (!account) return <View />;

	const network = getNetwork(account.networkKey);

	const protocol = network.protocol;

	const onDelete = (): void => {
		alertDeleteLegacyAccount(
			setAlert,
			account.name || account.address || 'this account',
			async () => {
				await accountsStore.deleteAccount(selectedKey);
				if (accounts.size === 0) {
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
		<SafeAreaViewContainer>
			<ScrollView style={styles.scrollBody} bounces={false}>
				<View style={styles.header}>
					<AccountIcon address={''} network={network} style={styles.icon} />
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
					<QrView
						data={account.name ? `${selectedKey}:${account.name}` : selectedKey}
					/>
					{protocol === NetworkProtocols.UNKNOWN && <UnknownAccountWarning />}
				</View>
			</ScrollView>
			<QrScannerTab />
		</SafeAreaViewContainer>
	);
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.background.app,
		flex: 1
	},
	deleteText: {
		color: colors.signal.error
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
		paddingTop: 8
	}
});
