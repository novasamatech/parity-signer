// Copyright 2015-2021 Parity Technologies (UK) Ltd.
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
import { StyleSheet, Text, View } from 'react-native';

import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';

export default function NetworkInfoCard(props: {
	text: string;
	label: string;
	small?: boolean;
}): React.ReactElement {
	const { label, small, text } = props;
	return (
		<View style={styles.body}>
			<View style={styles.label}>
				<Text style={fontStyles.t_important}>{label}</Text>
			</View>
			<View style={styles.content}>
				<Text style={small ? fontStyles.t_codeS : fontStyles.t_code}>
					{text}
				</Text>
			</View>
		</View>
	);
}

const styles = StyleSheet.create({
	body: {
		backgroundColor: colors.background.app,
		flexDirection: 'row'
	},
	content: {
		alignItems: 'flex-start',
		flex: 3,
		justifyContent: 'center',
		padding: 20
	},
	label: {
		alignItems: 'flex-start',
		flex: 1,
		justifyContent: 'center',
		padding: 20
	}
});
