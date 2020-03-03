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

import React, { ReactElement } from 'react';
import { StyleSheet, View, ViewStyle } from 'react-native';

import Button from './Button';

import { ButtonListener } from 'types/props';

export default class ButtonMainAction extends React.PureComponent<{
	bottom?: boolean;
	disabled?: boolean;
	onPress: ButtonListener;
	testID?: string;
	title: string;
	style?: ViewStyle;
}> {
	render(): ReactElement {
		const { onPress, title, testID, bottom, disabled, style } = this.props;
		const finalViewStyles: ViewStyle[] = [styles.body];
		const finalButtonStyles: ViewStyle[] = [styles.button];

		if (bottom === false) {
			finalViewStyles.push(styles.p_relative);
			finalButtonStyles.push(styles.p_relative);
		}

		return (
			<View style={[finalViewStyles, style]} testID={testID}>
				<Button
					title={title}
					onPress={onPress}
					style={StyleSheet.flatten(finalButtonStyles)}
					disabled={disabled}
				/>
			</View>
		);
	}
}

const styles = StyleSheet.create({
	body: {
		bottom: 0,
		height: 120,
		position: 'absolute',
		width: '100%'
	},
	button: {
		alignSelf: 'center',
		elevation: 2,
		position: 'absolute'
	},
	p_relative: {
		bottom: 0,
		marginTop: 32,
		position: 'relative'
	}
});
