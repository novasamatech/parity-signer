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

import { FlatList } from 'react-native';
import React, { useContext, useState, useEffect } from 'react';

import { NetworkCard } from 'components/AccountCard';
import NetworkInfoCard from 'modules/network/components/NetworkInfoCard';
import { MetadataCard } from 'modules/network/components/MetadataCard';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { NetworksContext } from 'stores/NetworkContext';
import { NavigationProps } from 'types/props';
import { getSubstrateNetworkKeyByPathId } from 'utils/identitiesUtils';
import { MetadataHandle } from 'types/metadata';
import ScreenHeading from 'components/ScreenHeading';
import { getRelevantMetadata } from 'utils/db';

export default function MetadataManagement({
	navigation,
	route
}: NavigationProps<'MetadataManagement'>): React.ReactElement {
	const networkPathId = route.params.pathId;
	const { networks, getSubstrateNetwork, setMetadataVersion } = useContext(NetworksContext);
	const networkKey = getSubstrateNetworkKeyByPathId(networkPathId, networks);
	const networkParams = getSubstrateNetwork(networkKey);

	const [knownMetadata, setKnownMetadata] = useState<Array<MetadataHandle>>(new Array());

	useEffect(() => {
		const getKnownMetadata = async function (
			specName: string
		): Promise<void> {
			const newKnownMetadata = await getRelevantMetadata(specName);
			setKnownMetadata(newKnownMetadata);
		}
		getKnownMetadata(networkParams.metadata.specName);
	}, [networkPathId]);

	function setMetadata(metadataHandle: MetadataHandle): null {
		setMetadataVersion(networkKey, metadataHandle);
		navigation.goBack();
	}
	
	const renderMetadata = ({ item }): ReactElement => {
		return (
			<MetadataCard
				specName={item.specName}
				specVersion={item.specVersion}
				onPress={() => setMetadata(item)}
			/>
		)
	}

	return (
		<SafeAreaViewContainer>
			<ScreenHeading title={networkParams.title} />
			<FlatList
				data={knownMetadata}
				renderItem={renderMetadata}
				keyExtractor={(item) => item.hash}
			/>
		</SafeAreaViewContainer>
	);
}
