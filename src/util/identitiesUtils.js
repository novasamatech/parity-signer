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

import {
	NETWORK_LIST,
	NetworkProtocols,
	UnknownNetworkKeys
} from '../constants';
import { pathsRegex } from './regex';

//walk around to fix the regular expression support for positive look behind;
const removeSlash = str => str.replace(/\//g, '');

const extractPathId = path => {
	const matchNetworkPath = path.match(pathsRegex.networkPath);
	if (!matchNetworkPath) return null;
	return removeSlash(matchNetworkPath[0]);
};

const extractSubPathName = path => {
	const pathFragments = path.match(pathsRegex.allPath);
	if (!pathFragments || pathFragments.length <= 1) return '';
	return removeSlash(pathFragments.slice(1).join(''));
};

export const isSubstratePath = path => path.split('//')[1] !== undefined;

export function emptyIdentity() {
	return {
		accountIds: new Map(),
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

export const getPathsWithSubstrateNetwork = (paths, networkKey) =>
	paths.filter(path => extractPathId(path) === NETWORK_LIST[networkKey].pathId);

export const getNetworkKeyByPath = path => {
	if (!isSubstratePath(path) && NETWORK_LIST.hasOwnProperty(path)) {
		return path;
	}
	const pathId = extractPathId(path);
	if (!pathId) return UnknownNetworkKeys.UNKNOWN;

	const networkKeyIndex = Object.values(NETWORK_LIST).findIndex(
		networkParams => networkParams.pathId === pathId
	);
	if (networkKeyIndex !== -1) return Object.keys(NETWORK_LIST)[networkKeyIndex];

	return UnknownNetworkKeys.UNKNOWN;
};

export const getAccountIdWithPath = (path, identity) => {
	const pathMeta = identity.meta.get(path);
	if (pathMeta && pathMeta.accountId) return pathMeta.accountId;
	return '';
};

export const getAvailableNetworkKeys = identity => {
	const accountIdsList = Array.from(identity.accountIds.values());
	const networkKeysSet = accountIdsList.reduce((networksSet, path) => {
		let networkKey;
		if (isSubstratePath(path)) {
			networkKey = getNetworkKeyByPath(path);
		} else {
			networkKey = path;
		}
		return { ...networksSet, [networkKey]: true };
	}, {});
	return Object.keys(networkKeysSet);
};

export const getAddressFromAccountId = (accountId, protocol) => {
	if (!accountId) return '';
	if (protocol === NetworkProtocols.SUBSTRATE) {
		return accountId.split(':')[1] || accountId;
	} else {
		const withoutPrefix = accountId.split(':')[1] || accountId;
		const withOut0x = withoutPrefix.split('0x')[1] || accountId;
		return withOut0x.split('@')[0];
	}
};

export const validateDerivedPath = derivedPath =>
	pathsRegex.validateDerivedPath.test(derivedPath);

export const getIdentityName = (identity, identities) => {
	if (identity.name) return identity.name;
	const identityIndex = identities.findIndex(
		i => i.encryptedSeed === identity.encryptedSeed
	);
	return `Identity_${identityIndex}`;
};

export const getPathName = (path, lookUpIdentity) => {
	if (
		lookUpIdentity &&
		lookUpIdentity.meta.has(path) &&
		lookUpIdentity.meta.get(path).name !== ''
	) {
		return lookUpIdentity.meta.get(path).name;
	}
	if (!isSubstratePath(path)) {
		return 'New Account';
	}
	return extractSubPathName(path);
};

export const groupPaths = paths => {
	const unSortedPaths = paths.reduce((groupedPath, path) => {
		const pathId = extractPathId(path) || '';
		const subPath = path.slice(pathId.length + 2);

		const groupName = removeSlash(subPath.match(pathsRegex.firstPath)[0]);

		const existedItem = groupedPath.find(p => p.title === groupName);
		if (existedItem) {
			existedItem.paths.push(path);
		} else {
			groupedPath.push({ paths: [path], title: groupName });
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
