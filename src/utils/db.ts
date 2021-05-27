// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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

import { deserializeIdentities, serializeIdentities } from './identitiesUtils';

import { mergeNetworks, serializeNetworks } from 'utils/networksUtils';
import { SUBSTRATE_NETWORK_LIST } from 'constants/networkSpecs';
import { allBuiltInMetadata } from 'constants/networkMetadataList';
import { SubstrateNetworkParams } from 'types/networkTypes';
import { Account, Identity } from 'types/identityTypes';
import { MetadataHandle } from 'types/metadata';
import {
	metadataHandleToKey,
	metadataStorage,
	getMetadataHandleFromRaw
} from 'utils/metadataUtils';

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
 *	Metadata Store
 * ========================================
 */

export async function getMetadata(
	metadataHandle: MetadataHandle | null
): Promise<string> {
	try {
		if (!metadataHandle) return '';
		const metadataKey = metadataHandleToKey(metadataHandle);
		console.log(metadataKey);
		const metadataRecord = await AsyncStorage.getItem(metadataKey);
		return metadataRecord ? metadataRecord : '';
	} catch (e) {
		handleError(e, 'load metadata');
		return '';
	}
}

function isMetadataKey(this: string): boolean {
	//check if the line begins with 'signer_metadata_' - not ideomatic but safe
	return this.substr(0, metadataStorage.length) === metadataStorage;
}

export async function dumpMetadataDB(): Promise<
	Array<[string, string | null]>
> {
	try {
		const allKeys = await AsyncStorage.getAllKeys();
		const metadataKeys = allKeys.filter(isMetadataKey);
		const allMetadataMap = await AsyncStorage.multiGet(metadataKeys);
		return allMetadataMap;
	} catch (e) {
		handleError(e, 'metadata db fetch failed');
		return [];
	}
}

export async function getAllMetadata(): Promise<Array<MetadataHandle>> {
	try {
		const allMetadataMap = await dumpMetadataDB();
		const handles: Array<MetadataHandle> = [];

		// Uncomment this to clean up
		/*
		for (let deleteme of metadataKeys) {
			await SecureStorage.deleteItem(deleteme, metadataStorage);
		}
		*/
		for (const metadataValue of allMetadataMap) {
			handles.push(await getMetadataHandleFromRaw(metadataValue[1]));
		}
		return handles;
	} catch (e) {
		handleError(e, 'getAllMetadata');
		return [];
	}
}

function isRelevant(this: string, element: MetadataHandle): boolean {
	return String(element.specName) === this;
}

export async function getRelevantMetadata(
	specName: string
): Promise<Array<MetadataHandle>> {
	const handles = await getAllMetadata();
	return handles.filter(isRelevant, specName);
}

export async function saveMetadata(newMetadata: string): Promise<void> {
	try {
		const metadataHandle = await getMetadataHandleFromRaw(newMetadata);
		const newMetadataKey = metadataHandleToKey(metadataHandle);
		await AsyncStorage.setItem(newMetadataKey, newMetadata);
		console.log('Saved: ' + newMetadataKey);
	} catch (e) {
		handleError(e, 'save metadata');
	}
}

export async function populateMetadata(): Promise<void> {
	console.log('loading built-in metadata...');
	for (const metadataString of allBuiltInMetadata) {
		await saveMetadata(metadataString);
	}
}

export async function deleteMetadata(
	metadataHandle: MetadataHandle
): Promise<void> {
	try {
		await AsyncStorage.removeItem(metadataHandleToKey(metadataHandle));
		console.log('metadata successfully removed: ');
		console.log(metadataHandle);
	} catch (e) {
		handleError(e, 'metadata deletion');
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
