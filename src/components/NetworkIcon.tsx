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

import React, { ReactElement } from 'react';
import { Image, ImageStyle, StyleSheet, View, ViewStyle } from 'react-native';
import FontAwesome from 'react-native-vector-icons/FontAwesome';

import { networkLogo } from 'utils/networkLogo';
import colors from 'styles/colors';

export default function NetworkIcon(props: {
	logo: string;
	style?: ViewStyle | ImageStyle;
}): ReactElement {
	const logoHandle = networkLogo(props.logo);
	return (
		<View style={styles.icon}>
			{logoHandle ? (
				<Image
					source={logoHandle} //TODO: dynamic logo storage
					style={styles.logo}
				/>
			) : (
				<View style={styles.logo}>
					<FontAwesome name="question" color={colors.text.main} size={28} />
				</View>
			)}
		</View>
	);
}

const styles = StyleSheet.create({
	icon: {
		height: 40,
		width: 40
	},
	logo: {
		alignItems: 'center',
		height: 36,
		justifyContent: 'center',
		marginHorizontal: 2,
		opacity: 0.7,
		width: 36
	}
});
