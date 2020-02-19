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
import { Image, StyleSheet, Text, View, ViewStyle } from 'react-native';

import colors from 'styles/colors';
import fonts from 'styles/fonts';
import iconLogo from 'res/img/icon.png';

export default class HeaderLeftHome extends React.PureComponent<{
	style?: ViewStyle;
}> {
	render(): React.ReactElement {
		return (
			<View
				style={[
					{
						alignItems: 'center',
						flexDirection: 'row',
						marginTop: -10,
						paddingLeft: 12
					},
					this.props.style
				]}
			>
				<Image source={iconLogo} style={styles.logo} />
				<Text style={[styles.headerTextLeft, styles.t_bold]}>parity</Text>
				<Text style={styles.headerTextLeft}>signer</Text>
			</View>
		);
	}
}

const styles = StyleSheet.create({
	headerTextLeft: {
		color: colors.bg_text,
		fontFamily: fonts.light,
		fontSize: 14,
		marginRight: 2,
		marginTop: 15
	},
	logo: {
		height: 24,
		width: 24
	},
	t_bold: {
		fontFamily: fonts.semiBold
	}
});
