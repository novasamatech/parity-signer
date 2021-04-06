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
import { Text } from 'react-native';

import { fontStyles } from 'styles';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { AccountsContext } from 'stores/AccountsContext';
import { AlertStateContext } from 'stores/alertContext';
import { NavigationProps } from 'types/props';
import ScreenHeading from 'components/ScreenHeading';
import { alertIdentityCreationError } from 'utils/alertUtils';
import Button from 'components/Button';
import { useNewSeedRef } from 'utils/seedRefHooks';
import { resetNavigationTo } from 'utils/navigationHelpers';
import AccountSeed from 'components/AccountSeed';

function CreateWallet3({
	navigation,
	route
}: NavigationProps<'CreateWallet3'>): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const [isSeedMatching, setIsSeedMatching] = useState(false);
	const { setAlert } = useContext(AlertStateContext);
	const createSeedRefWithNewSeed = useNewSeedRef();
	const createWallet = async (): Promise<void> => {
		try {
			await accountsStore.saveNewIdentity(
				route.params.seedPhrase,
				createSeedRefWithNewSeed
			);
			resetNavigationTo(navigation, 'Wallet');
		} catch (e) {
			alertIdentityCreationError(setAlert, e.message);
		}
	};

	const onSeedTextInput = (inputSeedPhrase: string): void => {
		setIsSeedMatching(inputSeedPhrase === route.params.seedPhrase);
	};

	return (
		<SafeAreaViewContainer>
			<ScreenHeading
				title={'Verify Recovery Phrase'}
				subtitle={'Retype the recovery phrase as shown on the prior screen.'}
			/>
			<AccountSeed
				onChangeText={onSeedTextInput}
				onSubmitEditing={createWallet}
				returnKeyType="done"
				valid={isSeedMatching}
			/>
			<Button
				title={'Confirm'}
				onPress={createWallet}
				disabled={!isSeedMatching}
			/>
			<Button title={'Go back'} onPress={(): void => navigation.goBack()} />
		</SafeAreaViewContainer>
	);
}

export default CreateWallet3;
