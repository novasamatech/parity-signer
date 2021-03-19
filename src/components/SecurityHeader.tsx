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
import { StyleSheet, View } from 'react-native';
import { useNavigation } from '@react-navigation/native';
import { StackNavigationProp } from '@react-navigation/stack';

import ButtonIcon from './ButtonIcon';

import { RootStackParamList } from 'types/routes';
import { navigateToIdentitySwitch } from 'utils/navigationHelpers';
import testIDs from 'e2e/testIDs';

function SecurityHeader(): React.ReactElement {
	const navigation = useNavigation<StackNavigationProp<RootStackParamList>>();
	const routes = navigation.dangerouslyGetState().routes;
	const route = routes[routes.length - 1].name;

	return (
		<View style={styles.body}>
			<ButtonIcon
				onPress={(): void => {
					if (route !== 'IdentitySwitch') navigateToIdentitySwitch(navigation);
				}}
				iconName="user"
				iconType="antdesign"
				iconBgStyle={{ backgroundColor: 'transparent' }}
				testID={testIDs.IdentitiesSwitch.toggleButton}
				style={{ paddingHorizontal: 6 }}
				iconSize={26}
			/>
		</View>
	);
}

const styles = StyleSheet.create({
	body: {
		flexDirection: 'row',
		justifyContent: 'center'
	}
});

export default SecurityHeader;
