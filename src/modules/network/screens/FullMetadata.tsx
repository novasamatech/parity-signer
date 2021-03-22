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

import React, { ReactElement, useContext, useEffect, useState } from 'react';
import { FlatList, StyleSheet, Text } from 'react-native';

import testIDs from 'e2e/testIDs';
import { NetworkInfoCard } from 'modules/network/components/NetworkInfoCard';
import { filterNetworks } from 'modules/network/utils';
import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import { NetworksContext } from 'stores/NetworkContext';
import { NetworkParams, SubstrateNetworkParams } from 'types/networkTypes';
import { NavigationProps } from 'types/props';
import { getSubstrateNetworkKeyByPathId } from 'utils/identitiesUtils';
import { RegistriesContext } from 'stores/RegistriesContext';
import { getMetadata } from 'utils/db';
//import { useFullMetadataHook } from 'modules/network/networksHooks';
import colors from 'styles/colors';
import fonts from 'styles/fonts';
import ScreenHeading from 'components/ScreenHeading';

export default function FullMetadata({
	navigation,
	route
}: NavigationProps<'NetworkSettings'>): React.ReactElement {
	const networkPathId = route.params.pathId;
	const { networks, getSubstrateNetwork } = useContext(NetworksContext);
	const { getTypeRegistry, registry } = useContext(RegistriesContext);
	const [ savedMetadata, setSavedMetadata ] = useState<string>('');
	const networkKey = getSubstrateNetworkKeyByPathId(networkPathId, networks);
	const networkParams = getSubstrateNetwork(networkKey);
	const metadataHandle = networks.get(networkKey).metadata;
	const typeRegistry = getTypeRegistry(networks, networkKey, metadataHandle);
	//const registeredMetadata = getRegisteredMetadata(typeRegistry, metadataHandle);
	//const [ metadataReady, savedMetadata ] = useFullMetadataHook(metadataHandle);
	const [ metadataReady, setMetadataReady ] = useState<bool>(false);

	useEffect(() => {
		const getSavedMetadata = async function (): Promise<void> {
			const getSavedMetadata = await getMetadata(metadataHandle);
			setSavedMetadata(getSavedMetadata);
			setMetadataReady(true);
		};
		getSavedMetadata();
	}, [setSavedMetadata, setMetadataReady, metadataHandle]);
	console.log(typeof savedMetadata);
	console.log(metadataReady);

	
	function showFullMetadata(): React.ReactNode{
		if(metadataReady) {
			return (
				<Text>{savedMetadata}</Text>
			);
		} else {
			return;
		}
	}

	return (
		<SafeAreaScrollViewContainer style={styles.body}>
				{showFullMetadata()}
		</SafeAreaScrollViewContainer>
	);
}

const styles = StyleSheet.create({
	body: {
		padding: 20
	},
	bodyContent: {
		paddingBottom: 40
	},
	descSecondary: {
		color: colors.background.app,
		flex: 1,
		fontFamily: fonts.bold,
		fontSize: 14,
		paddingBottom: 20
	},
	descTitle: {
		color: colors.text.main,
		fontFamily: fonts.bold,
		fontSize: 18,
		paddingBottom: 10,
		textAlign: 'center'
	}
});
