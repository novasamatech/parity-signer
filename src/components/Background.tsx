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

import React from 'react';
import { StyleSheet, View } from 'react-native';

import colors from 'styles/colors';

export default class Background extends React.PureComponent {
	render(): React.ReactElement {
		return (
			<View style={styles.bg}>
				{/* <View style={styles.lines}>{lines}</View> */}
			</View>
		);
	}
}

const styles = StyleSheet.create({
	bg: {
		backgroundColor: colors.bg,
		flex: 1,
		position: 'absolute'
	},
	line: {
		backgroundColor: colors.bg,
		borderBottomColor: '#3d3d3d',
		borderBottomWidth: 2,
		height: 60,
		width: 4000,
		zIndex: -1000
	},
	lines: {
		position: 'absolute',
		transform: [
			{ rotate: '-30deg' },
			{ translateX: -300 },
			{ translateY: -3100 },
			{ scale: 0.2 }
		],
		zIndex: -1000
	}
});
