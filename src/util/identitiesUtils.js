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

// import { NetworkProtocols } from '../constants';

export function emptyIdentity() {
	return {
		addresses: new Map(),
		derivationPassword: '',
		encryptedSeedPhrase: '',
		meta: new Map(),
		name: ''
	};
}

export const serializeIdentities = identities => {
	const changeMapToObject = identity =>
		Object.entries(identity).reduce((newIdentity, entry) => {
			let [key, value] = entry;
			if (value instanceof Map) {
				newIdentity[key] = Array.from(value.entries());
			} else {
				newIdentity[key] = value;
			}
			return newIdentity;
		}, {});
	const identitiesWithObject = identities.map(changeMapToObject);
	return JSON.stringify({ identities: identitiesWithObject });
};

export const deserializeIdentities = identitiesJSON => {
	const identitiesWithObject = JSON.parse(identitiesJSON).identities;
	const changeObjectToMap = identity =>
		Object.entries(identity).reduce((newIdentity, entry) => {
			let [key, value] = entry;
			if (value instanceof Array) {
				newIdentity[key] = new Map(value);
			} else {
				newIdentity[key] = value;
			}
			return newIdentity;
		}, {});
	return identitiesWithObject.map(changeObjectToMap);
};

// export function omit(object, omitKeys) {
// 	const result = Object.assign({}, object);
// 	for (const omitKey of omitKeys) {
// 		delete result[omitKey];
// 	}
// 	return result;
// }

// export function checkIdentityExistence(encryptedSeed, accountStore) {
// 	return accountStore.identities.find(
// 		identity => identity.encryptedSeed === encryptedSeed
// 	);
// }
//
// export function findIdentityIndexByAccountId(accountId, accountsStore) {
// 	const [, address, genesisHash] = accountId.split(':');
// 	accountsStore.identities.findIndex();
// }
