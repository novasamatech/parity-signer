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
import { StyleSheet, Text } from 'react-native';
import { useNavigation } from '@react-navigation/native';
import { StackNavigationProp } from '@react-navigation/stack';
import { Icon } from 'react-native-elements';

import TouchableItem from './TouchableItem';

import colors from 'styles/colors';
import { navigateToQrScanner } from 'utils/navigationHelpers';
import testIDs from 'e2e/testIDs';
import fontStyles from 'styles/fontStyles';
import { RootStackParamList } from 'types/routes';

export default function QrScannerTab(): React.ReactElement {
	const navigation = useNavigation<StackNavigationProp<RootStackParamList>>();

	return (
		<TouchableItem
			onPress={(): void => navigateToQrScanner(navigation)}
			testID={testIDs.SecurityHeader.scanButton}
			style={styles.body}
		>
			<Icon
				color={colors.text.main}
				size={fontStyles.i_large.fontSize}
				name="qrcode-scan"
				type="material-community"
			/>
			<Text style={styles.textLabel}>QR Scanner</Text>
		</TouchableItem>
	);
}

const styles = StyleSheet.create({
	body: {
		alignItems: 'center',
		backgroundColor: colors.background.os,
		borderBottomColor: colors.background.app,
		borderBottomWidth: 1,
		height: 72,
		justifyContent: 'center',
		paddingVertical: 9
	},
	textLabel: {
		...fontStyles.a_text,
		color: colors.text.faded,
		marginTop: 4
	}
});
