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

import PropTypes from 'prop-types';
import React from 'react';
import { ScrollView, StyleSheet, Text } from 'react-native';
import { Subscribe } from 'unstated';
import colors from '../colors';
import Background from '../components/Background';
import AccountsStore from '../stores/AccountsStore';

export default class Metadata extends React.PureComponent {
	static navigationOptions = {
		title: 'Manage Metadata'
	};
	render() {
		return (
			<Subscribe to={[AccountsStore]}>
				{accountsStore => {
					<MetadataListView
						{...this.props}
						accountsStore={accountsStore}
						onSelect={() => {
							this.props.navigation.navigation('MetadataDetailsView');
						}}
					/>;
				}}
			</Subscribe>
		);
	}
}

export class MetadataListView extends React.PureComponent {
	static propTypes = {
		accountsStore: PropTypes.instanceOf(AccountsStore).isRequired,
		onSelect: PropTypes.func.isRequired
	};

	render() {
		// const { accountsStore, onSelect } = this.props;

		<ScrollView contentContainerStyle={styles.bodyContent} style={styles.body}>
			<Background />
			<Text style={styles.topTitle}>Supported Metadata</Text>
			{
				// for each network key in NetworksStore
				// show network name, genesis hash
				// onselect, go to the details page
				// onAddNew, go to QRScanner with prop flag (scan new network spec)
			}
		</ScrollView>;
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
