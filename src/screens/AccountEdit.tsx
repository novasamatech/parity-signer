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
import { ScrollView, StyleSheet } from 'react-native';

import colors from 'styles/colors';
import AccountCard from 'components/AccountCard';
import TextInput from 'components/TextInput';
import AccountsStore from 'stores/AccountsStore';
import { withAccountStore } from 'utils/HOC';

const onNameInput = async (
	accounts: AccountsStore,
	name: string
): Promise<void> => {
	await accounts.updateSelectedAccount({ name });
	const selectedAccount = accounts.getSelected()!;
	await accounts.save(accounts.getSelectedKey(), selectedAccount);
};

function AccountEdit({
	accounts
}: {
	accounts: AccountsStore;
}): React.ReactElement {
	const selectedAccount = accounts.getSelected()!;
	if (!selectedAccount) {
		return <ScrollView style={styles.body} />;
	}

	return (
		<ScrollView style={styles.body}>
			<AccountCard
				address={selectedAccount.address}
				title={selectedAccount.name}
				networkKey={selectedAccount.networkKey}
			/>
			<TextInput
				label="Account Name"
				style={{ marginBottom: 40 }}
				onChangeText={(name: string): Promise<any> =>
					onNameInput(accounts, name)
				}
				value={selectedAccount.name}
				placeholder="New name"
			/>
		</ScrollView>
	);
}

export default withAccountStore(AccountEdit);

const styles = StyleSheet.create({
	body: {
		alignContent: 'flex-start',
		backgroundColor: colors.bg,
		flex: 1,
		paddingBottom: 40
	}
});
