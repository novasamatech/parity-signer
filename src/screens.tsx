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
	CardStyleInterpolators,
	HeaderStyleInterpolators,
	createStackNavigator
} from '@react-navigation/stack';
import * as React from 'react';
import { Text, View } from 'react-native';
import Icon from 'react-native-vector-icons/MaterialIcons';

import { colors, fonts } from 'styles/index';
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

const stackNavigationOptions = {
	// more transition animations refer to: https://reactnavigation.org/docs/en/stack-navigator.html#animations
	cardStyle: { backgroundColor: 'transparent' },
	cardStyleInterpolator: CardStyleInterpolators.forHorizontalIOS,
	headerBackImage: (_tintColor): React.ReactNode => {
		return (
			<View
				style={{
					height: 46,
					marginTop: 45,
					padding: 6,
					width: 46
					// backgroundColor: '#ccc'
				}}
			>
				<Icon
					name={'arrow-left'}
					size={32}
					color={colors.text.white}
					style={{ position: 'relative' }}
				/>
			</View>
		);
	},
	headerBackTitleVisible: false,
	headerStyle: {
		backgroundColor: colors.background.accent,
		borderBottomWidth: 0,
		elevation: 0,
		height: 154,
		shadowColor: 'transparent'
	},
	headerStyleInterpolator: HeaderStyleInterpolators.forStatic,
	headerTintColor: colors.text.white,
	headerTitle: (t): React.ReactNode => {
		let title, isRoot;
		switch (t.children) {
			case 'AddNetwork':
				title = 'Select Network';
				break;
			case 'ShowRecoveryPhrase':
				title = 'Show Recovery Phrase';
				break;
			case 'RenameWallet':
				title = 'Rename Wallet';
				break;
			case 'DeleteWallet':
				title = 'Delete Wallet';
				break;
			case 'CreateWallet':
				title = 'Create Wallet';
				break;
			case 'CreateWallet2':
				title = 'Create Wallet';
				break;
			case 'CreateWallet3':
				title = 'Confirm Recovery Phrase';
				break;
			case 'CreateWalletImport':
				title = 'Create Wallet';
				break;
			case 'Settings':
				title = 'Settings';
				isRoot = true;
				break;
			case 'Wallet':
				title = 'Wallet';
				isRoot = true;
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
		console.log(t);
		return (
			<Text
				style={{
					color: colors.text.white,
					fontFamily: fonts.bold,
					fontSize: 24,
					marginHorizontal: isRoot ? 0 : 28,
					paddingTop: 42,
					textAlign: 'left'
				}}
			>
				{title}
			</Text>
		);
	},
	headerTitleAlign: 'left',
	headerTitleContainerStyle: {
		left: 20
	}
};

export const AppNavigator = (): React.ReactElement => (
	<ScreenStack.Navigator
		initialRouteName="Wallet"
		screenOptions={stackNavigationOptions}
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
