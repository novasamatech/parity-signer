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
import { StyleSheet } from 'react-native';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import { NavigationAccountProps } from 'types/props';
import { Account } from 'types/identityTypes';
import AccountCard from 'components/AccountCard';
import { withAccountStore } from 'utils/HOC';

function LegacyAccountList({
	navigation,
	accounts
}: NavigationAccountProps<'LegacyAccountList'>): React.ReactElement {
	const onAccountSelected = async (key: string): Promise<void> => {
		await accounts.select(key);
		navigation.navigate('AccountDetails');
	};
	const accountsMap = accounts.getAccounts();

	const renderAccountCard = ([accountKey, account]: [
		string,
		Account
	]): React.ReactElement => (
		<AccountCard
			address={account.address}
			networkKey={account.networkKey}
			onPress={(): Promise<void> => onAccountSelected(accountKey)}
			style={{ paddingBottom: 0 }}
			title={account.name}
			key={accountKey}
		/>
	);

	return (
		<SafeAreaScrollViewContainer
			testID={testIDs.AccountListScreen.accountList}
			style={styles.content}
		>
			{Array.from(accountsMap.entries()).map(renderAccountCard)}
		</SafeAreaScrollViewContainer>
	);
}

export default withAccountStore(LegacyAccountList);

const styles = StyleSheet.create({
	content: {
		flex: 1,
		paddingBottom: 40
	}
});
