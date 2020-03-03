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
import { FlatList, StyleSheet, View } from 'react-native';
import { withNavigation } from 'react-navigation';

import testIDs from 'e2e/testIDs';
import { NavigationAccountProps } from 'types/props';
import { Account } from 'types/identityTypes';
import colors from 'styles/colors';
import AccountCard from 'components/AccountCard';
import Background from 'components/Background';
import { withAccountStore } from 'utils/HOC';

function LegacyAccountList({
	navigation,
	accounts
}: NavigationAccountProps<{}>): React.ReactElement {
	const onAccountSelected = async (key: string): Promise<void> => {
		await accounts.select(key);
		navigation.navigate('AccountDetails');
	};
	const accountsMap = accounts.getAccounts();

	return (
		<View style={styles.body} testID={testIDs.AccountListScreen.accountList}>
			<Background />
			<FlatList
				style={styles.content}
				contentContainerStyle={{ paddingBottom: 120 }}
				data={Array.from(accountsMap.entries())}
				keyExtractor={([key]: [string, any]): string => key}
				renderItem={({
					item: [accountKey, account]
				}: {
					item: [string, Account];
				}): React.ReactElement => {
					return (
						<AccountCard
							address={account.address}
							networkKey={account.networkKey}
							onPress={(): Promise<void> => onAccountSelected(accountKey)}
							style={{ paddingBottom: 0 }}
							title={account.name}
						/>
					);
				}}
			/>
		</View>
	);
}

export default withAccountStore(withNavigation(LegacyAccountList));

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column'
	},
	content: {
		flex: 1
	}
});
