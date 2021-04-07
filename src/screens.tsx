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
import { View, Text } from 'react-native';

import { colors, fonts } from 'styles';
import testIDs from 'e2e/testIDs';
import Wallet from 'modules/main/screens/Wallet';
import AddNetwork from 'screens/AddNetwork';
import ShowRecoveryPhrase from 'screens/ShowRecoveryPhrase';
import RenameWallet from 'screens/RenameWallet';
import DeleteWallet from 'screens/DeleteWallet';
import CreateWallet from 'screens/CreateWallet';
import CreateWallet2 from 'screens/CreateWallet2';
import CreateWallet3 from 'screens/CreateWallet3';
import CreateWalletImport from 'screens/CreateWalletImport';
import Settings from 'screens/Settings';
import ReceiveBalance from 'screens/ReceiveBalance';
import SendBalance from 'screens/SendBalance';
import SignTransaction from 'modules/sign/screens/SignTransaction';
import SignTransactionFinish from 'modules/sign/screens/SignTransactionFinish';
import { RootStackParamList } from 'types/routes';

const ScreenStack = createStackNavigator<RootStackParamList>();

const globalStackNavigationOptions = {
	// more transition animations refer to: https://reactnavigation.org/docs/en/stack-navigator.html#animations
	cardStyleInterpolator: CardStyleInterpolators.forHorizontalIOS,
	headerBackTitleStyle: {
	  color: colors.navText.main
	},
	headerBackTitleVisible: false,
	headerStyle: {
		backgroundColor: colors.background.accent,
		borderBottomWidth: 0,
		elevation: 0,
		height: 112,
		shadowColor: 'transparent'
	},
	headerTintColor: colors.text.white,
	headerTitle: (t): React.ReactNode => {
		let title;
		switch (t.children) {
			case 'AddNetwork':
				title = 'Add Network';
				break;
			case 'ShowRecoveryPhrase':
				title = 'Show Key Phrase';
				break;
			case 'RenameWallet':
				title = 'Rename Wallet';
				break;
			case 'DeleteWallet':
				title = 'Delete Wallet';
				break;
			case 'CreateWallet':
				title = 'New Wallet';
				break;
			case 'CreateWallet2':
				title = 'Key Phrase';
				break;
			case 'CreateWallet3':
				title = 'Verify Key Phrase';
				break;
			case 'CreateWalletImport':
				title = 'Import Wallet';
				break;
			case 'Settings':
				title = 'Settings';
				break;
			case 'ReceiveBalance':
				title = 'Receive Balance';
				break;
			case 'SendBalance':
				title = 'Send Balance';
				break;
			case 'SignTransaction':
				title = 'Sign Transaction';
				break;
			case 'SignTransactionFinish':
				title = 'Sign Transaction';
				break;
			default:
				title = t.children;
		}
		return (
			<Text
				style={{
					color: colors.text.white,
					fontFamily: fonts.bold,
					fontSize: 24,
					textAlign: 'left'
				}}
			>
				{title}
			</Text>
		);
	},
	headerTitleAlign: 'left'
};

const HeaderLeft = (): React.ReactElement => {
	const route = useRoute();
	const navigation = useNavigation();
	const isFirstRouteInParent = useNavigationState(
		state => state.routes[0].key === route.key
	);
	return isFirstRouteInParent ? (
		<View style={{ paddingLeft: 10 }}>
			<Text
				style={{
					color: colors.text.white,
					fontFamily: fonts.bold,
					fontSize: 26
				}}
			>
				Layer Wallet
			</Text>
		</View>
	) : (
		<View testID={testIDs.Header.headerBackButton}>
			<HeaderBackButton
				labelStyle={globalStackNavigationOptions.headerBackTitleStyle}
				labelVisible={false}
				tintColor={colors.text.white}
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
		<ScreenStack.Screen
			name="Wallet"
			component={Wallet}
			options={{ animationEnabled: false }}
		/>
		<ScreenStack.Screen name="AddNetwork" component={AddNetwork} />
		<ScreenStack.Screen
			name="ShowRecoveryPhrase"
			component={ShowRecoveryPhrase}
		/>
		<ScreenStack.Screen name="RenameWallet" component={RenameWallet} />
		<ScreenStack.Screen name="DeleteWallet" component={DeleteWallet} />
		<ScreenStack.Screen name="CreateWallet" component={CreateWallet} />
		<ScreenStack.Screen name="CreateWallet2" component={CreateWallet2} />
		<ScreenStack.Screen name="CreateWallet3" component={CreateWallet3} />
		<ScreenStack.Screen
			name="CreateWalletImport"
			component={CreateWalletImport}
		/>
		<ScreenStack.Screen
			name="Settings"
			component={Settings}
			options={{ animationEnabled: false }}
		/>
		<ScreenStack.Screen name="ReceiveBalance" component={ReceiveBalance} />
		<ScreenStack.Screen name="SendBalance" component={SendBalance} />
		<ScreenStack.Screen name="SignTransaction" component={SignTransaction} />
		<ScreenStack.Screen
			name="SignTransactionFinish"
			component={SignTransactionFinish}
		/>
	</ScreenStack.Navigator>
);
