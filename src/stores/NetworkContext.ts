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

import { useEffect, useState } from 'react';

import {
	SubstrateNetworkParams,
	SubstrateNetworkBasics
} from 'types/networkTypes';
import {
	getCompleteSubstrateNetworkSpec,
	checkNewNetworkSpecs
} from 'modules/network/utils';
import { loadNetworks } from 'utils/db';

// https://github.com/polkadot-js/ui/blob/f2f36e2db07f5faec14ee43cf4295f5e8a6f3cfa/packages/reactnative-identicon/src/icons/Polkadot.tsx#L37.

// we will need the generate function to be standardized to take an ss58 check address and isSixPoint boolean flag and returns a Circle https://github.com/polkadot-js/ui/blob/ff351a0f3160552f38e393b87fdf6e85051270de/packages/ui-shared/src/polkadotIcon.ts#L12.

type NetworkContextState = {
	networkSpecs: Map<string, SubstrateNetworkParams>;
};

const defaultState: NetworkContextState = {
	networkSpecs: new Map()
};

const deepCopy = (
	networkSpecs: Array<SubstrateNetworkParams>
): Array<SubstrateNetworkParams> => JSON.parse(JSON.stringify(networkSpecs));

export function useNetworksContext(): NetworkContextState {
	const [networkSpecs, setNetworkSpecs] = useState<
		Map<string, SubstrateNetworkParams>
	>(defaultState.networkSpecs);

	useEffect(() => {
		const refreshList = async function (): Promise<void> {
			const initNetworkSpecs = await loadNetworks();
			setNetworkSpecs(initNetworkSpecs);
		};
		refreshList();
	}, []);

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
		networkSpecs
	};
}
