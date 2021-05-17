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

import { FlatList } from 'react-native';
import React, { useContext, useState, useEffect, ReactElement } from 'react';

import { MetadataCard } from 'modules/network/components/MetadataCard';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { NetworksContext } from 'stores/NetworkContext';
import { NavigationProps } from 'types/props';
import { getSubstrateNetworkKeyByPathId } from 'utils/identitiesUtils';
import { MetadataHandle } from 'types/metadata';
import ScreenHeading from 'components/ScreenHeading';
import { getRelevantMetadata, getAllMetadata, deleteMetadata } from 'utils/db';
import MetadataManagerTab from 'components/MetadataManagerTab';
import testIDs from 'e2e/testIDs';

export default function MetadataManagement({
	navigation,
	route
}: NavigationProps<'MetadataManagement'>): React.ReactElement {
	const networkPathId = route.params.pathId;
	const {
		networks,
		getSubstrateNetwork,
		setMetadataVersion,
		isMetadataActive
	} = useContext(NetworksContext);
	const networkKey = getSubstrateNetworkKeyByPathId(networkPathId, networks);
	const networkParams = getSubstrateNetwork(networkKey);
	const [knownMetadata, setKnownMetadata] = useState<Array<MetadataHandle>>([]);
	const [showAll, setShowAll] = useState<boolean>(false);
	const [deletionMode, setDeletionMode] = useState<boolean>(false);

	useEffect(() => {
		const getKnownMetadata = async function (specName: string): Promise<void> {
			if (showAll) {
				const newKnownMetadata = await getAllMetadata();
				setKnownMetadata(newKnownMetadata);
			} else {
				const newKnownMetadata = await getRelevantMetadata(specName);
				setKnownMetadata(newKnownMetadata);
			}
		};
		if (networkParams.metadata)
			getKnownMetadata(networkParams.metadata.specName);
	}, [networkParams, showAll]);

	function chooseMetadata(metadataHandle: MetadataHandle): void {
		if (deletionMode) {
			console.log(metadataHandle);
			console.log(networkParams.metadata);
			if (isMetadataActive(metadataHandle)) {
				console.warn('Metadata in use, please release it first');
			} else {
				deleteMetadata(metadataHandle);
			}
		} else {
			setMetadataVersion(networkKey, metadataHandle);
		}
		navigation.goBack();
	}

	const renderMetadata = ({ item }: { item: MetadataHandle }): ReactElement => {
		return (
			<MetadataCard
				specName={item.specName}
				specVersion={String(item.specVersion)}
				metadataHash={item.hash}
				onPress={(): void => chooseMetadata(item)}
			/>
		);
	};

	return (
		<SafeAreaViewContainer>
			<ScreenHeading
				title={deletionMode ? 'Delete metadata record' : networkParams.title}
			/>
			<FlatList
				data={knownMetadata}
				renderItem={renderMetadata}
				keyExtractor={(item: MetadataHandle): string => item.hash}
			/>
			<MetadataManagerTab
				deletion={(): void => setDeletionMode(!deletionMode)}
				showall={(): void => setShowAll(!showAll)}
				isDeletion={deletionMode}
			/>
		</SafeAreaViewContainer>
	);
}
