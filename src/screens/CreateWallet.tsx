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

import React, { useContext, useEffect, useRef } from 'react';
import { StyleSheet, View } from 'react-native';

import { AccountsContext } from 'stores/AccountsContext';
import Button from 'components/Button';
import TextInput from 'components/TextInput';
import { NavigationProps } from 'types/props';
import { emptyIdentity } from 'utils/identitiesUtils';
import colors from 'styles/colors';
import ScreenHeading from 'components/ScreenHeading';
import KeyboardScrollView from 'components/KeyboardScrollView';

function CreateWallet({
	navigation
}: NavigationProps<'CreateWallet'>): React.ReactElement {
	const accountsStore = useContext(AccountsContext);
	const clearIdentity = useRef(() =>
		accountsStore.updateNewIdentity(emptyIdentity())
	);
	console.log(accountsStore.state.newIdentity);

	useEffect((): (() => void) => {
		clearIdentity.current();
		return clearIdentity.current;
	}, [clearIdentity]);

	const updateName = (name: string): void => {
		accountsStore.updateNewIdentity({ name });
	};

	return (
		<KeyboardScrollView bounces={false} style={styles.body}>
			<ScreenHeading title={'New Wallet'} />
			<TextInput
				onChangeText={updateName}
				value={accountsStore.state.newIdentity.name}
				placeholder="Wallet name"
			/>
			<View style={styles.btnBox}>
				<Button
					title="Create New"
					onPress={(): void => navigation.navigate('CreateWallet2')}
				/>
				<Button
					title="Import Wallet"
					onPress={(): void => navigation.navigate('CreateWalletImport')}
				/>
			</View>
		</KeyboardScrollView>
	);
}

export default CreateWallet;

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.background.app,
		flex: 1,
		overflow: 'hidden'
	},
	btnBox: {
		alignContent: 'center',
		marginTop: 32
	}
});
