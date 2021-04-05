// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Copyright 2021 Commonwealth Labs, Inc.
// This file is part of Layer Wallet.

// Layer Wallet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Layer Wallet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Layer Wallet. If not, see <http://www.gnu.org/licenses/>.

import { pathsRegex } from './regex';
import { decryptData } from './native';
import { parseSURI } from './suri';
import { generateAccountId } from './account';

import { NetworksContextState } from 'stores/NetworkContext';
import { SubstrateNetworkParams } from 'types/networkTypes';
import {
	ETHEREUM_NETWORK_LIST,
	SubstrateNetworkKeys,
	UnknownNetworkKeys,
	unknownNetworkPathId
} from 'constants/networkSpecs';
import {
	AccountMeta,
	FoundAccount,
	Identity,
	SerializedIdentity
} from 'types/identityTypes';
import {
	centrifugeAmberMetadata,
	centrifugeMetadata,
	edgewareMetadata,
	kulupuMetadata,
	kusamaMetadata,
	polkadotMetaData,
	rococoMetadata,
	westendMetadata
} from 'constants/networkMetadata';
import { PIN } from 'constants/pin';

//walk around to fix the regular expression support for positive look behind;
const removeSlash = (str: string): string => str.replace(/\//g, '');

const extractPathId = (path: string, pathIds: string[]): string => {
	const matchNetworkPath = path.match(pathsRegex.networkPath);
	if (matchNetworkPath && matchNetworkPath[0]) {
		const targetPathId = removeSlash(matchNetworkPath[0]);
		if (pathIds.includes(targetPathId)) {
			return targetPathId;
		}
	}
	return unknownNetworkPathId;
};

const extractSubPathName = (path: string): string => {
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
		encryptedSeed: '',
		meta: new Map(),
		name: ''
	};
}

const serializeIdentity = (identity: Identity): SerializedIdentity =>
	Object.entries(identity).reduce((newIdentity: any, entry: [string, any]) => {
		const [key, value] = entry;
		if (value instanceof Map) {
			newIdentity[key] = Array.from(value.entries());
		} else {
			newIdentity[key] = value;
		}
		return newIdentity;
	}, {});

const deserializeIdentity = (identityJSON: SerializedIdentity): Identity =>
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

export const getIdentitySeed = async (identity: Identity): Promise<string> => {
	const { encryptedSeed } = identity;
	const seed = await decryptData(encryptedSeed, PIN);
	const { phrase } = parseSURI(seed);
	return phrase;
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
	return `Wallet #${identityIndex + 1}`;
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

export const getMetadata = (networkKey: string): string | null => {
	switch (networkKey) {
		case SubstrateNetworkKeys.CENTRIFUGE:
			return centrifugeMetadata;
		case SubstrateNetworkKeys.CENTRIFUGE_AMBER:
			return centrifugeAmberMetadata;
		case SubstrateNetworkKeys.KUSAMA:
		case SubstrateNetworkKeys.KUSAMA_DEV:
			return kusamaMetadata;
		case SubstrateNetworkKeys.WESTEND:
			return westendMetadata;
		case SubstrateNetworkKeys.EDGEWARE:
			return edgewareMetadata;
		case SubstrateNetworkKeys.KULUPU:
			return kulupuMetadata;
		case SubstrateNetworkKeys.POLKADOT:
			return polkadotMetaData;
		case SubstrateNetworkKeys.ROCOCO:
			return rococoMetadata;
		default:
			return null;
	}
};
