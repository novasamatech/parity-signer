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

import React, { useContext, useEffect, useState } from 'react';
import {FlatList, Text, View} from 'react-native';

import NetworkInfoCard from 'components/NetworkInfoCard';
import { NetworkCard } from 'components/NetworkCard';
import { MetadataCard } from 'components/MetadataCard';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { NavigationProps } from 'types/props';
import Button from 'components/Button';
import testIDs from 'e2e/testIDs';
import { getNetworkSpecs, removeNetwork, removeMetadata } from 'utils/native';
import fontStyles from 'styles/fontStyles';
import { AlertStateContext } from 'stores/alertContext';

export default function NetworkDetails({
	navigation,
	route
}: NavigationProps<'NetworkDetails'>): React.ReactElement {
	const networkKey = route.params.networkKey;
	const [network, setNetwork] = useState();
	const [metadata, setMetadata] = useState();
	const { setAlert } = useContext(AlertStateContext);

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

	const onDeleteMetadata = async (): Promise<void> => {
		try {
			console.log('deleting metadata');
			await removeMetadata(network.name, parseInt(metadata.spec_version));
			setMetadata();
			const networkSpecs = await getNetworkSpecs(networkKey);
			setNetwork(networkSpecs);
		} catch (e) {
			console.log(e);
			setAlert('Error!', e.toString());
		}
	};

	const onDeleteNetwork = async (): Promise<void> => {
		try {
			console.log('deleting network');
			await removeNetwork(network.genesis_hash);
			navigateToLandingPage(navigation);
		} catch (e) {
			console.log(e);
			setAlert('Error!', e.toString());
		}
	};

	const onExportMetadata = async (): Promise<void> => {};

	const onExportNetwork = async (): Promise<void> => {};

	const onPressExportMetadata = async (): Promise<void> => {};

	const onPressExportNetwork = async (): Promise<void> => {};

	const renderMetadata = ({item, index, separators}: {item: any, index: number, separators: any}): React.ReactElement => {
		return (
			<MetadataCard
				specName={network.name}
				specVersion={item.spec_version}
				selected={metadata === item}
				metadataHash={item.meta_hash}
				onPress={(): void => metadata === item ? setMetadata() : setMetadata(item)}
				onPressDelete={(): void => 
					setAlert(
						'Metadata removal', 
						`You are about to remove metadata ${network.name} version ${metadata.spec_version}, are you sure?`, 
						[
							{
								onPress: onDeleteMetadata, 
								text: 'Confirm'
							},
							{
								text: 'Cancel'
							}
						]
					)
				}
				onPressExport={onPressExportMetadata}
			/>
		);
	};

	if (network) {
		return (
		<SafeAreaViewContainer
			testID={testIDs.NetworkDetails.networkDetailsScreen}
		>
			<NetworkCard
				network={network}
			/>
			<NetworkInfoCard
				text={network.genesis_hash}
				label="Genesis Hash"
				small
			/>
			<View style={{flexDirection: 'row', justifyContent: 'space-evenly'}}>
				<Text style={{...fontStyles.t_important}}>Unit: </Text>
				<Text style={{...fontStyles.t_code}}>{network.unit}</Text>
				<Text style={{...fontStyles.t_important}}> Decimals: </Text>
				<Text style={{...fontStyles.t_code}}>{network.decimals}</Text>
				<Text style={{...fontStyles.t_important}}> b58 prefix: </Text>
				<Text style={{...fontStyles.t_code}}>{network.base58prefix}</Text>
			</View>
			<View style={{flexDirection: 'row', justifyContent: 'space-evenly'}}>
				<Button
					title="Delete"
					onPress={(): void => 
						setAlert(
							'Network removal', 
							`You are about to remove network ${network.name} and all associated metadata and identities, are you sure?`, 
							[
								{
									onPress: onDeleteNetwork, 
									text: 'Confirm'
								},
								{
									text: 'Cancel'
								}
							]
						)
					}
				/>
				<Button
					title="Sign"
					onPress={onPressExportNetwork}
				/>
			</View>
			<Text style={{...fontStyles.t_important, fontSize: 18, textAlign: 'center'}}>Metadata available</Text>
			<FlatList 
				data={network.meta}
				renderItem={renderMetadata}
				keyExtractor={(item): string => item.spec_version}
			/>
		</SafeAreaViewContainer>
	);
	} else {
		return ( <View/> );
	}
}
