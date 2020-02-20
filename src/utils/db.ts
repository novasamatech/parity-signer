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

import { AsyncStorage } from 'react-native';
import SecureStorage from 'react-native-secure-storage';

import { generateAccountId } from './account';
import { deserializeIdentities, serializeIdentities } from './identitiesUtils';

import { Account, Identity } from 'types/identityTypes';
import { Tx, TxParticipant } from 'types/tx';

const currentAccountsStore = {
	keychainService: 'accounts_v3',
	sharedPreferencesName: 'accounts_v3'
};

export async function loadAccounts(version = 3): Promise<Map<string, any>> {
	if (!SecureStorage) {
		return Promise.resolve(new Map());
	}

	const accountStoreVersion =
		version === 1 ? 'accounts' : `accounts_v${version}`;
	const accountsStore = {
		keychainService: accountStoreVersion,
		sharedPreferencesName: accountStoreVersion
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

const identitiesStore = {
	keychainService: 'parity_signer_identities',
	sharedPreferencesName: 'parity_signer_identities'
};
const currentIdentityStorageLabel = 'identities_v4';

export async function loadIdentities(version = 4): Promise<Identity[]> {
	function handleError(e: Error): Identity[] {
		console.warn('loading identities error', e);
		return [];
	}

	const identityStorageLabel = `identities_v${version}`;
	try {
		const identities = await SecureStorage.getItem(
			identityStorageLabel,
			identitiesStore
		);
		if (!identities) return [];
		return deserializeIdentities(identities);
	} catch (e) {
		return handleError(e);
	}
}

export const saveIdentities = (identities: Identity[]): void => {
	SecureStorage.setItem(
		currentIdentityStorageLabel,
		serializeIdentities(identities),
		identitiesStore
	);
};

function accountTxsKey({
	address,
	networkKey
}: {
	address: string;
	networkKey: string;
}): string {
	return 'account_txs_' + generateAccountId({ address, networkKey });
}

function txKey(hash: string): string {
	return 'tx_' + hash;
}

export const deleteAccount = (accountKey: string): void =>
	SecureStorage.deleteItem(accountKey, currentAccountsStore);

export const saveAccount = (accountKey: string, account: Account): void =>
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

export async function saveTx(tx: Tx): Promise<void> {
	if (!tx.sender) {
		throw new Error('Tx should contain sender to save');
	}

	if (!tx.recipient) {
		throw new Error('Tx should contain recipient to save');
	}

	await Promise.all([
		storagePushValue(accountTxsKey(tx.sender), tx.hash),
		storagePushValue(accountTxsKey(tx.recipient), tx.hash),
		AsyncStorage.setItem(txKey(tx.hash), JSON.stringify(tx))
	]);
}

export async function loadAccountTxHashes(
	account: TxParticipant
): Promise<string[]> {
	const result = await AsyncStorage.getItem(accountTxsKey(account));

	return result ? JSON.parse(result) : [];
}

export async function loadAccountTxs(
	account: TxParticipant
): Promise<Array<[string, Tx]>> {
	const hashes = await loadAccountTxHashes(account);

	return (
		await AsyncStorage.multiGet(hashes.map(txKey))
	).map((v: [string, any]) => [v[0], JSON.parse(v[1])]);
}

export async function loadToCAndPPConfirmation(): Promise<boolean> {
	const result = await AsyncStorage.getItem('ToCAndPPConfirmation_v4');

	return !!result;
}

export async function saveToCAndPPConfirmation(): Promise<void> {
	await AsyncStorage.setItem('ToCAndPPConfirmation_v4', 'yes');
}
