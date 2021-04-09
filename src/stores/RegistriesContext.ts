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

import { Metadata } from '@polkadot/metadata';
import { TypeRegistry } from '@polkadot/types';
import { RegisteredTypes } from '@polkadot/types/types';
import { getSpecTypes } from '@polkadot/types-known';
import { spec } from '@edgeware/node-types';
import React, { useState } from 'react';

import { deepCopyMap } from 'stores/utils';
import { SubstrateNetworkParams } from 'types/networkTypes';
import { getMetadata } from 'utils/identitiesUtils';

//Map PathId to Polkadot.js/api spec names and chain names
type NetworkTypesMap = {
	[key: string]: RegisteredTypes;
};
const networkTypesMap: NetworkTypesMap = {
	edgeware: { ...spec },
	kulupu: {
		types: {
			CampaignIdentifier: '[u8; 4]'
		}
	},
	kusama: {},
	polkadot: {},
	rococo: {},
	westend: {}
};

export const getOverrideTypes = (
	registry: TypeRegistry,
	pathId: string,
	specVersion: number
): any => {
	let specName = '';
	let chainName = '';
	Object.entries(networkTypesMap).find(
		([networkName, networkTypes]: [string, RegisteredTypes]) => {
			if (networkName === pathId) {
				specName = networkName;
				chainName = specName;
				registry.setKnownTypes(networkTypes);
				return true;
			} else {
				return false;
			}
		}
	);
	return getSpecTypes(registry, chainName, specName, specVersion);
};

export type RegistriesStoreState = {
	registries: Map<string, TypeRegistry>;
	getTypeRegistry: (
		networks: Map<string, SubstrateNetworkParams>,
		networkKey: string
	) => [TypeRegistry, Record<string, string>] | null;
};

export function useRegistriesStore(): RegistriesStoreState {
	const [registries, setRegistries] = useState(new Map());

	function getTypeRegistry(
		networks: Map<string, SubstrateNetworkParams>,
		networkKey: string
	): [TypeRegistry, Record<string, string>] | null {
		try {
			const foundMetadata = getMetadata(networkKey);
			if (foundMetadata === null) return null;
			const [networkMetadataRaw, specVersion] = foundMetadata;
			if (networkMetadataRaw === null) return null;
			const networkParams = networks.get(networkKey)!;
			const metadataKey = `${networkParams.genesisHash}-${specVersion}`;
			const metadataApiObj = { [metadataKey]: networkMetadataRaw };

			if (registries.has(networkKey)) {
				return [registries.get(networkKey)!, metadataApiObj];
			}

			const newRegistry = new TypeRegistry();
			const overrideTypes = getOverrideTypes(
				newRegistry,
				networkParams.pathId,
				specVersion
			);
			newRegistry.register(overrideTypes);
			const metadata = new Metadata(newRegistry, networkMetadataRaw);
			newRegistry.setMetadata(metadata);
			const newRegistries = deepCopyMap(registries);
			newRegistries.set(networkKey, newRegistry);
			setRegistries(newRegistries);
			return [newRegistry, metadataApiObj];
		} catch (e) {
			console.log('error', e);
			return null;
		}
	}

	return { getTypeRegistry, registries };
}

export const RegistriesContext = React.createContext(
	{} as RegistriesStoreState
);
