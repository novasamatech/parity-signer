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

import React, { useContext } from 'react';

import { NetworkCard } from 'components/AccountCard';
import NetworkInfoCard from 'modules/network/components/NetworkInfoCard';
import { MetadataCard } from 'modules/network/components/MetadataCard';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import { NetworksContext } from 'stores/NetworkContext';
import { NavigationProps } from 'types/props';
import { getSubstrateNetworkKeyByPathId } from 'utils/identitiesUtils';
import Button from 'components/Button';
import testIDs from 'e2e/testIDs';

export default function NetworkDetails({
	navigation,
	route
}: NavigationProps<'NetworkDetails'>): React.ReactElement {
	const networkPathId = route.params.pathId as string;
	const { networks, getSubstrateNetwork } = useContext(NetworksContext);
	const networkKey = getSubstrateNetworkKeyByPathId(networkPathId, networks);
	const networkParams = getSubstrateNetwork(networkKey);
	const metadataHandle = networkParams.metadata;

	const metadataValid = (): React.ReactElement => (
		<>
			<MetadataCard
				specName={metadataHandle ? metadataHandle.specName : ''}
				specVersion={metadataHandle ? String(metadataHandle.specVersion) : ''}
				metadataHash={metadataHandle ? metadataHandle.hash : ''}
				onPress={(): void =>
					navigation.navigate('FullMetadata', {
						pathId: networkPathId
					})
				}
			/>
			<Button
				onPress={(): void =>
					navigation.navigate('MetadataManagement', {
						pathId: networkPathId
					})
				}
				testID={testIDs.NetworkDetails.manageValidMetadata}
				title="Manage metadata"
			/>
		</>
	);

	const metadataInvalid = (): React.ReactElement => (
		<>
			<MetadataCard
				specName="invalid"
				specVersion="invalid"
				metadataHash={metadataHandle ? metadataHandle.hash : 'invalid'}
				onPress={(): void =>
					navigation.navigate('FullMetadata', {
						pathId: networkPathId
					})
				}
			/>
			<Button
				onPress={(): void =>
					navigation.navigate('MetadataManagement', {
						pathId: networkPathId
					})
				}
				title="Please add metadata!"
			/>
		</>
	);

	return (
		<SafeAreaScrollViewContainer
			testID={testIDs.NetworkDetails.networkDetailsScreen}
		>
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
			{metadataHandle ? metadataValid() : metadataInvalid()}
		</SafeAreaScrollViewContainer>
	);
}
