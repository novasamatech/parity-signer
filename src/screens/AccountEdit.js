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
import { ScrollView, StyleSheet } from 'react-native';
import { Subscribe } from 'unstated';
import colors from '../colors';
import AccountCard from '../components/AccountCard';
import TextInput from '../components/TextInput';
import AccountsStore from '../stores/AccountsStore';

const onNameInput = async (accounts, name) => {
	await accounts.updateSelectedAccount({ name });
	await accounts.save(accounts.getSelectedKey(), accounts.getSelected());
};

export default class AccountEdit extends React.PureComponent {
	constructor(props) {
		super(props);
	}

	render() {
		return (
			<Subscribe to={[AccountsStore]}>
				{accounts => {
					const selected = accounts.getSelected();

					if (!selected) {
						return null;
					}

					return (
						<ScrollView style={styles.body}>
							<AccountCard
								title={selected.name}
								accountId={selected.address}
								networkKey={selected.networkKey}
							/>
							<TextInput
								label="Account Name"
								style={{ marginBottom: 40 }}
								onChangeText={name => onNameInput(accounts, name)}
								value={selected.name}
								placeholder="New name"
							/>
						</ScrollView>
					);
				}}
			</Subscribe>
		);
	}
}

const styles = StyleSheet.create({
	body: {
		alignContent: 'flex-start',
		backgroundColor: colors.bg,
		flex: 1,
		paddingBottom: 40
	}
});
