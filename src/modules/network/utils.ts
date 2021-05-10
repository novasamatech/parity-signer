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

import {
	SubstrateNetworkKeys,
	defaultNetworkKey,
	NETWORK_LIST,
	NetworkProtocols,
	UnknownNetworkKeys
} from 'constants/networkSpecs';
import {
	NetworkParams,
	SubstrateNetworkBasics,
	SubstrateNetworkParams
} from 'types/networkTypes';

export const filterNetworks = (
	networkList: Map<string, NetworkParams>,
	extraFilter?: (networkKey: string, shouldExclude: boolean) => boolean
): Array<[string, NetworkParams]> => {
	const excludedNetworks = [UnknownNetworkKeys.UNKNOWN];
	if (!__DEV__) {
		excludedNetworks.push(SubstrateNetworkKeys.SUBSTRATE_DEV);
		excludedNetworks.push(SubstrateNetworkKeys.KUSAMA_DEV);
	}

	const filterNetworkKeys = ([networkKey]: [string, any]): boolean => {
		const shouldExclude = excludedNetworks.includes(networkKey);
		if (extraFilter !== undefined)
			return extraFilter(networkKey, shouldExclude);
		return !shouldExclude;
	};
	return Array.from(networkList.entries())
		.filter(filterNetworkKeys)
		.sort((a, b) => a[1].order - b[1].order);
};

export const checkNewNetworkSpecs = (
	newNetworkSpec: SubstrateNetworkBasics
): void => {
	//TODO give feedback to UI, check unique of pathId
	if (
		!newNetworkSpec.genesisHash ||
		!newNetworkSpec.title ||
		!newNetworkSpec.unit ||
		!newNetworkSpec.pathId
	) {
		throw new Error('Network spec must include required field to be valid.');
	}
};

function generateRandomColor(): string {
	const letters = '0123456789ABCDEF';
	let color = '#';
	for (let i = 0; i < 6; i++) {
		color += letters[Math.floor(Math.random() * 16)];
	}
	return color;
}

export function getCompleteSubstrateNetworkSpec(
	newNetworkParams: SubstrateNetworkBasics
): SubstrateNetworkParams {
	const defaultNetworkSpec = NETWORK_LIST[
		defaultNetworkKey
	] as SubstrateNetworkParams;
	const defaultNewNetworkSpecParams = {
		color: generateRandomColor(),
		decimals: defaultNetworkSpec.decimals,
		deleted: false,
		logo: defaultNetworkSpec.logo,
		order: 102, //TODO
		prefix: defaultNetworkSpec.prefix,
		protocol: NetworkProtocols.SUBSTRATE,
		secondaryColor: generateRandomColor()
	};
	return { ...defaultNewNetworkSpecParams, ...newNetworkParams };
}
