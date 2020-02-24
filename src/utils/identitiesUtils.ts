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

import { pathsRegex } from './regex';
import { decryptData } from './native';
import { parseSURI } from './suri';
import { generateAccountId } from './account';

import {
	NETWORK_LIST,
	SUBSTRATE_NETWORK_LIST,
	UnknownNetworkKeys
} from 'constants/networkSpecs';
import {
	Account,
	FoundAccount,
	FoundLegacyAccount,
	Identity,
	LockedAccount,
	PathGroup,
	SerializedIdentity,
	UnlockedAccount
} from 'types/identityTypes';

//walk around to fix the regular expression support for positive look behind;
export const removeSlash = (str: string): string => str.replace(/\//g, '');

export function isLegacyFoundAccount(
	foundAccount: FoundAccount
): foundAccount is FoundLegacyAccount {
	return foundAccount.isLegacy;
}

const extractPathId = (path: string): string | null => {
	const matchNetworkPath = path.match(pathsRegex.networkPath);
	if (!matchNetworkPath) return null;
	return removeSlash(matchNetworkPath[0]);
};

export const extractSubPathName = (path: string): string => {
	const pathFragments = path.match(pathsRegex.allPath);
	if (!pathFragments || pathFragments.length === 0) return '';
	if (pathFragments.length === 1) return removeSlash(pathFragments[0]);
	return removeSlash(pathFragments.slice(1).join(''));
};

export const isSubstratePath = (path: string): boolean =>
	path.match(pathsRegex.allPath) !== null || path === '';

export const isEthereumAccountId = (v: string): boolean =>
	v.indexOf('ethereum:') === 0;

export const extractAddressFromAccountId = (id: string): string => {
	const withoutNetwork = id.split(':')[1];
	const address = withoutNetwork.split('@')[0];
	if (address.indexOf('0x') !== -1) {
		return address.slice(2);
	}
	return address;
};

export const getAddressKeyByPath = (address: string, path: string): string =>
	isSubstratePath(path)
		? address
		: generateAccountId({ address, networkKey: getNetworkKeyByPath(path) });

export function emptyIdentity(): Identity {
	return {
		addresses: new Map(),
		derivationPassword: '',
		encryptedSeed: '',
		meta: new Map(),
		name: ''
	};
}

export const serializeIdentity = (identity: Identity): SerializedIdentity =>
	Object.entries(identity).reduce((newIdentity: any, entry: [string, any]) => {
		const [key, value] = entry;
		if (value instanceof Map) {
			newIdentity[key] = Array.from(value.entries());
		} else {
			newIdentity[key] = value;
		}
		return newIdentity;
	}, {});

export const deserializeIdentity = (
	identityJSON: SerializedIdentity
): Identity =>
	Object.entries(identityJSON).reduce(
		(newIdentity: any, entry: [string, any]) => {
			const [key, value] = entry;
			if (value instanceof Array) {
				newIdentity[key] = new Map(value);
			} else {
				newIdentity[key] = value;
			}
			return newIdentity;
		},
		{}
	);

export const serializeIdentities = (identities: Identity[]): string => {
	const identitiesWithObject = identities.map(serializeIdentity);
	return JSON.stringify(identitiesWithObject);
};

export const deserializeIdentities = (identitiesJSON: string): Identity[] => {
	const identitiesWithObject = JSON.parse(identitiesJSON);
	return identitiesWithObject.map(deserializeIdentity);
};

export const deepCopyIdentities = (identities: Identity[]): Identity[] =>
	deserializeIdentities(serializeIdentities(identities));

export const deepCopyIdentity = (identity: Identity): Identity =>
	deserializeIdentity(serializeIdentity(identity));

export const getPathsWithSubstrateNetworkKey = (
	paths: string[],
	networkKey: string
): string[] => {
	if (networkKey === UnknownNetworkKeys.UNKNOWN) {
		const pathIdList = Object.values(SUBSTRATE_NETWORK_LIST).map(
			networkParams => networkParams.pathId
		);
		return paths.filter(path => {
			const pathId = extractPathId(path);
			if (!isSubstratePath(path)) return false;
			return !pathId || !pathIdList.includes(pathId);
		});
	}
	return paths.filter(
		path => extractPathId(path) === SUBSTRATE_NETWORK_LIST[networkKey].pathId
	);
};

const getNetworkKeyByPathId = (pathId: string): string => {
	const networkKeyIndex = Object.values(SUBSTRATE_NETWORK_LIST).findIndex(
		networkParams => networkParams.pathId === pathId
	);
	if (networkKeyIndex !== -1)
		return Object.keys(SUBSTRATE_NETWORK_LIST)[networkKeyIndex];
	return UnknownNetworkKeys.UNKNOWN;
};

export const getNetworkKey = (path: string, identity: Identity): string => {
	if (identity.meta.has(path)) {
		const networkPathId = identity.meta.get(path)!.networkPathId;
		if (networkPathId) return getNetworkKeyByPathId(networkPathId);
	}
	return getNetworkKeyByPath(path);
};

export const getNetworkKeyByPath = (path: string): string => {
	if (!isSubstratePath(path) && NETWORK_LIST.hasOwnProperty(path)) {
		return path;
	}
	const pathId = extractPathId(path);
	if (!pathId) return UnknownNetworkKeys.UNKNOWN;

	return getNetworkKeyByPathId(pathId);
};

export const parseFoundLegacyAccount = (
	legacyAccount: Account,
	accountId: string
): FoundLegacyAccount => {
	const returnAccount: FoundLegacyAccount = {
		accountId,
		address: legacyAccount.address,
		createdAt: legacyAccount.createdAt,
		isLegacy: true,
		name: legacyAccount.name,
		networkKey: legacyAccount.networkKey,
		updatedAt: legacyAccount.updatedAt,
		validBip39Seed: legacyAccount.validBip39Seed
	};
	if (legacyAccount.hasOwnProperty('encryptedSeed')) {
		returnAccount.encryptedSeed = (legacyAccount as LockedAccount).encryptedSeed;
	}
	if (legacyAccount.hasOwnProperty('derivationPath')) {
		returnAccount.path = (legacyAccount as UnlockedAccount).derivationPath;
	}
	return returnAccount;
};

export const getIdentityFromSender = (
	sender: FoundAccount,
	identities: Identity[]
): Identity | undefined =>
	identities.find(i => i.encryptedSeed === sender.encryptedSeed);

export const getAddressWithPath = (
	path: string,
	identity: Identity
): string => {
	const pathMeta = identity.meta.get(path);
	if (!pathMeta) return '';
	const { address } = pathMeta;
	return isEthereumAccountId(address)
		? extractAddressFromAccountId(address)
		: address;
};

export const unlockIdentitySeed = async (
	pin: string,
	identity: Identity
): Promise<string> => {
	const { encryptedSeed } = identity;
	const seed = await decryptData(encryptedSeed, pin);
	const { phrase } = parseSURI(seed);
	return phrase;
};

export const getExistedNetworkKeys = (identity: Identity): string[] => {
	const pathsList = Array.from(identity.addresses.values());
	const networkKeysSet = pathsList.reduce((networksSet, path) => {
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

export const validateDerivedPath = (derivedPath: string): boolean =>
	pathsRegex.validateDerivedPath.test(derivedPath);

export const getIdentityName = (
	identity: Identity,
	identities: Identity[]
): string => {
	if (identity.name) return identity.name;
	const identityIndex = identities.findIndex(
		i => i.encryptedSeed === identity.encryptedSeed
	);
	return `Identity_${identityIndex}`;
};

export const getPathName = (path: string, lookUpIdentity: Identity): string => {
	if (
		lookUpIdentity &&
		lookUpIdentity.meta.has(path) &&
		lookUpIdentity.meta.get(path)!.name !== ''
	) {
		return lookUpIdentity.meta.get(path)!.name;
	}
	if (!isSubstratePath(path)) return 'No name';
	if (path === '') return 'Identity root';
	return extractSubPathName(path);
};

/**
 * This function decides how to group the list of derivation paths in the display based on the following rules.
 * If the network is unknown: group by the first subpath, e.g. '/random' of '/random//derivation/1'
 * If the network is known: group by the second subpath, e.g. '//staking' of '//kusama//staking/0'
 * Please refer to identitiesUtils.spec.js for more examples.
 **/
export const groupPaths = (paths: string[]): PathGroup[] => {
	const insertPathIntoGroup = (
		matchingPath: string,
		fullPath: string,
		pathGroup: PathGroup[]
	): void => {
		const matchResult = matchingPath.match(pathsRegex.firstPath);
		const groupName = matchResult ? matchResult[0] : '-';

		const existedItem = pathGroup.find(p => p.title === groupName);
		if (existedItem) {
			existedItem.paths.push(fullPath);
			existedItem.paths.sort();
		} else {
			pathGroup.push({ paths: [fullPath], title: groupName });
		}
	};

	const groupedPaths = paths.reduce(
		(groupedPath: PathGroup[], path: string) => {
			if (path === '') {
				groupedPath.push({ paths: [''], title: 'Identity root' });
				return groupedPath;
			}

			const rootPath = path.match(pathsRegex.firstPath)?.[0];
			if (rootPath === undefined) return groupedPath;

			const networkParams = Object.values(SUBSTRATE_NETWORK_LIST).find(
				v => `//${v.pathId}` === rootPath
			);
			if (networkParams === undefined) {
				insertPathIntoGroup(path, path, groupedPath);
				return groupedPath;
			}

			const isRootPath = path === rootPath;
			if (isRootPath) {
				groupedPath.push({
					paths: [path],
					title: `${networkParams.title} root`
				});
				return groupedPath;
			}

			const subPath = path.slice(rootPath.length);
			insertPathIntoGroup(subPath, path, groupedPath);

			return groupedPath;
		},
		[]
	);
	return groupedPaths.sort((a, b) => {
		if (a.paths.length === 1 && b.paths.length === 1) {
			return a.paths[0].length - b.paths[0].length;
		}
		return a.paths.length - b.paths.length;
	});
};
