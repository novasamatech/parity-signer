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
import { MetadataCard } from 'modules/network/components/MetadataCard';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import { NetworksContext } from 'stores/NetworkContext';
import { NavigationProps } from 'types/props';
import { getSubstrateNetworkKeyByPathId } from 'utils/identitiesUtils';
import Button from 'components/Button';
import { RegistriesContext } from 'stores/RegistriesContext';
import { expandMetadata } from '@polkadot/metadata/decorate';
import { metadata } from '@polkadot/metadata';
import { RuntimeVersion } from '@polkadot/types/interfaces';

export default function NetworkDetails({
	navigation,
	route
}: NavigationProps<'NetworkDetails'>): React.ReactElement {
	const networkPathId = route.params.pathId;
	const { networks, getSubstrateNetwork } = useContext(NetworksContext);
	const networkKey = getSubstrateNetworkKeyByPathId(networkPathId, networks);
	const networkParams = getSubstrateNetwork(networkKey);
	const metadataHandle = networks.get(networkKey).metadata;

	const metadataValid = (): React.ReactElement => (
		<> 
			<MetadataCard
				spec_name={metadataHandle.specName}
				spec_version={metadataHandle.specVersion}
				onPress={(): void =>
					navigation.navigate('FullMetadata', {
						pathId: networkPathId
					})
				}
			/>
			<Button
				onPress={(): void => navigation.navigate('MetadataManagement')}
				title="Manage metadata"
			/>
		</>
	)

	const metadataInvalid = (): React.ReactElement => (
		<>
			<MetadataCard
				spec_name="invalid"
				spec_version="invalid"
				onPress={(): void =>
					navigation.navigate('FullMetadata')
				}
			/>

			<Button
				onPress={(): void => navigation.navigate('MetadataManagement')}
				title="Please add metadata!"
			/>
		</>
	)
	
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
			{metadataHandle? metadataValid() : metadataInvalid()}
		</SafeAreaScrollViewContainer>
	);
}
