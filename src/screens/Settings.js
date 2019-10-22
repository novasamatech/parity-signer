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
import { StyleSheet, Text, View } from 'react-native';

import colors from '../colors';
import fonts from '../fonts';
import Card from '../components/Card';

export default class Settings extends React.PureComponent {
	static navigationOptions = {
		title: 'Settings'
	};

	render() {
		const { navigate } = this.props.navigation;

		return (
			<View style={styles.body}>
				<View style={styles.header}>
					<Text style={styles.title}>SETTINGS</Text>
				</View>
				<View style={styles.menuView}>
					<Card
						onPress={() => navigate('AccountSettings')}
						title="Account Settings"
						secondaryText="Create or recover an account."
						style={{ padding: 10 }}
					/>
					<Card
						onPress={() => navigate('NetworkSettings')}
						title="Network Settings"
						secondaryText="Add, update, or delete network specs."
						style={{ padding: 10 }}
					/>
				</View>
			</View>
		);
	}
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.bg,
		flex: 1,
		flexDirection: 'column',
		padding: 20
	},
	header: {
		alignItems: 'center',
		flexDirection: 'row',
		justifyContent: 'flex-start',
		paddingBottom: 20
	},
	menuView: {
		alignItems: 'stretch',
		display: 'flex',
		flex: 1,
		flexDirection: 'column',
		justifyContent: 'flex-start'
	},
	title: {
		color: colors.bg_text_sec,
		flexDirection: 'column',
		fontFamily: fonts.bold,
		fontSize: 18,
		justifyContent: 'center'
	}
});
