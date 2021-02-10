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
import AccountCard from 'components/AccountCard';
import Button from 'components/Button';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import TextInput from 'components/TextInput';
import t from 'modules/unlock/strings';
import React, { useCallback, useContext, useEffect, useState } from 'react';
import { StyleSheet } from 'react-native';
import { AccountsContext } from 'stores/AccountsContext';

// TODO FIXME test this
export default function AccountEdit(): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const selectedAccount = accountsStore.getSelected();
	const [name, setName] = useState(selectedAccount?.name || '')
	const navigation = useNavigation();

	useEffect(() => {
		if (!selectedAccount){
			console.error('no selected account')

			return;
		}
	}, [selectedAccount])

	const onSubmit = useCallback(() => {
		if (!selectedAccount) {
			return;
		}

		accountsStore.save({ ...selectedAccount, name })
			.then(() => {
				navigation.goBack()
			})
			.catch(console.error);

	}, [accountsStore, name, navigation, selectedAccount])

	if (!selectedAccount) {
		return <SafeAreaScrollViewContainer/>
	}

	return (
		<SafeAreaScrollViewContainer style={styles.body}>
			<AccountCard
				address={selectedAccount.address}
				networkKey={selectedAccount.networkKey}
				title={name}
			/>
			<TextInput
				label="Account Name"
				onChangeText={setName}
				placeholder="New name"
				style={{ marginBottom: 40 }}
				value={name}
			/>
			<Button
				onPress={onSubmit}
				title={t.doneButton.pinUnlock}
			/>
		</SafeAreaScrollViewContainer>
	);
}

const styles = StyleSheet.create({
	body: {
		alignContent: 'flex-start',
		paddingBottom: 40
	}
});
