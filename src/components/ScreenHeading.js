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

// @flow

import PropTypes from 'prop-types';
import React from 'react';
import { View, Text } from 'react-native';
import fontStyles from '../fontStyles';
import fonts from '../fonts';

export default class ScreenHeading extends React.PureComponent {
	static propTypes = {
		small: PropTypes.bool,
		title: PropTypes.string
	};
	render() {
		const { title, small } = this.props;
		const finalViewStyles = [styles.body];
		const finalTextStyles = [fontStyles.h1];

		if (small) {
			finalViewStyles.push(styles.bodyS);
			finalTextStyles.push([fontStyles.h2, styles.titleS]);
		}

		return (
			<View style={finalViewStyles}>
				<Text style={finalTextStyles}>{title}</Text>
			</View>
		);
	}
}

const styles = {
	body: {
		alignItems: 'center',
		flexDirection: 'row',
		justifyContent: 'center',
		paddingBottom: 24,
		paddingHorizontal: 16
	},
	bodyS: {
		justifyContent: 'flex-start',
		paddingLeft: 72,
		paddingRight: 16
	},
	titleS: {
		fontFamily: fonts.roboto
	}
};
