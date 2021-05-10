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

import { pathsRegex } from './regex';
import { decryptData, substrateAddress } from './native';
import { constructSURI, parseSURI } from './suri';
import { generateAccountId } from './account';

import { NetworksContextState } from 'stores/NetworkContext';
import strings from 'modules/sign/strings';
import { SubstrateNetworkParams } from 'types/networkTypes';
import { TryCreateFunc } from 'utils/seedRefHooks';
import {
	ETHEREUM_NETWORK_LIST,
	UnknownNetworkKeys,
	unknownNetworkPathId
} from 'constants/networkSpecs';
import {
	Account,
	AccountMeta,
	FoundAccount,
	FoundLegacyAccount,
	Identity,
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

export const extractPathId = (path: string, pathIds: string[]): string => {
	const matchNetworkPath = path.match(pathsRegex.networkPath);
	if (matchNetworkPath && matchNetworkPath[0]) {
		const targetPathId = removeSlash(matchNetworkPath[0]);
		if (pathIds.includes(targetPathId)) {
			return targetPathId;
		}
	}
	return unknownNetworkPathId;
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

export const isSubstrateHardDerivedPath = (path: string): boolean => {
	if (!isSubstratePath(path)) return false;
	const pathFragments = path.match(pathsRegex.allPath);
	if (!pathFragments || pathFragments.length === 0) return false;
	return pathFragments.every((pathFragment: string) => {
		return pathFragment.substring(0, 2) === '//';
	});
};

export const extractAddressFromAccountId = (id: string): string => {
	const withoutNetwork = id.split(':')[1];
	const address = withoutNetwork.split('@')[0];
	if (address.indexOf('0x') !== -1) {
		return address.slice(2);
	}
	return address;
};

export const getAddressKeyByPath = (
	path: string,
	pathMeta: AccountMeta,
	networkContext: NetworksContextState
): string => {
	const { allNetworks } = networkContext;
	const address = pathMeta.address;
	return isSubstratePath(path)
		? address
		: generateAccountId(
				address,
				getNetworkKeyByPath(path, pathMeta, networkContext),
				allNetworks
		  );
};

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
	identity: Identity,
	networkKey: string,
	networkContextState: NetworksContextState
): string[] => {
	const { networks, pathIds } = networkContextState;
	const pathEntries = Array.from(identity.meta.entries());
	const targetPathId = networks.has(networkKey)
		? networks.get(networkKey)!.pathId
		: unknownNetworkPathId;
	const pathReducer = (
		groupedPaths: string[],
		[path, pathMeta]: [string, AccountMeta]
	): string[] => {
		let pathId;
		if (!isSubstratePath(path)) return groupedPaths;
		if (pathMeta.networkPathId !== undefined) {
			pathId = pathIds.includes(pathMeta.networkPathId)
				? pathMeta.networkPathId
				: unknownNetworkPathId;
		} else {
			pathId = extractPathId(path, pathIds);
		}
		if (pathId === targetPathId) {
			groupedPaths.push(path);
			return groupedPaths;
		}
		return groupedPaths;
	};
	return pathEntries.reduce(pathReducer, []);
};

export const getSubstrateNetworkKeyByPathId = (
	pathId: string,
	networks: Map<string, SubstrateNetworkParams>
): string => {
	const networkKeyIndex = Array.from(networks.entries()).findIndex(
		([, networkParams]) => networkParams.pathId === pathId
	);
	if (networkKeyIndex !== -1) {
		const findNetworkEntry: [string, SubstrateNetworkParams] = Array.from(
			networks.entries()
		)[networkKeyIndex];
		return findNetworkEntry[0];
	}
	return UnknownNetworkKeys.UNKNOWN;
};

export const getNetworkKey = (
	path: string,
	identity: Identity,
	networkContextState: NetworksContextState
): string => {
	if (identity.meta.has(path)) {
		return getNetworkKeyByPath(
			path,
			identity.meta.get(path)!,
			networkContextState
		);
	}
	return UnknownNetworkKeys.UNKNOWN;
};

export const getNetworkKeyByPath = (
	path: string,
	pathMeta: AccountMeta,
	networkContextState: NetworksContextState
): string => {
	const { networks, pathIds } = networkContextState;
	if (!isSubstratePath(path) && ETHEREUM_NETWORK_LIST.hasOwnProperty(path)) {
		//It is a ethereum path
		return path;
	}
	const pathId = pathMeta.networkPathId || extractPathId(path, pathIds);

	return getSubstrateNetworkKeyByPathId(pathId, networks);
};

export const parseFoundLegacyAccount = (
	legacyAccount: Account,
	accountId: string
): FoundLegacyAccount => {
	const returnAccount: FoundLegacyAccount = {
		accountId,
		address: legacyAccount.address,
		createdAt: legacyAccount.createdAt,
		encryptedSeed: legacyAccount.encryptedSeed,
		isLegacy: true,
		name: legacyAccount.name,
		networkKey: legacyAccount.networkKey,
		updatedAt: legacyAccount.updatedAt,
		validBip39Seed: legacyAccount.validBip39Seed
	};
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
	identity: Identity | null
): string => {
	if (identity == null) return '';
	const pathMeta = identity.meta.get(path);
	if (!pathMeta) return '';
	const { address } = pathMeta;
	return isEthereumAccountId(address)
		? extractAddressFromAccountId(address)
		: address;
};

export const unlockIdentitySeedWithReturn = async (
	pin: string,
	identity: Identity,
	createSeedRef: TryCreateFunc
): Promise<string> => {
	const { encryptedSeed } = identity;
	const seed = await decryptData(encryptedSeed, pin);
	await createSeedRef(pin);
	const { phrase } = parseSURI(seed);
	return phrase;
};

export const verifyPassword = async (
	password: string,
	seedPhrase: string,
	identity: Identity,
	path: string,
	networkContextState: NetworksContextState
): Promise<boolean> => {
	const { networks } = networkContextState;
	const suri = constructSURI({
		derivePath: path,
		password: password,
		phrase: seedPhrase
	});
	const networkKey = getNetworkKey(path, identity, networkContextState);
	const networkParams = networks.get(networkKey);
	if (!networkParams) throw new Error(strings.ERROR_NO_NETWORK);
	const address = await substrateAddress(suri, networkParams.prefix);
	const accountMeta = identity.meta.get(path);
	return address === accountMeta?.address;
};

export const getExistedNetworkKeys = (
	identity: Identity,
	networkContextState: NetworksContextState
): string[] => {
	const pathEntries = Array.from(identity.meta.entries());
	const networkKeysSet = pathEntries.reduce(
		(networksSet, [path, pathMeta]: [string, AccountMeta]) => {
			let networkKey;
			if (isSubstratePath(path)) {
				networkKey = getNetworkKeyByPath(path, pathMeta, networkContextState);
			} else {
				networkKey = path;
			}
			return { ...networksSet, [networkKey]: true };
		},
		{}
	);
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

export const getPathName = (
	path: string,
	lookUpIdentity: Identity | null
): string => {
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

const _comparePathGroups = (a: PathGroup, b: PathGroup): number => {
	const isSingleGroupA = a.paths.length === 1;
	const isSingleGroupB = b.paths.length === 1;
	if (isSingleGroupA && isSingleGroupB) {
		return a.paths[0].length - b.paths[0].length;
	}
	if (isSingleGroupA !== isSingleGroupB) {
		return isSingleGroupA ? -1 : 1;
	}
	return a.title.localeCompare(b.title);
};

const _comparePathsInGroup = (a: string, b: string): number => {
	const pathFragmentsA = a.match(pathsRegex.allPath)!;
	const pathFragmentsB = b.match(pathsRegex.allPath)!;
	if (pathFragmentsA.length !== pathFragmentsB.length) {
		return pathFragmentsA.length - pathFragmentsB.length;
	}
	const lastFragmentA = pathFragmentsA[pathFragmentsA.length - 1];
	const lastFragmentB = pathFragmentsB[pathFragmentsB.length - 1];
	const numberA = parseInt(removeSlash(lastFragmentA), 10);
	const numberB = parseInt(removeSlash(lastFragmentB), 10);
	const isNumberA = !isNaN(numberA);
	const isNumberB = !isNaN(numberB);
	if (isNumberA && isNumberB) {
		return numberA - numberB;
	}
	if (isNumberA !== isNumberB) {
		return isNumberA ? -1 : 1;
	}
	return lastFragmentA.localeCompare(lastFragmentB);
};

/**
 * This function decides how to group the list of derivation paths in the display based on the following rules.
 * If the network is unknown: group by the first subpath, e.g. '/random' of '/random//derivation/1'
 * If the network is known: group by the second subpath, e.g. '//staking' of '//kusama//staking/0'
 * Please refer to identitiesUtils.spec.js for more examples.
 **/
export const groupPaths = (
	paths: string[],
	networks: Map<string, SubstrateNetworkParams>
): PathGroup[] => {
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
			existedItem.paths.sort(_comparePathsInGroup);
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

			const networkEntry = Array.from(networks.entries()).find(
				([, v]) => `//${v.pathId}` === rootPath
			);
			if (networkEntry === undefined) {
				insertPathIntoGroup(path, path, groupedPath);
				return groupedPath;
			}

			const isRootPath = path === rootPath;
			if (isRootPath) {
				groupedPath.push({
					paths: [path],
					title: `${networkEntry[1].title} root`
				});
				return groupedPath;
			}

			const subPath = path.slice(rootPath.length);
			insertPathIntoGroup(subPath, path, groupedPath);

			return groupedPath;
		},
		[]
	);
	return groupedPaths.sort(_comparePathGroups);
};
