// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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

import AsyncStorage from '@react-native-community/async-storage';
import SecureStorage from 'react-native-secure-storage';

import { generateAccountId } from './account';
import { deserializeIdentities, serializeIdentities } from './identitiesUtils';

import { mergeNetworks, serializeNetworks } from 'utils/networksUtils';
import { SUBSTRATE_NETWORK_LIST } from 'constants/networkSpecs';
import { NetworkParams, SubstrateNetworkParams } from 'types/networkTypes';
import { Account, Identity } from 'types/identityTypes';
import { Tx, TxParticipant } from 'types/tx';

function handleError(e: Error, label: string): any[] {
	console.warn(`loading ${label} error`, e);
	return [];
}

/*
 * ========================================
 *	Accounts Store
 * ========================================
 */
const currentAccountsStore = {
	keychainService: 'accounts_v3',
	sharedPreferencesName: 'accounts_v3'
};

export async function loadAccounts(version = 3): Promise<Map<string, any>> {
	if (!SecureStorage) {
		return Promise.resolve(new Map());
	}

	const accountsStoreVersion =
		version === 1 ? 'accounts' : `accounts_v${version}`;
	const accountsStore = {
		keychainService: accountsStoreVersion,
		sharedPreferencesName: accountsStoreVersion
	};

	return SecureStorage.getAllItems(accountsStore).then(
		(accounts: { [key: string]: string }) => {
			const accountMap = new Map();
			for (const [key, value] of Object.entries(accounts)) {
				const account = JSON.parse(value);
				accountMap.set(key, { ...account });
			}

			return accountMap;
		}
	);
}

/*
 * ========================================
 *	Identities Store
 * ========================================
 */
const identitiesStore = {
	keychainService: 'parity_signer_identities',
	sharedPreferencesName: 'parity_signer_identities'
};
const currentIdentityStorageLabel = 'identities_v4';

export async function loadIdentities(version = 4): Promise<Identity[]> {
	const identityStorageLabel = `identities_v${version}`;
	try {
		const identities = await SecureStorage.getItem(
			identityStorageLabel,
			identitiesStore
		);
		if (!identities) return [];
		return deserializeIdentities(identities);
	} catch (e) {
		return handleError(e, 'identity');
	}
}

export const saveIdentities = (identities: Identity[]): void => {
	SecureStorage.setItem(
		currentIdentityStorageLabel,
		serializeIdentities(identities),
		identitiesStore
	);
};

/*
 * ========================================
 *	Networks Store
 * ========================================
 */
const networkStorage = {
	keychainService: 'parity_signer_updated_networks',
	sharedPreferencesName: 'parity_signer_updated_networks'
};
const currentNetworkStorageLabel = 'networks_v4';

export async function loadNetworks(): Promise<
	Map<string, SubstrateNetworkParams>
> {
	try {
		const networksJson = await SecureStorage.getItem(
			currentNetworkStorageLabel,
			networkStorage
		);

		if (!networksJson) return new Map(Object.entries(SUBSTRATE_NETWORK_LIST));
		const networksEntries = JSON.parse(networksJson);
		return mergeNetworks(SUBSTRATE_NETWORK_LIST, networksEntries);
	} catch (e) {
		handleError(e, 'networks');
		return new Map();
	}
}

export async function saveNetworks(
	newNetwork: SubstrateNetworkParams
): Promise<void> {
	try {
		let addedNetworks = new Map();
		const addedNetworkJson = await SecureStorage.getItem(
			currentNetworkStorageLabel,
			networkStorage
		);
		if (addedNetworkJson) addedNetworks = new Map(JSON.parse(addedNetworkJson));

		addedNetworks.set(newNetwork.genesisHash, newNetwork);
		SecureStorage.setItem(
			currentNetworkStorageLabel,
			serializeNetworks(addedNetworks),
			networkStorage
		);
	} catch (e) {
		handleError(e, 'networks');
	}
}

/*
 * ========================================
 *	Privacy Policy and Terms Conditions Store
 * ========================================
 */

export async function loadToCAndPPConfirmation(): Promise<boolean> {
	const result = await AsyncStorage.getItem('ToCAndPPConfirmation_v4');

	return !!result;
}

