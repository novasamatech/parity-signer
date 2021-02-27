// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Modifications Copyright (c) 2021 Thibaut Sardan

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

import AccountCard from 'components/AccountCard';
import AccountIcon from 'components/AccountIcon';
import PopupMenu from 'components/PopupMenu';
import QrScannerTab from 'components/QrScannerTab';
import QrView from 'components/QrView';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { UnknownAccountWarning } from 'components/Warnings';
import { NetworkProtocols } from 'constants/networkSpecs';
import React, { useContext, useMemo } from 'react';
import { ScrollView, StyleSheet, Text, View } from 'react-native';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { EthereumNetwork, isSubstrateNetwork } from 'types/networkTypes';
import { NavigationProps } from 'types/props';
import { alertDeleteLegacyAccount } from 'utils/alertUtils';
import { navigateToLegacyAccountList } from 'utils/navigationHelpers';

import { AccountsContext, AlertContext, NetworksContext } from '../context';

export default function AccountDetails({ navigation }: NavigationProps<'AccountDetails'>): React.ReactElement {
	const { deleteAccount, getSelectedAccount } = useContext(AccountsContext);
	const selectedAccount = getSelectedAccount();
	const { address, name, networkKey } = selectedAccount || { address: '', name: '', networkKey: '' };
	const { getNetwork } = useContext(NetworksContext);
	const { setAlert } = useContext(AlertContext);
	const network = getNetwork(networkKey);

	const accountId = useMemo((): string => {
		if (!network){
			console.log('Account without network')

			return '';
		}

		const { protocol } = network;

		if (isSubstrateNetwork(network)) {
			const { genesisHash } = network;

			return `${protocol}:${address}:${genesisHash ?? ''}`;
		} else {
			const { ethereumChainId } = network as EthereumNetwork;

			return `${protocol}:0x${address}@${ethereumChainId}`;
		}
	}, [address, network])

	if (!address || !network) return <View />;

	const protocol = network?.protocol;

	const onDelete = (): void => {
		alertDeleteLegacyAccount(setAlert,
			name || address || 'this account',
			async () => {
				await deleteAccount(address);

				navigateToLegacyAccountList(navigation);
			});
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
			<ScrollView bounces={false}
				style={styles.scrollBody}>
				<View style={styles.header}>
					<AccountIcon address={''}
						network={network}
						style={styles.icon} />
					<Text style={fontStyles.h2}>Public Address</Text>
					<View style={styles.menuView}>
						<PopupMenu
							menuItems={[
								{ text: 'Change name', value: 'AccountEdit' },
								{ text: 'Change pin', value: 'AccountPin' },
								{ text: 'View recovery phrase', value: 'LegacyMnemonic' },
								{ text: 'Delete', textStyle: styles.deleteText, value: 'AccountDelete' }
							]}
							menuTriggerIconName={'more-vert'}
							onSelect={onOptionSelect}
						/>
					</View>
				</View>
				<AccountCard address={address} />
				<View>
					<QrView
						data={
							name
								? `${accountId}:${name}`
								: accountId
						}
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
