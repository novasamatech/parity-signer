// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// Copyright 2021 Commonwealth Labs, Inc.
// This file is part of Layer Wallet.

// Layer Wallet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Layer Wallet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Layer Wallet. If not, see <http://www.gnu.org/licenses/>.

import React from 'react';
import { StyleSheet, Text, View } from 'react-native';
import { useNavigation, useRoute } from '@react-navigation/native';
import { StackNavigationProp } from '@react-navigation/stack';
import { Icon } from 'react-native-elements';

import TouchableItem from './TouchableItem';

import colors from 'styles/colors';
import { resetNavigationTo } from 'utils/navigationHelpers';
import testIDs from 'e2e/testIDs';
import fontStyles from 'styles/fontStyles';
import { RootStackParamList } from 'types/routes';

export default function NavigationTab(): React.ReactElement {
	const navigation = useNavigation<StackNavigationProp<RootStackParamList>>();
	const route = useRoute();

	return (
		<View style={styles.row}>
			<TouchableItem
				onPress={(): void => resetNavigationTo(navigation, 'Wallet')}
				testID={testIDs.NavigationTab.wallet}
				style={styles.item}
				disabled={route.name === 'Wallet'}
			>
				<Icon
					color={
						route.name === 'Wallet' ? colors.text.main : colors.text.disabled
					}
					size={fontStyles.i_large.fontSize}
					name="account-balance-wallet"
					type="material"
				/>
				<Text
					style={
						route.name === 'Wallet'
							? styles.textLabel
							: styles.disabledTextLabel
					}
				>
					Wallet
				</Text>
			</TouchableItem>
			<TouchableItem
				onPress={(): void => resetNavigationTo(navigation, 'Settings')}
				testID={testIDs.NavigationTab.settings}
				style={styles.item}
				disabled={route.name === 'Settings'}
			>
				<Icon
					color={
						route.name === 'Settings' ? colors.text.main : colors.text.disabled
					}
					size={fontStyles.i_large.fontSize}
					name="settings"
					type="material"
				/>
				<Text
					style={
						route.name === 'Settings'
							? styles.textLabel
							: styles.disabledTextLabel
					}
				>
					Settings
				</Text>
			</TouchableItem>
		</View>
	);
}

const styles = StyleSheet.create({
	disabledTextLabel: {
		...fontStyles.a_text,
		color: colors.text.disabled,
		marginTop: 4
	},
	item: {
		alignItems: 'center',
		alignSelf: 'flex-start',
		backgroundColor: colors.background.os,
		borderBottomColor: colors.background.app,
		borderBottomWidth: 1,
		flexGrow: 1,
		justifyContent: 'center',
		paddingBottom: 30,
		paddingTop: 20
	},
	row: {
		flexDirection: 'row',
		flexWrap: 'wrap'
	},
	textLabel: {
		...fontStyles.a_text,
		color: colors.text.main,
		marginTop: 4
	}
});