export async function saveToCAndPPConfirmation(): Promise<void> {
	await AsyncStorage.setItem('ToCAndPPConfirmation_v4', 'yes');
}

/*
 * ========================================
 *	Tx Store (Archived)
 * ========================================
 */

function accountTxsKey(
	address: string,
	networkKey: string,
	allNetworks: Map<string, NetworkParams>
): string {
	return 'account_txs_' + generateAccountId(address, networkKey, allNetworks);
}

function txKey(hash: string): string {
	return 'tx_' + hash;
}

export const deleteAccount = (accountKey: string): Promise<void> =>
	SecureStorage.deleteItem(accountKey, currentAccountsStore);

export const saveAccount = (
	accountKey: string,
	account: Account
): Promise<void> =>
	SecureStorage.setItem(
		accountKey,
		JSON.stringify(account, null, 0),
		currentAccountsStore
	);

async function storagePushValue(key: string, value: string): Promise<void> {
	let currentVal = await AsyncStorage.getItem(key);

	if (currentVal === null) {
		return AsyncStorage.setItem(key, JSON.stringify([value]));
	} else {
		currentVal = JSON.parse(currentVal);
		const newVal = new Set([...(currentVal as NonNullable<any>), value]);
		return AsyncStorage.setItem(key, JSON.stringify(Array.from(newVal)));
	}
}

export async function saveTx(
	tx: Tx,
	allNetworks: Map<string, NetworkParams>
): Promise<void> {
	if (!tx.sender) {
		throw new Error('Tx should contain sender to save');
	}

	if (!tx.recipient) {
		throw new Error('Tx should contain recipient to save');
	}

	await Promise.all([
		storagePushValue(
			accountTxsKey(tx.sender.address, tx.sender.networkKey, allNetworks),
			tx.hash
		),
		storagePushValue(
			accountTxsKey(tx.recipient.address, tx.sender.networkKey, allNetworks),
			tx.hash
		),
		AsyncStorage.setItem(txKey(tx.hash), JSON.stringify(tx))
	]);
}

export async function loadAccountTxHashes(
	account: TxParticipant,
	allNetworks: Map<string, NetworkParams>
): Promise<string[]> {
	const result = await AsyncStorage.getItem(
		accountTxsKey(account.address, account.networkKey, allNetworks)
	);

	return result ? JSON.parse(result) : [];
}

export async function loadAccountTxs(
	account: TxParticipant,
	allNetworks: Map<string, NetworkParams>
): Promise<Array<[string, Tx]>> {
	const hashes = await loadAccountTxHashes(account, allNetworks);

	return (
		await AsyncStorage.multiGet(hashes.map(txKey))
	).map((v: [string, any]) => [v[0], JSON.parse(v[1])]);
}

/*
 * ========================================
 *	NETWORK SPECS
 * ========================================
 */

// const networkSpecsStorageLabel = 'network_specs_v4';
//
// /*
//  * save the new network specs array
//  */
// export function saveNetworkSpecs(networkSpecs: SubstrateNetworkParams[]): void {
// 	AsyncStorage.setItem(networkSpecsStorageLabel, JSON.stringify(networkSpecs));
// }
//
// /*
//  * get all the network specs
//  */
// export async function getNetworkSpecs(): Promise<SubstrateNetworkParams[]> {
// 	let networkSpecs;
// 	try {
// 		const networkSpecsString = await AsyncStorage.getItem(
// 			networkSpecsStorageLabel
// 		);
// 		networkSpecs = JSON.parse(networkSpecsString ?? '');
// 	} catch (e) {
// 		console.warn('loading network specifications error', e);
// 	}
// 	if (networkSpecs === null) return defaultNetworkSpecs();
//
// 	return JSON.parse(networkSpecs ?? '');
// }
//
// /*
//  * Called once during onboarding. Populate the local storage with the default network specs.
//  */
// export async function saveDefaultNetworks(): Promise<void> {
// 	const networkSpecsString = JSON.stringify(defaultNetworkSpecs());
// 	// AsyncStorage.setItem(networkSpecsStorageLabel, networkSpecsString);
// }
