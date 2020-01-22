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
import PropTypes from 'prop-types';
import { StyleSheet, Text, View } from 'react-native';
import fontStyles from '../fontStyles';
import fonts from '../fonts';
import { SUBSTRATE_NETWORK_LIST } from '../constants';
import Icon from 'react-native-vector-icons/MaterialIcons';
import colors from '../colors';

NetworkSelector.protoTypes = {};

const onOptionSelect = value => {
	console.log('value selected', value);
};

export function NetworkSelector({ testID }) {
	const menuOptions = Object.entries(SUBSTRATE_NETWORK_LIST).map(
		([networkKey, networkParams]) => {
			return (
				<View style={styles.optionWrapper} key={networkKey} value={networkKey}>
					<Text style={styles.optionText}>{networkParams.title}</Text>
				</View>
			);
		}
	);
	return (
		<View style={styles.body}>
			<Text style={styles.label}>Account Network</Text>
			<View style={styles.menu} onSelect={onOptionSelect}>
				<View styles={styles.triggerWrapper}>
					<Text style={styles.triggerLabel}>Kusama</Text>
					<Icon
						name="more-vert"
						size={25}
						color={colors.bg_text}
						testID={testID}
					/>
				</View>
			</View>
			<View style={styles.optionsWrapper}>{menuOptions}</View>
		</View>
	);
}

export function NetworkOptions({}) {
	const menuOptions = Object.entries(SUBSTRATE_NETWORK_LIST).map(
		([networkKey, networkParams]) => {
			return (
				<View style={styles.optionWrapper} key={networkKey} value={networkKey}>
					<Text style={styles.optionText}>{networkParams.title}</Text>
				</View>
			);
		}
	);

	return <View style={styles.optionsWrapper}>{menuOptions}</View>;
}

const styles = StyleSheet.create({
	body: {
		flex: 1,
		marginVertical: 8,
		paddingHorizontal: 16
	},
	label: {
		flex: 1,
		marginBottom: 3,
		...fontStyles.t_regular
	},
	menu: {
		flex: 1
	},
	menuOption: {
		width: '100%'
	},
	triggerWrapper: {
		height: 40,
		paddingTop: 8,
		backgroundColor: colors.bg,
		borderBottomWidth: 1,
		borderBottomColor: colors.bg_text,
		alignItems: 'center',
		flexDirection: 'row'
	},
	triggerLabel: {
		flex: 1,
		...fontStyles.h2
	},
	optionsWrapper: {
		backgroundColor: 'white',
		position: 'absolute',
		left: 0,
		right: 0,
		bottom: 0
	},
	optionText: {
		fontFamily: fonts.regular,
		fontSize: 16
	},
	optionWrapper: {}
});
