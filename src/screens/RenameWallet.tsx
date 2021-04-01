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

import React, { useContext } from 'react';
import { View } from 'react-native';

import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { AlertStateContext } from 'stores/alertContext';
import { NavigationAccountIdentityProps } from 'types/props';
import { withCurrentIdentity } from 'utils/HOC';
import TextInput from 'components/TextInput';
import { alertError } from 'utils/alertUtils';
import ScreenHeading from 'components/ScreenHeading';

type Props = NavigationAccountIdentityProps<'RenameWallet'>;

function RenameWallet({ accountsStore }: Props): React.ReactElement {
	const { currentIdentity } = accountsStore.state;
	const { setAlert } = useContext(AlertStateContext);
	if (!currentIdentity) return <View />;

	const onRenameIdentity = async (name: string): Promise<void> => {
		try {
			accountsStore.updateIdentityName(name);
		} catch (err) {
			alertError(setAlert, `Can't rename: ${err.message}`);
		}
	};

	return (
		<SafeAreaViewContainer>
			<ScreenHeading title="Rename Wallet" />
			<TextInput
				label="Display Name"
				onChangeText={onRenameIdentity}
				value={currentIdentity.name}
				placeholder="Enter a new wallet name"
				focus
			/>
		</SafeAreaViewContainer>
	);
}

export default withCurrentIdentity(RenameWallet);
