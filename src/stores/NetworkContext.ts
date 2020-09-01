// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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

import { default as React, useEffect, useMemo, useState } from 'react';

import {
	dummySubstrateNetworkParams,
	ETHEREUM_NETWORK_LIST,
	unknownNetworkPathId
} from 'constants/networkSpecs';
import { SubstrateNetworkParams, NetworkParams } from 'types/networkTypes';
import { loadNetworks } from 'utils/db';

// https://github.com/polkadot-js/ui/blob/f2f36e2db07f5faec14ee43cf4295f5e8a6f3cfa/packages/reactnative-identicon/src/icons/Polkadot.tsx#L37.

// we will need the generate function to be standardized to take an ss58 check address and isSixPoint boolean flag and returns a Circle https://github.com/polkadot-js/ui/blob/ff351a0f3160552f38e393b87fdf6e85051270de/packages/ui-shared/src/polkadotIcon.ts#L12.

export type GetNetwork = (networkKey: string) => NetworkParams;
export type GetSubstrateNetwork = (
	networkKey: string
) => SubstrateNetworkParams;
export type NetworksContextState = {
	networks: Map<string, SubstrateNetworkParams>;
	allNetworks: Map<string, NetworkParams>;
	getSubstrateNetwork: GetSubstrateNetwork;
	getNetwork: GetNetwork;
	pathIds: string[];
};

const deepCopy = (
	networkSpecs: Array<SubstrateNetworkParams>
): Array<SubstrateNetworkParams> => JSON.parse(JSON.stringify(networkSpecs));

export function useNetworksContext(): NetworksContextState {
	const [substrateNetworks, setSubstrateNetworks] = useState<
		Map<string, SubstrateNetworkParams>
	>(new Map());
	const allNetworks: Map<string, NetworkParams> = useMemo(() => {
		const ethereumNetworks: Map<string, NetworkParams> = new Map(
			Object.entries(ETHEREUM_NETWORK_LIST)
		);
		const all = new Map([...ethereumNetworks, ...substrateNetworks]);
		console.log('all is', all);
		return all;
	}, [substrateNetworks]);

	const pathIds = useMemo(() => {
		const result = Array.from(substrateNetworks.values())
			.map(n => n.pathId)
			.concat([unknownNetworkPathId]);
		console.log('path ids are', result);
		return result;
	}, [substrateNetworks]);

	useEffect(() => {
		const refreshList = async function (): Promise<void> {
			const initNetworkSpecs = await loadNetworks();
			setSubstrateNetworks(initNetworkSpecs);
			console.log('loaded Networks are:', initNetworkSpecs);
		};
		refreshList();
	}, []);

	function getSubstrateNetworkParams(
		networkKey: string
	): SubstrateNetworkParams {
		return substrateNetworks.get(networkKey) || dummySubstrateNetworkParams;
	}

	function getNetwork(networkKey: string): NetworkParams {
		return allNetworks.get(networkKey) || dummySubstrateNetworkParams;
	}

	// async function submitNewNetworkSpec(): Promise<void> {
	// 	if (newNetworkSpecs === null)
	// 		throw new Error('NetworkKey is not initialized.');
	//
	// 	checkNewNetworkSpecs(newNetworkSpecs);
	// 	const updatedNetworkSpecs = deepCopy(networkSpecs);
	// 	const networkIndex = updatedNetworkSpecs.findIndex(
	// 		networkSpec => networkSpec.genesisHash === newNetworkSpecs.genesisHash
	// 	);
	// 	const completeNetworkSpec = getCompleteSubstrateNetworkSpec(
	// 		newNetworkSpecs
	// 	);
	// 	if (networkIndex === -1) {
	// 		updatedNetworkSpecs.push(completeNetworkSpec);
	// 	} else {
	// 		updatedNetworkSpecs.splice(networkIndex, 1, completeNetworkSpec);
	// 	}
	//
	// 	setNetworkSpecs(updatedNetworkSpecs);
	// 	setNewNetworkSpecs(defaultState.newNetworkSpecs);
	//
	// 	try {
	// 		await saveNetworkSpecs(updatedNetworkSpecs);
	// 	} catch (e) {
	// 		//TODO give feedback to UI
	// 		console.error(e);
	// 	}
	// }
	//
	// async function deleteNetwork(networkKey: string): Promise<void> {
	// 	const updatedNetworkSpecs = deepCopy(networkSpecs);
	// 	const networkIndex = updatedNetworkSpecs.findIndex(
	// 		networkSpec => networkSpec.genesisHash === networkKey
	// 	);
	// 	if (networkIndex === -1) return;
	//
	// 	updatedNetworkSpecs.splice(networkIndex, 1);
	// 	setNetworkSpecs(networkSpecs);
	//
	// 	try {
	// 		await saveNetworkSpecs(updatedNetworkSpecs);
	// 	} catch (e) {
	// 		//TODO give feedback to UI
	// 		console.error(e);
	// 	}
	// }

	return {
		allNetworks: allNetworks,
		getNetwork,
		getSubstrateNetwork: getSubstrateNetworkParams,
		networks: substrateNetworks,
		pathIds
	};
}

export const NetworksContext = React.createContext({} as NetworksContextState);
