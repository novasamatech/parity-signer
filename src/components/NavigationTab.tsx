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
import { StyleSheet, Text, View } from 'react-native';
import { useNavigation } from '@react-navigation/native';
import { StackNavigationProp } from '@react-navigation/stack';
import { Icon } from 'react-native-elements';

import TouchableItem from './TouchableItem';

import colors from 'styles/colors';
import {
	navigateToMain,
	navigateToNetworkSettings
} from 'utils/navigationHelpers';
import testIDs from 'e2e/testIDs';
import fontStyles from 'styles/fontStyles';
import { RootStackParamList } from 'types/routes';

export default function NavigationTab(): React.ReactElement {
	const navigation = useNavigation<StackNavigationProp<RootStackParamList>>();
	const routes = navigation.dangerouslyGetState().routes;
	const route = routes[routes.length - 1].name;

	return (
		<View style={styles.row}>
			<TouchableItem
				onPress={(): void => navigateToMain(navigation)}
				testID={testIDs.NavigationTab.wallet}
				style={styles.item}
				disabled={route === 'Main'}
			>
				<Icon
					color={route === 'Main' ? colors.text.disabled : colors.text.main}
					size={fontStyles.i_large.fontSize}
					name="account-balance-wallet"
					type="material"
				/>
				<Text
					style={route === 'Main' ? styles.disabledTextLabel : styles.textLabel}
				>
					Wallet
				</Text>
			</TouchableItem>
			<TouchableItem
				onPress={(): void => navigateToNetworkSettings(navigation)}
				testID={testIDs.NavigationTab.settings}
				style={styles.item}
				disabled={route === 'NetworkSettings'}
			>
				<Icon
					color={
						route === 'NetworkSettings'
							? colors.text.disabled
							: colors.text.main
					}
					size={fontStyles.i_large.fontSize}
					name="settings"
					type="material"
				/>
				<Text
					style={
						route === 'NetworkSettings'
							? styles.disabledTextLabel
							: styles.textLabel
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
		height: 72,
		justifyContent: 'center',
		paddingVertical: 9
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
