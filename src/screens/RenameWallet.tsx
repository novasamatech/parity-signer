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

import { AccountsContext } from 'stores/AccountsContext';
import { AlertStateContext } from 'stores/alertContext';
import { NetworksContext } from 'stores/NetworkContext';
import Button from 'components/Button';
import TextInput from 'components/TextInput';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import ScreenHeading from 'components/ScreenHeading';
import { UnknownNetworkKeys } from 'constants/networkSpecs';
import { NavigationAccountIdentityProps } from 'types/props';
import { alertError } from 'utils/alertUtils';
import { getNetworkKey } from 'utils/identitiesUtils';

type Props = NavigationAccountIdentityProps<'RenameWallet'>;

function RenameWallet({ navigation, route }: Props): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const { setAlert } = useContext(AlertStateContext);
	const { identity } = route.params;
	const [newIdentityName, setNewIdentityName] = useState(identity?.name || '');

	const path = route.params.path;
	const networksContextState = useContext(NetworksContext);
	const networkKey = getNetworkKey(
		path,
		accountsStore.state.currentIdentity,
		networksContextState
	);

	if (!identity) return <View />;

	const onChangeIdentity = async (name: string): Promise<void> => {
		setNewIdentityName(name);
	};

	const onSaveIdentity = async (): Promise<void> => {
		try {
			accountsStore.updateIdentityName(newIdentityName);
			navigation.goBack();
		} catch (err) {
			alertError(setAlert, `Can't rename: ${err.message}`);
		}
	};

	return (
		<SafeAreaViewContainer>
			<ScreenHeading title="Rename Wallet" />
			<TextInput
				label="Display Name"
				onChangeText={onChangeIdentity}
				value={newIdentityName}
				placeholder="Enter a new wallet name"
				focus
			/>
			<Button title="Save" onPress={onSaveIdentity} />
		</SafeAreaViewContainer>
	);
}

export default RenameWallet;
