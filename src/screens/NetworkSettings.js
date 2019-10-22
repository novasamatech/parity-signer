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
import { StyleSheet, Text } from 'react-native';
import { Subscribe } from 'unstated';

import colors from '../colors';
import { NetworksStore } from '../stores/NetworksStore';

export default class NetworkSettings extends React.PureComponent {
	render() {
		return (
			<Subscribe to={[NetworksStore]}>
				{networksStore => {
					return (
						<NetworkListView
							networksStore={networksStore}
							onSelect={() =>
								this.props.navigation.navigate('NetworkSpecDetailsView')
							}
						/>
					);
				}}
			</Subscribe>
		);
	}
}

class NetworkListView extends React.PureComponent {
	render() {
		// const { networksStore, onSelect } = this.props;

		debugger;

		return (
			// <ScrollView contentContainerStyle={styles.bodyContent} style={styles.body}>
			//   <Background />
			<Text style={styles.topTitle}>Supported Networks</Text>
			// {
			// for each network key in NetworksStore
			// show network name, genesis hash
			// onselect, go to the details page
			// onAddNew, go to QRScanner with prop flag (scan new network spec)
			// }
			// </ScrollView>
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
	}
});
