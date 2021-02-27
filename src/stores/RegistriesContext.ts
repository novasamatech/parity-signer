// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Modifications Copyright (c) 2021 Thibaut Sardan

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

import React, { useState } from 'react';
import { deepCopyMap } from 'stores/utils';
import { SubstrateNetworkParams } from 'types/networkTypes';
import { getMetadata } from 'utils/identitiesUtils';

import { Metadata } from '@polkadot/metadata';
import { TypeRegistry } from '@polkadot/types';
import { getSpecTypes } from '@polkadot/types-known';

//Map PathId to Polkadot.js/api spec names and chain names
type NetworkTypes = {
	alias?: string;
	chains: {
		[key: string]: string;
	};
};
type NetworkTypesMap = { [key: string]: NetworkTypes };
const networkTypesMap: NetworkTypesMap = {
	centrifuge: {
		alias: 'centrifuge-chain',
		chains: { centrifuge_amber: 'centrifuge-chain-amber' }
	},
	kusama: { chains: {} },
	polkadot: { chains: { westend: 'Westend' } },
	rococo: { chains: {} }
};

export const getOverrideTypes = (registry: TypeRegistry,
	pathId: string): any => {
	let specName = '',
		chainName = '';

	Object.entries(networkTypesMap).find(([networkName, networkTypes]: [string, NetworkTypes]) => {
		if (networkName === pathId) {
			specName = networkTypes.alias ?? networkName;
		} else if (networkTypes.chains.hasOwnProperty(pathId)) {
			const chainAlias = networkTypes.chains[pathId];

			specName = networkTypes.alias ?? networkName;
			chainName = chainAlias ?? pathId;
		} else {
			return false;
		}

		return true;
	});

	return getSpecTypes(registry, chainName, specName, Number.MAX_SAFE_INTEGER);
};

export interface RegistriesStoreState {
	registries: Map<string, TypeRegistry>;
	getTypeRegistry: (networks: Map<string, SubstrateNetworkParams>, networkKey: string) => TypeRegistry | null;
};

export function useRegistriesStore(): RegistriesStoreState {
	const [registries, setRegistries] = useState(new Map());

	function getTypeRegistry(networks: Map<string, SubstrateNetworkParams>,
		networkKey: string): TypeRegistry | null {
		try {
			const networkMetadataRaw = getMetadata(networkKey);

			if (networkMetadataRaw === null) return null;

			if (registries.has(networkKey)) return registries.get(networkKey)!;

			const networkParams = networks.get(networkKey)!;
			const newRegistry = new TypeRegistry();
			const overrideTypes = getOverrideTypes(newRegistry, networkParams.pathId);

			newRegistry.register(overrideTypes);
			const metadata = new Metadata(newRegistry, networkMetadataRaw);

			newRegistry.setMetadata(metadata);
			const newRegistries = deepCopyMap(registries);

			newRegistries.set(networkKey, newRegistry);
			setRegistries(newRegistries);

			return newRegistry;
		} catch (e) {
			console.log('oops', e);

			return null;
		}
	}

	return { getTypeRegistry, registries };
}

export const RegistriesContext = React.createContext({} as RegistriesStoreState);
