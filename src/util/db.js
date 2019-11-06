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

// @flow
'use strict';

import { AsyncStorage } from 'react-native';
import SecureStorage from 'react-native-secure-storage';

import { accountId } from './account';
import { defaultNetworkSpecs } from './networkSpecs';
import kusamaMeta from './static-kusama';
import substrateMeta from './static-substrate';
import { SubstrateNetworkKeys } from '../constants';

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

const accountsStore = {
	keychainService: 'accounts_v3',
	sharedPreferencesName: 'accounts_v3'
};

function accountTxsKey({ address, networkKey }) {
	return 'account_txs_' + accountId({ address, networkKey });
}

function txKey(hash) {
	return 'tx_' + hash;
}

export const deleteAccount = async accountKey =>
	SecureStorage.deleteItem(accountKey, accountsStore);

export const saveAccount = (accountKey, account) =>
	SecureStorage.setItem(
		accountKey,
		JSON.stringify(account, null, 0),
		accountsStore
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

/*
 * Called once during onboarding. Populate the local storage with the default network specs with key being network_${genesisHash}.
 */
export async function saveDefaultNetworks() {
	Object.entries(defaultNetworkSpecs()).forEach(async ([key, value]) => {
		await AsyncStorage.setItem(
			`network_${key}`,
			JSON.stringify(value, null, 0)
		);
	});
}

/*
 * @dev map: networkKey => metadata blob
 */
export async function saveDefaultMetadata() {
	await AsyncStorage.setItem(
		SubstrateNetworkKeys.KUSAMA,
		JSON.stringify(kusamaMeta)
	);
	await AsyncStorage.setItem(
		SubstrateNetworkKeys.KUSAMA_DEV,
		JSON.stringify(kusamaMeta)
	);
	await AsyncStorage.setItem(
		SubstrateNetworkKeys.SUBSTRATE_DEV,
		JSON.stringify(substrateMeta)
	);
}

/*
 * @dev add or update a networkSpec at index of networkKey
 */
export async function addNetworkSpec(networkKey, networkSpec) {
	if (!networkKey) {
		throw new Error('Must supply a network key to add new network spec.');
	}

	if (!networkSpec.prefix) {
		throw new Error('Network spec must include prefix to be valid.');
	}

	if (!networkSpec.identiconFn) {
		throw new Error(
			'Network spec must include a valid identicon generation function.'
		);
	}

	await AsyncStorage.setItem(networkKey, JSON.stringify(networkSpec, null, 0));
}

/*
 * @dev get all the network keys
 */
export async function getAllNetworkSpecs() {
	const allKeys = await AsyncStorage.getAllKeys();
	let result = [];
	// network keys are prefixed with network_
	await asyncForEach(allKeys, async key => {
		if (key.slice(0, 8) === 'network_') {
			const spec = await getNetworkSpecByKey(key);
			console.log('spec here -> ', spec);
			console.log('json spec here -> ', JSON.parse(spec));
			result.push(JSON.parse(spec));
		}
	});
	console.log('all the network specs -< ', result);
	return result;
}

/*
 * @dev get a specific network spec by networkKey (genesisHash)
 */
export async function getNetworkSpecByKey(networkKey) {
	return await AsyncStorage.getItem(networkKey);
}

export async function clearAsyncStorage() {
	await AsyncStorage.clear();
}

async function asyncForEach(array, callback) {
	for (let i = 0; i < array.length; i++) {
		await callback(array[i], i, array);
	}
}
