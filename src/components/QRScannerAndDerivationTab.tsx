// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Modifications Copyright (c) 2021 Thibaut Sardan

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

import QrScannerTab from 'components/QrScannerTab';
import React from 'react';
import { StyleSheet, Text, View } from 'react-native';
import colors from 'styles/colors';
import fontStyles from 'styles/fontStyles';
import { ButtonListener } from 'types/props';

import Separator from './Separator';
import TouchableItem from './TouchableItem';

export default class QRScannerAndDerivationTab extends React.PureComponent<{
	onPress: ButtonListener;
	title: string;
	derivationTestID?: string;
}> {
	render(): React.ReactElement {
		const { derivationTestID, onPress, title } = this.props;

		return (
			<View style={styles.body}>
				<Separator
					shadow={true}
					shadowStyle={{ height: 16, marginTop: -16 }}
					style={{ backgroundColor: 'transparent', marginVertical: 0 }}
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
