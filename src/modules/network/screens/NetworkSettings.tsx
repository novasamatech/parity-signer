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

'use strict';

import React from 'react';
import { Button, StyleSheet } from 'react-native';

import { SafeAreaScrollViewContainer } from 'components/SafeAreaContainer';
import { useNetworksContext } from 'stores/NetworkContext';
import { NavigationProps } from 'types/props';
import colors from 'styles/colors';
import NetworkCard from 'modules/network/components/NetworkCard';
import fonts from 'styles/fonts';
import ScreenHeading from 'components/ScreenHeading';

export default function NetworkSettings({
	navigation
}: NavigationProps<'NetworkSettings'>): React.ReactElement {
	const { networkSpecs } = useNetworksContext();

	return (
		<SafeAreaScrollViewContainer
			contentContainerStyle={styles.bodyContent}
			style={styles.body}
		>
			<ScreenHeading title="Supported Networks" />
			{networkSpecs.map(networkSpec => (
				<NetworkCard
					key={networkSpec.genesisHash}
					title={networkSpec.title}
					secondaryText={networkSpec.genesisHash}
					onPress={() =>
						navigation.navigate('NetworkDetails', {
							pathId: networkSpec.pathId
						})
					}
				/>
			))}
			<Button
				title="Add new network"
				onPress={() => {
					navigation.navigate('QrScanner', {
						isScanningNetworkSpec: true
					});
				}}
			/>
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
		color: colors.background.app,
		fontFamily: fonts.bold,
		fontSize: 18,
		paddingBottom: 10,
		textAlign: 'center'
	}
});
