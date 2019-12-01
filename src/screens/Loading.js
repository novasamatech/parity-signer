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

import colors from '../colors';
import { generateAccountId } from '../util/account';
import {
	loadAccounts,
	loadIdentities,
	loadToCAndPPConfirmation,
	saveAccount,
	saveIdentities
} from '../util/db';
import {
	extractAddressFromAccountId,
	isEthereumAccountId
} from '../util/identitiesUtils';

export default class Loading extends React.PureComponent {
	async componentDidMount() {
		const tocPP = await loadToCAndPPConfirmation();
		const { navigate } = this.props.navigation;
		if (!tocPP) {
			this.migrateAccounts();
			this.migrateIdentity();
			navigate('TocAndPrivacyPolicy');
		} else {
			navigate('Welcome');
		}
	}

	// TODO migrate identities only on internal test devices, remove them in v4.1
	async migrateIdentity() {
		const identities = await loadIdentities(3);

		const migrationIdentityFunction = identity => {
			const getAddressKey = accountId =>
				isEthereumAccountId(accountId)
					? accountId
					: extractAddressFromAccountId(accountId);

			if (identity.hasOwnProperty('addresses')) {
				return identity;
			}
			const addressMap = new Map();
			identity.accountIds.forEach((path, accountId) => {
				addressMap.set(getAddressKey(accountId), path);
			});
			identity.addresses = addressMap;
			delete identity.accountIds;

			const metaMap = new Map();
			identity.meta.forEach((metaData, path) => {
				if (metaData.hasOwnProperty('accountId')) {
					const { accountId } = metaData;
					metaData.address = getAddressKey(accountId);
					delete metaData.accountId;
					return metaMap.set(path, metaData);
				}
				metaMap.set(path, metaData);
			});
			identity.meta = metaMap;

			return identity;
		};
		saveIdentities(identities.map(migrationIdentityFunction));
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
			let result = {};
			if (value.chainId) {
				// The networkKey for Ethereum accounts is the chain id
				result = { ...value, networkKey: value.chainId, recovered: true };
				delete result.chainId;
				delete result.networkType;
			}
			return result;
		});

		accounts.forEach(account => {
			try {
				saveAccount(generateAccountId(account), account);
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
