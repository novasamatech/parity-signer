// Copyright 2015-2019 Parity Technologies (UK) Ltd.
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

import '../shim';
import 'utils/iconLoader';

import * as React from 'react';
import { StatusBar, View, YellowBox } from 'react-native';
import {
	NavigationContainer,
	RouteProp,
	useRoute,useNavigationState,useNavigation
} from '@react-navigation/native';
import {
	CardStyleInterpolators,
	createStackNavigator,
	HeaderBackButton,
	StackNavigationOptions
} from '@react-navigation/stack';
import {RootStackParamList} from 'types/router';
import { Provider as UnstatedProvider } from 'unstated';
import { MenuProvider } from 'react-native-popup-menu';

import Background from 'components/Background';
import colors from 'styles/colors';
import HeaderLeftHome from 'components/HeaderLeftHome';
import SecurityHeader from 'components/SecurityHeader';
import '../ReactotronConfig';
import About from 'screens/About';
import LegacyAccountBackup from 'screens/LegacyAccountBackup';
import AccountDetails from 'screens/AccountDetails';
import AccountEdit from 'screens/AccountEdit';
import AccountNetworkChooser from 'screens/AccountNetworkChooser';
import AccountNew from 'screens/AccountNew';
import AccountPin from 'screens/AccountPin';
import { AccountUnlock, AccountUnlockAndSign } from 'screens/AccountUnlock';
import LegacyAccountList from 'screens/LegacyAccountList';
import Loading from 'screens/Loading';
import IdentityBackup from 'screens/IdentityBackup';
import IdentityManagement from 'screens/IdentityManagement';
import IdentityNew from 'screens/IdentityNew';
import IdentityPin from 'screens/IdentityPin';
import MessageDetails from 'screens/MessageDetails';
import PathDerivation from 'screens/PathDerivation';
import PathDetails from 'screens/PathDetails';
import PathsList from 'screens/PathsList';
import PathManagement from 'screens/PathManagement';
import PrivacyPolicy from 'screens/PrivacyPolicy';
import QrScanner from 'screens/QrScanner';
import Security from 'screens/Security';
import SignedMessage from 'screens/SignedMessage';
import SignedTx from 'screens/SignedTx';
import TermsAndConditions from 'screens/TermsAndConditions';
import TxDetails from 'screens/TxDetails';
import LegacyNetworkChooser from 'screens/LegacyNetworkChooser';
import testIDs from 'e2e/testIDs';
import { AppProps, getLaunchArgs } from 'e2e/injections';

export default function App(props): React.ReactElement<AppProps> {
	getLaunchArgs(props);
	if (__DEV__) {
		YellowBox.ignoreWarnings([
			'Warning: componentWillReceiveProps',
			'Warning: componentWillMount',
			'Warning: componentWillUpdate',
			'Warning: Sending `onAnimatedValueUpdate`'
		]);
	}

	return (
		<UnstatedProvider>
			<MenuProvider backHandler={true}>
				<StatusBar barStyle="light-content" backgroundColor={colors.bg} />
				<Background />
				<ScreensContainer />
			</MenuProvider>
		</UnstatedProvider>
	);
}

const globalStackNavigationOptions = {
	//more transition animations refer to: https://reactnavigation.org/docs/en/stack-navigator.html#animations
	cardStyleInterpolator: CardStyleInterpolators.forHorizontalIOS,
	headerBackTitleStyle: {
		color: colors.bg_text_sec
	},
	headerBackTitleVisible: false,
	headerLeft: (): React.ReactElement => {
		const route = useRoute();
		const isFirstRouteInParent = useNavigationState(
			state => state.routes[0].key === route.key
		);
		return isFirstRouteInParent? (
			<HeaderLeftHome/>
		) : (
			<HeaderLeftWithBack/>
		);
	},
	headerRight: (): React.ReactElement => <SecurityHeader />,
	headerStyle: {
		backgroundColor: colors.bg,
		borderBottomColor: colors.bg,
		borderBottomWidth: 0,
		elevation: 0,
		height: 60,
		shadowColor: 'transparent'
	},
	headerTintColor: colors.bg_text_sec,
	headerTitle: (): React.ReactNode => null
};

const HeaderLeftWithBack = () => {
	const {navigation} = useNavigation();
	return (
		<View
			style={{flexDirection: 'row'}}
			testID={testIDs.Header.headerBackButton}
		>
			<HeaderBackButton
				{...this.props}
				labelStyle={
					globalStackNavigationOptions.headerBackTitleStyle
				}
				labelVisible={false}
				tintColor={colors.bg_text}
				onPress={(): boolean => navigation.goBack()}
			/>
		</View>
	);
};

/* eslint-disable sort-keys */
const tocAndPrivacyPolicyScreens = {
	TermsAndConditions: {
		options: { headerRight: (): React.ReactNode => null },
		screen: TermsAndConditions
	},
	PrivacyPolicy: {
		options: { headerRight: (): React.ReactNode => null },
		screen: PrivacyPolicy
	}
};

const ScreenStack = createStackNavigator<RootStackParamList>();

