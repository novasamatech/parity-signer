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
import { StyleSheet, View } from 'react-native';
import { NavigationActions, StackActions } from 'react-navigation';

import colors from '../colors';
import { accountId } from '../util/account';
import {
	loadAccounts,
	loadToCAndPPConfirmation,
	saveAccount
} from '../util/db';

export default class Loading extends React.PureComponent {
	static navigationOptions = {
		headerBackTitle: 'Back',
		title: 'Add Account'
	};

	async componentDidMount() {
		const tocPP = await loadToCAndPPConfirmation();
		const firstScreen = 'Welcome';
		const firstScreenActions = StackActions.reset({
			actions: [NavigationActions.navigate({ routeName: firstScreen })],
			index: 0,
			key: null
		});
		let tocActions;

		if (!tocPP) {
			this.migrateAccounts();

			tocActions = StackActions.reset({
				actions: [
					NavigationActions.navigate({
						params: {
							firstScreenActions
						},
						routeName: 'TocAndPrivacyPolicy'
					})
				],
				index: 0
			});
		} else {
			tocActions = firstScreenActions;
		}

		await loadAccounts();
		this.props.navigation.dispatch(tocActions);
	}

	async migrateAccounts() {
		const oldAccounts_v1 = await loadAccounts(1);
		// get a map from old accounts
		// v2 accounts (up to v2.2.2) are only ethereum accounts
		// with now deprectaded `chainId` and `networkType: 'ethereum'` properties
		// networkKey property is missing since it was introduced in v3.
		const oldAccounts_v2 = await loadAccounts(2);
		const oldAccounts = [...oldAccounts_v1, ...oldAccounts_v2];
		const accounts = oldAccounts.map(([_, value]) => {
			if (value.chainId) {
				// The networkKey for Ethereum accounts is the chain id
				const result = { ...value, networkKey: value.chainId, recovered: true };
				delete result.chainId;
				delete result.networkType;
				return result;
			}
			return value;
		});

		accounts.forEach(account => {
			try {
				saveAccount(accountId(account), account);
			} catch (e) {
				console.error(e);
			}
		});
	}

	render() {
		return <View style={styles.body} />;
	}
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column',
		padding: 20
	}
});
