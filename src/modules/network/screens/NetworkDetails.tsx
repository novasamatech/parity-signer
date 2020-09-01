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

import React, { useContext } from 'react';

import { NetworkCard } from 'components/AccountCard';
import NetworkInfoCard from 'modules/network/components/NetworkInfoCard';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import { NetworksContext } from 'stores/NetworkContext';
import { NavigationProps } from 'types/props';
import { getSubstrateNetworkKeyByPathId } from 'utils/identitiesUtils';

export default function NetworkDetails({
	route
}: NavigationProps<'NetworkDetails'>): React.ReactElement {
	const networkPathId = route.params.pathId;
	const { networks, getSubstrateNetwork } = useContext(NetworksContext);
	const networkKey = getSubstrateNetworkKeyByPathId(networkPathId, networks);
	const networkParams = getSubstrateNetwork(networkKey);
	return (
		<SafeAreaScrollViewContainer>
			<NetworkCard
				networkKey={networkParams.genesisHash}
				title={networkParams.title}
			/>
			<NetworkInfoCard text={networkParams.title} label="Title" />
			<NetworkInfoCard text={networkParams.pathId} label="Path ID" />
			<NetworkInfoCard
				text={networkParams.genesisHash}
				label="Genesis Hash"
				small
			/>
			<NetworkInfoCard text={networkParams.unit} label="Unit" />
			<NetworkInfoCard
				text={networkParams.decimals.toString()}
				label="Decimals"
			/>
			<NetworkInfoCard
				text={networkParams.prefix.toString()}
				label="Address prefix"
			/>
		</SafeAreaScrollViewContainer>
	);
}
