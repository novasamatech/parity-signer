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

import { useNavigation } from '@react-navigation/native';
import AccountCard from 'components/AccountCard';
import Button from 'components/Button';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import TextInput from 'components/TextInput';
import t from 'modules/unlock/strings';
import React, { useCallback, useContext, useEffect, useState } from 'react';
import { StyleSheet } from 'react-native';

import { AccountsContext } from '../context';

// TODO FIXME test this
export default function AccountEdit(): React.ReactElement {
	const { getSelectedAccount, saveAccount } = useContext(AccountsContext);
	const selectedAccount = getSelectedAccount();
	const [newName, setNewName] = useState(selectedAccount?.name || '')
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

		saveAccount({ ...selectedAccount, name: newName })
			.then(() => {
				navigation.goBack()
			})
			.catch(console.error);

	}, [newName, navigation, saveAccount, selectedAccount])

	if (!selectedAccount) {
		return <SafeAreaScrollViewContainer/>
	}

	return (
		<SafeAreaScrollViewContainer style={styles.body}>
			<AccountCard
				address={selectedAccount.address}
				networkKey={selectedAccount.networkKey}
				title={newName}
			/>
			<TextInput
				label="Account Name"
				onChangeText={setNewName}
				placeholder="New name"
				style={{ marginBottom: 40 }}
				value={newName}
			/>
			<Button
				onPress={onSubmit}
				title={t.doneButton.nameChange}
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
