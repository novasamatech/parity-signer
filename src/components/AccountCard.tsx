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

import React, { ReactElement, useContext } from 'react';
import { StyleSheet, Text, View, ViewStyle } from 'react-native';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { ButtonListener } from 'types/props';

import { AccountsContext, NetworksContext } from '../context';
import AccountIcon from './AccountIcon';
import AccountPrefixedTitle from './AccountPrefixedTitle';
import Address from './Address';
import { NetworkFooter } from './NetworkFooter';
import TouchableItem from './TouchableItem';

interface AccountCardProps{
	address: string;
	networkKey?: string;
	onPress?: ButtonListener;
	seedType?: string;
	style?: ViewStyle;
	testID?: string;
	title?: string;
	titlePrefix?: string;
};

export default function AccountCard({ address, networkKey, onPress, seedType, style, testID, title, titlePrefix }: AccountCardProps): ReactElement {
	const { getNetwork } = useContext(NetworksContext);
	const { getAccountByAddress } = useContext(AccountsContext);
	const account = getAccountByAddress(address)

	const displayTitle = account?.name || title || 'Unknown';
	const seedTypeDisplay = seedType || '';
	const network = account?.networkKey ? getNetwork(account.networkKey) : getNetwork(networkKey);

	return (
		<TouchableItem
			disabled={false}
			onPress={onPress}
			testID={testID}
		>
			<View style={[styles.content, style]}>
				<AccountIcon
					address={account?.address || address}
					network={network}
					style={styles.icon} />
				<View style={styles.desc}>
					<View>
						<Text style={[fontStyles.t_regular, { color: colors.text.faded }]}>
							{`${network?.title}${seedTypeDisplay} `}
						</Text>
					</View>
					<AccountPrefixedTitle
						title={displayTitle}
						titlePrefix={titlePrefix}
					/>
					{address !== '' && (
						<Address
							address={address}
							protocol={network?.protocol}
						/>
					)}
				</View>
				<NetworkFooter color={network?.color} />
			</View>
		</TouchableItem>
	);
}

const styles = StyleSheet.create({
	content: {
		alignItems: 'center',
		flexDirection: 'row',
		paddingLeft: 16
	},
	desc: {
		flex: 1,
		flexDirection: 'column',
		justifyContent: 'space-between',
		paddingLeft: 16
	},
	icon: {
		height: 40,
		width: 40
	}
});
