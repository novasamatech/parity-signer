// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Modifications Copyright (c) 2021 Thibaut Sardan

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

import { NetworkCard } from 'components/NetworkCard';
import { SafeAreaViewContainer } from 'components/SafeAreaContainer';
import ScreenHeading from 'components/ScreenHeading';
import testIDs from 'e2e/testIDs';
import { filterNetworks } from 'modules/network/utils';
import React, { ReactElement, useContext } from 'react';
import { FlatList, StyleSheet } from 'react-native';
import colors from 'styles/colors';
import fonts from 'styles/fonts';
import { NetworkParams, SubstrateNetworkParams } from 'types/networkTypes';
import { NavigationProps } from 'types/props';

import { NetworksContext } from '../../../context';

export default function NetworkSettings({ navigation }: NavigationProps<'NetworkSettings'>): React.ReactElement {
	const { networks } = useContext(NetworksContext);
	const networkParams = filterNetworks(networks) as Array<
		[string, SubstrateNetworkParams]
	>;

	const renderNetwork = ({ item }: {
		item: [string, SubstrateNetworkParams];
	}): ReactElement => {
		const networkSpec = item[1];

		return (
			<NetworkCard
				key={networkSpec.genesisHash + networkSpec.pathId}
				networkKey={networkSpec.genesisHash}
				onPress={(): void =>
					navigation.navigate('NetworkDetails', { pathId: networkSpec.pathId })
				}
				testID={testIDs.NetworkSettings.networkCard + networkSpec.genesisHash}
				title={networkSpec.title}
			/>
		);
	};

	return (
		<SafeAreaViewContainer style={styles.body}>
			<ScreenHeading title="Supported Networks" />
			<FlatList
				data={networkParams}
				keyExtractor={(item: [string, NetworkParams]): string => item[0]}
				renderItem={renderNetwork}
			/>
		</SafeAreaViewContainer>
	);
}

const styles = StyleSheet.create({
	body: { padding: 20 },
	bodyContent: { paddingBottom: 40 },
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
