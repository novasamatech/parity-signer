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

// @flow

import { SUBSTRATE_NETWORK_LIST, SubstrateNetworkKeys } from '../constants';

export function empty() {
	return {
		color: undefined,
		decimals: undefined,
		genesisHash: undefined,
		prefix: undefined,
		protocol: undefined,
		secondaryColor: undefined,
		title: undefined,
		unit: undefined
	};
}

export function defaultNetworkSpecs() {
	const excludedNetworks = [SubstrateNetworkKeys.KUSAMA_CC2];
	if (!__DEV__) {
		excludedNetworks.push(SubstrateNetworkKeys.SUBSTRATE_DEV);
		excludedNetworks.push(SubstrateNetworkKeys.KUSAMA_DEV);
	}
	return Object.entries(SUBSTRATE_NETWORK_LIST).reduce(
		(networkSpecsList, [networkKey, networkParams]) => {
			if (excludedNetworks.includes(networkKey)) return networkSpecsList;
			networkSpecsList.push(networkParams);
			return networkSpecsList;
		},
		[]
	);
}
