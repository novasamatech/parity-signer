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

import { dummySubstrateNetworkParams, ETHEREUM_NETWORK_LIST, UnknownNetworkKeys, unknownNetworkParams, unknownNetworkPathId } from 'constants/networkSpecs';
import { createContext, default as React, useEffect, useMemo, useState } from 'react';
import { NetworkParams,SubstrateNetworkParams } from 'types/networkTypes';
import { NetworkParsedData } from 'types/scannerTypes';
import { loadNetworks, saveNetworks } from 'utils/db';
import { deepCopyNetworks, generateNetworkParamsFromParsedData } from 'utils/networksUtils';

// https://github.com/polkadot-js/ui/blob/f2f36e2db07f5faec14ee43cf4295f5e8a6f3cfa/packages/reactnative-identicon/src/icons/Polkadot.tsx#L37.

// we will need the generate function to be standardized to take an ss58 check address and isSixPoint boolean flag and returns a Circle https://github.com/polkadot-js/ui/blob/ff351a0f3160552f38e393b87fdf6e85051270de/packages/ui-shared/src/polkadotIcon.ts#L12.

export interface NetworksContextType {
	addNetwork(networkParsedData: NetworkParsedData): void;
	networks: Map<string, SubstrateNetworkParams>;
	allNetworks: Map<string, NetworkParams>;
	getSubstrateNetwork: (networkKey: string) => SubstrateNetworkParams;
	getNetwork: (networkKey?: string) => NetworkParams | null;
	pathIds: string[];
};

interface NetworksContextProviderProps {
	children?: React.ReactElement;
}

export const NetworksContext = createContext({} as NetworksContextType);

export function NetworksContextProvider({ children }: NetworksContextProviderProps): React.ReactElement {
	const [substrateNetworks, setSubstrateNetworks] = useState<Map<string, SubstrateNetworkParams>>(new Map());
	const allNetworks: Map<string, NetworkParams> = useMemo(() => {
		const ethereumNetworks: Map<string, NetworkParams> = new Map(Object.entries(ETHEREUM_NETWORK_LIST));

		return new Map([
			...ethereumNetworks,
			...substrateNetworks,
			[UnknownNetworkKeys.UNKNOWN, unknownNetworkParams]
		]);
	}, [substrateNetworks]);

	const pathIds = useMemo(() => {
		const result = Array.from(substrateNetworks.values())
			.map(n => n.pathId)
			.concat([unknownNetworkPathId]);

		return result;
	}, [substrateNetworks]);

	useEffect(() => {
		const refreshList = async function (): Promise<void> {
			const initNetworkSpecs = await loadNetworks();

			setSubstrateNetworks(initNetworkSpecs);
		};

		refreshList();
	}, []);

	function getSubstrateNetworkParams(networkKey: string): SubstrateNetworkParams {

		return substrateNetworks.get(networkKey) || dummySubstrateNetworkParams;
	}

	function getNetwork(networkKey?: string): NetworkParams | null {
		if (!networkKey) {
			return null;
		}

		return allNetworks.get(networkKey) || dummySubstrateNetworkParams;
	}

	function addNetwork(networkParsedData: NetworkParsedData): void {
		const newNetworkParams = generateNetworkParamsFromParsedData(networkParsedData);
		const networkKey = newNetworkParams.genesisHash;
		const newNetworksList = deepCopyNetworks(substrateNetworks);

		newNetworksList.set(networkKey, newNetworkParams);
		setSubstrateNetworks(newNetworksList);
		saveNetworks(newNetworkParams);
	}

	return (
		<NetworksContext.Provider value={{ addNetwork, allNetworks, getNetwork, getSubstrateNetwork: getSubstrateNetworkParams, networks: substrateNetworks, pathIds }}>
			{children}
		</NetworksContext.Provider>
	);
}
