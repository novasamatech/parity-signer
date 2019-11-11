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
import Button from './Button';

import { StyleSheet } from 'react-native';
import colors from '../colors';
import fonts from '../fonts';
import fontStyles from '../fontStyles';

export default class ButtonNewDerivation extends React.PureComponent {
	static propTypes = {
		onPress: PropTypes.func
	};
	render() {
		const { onPress, title } = this.props;
		return (
			<Button
				title={title}
				onPress={onPress}
				style={{ position: 'absolute' }}
				textStyles={[fontStyles.h2, styles.text]}
				buttonStyles={styles.button}
			/>
		);
	}
}

const styles = StyleSheet.create({
	button: {
		alignItems: 'center',
		backgroundColor: 'transparent',
		borderColor: colors.card_bgSolid,
		borderRadius: 0,
		borderStyle: 'dashed',
		borderWidth: 1,
		elevation: 0,
		height: 56,
		justifyContent: 'center',
		marginBottom: 120,
		marginHorizontal: 8,
		marginTop: 16,
		paddingHorizontal: 64
	},
	text: {
		fontFamily: fonts.roboto,
		opacity: 0.4
	}
});
