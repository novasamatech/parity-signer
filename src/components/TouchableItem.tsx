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

/**
 * TouchableItem renders a touchable that looks native on both iOS and Android.
 *
 * It provides an abstraction on top of TouchableNativeFeedback and
 * TouchableOpacity.
 *
 * On iOS you can pass the props of TouchableOpacity, on Android pass the props
 * of TouchableNativeFeedback.
 */

import React, { ReactElement } from 'react';
import {
	Platform,
	TouchableNativeFeedback,
	TouchableOpacity,
	TouchableOpacityProps,
	View
} from 'react-native';

const ANDROID_VERSION_LOLLIPOP = 21;

interface Props extends TouchableOpacityProps {
	borderless: boolean;
	pressColor: string;
}

export default class TouchableItem extends React.PureComponent<Props> {
	static defaultProps = {
		borderless: false,
		pressColor: 'rgba(0, 0, 0, .32)'
	};

	render(): ReactElement {
		/*
		 * TouchableNativeFeedback.Ripple causes a crash on old Android versions,
		 * therefore only enable it on Android Lollipop and above.
		 *
		 * All touchables on Android should have the ripple effect according to
		 * platform design guidelines.
		 * We need to pass the background prop to specify a borderless ripple effect.
		 */
		if (
			Platform.OS === 'android' &&
			Platform.Version >= ANDROID_VERSION_LOLLIPOP
		) {
			const { style, ...rest } = this.props;
			return (
				<TouchableNativeFeedback
					{...rest}
					style={null}
					background={TouchableNativeFeedback.Ripple(
						this.props.pressColor,
						this.props.borderless
					)}
				>
					<View style={style}>{this.props.children}</View>
				</TouchableNativeFeedback>
			);
		}

		return (
			<TouchableOpacity {...this.props}>{this.props.children}</TouchableOpacity>
		);
	}
}
