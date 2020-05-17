// Copyright 2015-2020 Parity Technologies (UK) Ltd.
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
import { StyleSheet, Text } from 'react-native';

import TouchableItem from './TouchableItem';
import Separator from './Separator';

import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { ButtonListener } from 'types/props';

export default class ButtonNewDerivation extends React.PureComponent<{
	onPress: ButtonListener;
	title: string;
	testID?: string;
}> {
	render(): React.ReactElement {
		const { onPress, title, testID } = this.props;
		return (
			<TouchableItem onPress={onPress} testID={testID} style={styles.body}>
				<Separator shadow={true} style={{ backgroundColor: 'transparent' }} />
				<Text style={styles.icon}>//</Text>
				<Text style={styles.textLabel}>{title}</Text>
			</TouchableItem>
		);
	}
}

const styles = StyleSheet.create({
	body: {
		alignItems: 'center',
		backgroundColor: colors.background.app,
		height: 76
	},
	icon: {
		...fontStyles.i_large,
		color: colors.signal.main
	},
	textLabel: {
		...fontStyles.a_text,
		color: colors.text.faded,
		marginTop: 4
	}
});
