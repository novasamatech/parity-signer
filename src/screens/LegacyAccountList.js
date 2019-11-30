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
import { FlatList, StyleSheet, View } from 'react-native';
import { withNavigation } from 'react-navigation';

import colors from '../colors';
import AccountCard from '../components/AccountCard';
import Background from '../components/Background';
import testIDs from '../../e2e/testIDs';
import ButtonMainAction from '../components/ButtonMainAction';
import { withAccountStore } from '../util/HOC';

function LegacyAccountList({ navigation, accounts }) {
	const onAccountSelected = async key => {
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
				keyExtractor={([key]) => key}
				renderItem={({ item: [accountKey, account] }) => {
					return (
						<AccountCard
							address={account.address}
							networkKey={account.networkKey}
							onPress={() => onAccountSelected(accountKey)}
							style={{ paddingBottom: null }}
							title={account.name}
						/>
					);
				}}
				enableEmptySections
			/>
			<ButtonMainAction
				title={'Scan'}
				onPress={() => navigation.navigate('QrScanner')}
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
