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
	SUBSTRATE_NETWORK_LIST,
	UnknownNetworkKeys
} from '../constants';
import { pathsRegex } from './regex';
import { decryptData } from './native';
import { parseSURI } from './suri';
import { generateAccountId } from './account';

//walk around to fix the regular expression support for positive look behind;
export const removeSlash = str => str.replace(/\//g, '');

const extractPathId = path => {
	const matchNetworkPath = path.match(pathsRegex.networkPath);
	if (!matchNetworkPath) return null;
	return removeSlash(matchNetworkPath[0]);
};

export const extractSubPathName = path => {
	const pathFragments = path.match(pathsRegex.allPath);
	if (!pathFragments || pathFragments.length === 0) return '';
	if (pathFragments.length === 1) return removeSlash(pathFragments[0]);
	return removeSlash(pathFragments.slice(1).join(''));
};

export const isSubstratePath = path =>
	path.match(pathsRegex.allPath) != null || path === '';

export const isEthereumAccountId = v => v.indexOf('ethereum:') === 0;

export const extractAddressFromAccountId = id => {
	const withoutNetwork = id.split(':')[1];
	const address = withoutNetwork.split('@')[0];
	if (address.indexOf('0x') !== -1) {
		return address.slice(2);
	}
	return address;
};

export const getAddressKeyByPath = (address, path) =>
	isSubstratePath(path)
		? address
		: generateAccountId({ address, networkKey: getNetworkKeyByPath(path) });

export function emptyIdentity() {
	return {
		addresses: new Map(),
		derivationPassword: '',
		encryptedSeed: '',
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

export const getPathsWithSubstrateNetwork = (paths, networkKey) => {
	if (networkKey === UnknownNetworkKeys.UNKNOWN) {
		const pathIdList = Object.values(SUBSTRATE_NETWORK_LIST).map(
			networkParams => networkParams.pathId
		);
		return paths.filter(
			path => isSubstratePath(path) && !pathIdList.includes(extractPathId(path))
		);
	}
	return paths.filter(
		path => extractPathId(path) === NETWORK_LIST[networkKey].pathId
	);
};

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

export const getIdentityFromSender = (sender, identities) =>
	identities.find(i => i.encryptedSeed === sender.encryptedSeed);

export const getAddressWithPath = (path, identity) => {
	const pathMeta = identity.meta.get(path);
	if (!pathMeta) return '';
	const { address } = pathMeta;
	return isEthereumAccountId(address)
		? extractAddressFromAccountId(address)
		: address;
};

export const getRootPathMeta = (identity, networkKey) => {
	const rootPathId = `//${NETWORK_LIST[networkKey].pathId}`;
	if (identity.meta.has(rootPathId)) {
		return identity.meta.get(rootPathId);
	} else {
		return null;
	}
};

export const unlockIdentitySeed = async (pin, identity) => {
	const { encryptedSeed } = identity;
	const seed = await decryptData(encryptedSeed, pin);
	const { phrase } = parseSURI(seed);
	return phrase;
};

export const getExistedNetworkKeys = identity => {
	const pathsList = Array.from(identity.addresses.values());
	const networkKeysSet = pathsList.reduce((networksSet, path) => {
		if (path === '') return networksSet;
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
		return 'No name';
	}
	return extractSubPathName(path);
};

export const groupPaths = paths => {
	const unSortedPaths = paths.reduce((groupedPath, path) => {
		if (path === '') {
			return groupedPath;
		}
		const rootPath = path.match(pathsRegex.firstPath)[0];
		const isRootPath = path === rootPath;
		if (isRootPath) {
			const isUnknownRootPath = Object.values(NETWORK_LIST).every(
				v => `//${v.pathId}` !== rootPath
			);
			if (isUnknownRootPath) {
				groupedPath.push({ paths: [path], title: removeSlash(rootPath) });
			}
			return groupedPath;
		}

		const subPath = path.slice(rootPath.length);

		const groupName = subPath.match(pathsRegex.firstPath)[0];

		const existedItem = groupedPath.find(p => p.title === groupName);
		if (existedItem) {
			existedItem.paths.push(path);
			existedItem.paths.sort();
		} else {
			groupedPath.push({ paths: [path], title: groupName });
		}
		return groupedPath;
	}, []);
	return unSortedPaths.sort((a, b) => a.paths.length - b.paths.length);
};
