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

import {
	useNavigation,
	useNavigationState,
	useRoute
} from '@react-navigation/native';
import {
	CardStyleInterpolators,
	createStackNavigator,
	HeaderBackButton
} from '@react-navigation/stack';
import * as React from 'react';
import { View } from 'react-native';

import PinNew from 'modules/unlock/screens/PinNew';
import PinUnlock from 'modules/unlock/screens/PinUnlock';
import HeaderLeftHome from 'components/HeaderLeftHome';
import testIDs from 'e2e/testIDs';
import Main from 'modules/main/screens/Main';
import IdentityBackup from 'screens/IdentityBackup';
import IdentityManagement from 'screens/IdentityManagement';
import IdentityNew from 'screens/IdentityNew';
import IdentitySwitch from 'screens/IdentitySwitch';
import PathDetails from 'screens/PathDetails';
import QrScanner from 'modules/sign/screens/QrScanner';
import SignedMessage from 'modules/sign/screens/SignedMessage';
import SignedTx from 'modules/sign/screens/SignedTx';
import colors from 'styles/colors';
import { headerHeight } from 'styles/containerStyles';
import { RootStackParamList } from 'types/routes';

export const ScreenStack = createStackNavigator<RootStackParamList>();

const HeaderLeft = (): React.ReactElement => {
	const route = useRoute();
	const isFirstRouteInParent = useNavigationState(
		state => state.routes[0].key === route.key
	);
	return isFirstRouteInParent ? <HeaderLeftHome /> : <HeaderLeftWithBack />;
};

const globalStackNavigationOptions = {
	//more transition animations refer to: https://reactnavigation.org/docs/en/stack-navigator.html#animations
	cardStyleInterpolator: CardStyleInterpolators.forHorizontalIOS,
	headerBackTitleStyle: {
		color: colors.text.main
	},
	headerBackTitleVisible: false,
	headerLeft: (): React.ReactElement => <HeaderLeft />,
	headerLeftContainerStyle: {
		height: headerHeight,
		paddingLeft: 8
	},
	headerStyle: {
		backgroundColor: colors.background.app,
		borderBottomColor: colors.background.app,
		borderBottomWidth: 0,
		elevation: 0,
		height: headerHeight,
		shadowColor: 'transparent'
	},
	headerTintColor: colors.text.main,
	headerTitle: (): React.ReactNode => null
};

const HeaderLeftWithBack = (): React.ReactElement => {
	const navigation = useNavigation();
	return (
		<View testID={testIDs.Header.headerBackButton}>
			<HeaderBackButton
				labelStyle={globalStackNavigationOptions.headerBackTitleStyle}
				labelVisible={false}
				tintColor={colors.text.main}
				onPress={(): void => navigation.goBack()}
			/>
		</View>
	);
};

export const AppNavigator = (): React.ReactElement => (
	<ScreenStack.Navigator
		initialRouteName="Main"
		screenOptions={globalStackNavigationOptions}
	>
		<ScreenStack.Screen name="Main" component={Main} />
		<ScreenStack.Screen name="IdentityBackup" component={IdentityBackup} />
		<ScreenStack.Screen
			name="IdentityManagement"
			component={IdentityManagement}
		/>
		<ScreenStack.Screen name="IdentityNew" component={IdentityNew} />
		<ScreenStack.Screen name="IdentitySwitch" component={IdentitySwitch} />
		<ScreenStack.Screen name="PathDetails" component={PathDetails} />
		<ScreenStack.Screen name="PinNew" component={PinNew} />
		<ScreenStack.Screen name="PinUnlock" component={PinUnlock} />
		<ScreenStack.Screen name="QrScanner" component={QrScanner} />
		<ScreenStack.Screen name="SignedMessage" component={SignedMessage} />
		<ScreenStack.Screen name="SignedTx" component={SignedTx} />
	</ScreenStack.Navigator>
);