const TocAndPrivacyPolicyScreens = () => (
	<>
		<ScreenStack.Screen
			name="TermsAndConditions"
			component={TermsAndConditions}
			options={{ headerRight: (): React.ReactNode => null }}
		/>
		<ScreenStack.Screen
			name="PrivacyPolicy"
			component={PrivacyPolicy}
			options={{ headerRight: (): React.ReactNode => null }}
		/>
	</>
);

const ToCStacks = () => {
};

const ScreenStacks = () => (
	<ScreenStack.Navigator
		initialRouteName="Screen"
		screenOptions={}
	>
		{
			<ScreenStack.Screen
				name="AccountNetworkChooser"
				component={AccountNetworkChooser}
			/>
			< ScreenStack.Screen
			name="AccountPin"
			component={AccountPin}
			/>
			<ScreenStack.Screen
			name="AccountUnlock"
			component={AccountUnlock}
			/>
			<ScreenStack.Screen
			name="About"
			component={About}
			/>
			<ScreenStack.Screen
			name="AccountDetails"
			component={AccountDetails}
			/>
			<ScreenStack.Screen
			name="AccountEdit"
			component={AccountEdit}
			/>
			<ScreenStack.Screen
			name="AccountNew"
			component={AccountNew}
			/>
			<ScreenStack.Screen
			name="AccountUnlockAndSign"
			component={AccountUnlockAndSign}
			/>
			<ScreenStack.Screen
			name="LegacyAccountBackup"
			component={LegacyAccountBackup}
			/>
			<ScreenStack.Screen
			name="LegacyAccountList"
			component={LegacyAccountList}
			/>
			<ScreenStack.Screen
			name="LegacyNetworkChooser"
			component={LegacyNetworkChooser}
			/>
			<ScreenStack.Screen
			name="IdentityBackup"
			component={IdentityBackup}
			/>
			<ScreenStack.Screen
			name="IdentityManagement"
			component={IdentityManagement}
			/>
			<ScreenStack.Screen
			name="IdentityNew"
			component={IdentityNew}
			/>
			<ScreenStack.Screen
			name="IdentityPin"
			component={IdentityPin}
			/>
			<ScreenStack.Screen
			name="MessageDetails"
			component={MessageDetails}
			/>
			<ScreenStack.Screen
			name="PathDerivation"
			component={PathDerivation}
			/>
			<ScreenStack.Screen
			name="PathDetails"
			component={PathDetails}
			/>
			<ScreenStack.Screen
			name="PathsList"
			component={PathsList}
			/>
			<ScreenStack.Screen
			name="PathManagement"
			component={PathManagement}
			/>
			<ScreenStack.Screen
			name="QrScanner"
			component={QrScanner}
			/>
			<ScreenStack.Screen
			name="SignedMessage"
			component={SignedMessage}
			/>
			<ScreenStack.Screen
			name="SignedTx"
			component={SignedTx}
			/>
			<ScreenStack.Screen
			name="TxDetails"
			component={TxDetails}
			/>
		{TocAndPrivacyPolicyScreens}
		}
	</ScreenStack.Navigator>
);

const Screens = createSwitchNavigator(
	{
		Loading: {
			screen: Loading
		},
		TocAndPrivacyPolicy: createStackNavigator(tocAndPrivacyPolicyScreens, {
			defaultNavigationOptions: globalStackNavigationOptions
		}),
		Welcome: {
			screen: createStackNavigator(
				{
					AccountNetworkChooser: {
						screen: AccountNetworkChooser
					},
					AccountPin: {
						screen: AccountPin
					},
					AccountUnlock: {
						screen: AccountUnlock
					},
					About: {
						screen: About
					},
					AccountDetails: {
						screen: AccountDetails
					},
					AccountEdit: {
						screen: AccountEdit
					},
					AccountNew: {
						screen: AccountNew
					},
					AccountUnlockAndSign: {
						screen: AccountUnlockAndSign
					},
					LegacyAccountBackup: {
						screen: LegacyAccountBackup
					},
					LegacyAccountList: {
						screen: LegacyAccountList
					},
					LegacyNetworkChooser: {
						screen: LegacyNetworkChooser
					},
					IdentityBackup: {
						screen: IdentityBackup
					},
					IdentityManagement: {
						screen: IdentityManagement
					},
					IdentityNew: {
						screen: IdentityNew
					},
					IdentityPin: {
						screen: IdentityPin
					},
					MessageDetails: {
						screen: MessageDetails
					},
					PathDerivation: {
						screen: PathDerivation
					},
					PathDetails: {
						screen: PathDetails
					},
					PathsList: {
						screen: PathsList
					},
					PathManagement: {
						screen: PathManagement
					},
					QrScanner: {
						screen: QrScanner
					},
					SignedMessage: {
						screen: SignedMessage
					},
					SignedTx: {
						screen: SignedTx
					},
					TxDetails: {
						screen: TxDetails
					},
					Security: {
						navigationOptions: {
							headerRight: (): React.ReactNode => null
						},
						screen: Security
					},
					...tocAndPrivacyPolicyScreens
				},
				{
					defaultNavigationOptions: globalStackNavigationOptions
				}
			)
		}
	},
	{
		defaultNavigationOptions: globalStackNavigationOptions
	}
);

const ScreensContainer = createAppContainer(Screens);
