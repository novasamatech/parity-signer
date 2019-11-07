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

// import PropTypes from 'prop-types';
import React from 'react';
// import { Button, ScrollView, StyleSheet, Text, View } from 'react-native';
import { ScrollView } from 'react-native';
import { Subscribe } from 'unstated';

// import colors from '../colors';
import Background from '../components/Background';
// import Card from '../components/Card';
// import fonts from '../fonts';
import MetadataStore from '../stores/MetadataStore';
import { styles } from 'react-native-markdown-renderer';
import { SubstrateNetworkKeys } from '../constants';

export default class MetadataDetails extends React.PureComponent {
	render() {
		return (
			<Subscribe to={[MetadataStore]}>
				{metadataStore => {
					return (
						<MetadataDetailsView
							{...this.props}
							metadataStore={metadataStore}
							// networkKey={}
						/>
					);
				}}
			</Subscribe>
		);
	}
}

class MetadataDetailsView extends React.PureComponent {
	constructor(props) {
		super(props);
	}

	render() {
		const { metadataStore } = this.props;

		console.log(
			'meta new -> ',
			metadataStore.getMetadata(SubstrateNetworkKeys.KUSAMA)
		);

		return (
			<ScrollView
				contentContainerStyle={styles.bodyContent}
				style={styles.body}
			>
				<Background />
			</ScrollView>
		);
	}
}
