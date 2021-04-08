// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Copyright 2021 Commonwealth Labs, Inc.
// This file is part of Layer Wallet.

// Layer Wallet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Layer Wallet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Layer Wallet. If not, see <http://www.gnu.org/licenses/>.

import React, { useContext, useState } from 'react';
import { View } from 'react-native';
import { StackNavigationProp } from '@react-navigation/stack';

import { components } from 'styles';
import { NetworksContext } from 'stores/NetworkContext';
import { AccountsStoreStateWithIdentity } from 'types/identityTypes';
import { NavigationAccountIdentityProps } from 'types/props';
import { RootStackParamList } from 'types/routes';
import { withCurrentIdentity } from 'utils/HOC';
import { getNetworkKey } from 'utils/identitiesUtils';
import Button from 'components/Button';
import TextInput from 'components/TextInput';

interface Props {
	path: string;
	networkKey: string;
	navigation: StackNavigationProp<RootStackParamList, 'SendBalance'>;
	accountsStore: AccountsStoreStateWithIdentity;
}

function SendBalance({
	accountsStore,
	navigation,
	route
}: NavigationAccountIdentityProps<'SendBalance'>): React.ReactElement {
	const path = route.params.path;
	const networksContextState = useContext(NetworksContext);
	const networkKey = getNetworkKey(
		path,
		accountsStore.state.currentIdentity,
		networksContextState
	);
	const networkParams = networksContextState.getNetwork(networkKey ?? '');

	const [amount, setAmount] = useState('');
	const onChangeAmount = async (name: string): Promise<void> => {
		setAmount(name);
	};

	const [recipient, setRecipient] = useState('');
	const onChangeRecipient = async (name: string): Promise<void> => {
		setRecipient(name);
	};

	const [newAddressBookEntry, setNewAddressBookEntry] = useState('');
	const onChangeNewAddressBookEntry = async (name: string): Promise<void> => {
		setNewAddressBookEntry(name);
	};

	return (
		<View style={components.page}>
			<TextInput
				suffix={networkParams.unit}
				label="Amount"
				onChangeText={onChangeAmount}
				value={amount}
				placeholder="0"
				autoCorrect={false}
				clearButtonMode="unless-editing"
				keyboardType="numeric"
			/>
			<TextInput
				label="Recipient"
				onChangeText={onChangeRecipient}
				value={recipient}
				placeholder="Address"
				autoCorrect={false}
			/>
			<Button
				title="Send"
				fluid={true}
				onPress={(): void => {
					return;
				}}
			/>
			<TextInput
				label="Add to Address Book"
				onChangeText={onChangeNewAddressBookEntry}
				value={newAddressBookEntry}
				placeholder="Address"
				fluid={true}
				autoCorrect={false}
			/>
			<Button
				title="Add"
				fluid={true}
				onPress={(): void => {
					return;
				}}
			/>
		</View>
	);
}

export default withCurrentIdentity(SendBalance);
