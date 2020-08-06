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

import React, { ReactElement } from 'react';
import { FlatList, StyleSheet } from 'react-native';

import { SUBSTRATE_NETWORK_LIST } from 'constants/networkSpecs';
import { NetworkCard } from 'components/AccountCard';
import { filterSubstrateNetworks } from 'modules/network/utils';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import { NetworkParams, SubstrateNetworkParams } from 'types/networkSpecsTypes';
import { NavigationProps } from 'types/props';
import colors from 'styles/colors';
import fonts from 'styles/fonts';
import ScreenHeading from 'components/ScreenHeading';

export default function NetworkSettings({
	navigation
}: NavigationProps<'NetworkSettings'>): React.ReactElement {
	// const { networkSpecs } = useNetworksContext();
	const networkParams = filterSubstrateNetworks(SUBSTRATE_NETWORK_LIST) as Array<
		[string, SubstrateNetworkParams]
	>;
	const renderNetwork = ({
		item
	}: {
		item: [string, SubstrateNetworkParams];
	}): ReactElement => {
		const networkSpec = item[1];
		return (
			<NetworkCard
				key={networkSpec.genesisHash + networkSpec.pathId}
				networkKey={networkSpec.genesisHash}
				onPress={(): void =>
					navigation.navigate('NetworkDetails', {
						pathId: networkSpec.pathId
					})
				}
				title={networkSpec.title}
			/>
		);
	};

	return (
		<SafeAreaViewContainer style={styles.body}>
			<ScreenHeading title="Supported Networks" />
			<FlatList
				data={networkParams}
				renderItem={renderNetwork}
				keyExtractor={(item: [string, NetworkParams]): string => item[0]}
			/>
			{/**
			<Button
				title="Add new network"
				onPress={() => {
					navigation.navigate('QrScanner', {
						isScanningNetworkSpec: true
					});
				}}
			/>
			 **/}
		</SafeAreaViewContainer>
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
