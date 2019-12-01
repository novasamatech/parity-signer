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

import { AsyncStorage } from 'react-native';
import SecureStorage from 'react-native-secure-storage';
import { generateAccountId } from './account';
import {
	deserializeIdentities,
	extractAddressFromAccountId,
	isEthereumAccountId,
	serializeIdentities
} from './identitiesUtils';

const currentAccountsStore = {
	keychainService: 'accounts_v3',
	sharedPreferencesName: 'accounts_v3'
};

export async function loadAccounts(version = 3) {
	if (!SecureStorage) {
		return Promise.resolve([]);
	}

	const accountStoreVersion =
		version === 1 ? 'accounts' : `accounts_v${version}`;
	const accountsStore = {
		keychainService: accountStoreVersion,
		sharedPreferencesName: accountStoreVersion
	};

	return SecureStorage.getAllItems(accountsStore).then(accounts => {
		const accountMap = new Map();
		for (let [key, value] of Object.entries(accounts)) {
			const account = JSON.parse(value);
			accountMap.set(key, { ...account });
		}

		return accountMap;
	});
}

const identitiesStore = {
	keychainService: 'parity_signer_identities',
	sharedPreferencesName: 'parity_signer_identities'
};
const identityStorageLabel = 'identities_v4';

export async function loadIdentities(version = 3) {
	function handleError(e) {
		console.warn('loading identities error', e);
		return [];
	}
	try {
		// TODO to be deleted before merging, used for clean the keychain.
		// await SecureStorage.deleteItem(identityStorageLabel, identitiesStore);
		const identities = await SecureStorage.getItem(
			identityStorageLabel,
			identitiesStore
		);
		if (!identities) return [];
		// TODO migrate identities on test devices, remove them in v4.1
		const identitiesObject = deserializeIdentities(identities);
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

		return identitiesObject.map(migrationIdentityFunction);
	} catch (e) {
		handleError(e);
	}
}

export const saveIdentities = identities => {
	SecureStorage.setItem(
		identityStorageLabel,
		serializeIdentities(identities),
		identitiesStore
	);
};

function accountTxsKey({ address, networkKey }) {
	return 'account_txs_' + generateAccountId({ address, networkKey });
}

function txKey(hash) {
	return 'tx_' + hash;
}

export const deleteAccount = async accountKey =>
	SecureStorage.deleteItem(accountKey, currentAccountsStore);

export const saveAccount = (accountKey, account) =>
	SecureStorage.setItem(
		accountKey,
		JSON.stringify(account, null, 0),
		currentAccountsStore
	);

export async function saveTx(tx) {
	if (!tx.sender) {
		throw new Error('Tx should contain sender to save');
	}

	if (!tx.recipient) {
		throw new Error('Tx should contain recipient to save');
	}

	await [
		storagePushValue(accountTxsKey(tx.sender), tx.hash),
		storagePushValue(accountTxsKey(tx.recipient), tx.hash),
		AsyncStorage.setItem(txKey(tx.hash), JSON.stringify(tx))
	];
}

export async function loadAccountTxHashes(account) {
	const result = await AsyncStorage.getItem(accountTxsKey(account));

	return result ? JSON.parse(result) : [];
}

export async function loadAccountTxs(account) {
	const hashes = await loadAccountTxHashes(account);

	return (await AsyncStorage.multiGet(hashes.map(txKey))).map(v => [
		v[0],
		JSON.parse(v[1])
	]);
}

async function storagePushValue(key, value) {
	let currentVal = await AsyncStorage.getItem(key);

	if (currentVal === null) {
		return AsyncStorage.setItem(key, JSON.stringify([value]));
	} else {
		currentVal = JSON.parse(currentVal);
		const newVal = new Set([...currentVal, value]);
		return AsyncStorage.setItem(key, JSON.stringify(Array.from(newVal)));
	}
}

export async function loadToCAndPPConfirmation() {
	const result = await AsyncStorage.getItem('ToCAndPPConfirmation_v3');

	return !!result;
}

export async function saveToCAndPPConfirmation() {
	await AsyncStorage.setItem('ToCAndPPConfirmation_v3', 'yes');
}
