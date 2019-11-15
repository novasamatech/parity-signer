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
import { View } from 'react-native';

export default class ButtonMainAction extends React.PureComponent {
	static propTypes = {
		bottom: PropTypes.bool,
		disabled: PropTypes.bool,
		onPress: PropTypes.func,
		testID: PropTypes.string,
		title: PropTypes.string
	};
	render() {
		const { onPress, title, testID, bottom, disabled, style } = this.props;
		const finalViewStyles = [styles.body];
		const finalButtonStyles = [styles.button];

		if (bottom === false) {
			finalViewStyles.push(styles.p_relative);
			finalButtonStyles.push(styles.p_relative);
		}

		return (
			<View style={[finalViewStyles, style]}>
				<Button
					testID={testID}
					title={title}
					onPress={onPress}
					style={finalButtonStyles}
					disabled={disabled}
				/>
			</View>
		);
	}
}

const styles = {
	body: {
		alignItems: 'center',
		bottom: 40,
		position: 'absolute',
		width: '100%'
	},
	button: {
		elevation: 2,
		position: 'absolute'
	},
	p_relative: {
		bottom: 0,
		marginTop: 32,
		position: 'relative'
	}
};
