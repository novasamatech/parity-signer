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

import TouchableItem from './TouchableItem';
import Separator from './Separator';

import QrScannerTab from 'components/QrScannerTab';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { ButtonListener } from 'types/props';

export default class QRScannerAndDerivationTab extends React.PureComponent<{
	onPress: ButtonListener;
	title: string;
	derivationTestID?: string;
}> {
	render(): React.ReactElement {
		const { onPress, title, derivationTestID } = this.props;
		return (
			<View style={styles.body}>
				<Separator
					shadow={true}
					style={{ backgroundColor: 'transparent', marginVertical: 0 }}
					shadowStyle={{ height: 16, marginTop: -16 }}
				/>
				<View style={styles.tab}>
					<QrScannerTab />
				</View>
				<View style={styles.tab}>
					<TouchableItem
						onPress={onPress}
						style={styles.derivationButton}
						testID={derivationTestID}
					>
						<Text style={styles.icon}>+</Text>
						<Text style={styles.textLabel}>{title}</Text>
					</TouchableItem>
				</View>
			</View>
		);
	}
}

const styles = StyleSheet.create({
	body: { flexDirection: 'row' },
	derivationButton: {
		alignItems: 'center',
		backgroundColor: colors.background.os,
		height: 72
	},
	icon: {
		...fontStyles.i_large,
		color: colors.signal.main,
		fontWeight: 'bold',
		marginTop: 8
	},
	tab: {
		flex: 1,
		flexGrow: 1
	},
	textLabel: {
		...fontStyles.a_text,
		color: colors.text.faded,
		marginTop: 4
	}
});
