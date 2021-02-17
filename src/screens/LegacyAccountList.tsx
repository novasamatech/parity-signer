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

import { useNavigation } from '@react-navigation/native';
import { StackNavigationProp } from '@react-navigation/stack';
import AccountCard from 'components/AccountCard';
import QrScannerTab from 'components/QrScannerTab';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import testIDs from 'e2e/testIDs';
import React, { useContext } from 'react';
import { ScrollView, StyleSheet } from 'react-native';
import { LegacyAccount } from 'types/identityTypes';
import { RootStackParamList } from 'types/routes';

import { AccountsContext } from '../context';

function LegacyAccountList(): React.ReactElement {
	const navigation: StackNavigationProp<RootStackParamList> = useNavigation();
	const accountsStore = useContext(AccountsContext);

	const onAccountSelected = async (key: string): Promise<void> => {
		await accountsStore.select(key);
		navigation.navigate('AccountDetails');
	};

	const { accounts } = accountsStore.state;

	// console.log('legacy', accounts)

	const renderAccountCard = ({ address, name, networkKey }: LegacyAccount): React.ReactElement => (
		<AccountCard
			address={address}
			key={address}
			networkKey={networkKey}
			onPress={(): Promise<void> => onAccountSelected(address)}
			style={{ paddingBottom: 0 }}
			title={name}
		/>
	);

	return (
		<SafeAreaViewContainer>
			<ScrollView
				style={styles.content}
				testID={testIDs.AccountListScreen.accountList}
			>
				{accounts.map(renderAccountCard)}
			</ScrollView>
			<QrScannerTab />
		</SafeAreaViewContainer>
	);
}

export default LegacyAccountList;

const styles = StyleSheet.create({
	content: {
		flex: 1,
		paddingBottom: 40
	}
});
