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

import { Metadata, TypeRegistry } from '@polkadot/types';
import { getSpecTypes } from '@polkadot/types-known';
import { Container } from 'unstated';

import { SUBSTRATE_NETWORK_LIST } from 'constants/networkSpecs';
import { getMetadata } from 'utils/identitiesUtils';

//Map PathId to Polkadot.js/api spec names and chain names
type NetworkTypes = {
	alias?: string;
	chains: {
		[key: string]: string;
	};
};
type NetworkTypesMap = {
	[key: string]: NetworkTypes;
};
const networkTypesMap: NetworkTypesMap = {
	centrifuge: {
		alias: 'centrifuge-chain',
		chains: {}
	},
	kusama: { chains: {} },
	polkadot: {
		chains: {
			westend: 'Westend'
		}
	}
};

export const getOverrideTypes = (
	registry: TypeRegistry,
	pathId: string
): any => {
	let specName = '',
		chainName = '';
	Object.entries(networkTypesMap).find(
		([networkName, networkTypes]: [string, NetworkTypes]) => {
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
		}
	);
	return getSpecTypes(registry, chainName, specName);
};

type RegistriesStoreState = {
	registries: Map<string, TypeRegistry>;
	dumbRegistry: TypeRegistry;
};

export default class RegistriesStore extends Container<RegistriesStoreState> {
	state: RegistriesStoreState = {
		dumbRegistry: new TypeRegistry(),
		registries: new Map()
	};

	get(networkKey: string): TypeRegistry {
		const { registries } = this.state;
		if (!SUBSTRATE_NETWORK_LIST.hasOwnProperty(networkKey))
			return this.state.dumbRegistry;
		if (registries.has(networkKey)) return registries.get(networkKey)!;

		const networkParams = SUBSTRATE_NETWORK_LIST[networkKey];
		const newRegistry = new TypeRegistry();
		const networkMetadataRaw = getMetadata(networkKey);
		newRegistry.register(getOverrideTypes(newRegistry, networkParams.pathId));
		const metadata = new Metadata(newRegistry, networkMetadataRaw);
		newRegistry.setMetadata(metadata);
		registries.set(networkKey, newRegistry);
		return newRegistry;
	}
}
