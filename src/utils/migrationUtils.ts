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

import {
	Account,
	AccountMeta,
	Identity,
	LockedAccount
} from 'types/identityTypes';
import { generateAccountId } from 'utils/account';
import {
	loadAccounts,
	loadIdentities,
	saveAccount,
	saveIdentities
} from 'utils/db';
import {
	extractAddressFromAccountId,
	isEthereumAccountId
} from 'utils/identitiesUtils';

interface LegacyMeta extends AccountMeta {
	accountId: string;
}

interface LegacyIdentity extends Identity {
	meta: Map<string, LegacyMeta>;
	accountIds: Map<string, string>;
}

interface LegacyAccount extends LockedAccount {
	chainId: string;
	networkType: string;
}

export const migrateIdentity = async (): Promise<void> => {
	const identities = await loadIdentities(4);

	const migrationIdentityFunction = (identity: LegacyIdentity): Identity => {
		const getAddressKey = (accountId: string): string =>
			isEthereumAccountId(accountId)
				? accountId
				: extractAddressFromAccountId(accountId);

		if (identity.hasOwnProperty('addresses')) {
			return identity;
		}
		const addressMap = new Map();
		identity.accountIds.forEach((path: string, accountId: string): void => {
			addressMap.set(getAddressKey(accountId), path);
		});
		identity.addresses = addressMap;
		delete identity.accountIds;

		const metaMap = new Map();
		identity.meta.forEach((metaData: LegacyMeta, path: string): void => {
			if (metaData.hasOwnProperty('accountId')) {
				const { accountId } = metaData;
				metaData.address = extractAddressFromAccountId(accountId);
				delete metaData.accountId;
				metaMap.set(path, metaData);
			} else {
				metaMap.set(path, metaData);
			}
		});
		identity.meta = metaMap;

		return identity;
	};
	saveIdentities(
		(identities as LegacyIdentity[]).map(migrationIdentityFunction)
	);
};

export const migrateAccounts = async (): Promise<void> => {
	const oldAccounts_v1 = await loadAccounts(1);
	// get a map from old accounts
	// v2 accounts (up to v2.2.2) are only ethereum accounts
	// with now deprectaded `chainId` and `networkType: 'ethereum'` properties
	// networkKey property is missing since it was introduced in v3.
	const oldAccounts_v2 = await loadAccounts(2);
	const oldAccounts = [...oldAccounts_v1, ...oldAccounts_v2];
	const accounts = oldAccounts.map(
		([_, value]: [any, LegacyAccount]): Account => {
			let result = {} as LegacyAccount;
			if (value.chainId) {
				// The networkKey for Ethereum accounts is the chain id
				result = {
					...(value as LegacyAccount),
					networkKey: value.chainId,
					recovered: true
				};
				delete result.chainId;
				delete result.networkType;
			}
			return result;
		}
	);

	accounts.forEach((account: Account): void => {
		try {
			saveAccount(generateAccountId(account), account);
		} catch (e) {
			console.error(e);
		}
	});
};
