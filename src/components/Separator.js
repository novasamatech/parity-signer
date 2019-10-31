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

import PropTypes from 'prop-types';
import React from 'react';
import { View, ViewPropTypes, Image } from 'react-native';

export default class Separator extends React.PureComponent {
	static propTypes = {
		shadow: PropTypes.bool,
		shadowStyle: ViewPropTypes.style,
		style: ViewPropTypes.style
	};

	render() {
		const { shadow, shadowStyle, style } = this.props;

		return (
			<View
				style={[
					{
						alignSelf: 'stretch',
						backgroundColor: 'black',
						height: 1,
						marginVertical: 8
					},
					style
				]}
			>
				{shadow && (
					<Image
						source={require('../../res/img/card_shadow.png')}
						style={[
							{
								height: 32,
								marginTop: -32,
								width: '100%'
							},
							shadowStyle
						]}
					/>
				)}
			</View>
		);
	}
}
