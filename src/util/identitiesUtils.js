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

import { NETWORK_LIST, SubstrateNetworkKeys } from '../constants';

export const defaultNetworkKey = SubstrateNetworkKeys.KUSAMA;

export function emptyIdentity() {
	return {
		addresses: new Map(),
		derivationPassword: '',
		encryptedSeedPhrase: '',
		meta: new Map(),
		name: ''
	};
}

export const serializeIdentity = identity =>
	Object.entries(identity).reduce((newIdentity, entry) => {
		let [key, value] = entry;
		if (value instanceof Map) {
			newIdentity[key] = Array.from(value.entries());
		} else {
			newIdentity[key] = value;
		}
		return newIdentity;
	}, {});

export const deserializeIdentity = identityJSON =>
	Object.entries(identityJSON).reduce((newIdentity, entry) => {
		let [key, value] = entry;
		if (value instanceof Array) {
			newIdentity[key] = new Map(value);
		} else {
			newIdentity[key] = value;
		}
		return newIdentity;
	}, {});

export const serializeIdentities = identities => {
	const identitiesWithObject = identities.map(serializeIdentity);
	return JSON.stringify(identitiesWithObject);
};

export const deserializeIdentities = identitiesJSON => {
	const identitiesWithObject = JSON.parse(identitiesJSON);
	return identitiesWithObject.map(deserializeIdentity);
};

export const deepCopyIdentities = identities =>
	deserializeIdentities(serializeIdentities(identities));
export const deepCopyIdentity = identity =>
	deserializeIdentity(serializeIdentity(identity));

export const getPathsWithNetwork = (paths, networkKey) =>
	paths.filter(path => path.split('//')[1] === NETWORK_LIST[networkKey].pathId);

// export const getNetworkKeyByPath = path => {
// 	const networkKeyIndex = Object.values(NETWORK_LIST).findIndex(
// 		networkParams => networkParams.pathId === path.split('//'[1])
// 	);
// 	if (networkKeyIndex !== -1) return Object.keys(NETWORK_LIST)[networkKeyIndex];
// 	return UnknownNetworkKeys.UNKNOWN;
// };

export const validatePath = path =>
	/^\/\/([\w-_])+(\/\/?([\w-_])+)+$/.test(path);

export const validateDerivedPath = derivedPath =>
	/^(\/\/?([\w-_])+)+$/.test(derivedPath);

export const getIdentityName = (identity, identities) => {
	if (identity.name) return identity.name;
	const identityIndex = identities.findIndex(
		i => i.encryptedSeed === identity.encryptedSeed
	);
	return `Identity_${identityIndex}`;
};

export const groupPaths = paths => {
	const unSortedPaths = paths.reduce((groupedPath, path) => {
		const subPath = path.split('//')[2] || '';
		const hardSubPath = subPath.split('/')[0] || '';
		const existedItem = groupedPath.find(p => p.title === hardSubPath);
		if (existedItem) {
			existedItem.paths.push(path);
		} else {
			groupedPath.push({ paths: [path], title: hardSubPath });
		}
		return groupedPath;
	}, []);
	return unSortedPaths.sort((a, b) => a.paths.length - b.paths.length);
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
