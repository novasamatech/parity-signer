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

import React, {useEffect, useState} from 'react';
import {FlatList, View} from 'react-native';

import NetworkInfoCard from 'components/NetworkInfoCard';
import { NetworkCard } from 'components/NetworkCard';
import { MetadataCard } from 'components/MetadataCard';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import { NavigationProps } from 'types/props';
import Button from 'components/Button';
import testIDs from 'e2e/testIDs';
import { getNetworkSpecs, removeNetwork, removeMetadata } from 'utils/native';

export default function NetworkDetails({
	navigation,
	route
}: NavigationProps<'NetworkDetails'>): React.ReactElement {
	const networkKey = route.params.networkKey;
	const [network, setNetwork] = useState();

	useEffect(()=>{
		const prepareNetworkDetails = async (): Promise<void> => {
			try {
				const networkSpecs = await getNetworkSpecs(networkKey);
				setNetwork(networkSpecs);
			} catch (e) {
				console.log(e);
			}
		}
		prepareNetworkDetails();
	}, [networkKey]);

	const renderMetadata = ({item, index, separators}: {item: any, index: number, separators: any}): React.ReactElement => {
		return (
			<MetadataCard
				specName={network.name}
				specVersion={item.spec_version}
				metadataHash={item.meta_hash}
				onPress={(): void =>
					navigation.navigate('FullMetadata', {
						pathId: networkPathId
					})
				}
			/>
		);
	};

	if (network) {
		return (
		<SafeAreaScrollViewContainer
			testID={testIDs.NetworkDetails.networkDetailsScreen}
		>
			<NetworkCard
				network={network}
			/>
			<NetworkInfoCard text={network.title} label="Title" />
			<NetworkInfoCard text={network.path_id} label="Path ID" />
			<NetworkInfoCard
				text={network.genesis_hash}
				label="Genesis Hash"
				small
			/>
			<NetworkInfoCard text={network.unit} label="Unit" />
			<NetworkInfoCard
				text={network.decimals}
				label="Decimals"
			/>
			<NetworkInfoCard
				text={network.base58prefix}
				label="Base58 prefix"
			/>
			<FlatList 
				data={network.meta}
				renderItem={renderMetadata}
			/>
		</SafeAreaScrollViewContainer>
	);
	} else {
		return ( <View/> );
	}
}
