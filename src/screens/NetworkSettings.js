// Copyright 2015-2019 Parity Technologies (UK) Ltd.
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
import { Button, ScrollView, StyleSheet } from 'react-native';
import { Subscribe } from 'unstated';

import colors from 'styles/colors';
import Background from 'components/Background';
import NetworkCard from 'components/NetworkCard';
import fonts from 'styles/fonts';
// import MetadataDetails from '../screens/MetadataDetails';
import NetworksStore from 'stores/NetworksStore';
import ScreenHeading from 'components/ScreenHeading';

export default class NetworkSettings extends React.PureComponent {
	handleSelect = async (networkKey, networksStore) => {
		await networksStore.select(networkKey);

		this.props.navigation.navigate('MetadataDetails', {
			networkKey: networkKey
		});
	};

	render() {
		return (
			<Subscribe to={[NetworksStore]}>
				{networksStore => {
					return (
						<NetworkListView
							{...this.props}
							networkSpecs={networksStore.getNetworkSpecs()}
							onSelect={networkKey =>
								this.handleSelect(networkKey, networksStore)
							}
						/>
					);
				}}
			</Subscribe>
		);
	}
}

class NetworkListView extends React.PureComponent {
	constructor(props) {
		super(props);
	}

	render() {
		const { networkSpecs, onSelect, navigation } = this.props;

		return (
			<ScrollView
				contentContainerStyle={styles.bodyContent}
				style={styles.body}
			>
				<Background />
				<ScreenHeading title="Supported Networks" />
				{networkSpecs.map(networkSpec => (
					<NetworkCard
						key={networkSpec.genesisHash}
						title={networkSpec.title}
						secondaryText={networkSpec.genesisHash}
						onPress={() => onSelect(networkSpec.genesisHash)}
					/>
				))}
				<Button
					title="Add new network"
					onPress={() =>
						navigation.navigate('QrScanner', {
							isScanningNetworkSpec: true
						})
					}
				/>
			</ScrollView>
		);
	}
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column',
		overflow: 'scroll',
		padding: 20
	},
	bodyContent: {
		paddingBottom: 40
	},
	bodyContent2: {
		display: 'flex',
		flexDirection: 'column',
		height: '100%',
		justifyContent: 'space-between'
	},
	descSecondary: {
		color: colors.bg_text,
		flex: 1,
		fontFamily: fonts.bold,
		fontSize: 14,
		paddingBottom: 20
	},
	descTitle: {
		color: colors.bg_text,
		fontFamily: fonts.bold,
		fontSize: 18,
		paddingBottom: 10,
		textAlign: 'center'
	}
});
