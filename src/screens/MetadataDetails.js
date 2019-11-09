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
import { ScrollView, StyleSheet, Text } from 'react-native';
import { Subscribe } from 'unstated';

import Background from '../components/Background';
import MetadataStore from '../stores/MetadataStore';
import NetworksStore from '../stores/NetworksStore';

export default class MetadataDetails extends React.PureComponent {
	render() {
		const networkKey = this.props.navigation.getParam('networkKey');

		return (
			<Subscribe to={[MetadataStore, NetworksStore]}>
				{(metadataStore, networksStore) => {
					return (
						<MetadataDetailsView
							{...this.props}
							metadataBlob={metadataStore.getMetaByKey(networkKey)}
							networkSpec={networksStore.getSelected()}
							networkKey={networkKey}
						/>
					);
				}}
			</Subscribe>
		);
	}
}

function MetadataDetailsView(props) {
	const { networkSpec } = props;

	console.log(networkSpec);
	debugger;

	return (
		<ScrollView contentContainerStyle={styles.bodyContent} style={styles.body}>
			<Background />
			<Text>Viewing metadata for: {networkSpec.title}</Text>
		</ScrollView>
	);
}

const styles = StyleSheet.create({
	bodyContent: {
		paddingBottom: 40
	}
});
