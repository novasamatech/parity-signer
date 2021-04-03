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

import HeaderLeftHome from 'components/HeaderLeftHome';
import testIDs from 'e2e/testIDs';
import Wallet from 'modules/main/screens/Wallet';
import AddNetwork from 'screens/AddNetwork';
import ShowRecoveryPhrase from 'screens/ShowRecoveryPhrase';
import RenameWallet from 'screens/RenameWallet';
import DeleteWallet from 'screens/DeleteWallet';
import CreateWallet from 'screens/CreateWallet';
import Settings from 'screens/Settings';
import ReceiveBalance from 'screens/ReceiveBalance';
import SendBalance from 'screens/SendBalance';
import SignTransaction from 'modules/sign/screens/SignTransaction';
import SignTransactionFinish from 'modules/sign/screens/SignTransactionFinish';
import colors from 'styles/colors';
import { headerHeight } from 'styles/containerStyles';
import { RootStackParamList } from 'types/routes';

const ScreenStack = createStackNavigator<RootStackParamList>();

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
		initialRouteName="Wallet"
		screenOptions={globalStackNavigationOptions}
	>
		<ScreenStack.Screen name="Wallet" component={Wallet} options={{ animationEnabled: false }} />
		<ScreenStack.Screen name="AddNetwork" component={AddNetwork} />
		<ScreenStack.Screen name="ShowRecoveryPhrase" component={ShowRecoveryPhrase} />
		<ScreenStack.Screen name="RenameWallet" component={RenameWallet} />
		<ScreenStack.Screen name="DeleteWallet" component={DeleteWallet} />
		<ScreenStack.Screen name="CreateWallet" component={CreateWallet} />
		<ScreenStack.Screen name="Settings" component={Settings} options={{ animationEnabled: false }} />
		<ScreenStack.Screen name="ReceiveBalance" component={ReceiveBalance} />
		<ScreenStack.Screen name="SendBalance" component={SendBalance} />
		<ScreenStack.Screen name="SignTransaction" component={SignTransaction} />
		<ScreenStack.Screen name="SignTransactionFinish" component={SignTransactionFinish} />
	</ScreenStack.Navigator>
);
