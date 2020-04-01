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
import { TYPES_SPEC } from '@polkadot/types/known/overrides';
import { RegistryTypes } from '@polkadot/types/types';
import { Container } from 'unstated';

import { SUBSTRATE_NETWORK_LIST } from 'constants/networkSpecs';
import { getMetadata } from 'utils/identitiesUtils';

type RegistriesStoreState = {
	registries: Map<string, TypeRegistry>;
	dumbRegistry: TypeRegistry;
};

function getLatestVersionOverrideTypes(
	networkPathId: string
): undefined | RegistryTypes {
	if (!TYPES_SPEC.hasOwnProperty(networkPathId)) return undefined;
	const latestVersionNumber = TYPES_SPEC[networkPathId].length - 1;
	return TYPES_SPEC[networkPathId][latestVersionNumber].types;
}

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
		const override = getLatestVersionOverrideTypes(networkParams.pathId);
		if (override !== undefined) {
			newRegistry.register(override);
		}
		const metadata = new Metadata(newRegistry, networkMetadataRaw);
		newRegistry.setMetadata(metadata);
		registries.set(networkKey, newRegistry);
		return newRegistry;
	}
}
